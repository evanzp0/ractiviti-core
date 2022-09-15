use std::sync::Arc;
use sqlx::{Postgres, Transaction};
use crate::ArcRw;
use crate::manager::engine::{BaseOperator, BpmnElement, ContinueProcessOperator, NodeType,
                             OperateRst, Operator, OperatorContext, ServiceTaskBehavior, UserTaskBehavior};
use crate::model::{ApfRuExecution, ApfRuTask};
use color_eyre::Result;
use crate::dao::{ApfHiTaskinstDao, ApfRuIdentitylinkDao, ApfRuTaskDao};

#[derive(Debug)]
pub struct CompleteTaskCmd {
    base: BaseOperator,
}

impl CompleteTaskCmd {

    pub fn new(element: BpmnElement, proc_inst: Arc<ApfRuExecution>, current_exec: Option<ArcRw<ApfRuExecution>>,
               current_task: Option<Arc<ApfRuTask>>) -> Self {
        Self {
            base: BaseOperator::new(proc_inst, current_exec, element, None, current_task),
        }
    }

    pub async fn execute<'a> (&self, operator_ctx: &mut OperatorContext, tran: &mut Transaction<'a, Postgres>)
          -> Result<OperateRst> {
        let task = self.base.current_task_ex()?;

        self.base.check_complete_task_priviledge(task.clone(),
                                                 &self.base.element,
                                                 operator_ctx,
                                                 tran).await?;

        // execute behavior and mark end
        self.execute_behavior(task.clone(), operator_ctx, tran).await?;

        // update task history
        let hi_task_dao = ApfHiTaskinstDao::new();
        hi_task_dao.mark_end(&task.id, operator_ctx.user_id.clone(), tran).await?;

        // delete user and group data from ru_identity_link
        let ru_ident_dao = ApfRuIdentitylinkDao::new();
        ru_ident_dao.delete_by_task_id(&task.id, tran).await?;

        // delete runtime task
        let task_dao = ApfRuTaskDao::new();
        task_dao.delete(&task.id, tran).await?;

        // continue to next operator
        if operator_ctx.is_terminated()? {

            // break current flow and terminate to the endEvent_terminate
            let bpmn_process = operator_ctx.bpmn_process_ex()?;
            let end_event_terminate = bpmn_process.end_event_terminate_node_ex()?;

            let continue_operator = ContinueProcessOperator::new(
                end_event_terminate,
                Some(self.base.element.clone()),
                self.base.proc_inst.clone(),
                self.base.current_exec(),
                None);
            operator_ctx.queue.write().unwrap().push(Operator::ContinueProcessOperator(continue_operator));
        } else {
            self.base.continue_outflow(operator_ctx, tran).await?;
        }

        Ok(OperateRst::default())
    }

    async fn execute_behavior<'a>(&self, task: Arc<ApfRuTask>, operator_ctx: &mut OperatorContext, tran: &mut Transaction<'a, Postgres>)
            -> Result<()> {
        match &self.base.element {
            BpmnElement::Edge(_) => {},
            BpmnElement::Node(node) => {
                match node.get_node_type() {
                    NodeType::UserTask => {
                        let behaivor = UserTaskBehavior::new(
                            self.base.element.clone(),
                            self.base.proc_inst.clone(),
                            self.base.current_exec(),
                            Some(task));
                        behaivor.execute(operator_ctx, tran).await?;
                    },
                    NodeType::ServiceTask => {
                        let behaivor = ServiceTaskBehavior::new(
                            self.base.element.clone(),
                            self.base.proc_inst.clone(),
                            self.base.current_exec(),
                            Some(task));
                        behaivor.execute(operator_ctx, tran).await?;
                    },
                    _ => {}
                }
            },
        }

        Ok(())
    }
}