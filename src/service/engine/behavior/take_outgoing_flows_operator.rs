use std::rc::Rc;

use color_eyre::Result;
use log4rs_macros::debug;
use tokio_postgres::Transaction;

use crate::service::engine::{BaseOperator, BpmnElement, ContinueProcessOperator, OperateRst, Operator, OperatorContext};
use crate::model::ApfRuExecution;
use crate::{get_now, RcRefCell};
use crate::error::{AppError, ErrorCode};

#[derive(Debug)]
pub struct TakeOutgoingFlowsOperator {
    base: BaseOperator,
}

impl TakeOutgoingFlowsOperator {
    pub fn new(element: BpmnElement, proc_inst: Rc<ApfRuExecution>, current_exec: Option<RcRefCell<ApfRuExecution>>) -> Self {
        Self {
            base: BaseOperator::new(proc_inst, current_exec, element, None, None),
        }
    }

    pub async fn execute<'a> (&self, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>) -> Result<OperateRst> {

        match &self.base.element {
            BpmnElement::Edge(edge) => {
                let target_id = edge.get_target();
                let bpmn_process = operator_ctx.bpmn_process_ex()?;
                let target_element = bpmn_process.element_map
                    .get(&target_id)
                    .ok_or(
                        AppError::new(
                            ErrorCode::NotFound,
                            Some(&format!("target node({}) is not exist", target_id)),
                            concat!(file!(), ":", line!()),
                            None
                        )
                    )?;

                // #[cfg(debug_assertions)]
                debug!("Sequence Flow (process: {:?}, element: {}, source: {}, target: {})", self.base.proc_inst.id, edge.get_id(), edge.get_source(), target_id);

                // handle target node
                if let BpmnElement::Node(node) = target_element {
                    // set element id for current exection
                    self.base.mark_begin_exection(&node.get_id(), operator_ctx.user_id.clone(), get_now(), tran).await?;

                    let continue_operator = ContinueProcessOperator::new(
                        BpmnElement::Node(node.clone()),
                        None,
                        self.base.proc_inst.clone(),
                        self.base.current_exec(),
                        None
                    );

                    operator_ctx.queue.push(Operator::ContinueProcessOperator(continue_operator));
                }
            },
            BpmnElement::Node(node) => {
                Err(
                    AppError::new(
                        ErrorCode::NotSupportError,
                        Some(&format!("can not handle the edge({}), wrong element type", node.get_id())),
                        concat!(file!(), ":", line!()),
                        None
                    )
                )?
            }
        }

        Ok(OperateRst::default())
    }
}