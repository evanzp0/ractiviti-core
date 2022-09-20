use std::sync::{Arc, RwLock};

use color_eyre::Result;
use log4rs::debug;
use tokio_postgres::Transaction;

use crate::{ArcRw, get_now};
use crate::dao::ApfRuExecutionDao;
use crate::error::{AppError, ErrorCode};
use crate::service::engine::{
    BaseOperator, BpmnElement, OperateRst, Operator, OperatorContext, TakeOutgoingFlowsOperator
};
use crate::model::{ApfRuExecution, ApfRuTask};

pub struct ParallelGatewayBehavior {
    base: BaseOperator,
}

impl ParallelGatewayBehavior {
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

    pub async fn execute(&self, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>) -> Result<OperateRst> {
        // merge all in_flows
        let bpmn_process = operator_ctx.bpmn_process_ex()?;
        if let BpmnElement::Node(node) = &self.base.element {
            let in_flows_count = node.in_flows(&bpmn_process).len() as i64;

            let exec_dao = ApfRuExecutionDao::new(tran);
            let proc_inst_id = self.base.current_excution_ex()?.read().unwrap().proc_inst_id()?;
            let element_id = self.base.current_excution_ex()?.read().unwrap().element_id()?;

            let inactive_exec_count = exec_dao.count_inactive_by_element(&proc_inst_id, &element_id).await?;

            if inactive_exec_count + 1 < in_flows_count {
                let current_execution = self.base.current_excution_ex()?;
                let current_execution_id = &current_execution.read().unwrap().id;

                exec_dao.deactive_execution(current_execution_id).await?;
            } else {
                exec_dao.del_inactive_by_element(&proc_inst_id, &element_id).await?;

                // #[cfg(debug_assertions)]
                debug!("ParallelGateway (process: {:?}, element: {})", self.base.proc_inst.id, self.base.element.get_element_id());

                // create execution history for the element
                self.base.create_hi_actinst(None, tran).await?; // 多创建了一次

                // only all flows have been merged then it can leave
                self.leave(operator_ctx, tran).await?;
            }
        }

        Ok(OperateRst::default())
    }

    async fn leave<'a>(&self, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>) -> Result<OperateRst> {
        self.base.mark_end_execution(operator_ctx, tran).await?;

        let element = &self.base.element;
        let bpmn_process = operator_ctx.bpmn_process_ex()?;

        if let BpmnElement::Node(node) = element {
            let out_flows = node.out_flows(&*bpmn_process);
            let proc_inst = &self.base.proc_inst;

            if out_flows.is_empty() {
                Err(
                    AppError::new(
                        ErrorCode::NotFound,
                        Some(&format!("not found valid outflows for parallel gateway (proc_inst: {:?}, element: {})", proc_inst.id, node.get_id())),
                        concat!(file!(), ":", line!()),
                        None
                    )
                )?
            }

            // reuse exists current excution for first edge
            let mut outgoing_operators = vec![];

            let start_time = get_now();
            let first_flow = out_flows
                .get(0)
                .ok_or(AppError::new(ErrorCode::NotFound, Some("unexpected error"), "", None))?;

            self.base.mark_begin_exection(&first_flow.get_id(), operator_ctx.user_id.clone(), start_time.clone(), tran).await?;

            let next_operator = TakeOutgoingFlowsOperator::new(
                BpmnElement::Edge(first_flow.clone()),
                self.base.proc_inst.clone(),
                Some(self.base.current_excution_ex()?)
            );
            outgoing_operators.push(next_operator);

            // create new excution for others edge
            for flow in &out_flows[1..] {
                let current_exec = self.base.create_current_execution(
                    &flow.get_id(),
                    start_time.clone(),
                    operator_ctx.user_id.clone(), tran
                )
                .await?;
                let current_exec = Arc::new(RwLock::new(current_exec));
                let next_operator = TakeOutgoingFlowsOperator::new(
                    BpmnElement::Edge(flow.clone()),
                    self.base.proc_inst.clone(),
                    Some(current_exec)
                );

                outgoing_operators.push(next_operator);
            }

            // continue to handle the outflow
            for next_operator in outgoing_operators {
                operator_ctx.queue.write().unwrap().push(Operator::TakeOutgoingFlowsOperator(next_operator));
            }
        }

        Ok(OperateRst::default())
    }
}