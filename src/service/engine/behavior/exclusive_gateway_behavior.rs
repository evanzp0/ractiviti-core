use std::sync::Arc;

use color_eyre::Result;
use log4rs::{debug, error};
use tokio_postgres::Transaction;

use crate::{ArcRw, get_now};
use crate::error::{AppError, ErrorCode};
use crate::service::engine::{
    BaseOperator, BpmnElement, convert_map, Operator, OperatorContext, 
    run_script, TakeOutgoingFlowsOperator
};
use crate::model::{ApfRuExecution, ApfRuTask};

pub struct ExclusiveGatewayBehavior {
    base: BaseOperator,
}

impl ExclusiveGatewayBehavior {
    pub fn new(
        element: BpmnElement, 
        proc_inst: Arc<ApfRuExecution>, 
        current_exec: Option<ArcRw<ApfRuExecution>>, 
        current_task: Option<Arc<ApfRuTask>>
    ) -> Self {
        Self {
            base: BaseOperator::new(proc_inst, current_exec, element, None, current_task),
        }
    }

    pub async fn execute<'a>(&self, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>) -> Result<()> {
        debug!("ExclusiveGateway (process: {:?}, element: {})", self.base.proc_inst.id, self.base.element.get_element_id());

        self.base.create_hi_actinst(None, tran).await?;
        self.leave(operator_ctx, tran).await
    }

    pub async fn leave(&self, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>)-> Result<()>  {
        self.base.mark_end_execution(operator_ctx, tran).await?;

        let element = &self.base.element;
        let bpmn_process = operator_ctx.bpmn_process_ex()?;
        let data_map = &operator_ctx.variables;

        if let BpmnElement::Node(node) = element {
            let out_flows = node.out_flows(&*bpmn_process);
            let proc_inst = &self.base.proc_inst;
            let mut default_flow = None;
            let mut out_flow = None;
            for flow in out_flows {
                if let Some(expr) = flow.get_condition_expr() {
                    let js_global_vars = convert_map(&data_map);
                    let rst = run_script(expr, &js_global_vars);
                    match rst {
                        Ok(v) => {
                            let rst = v.as_boolean().unwrap();
                            if rst == true {
                                out_flow = Some(flow.clone());

                                break;
                            }
                        }
                        Err(_) => {
                            error!("process({:?}) flow({}) condition's result value is not boolean!", proc_inst.id, flow.get_id());
                        }
                    }
                } else {
                    default_flow = Some(flow.clone());
                }
            }

            // choose outflow
            if let None = out_flow {
                if let Some(_) = default_flow {
                    out_flow = default_flow;
                }
            }

            if let Some(flow) = &out_flow {
                let element_id = flow.get_id();
                let element = BpmnElement::Edge(flow.clone());

                // set element id for current exection
                self.base.mark_begin_exection(&element_id, operator_ctx.user_id.clone(), get_now(), tran).await?;

                // continue to handle the outflow
                let next_operator = TakeOutgoingFlowsOperator::new(
                    element, self.base.proc_inst.clone(), self.base.current_exec());
                operator_ctx.queue.push(Operator::TakeOutgoingFlowsOperator(next_operator));
            } else {
                Err(
                    AppError::new(
                        ErrorCode::NotFound,
                        Some(&format!("not found valid outflow for exclusive gateway (proc_inst: {:?}, element: {})", proc_inst.id, node.get_id())),
                        concat!(file!(), ":", line!()),
                        None
                    )
                )?
            }
        }

        Ok(())
    }
}