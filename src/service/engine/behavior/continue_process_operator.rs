use std::sync::Arc;

use color_eyre::Result;
use tokio_postgres::Transaction;
use crate::ArcRw;
use crate::error::{AppError, ErrorCode};
use crate::model::{ApfRuExecution, ApfRuTask};
use crate::service::engine::{
    BaseOperator, BpmnElement, CreateTaskCmd, EndEventBehavior, ExclusiveGatewayBehavior, 
    NodeType, OperateRst, Operator, OperatorContext, ParallelGatewayBehavior, 
    StartEventBehavior
};


#[derive(Debug)]
pub struct ContinueProcessOperator {
    base: BaseOperator,
}

impl ContinueProcessOperator {
    pub fn new(
        element: BpmnElement, 
        terminate_element: Option<BpmnElement>, 
        proc_inst: Arc<ApfRuExecution>,
        current_exec: Option<ArcRw<ApfRuExecution>>, 
        current_task: Option<Arc<ApfRuTask>>
    ) -> Self {
        Self {
            base: BaseOperator::new(proc_inst, current_exec, element, terminate_element, current_task),
        }
    }

    pub async fn execute<'a> (&self, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>) -> Result<OperateRst> 
    {
        match &self.base.element {
            BpmnElement::Edge(edge) => {
                Err(AppError::new(ErrorCode::NotSupportError,
                        Some(&format!("can not handle flow({}) in ContinueProcessOperator", edge.get_id())),
                        concat!(file!(), ":", line!()),
                        None))?
            },
            BpmnElement::Node(node) => match node.get_node_type() {
                NodeType::StartEvent => {
                    let mut behavior = StartEventBehavior::new(
                        self.base.element.clone(),
                        self.base.proc_inst.clone(),
                        None,
                        None);
                    behavior.execute(operator_ctx, tran).await?;
                },
                NodeType::EndEvent => {
                    let behavior = EndEventBehavior::new(
                        self.base.element.clone(),
                        self.base.terminate_element.clone(),
                        self.base.proc_inst.clone(),
                        self.base.current_exec(),
                        self.base.current_task.clone());
                    behavior.execute(operator_ctx, tran).await?;
                },
                NodeType::UserTask => {
                    self.create_task(operator_ctx);
                },
                NodeType::ServiceTask => {
                    self.create_task(operator_ctx);
                },
                NodeType::ExclusiveGateway => {
                    let behavior = ExclusiveGatewayBehavior::new(
                        self.base.element.clone(),
                        self.base.proc_inst.clone(),
                        self.base.current_exec(),
                        self.base.current_task.clone());
                    behavior.execute(operator_ctx, tran).await?;
                },
                NodeType::ParallelGateway => {
                    let behavior = ParallelGatewayBehavior::new(
                        self.base.element.clone(),
                        self.base.proc_inst.clone(),
                        self.base.current_exec(),
                        self.base.current_task.clone());
                    behavior.execute(operator_ctx, tran).await?;
                }
            },
        };

        Ok(OperateRst::default())
    }

    fn create_task(&self, operator_ctx: &mut OperatorContext) {
        let next_operator = CreateTaskCmd::new(
            self.base.element.clone(),
            self.base.proc_inst.clone(),
            self.base.current_exec());
        operator_ctx.queue.push(Operator::CreateTaskCmd(next_operator));
    }
}