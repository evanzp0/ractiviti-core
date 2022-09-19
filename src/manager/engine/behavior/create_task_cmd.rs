use std::sync::Arc;

use color_eyre::Result;
use tokio_postgres::Transaction;

use crate::{ArcRw, get_now};
use crate::dao::{ApfHiIdentitylinkDao, ApfHiTaskinstDao, ApfRuIdentitylinkDao, ApfRuTaskDao};
use crate::model::{ApfRuExecution, IdentType, NewApfRuIdentitylink, NewApfRuTask};
use crate::manager::engine::{BaseOperator, BpmnElement, CompleteTaskCmd, NodeType, OperateRst, Operator, OperatorContext};

#[derive(Debug)]
pub struct CreateTaskCmd {
    base: BaseOperator,
}

impl CreateTaskCmd {
    pub fn new(element: BpmnElement, proc_inst: Arc<ApfRuExecution>, current_exec: Option<ArcRw<ApfRuExecution>>) -> Self {
        Self {
            base: BaseOperator::new(proc_inst, current_exec, element, None, None),
        }
    }

    pub async fn execute<'a> (&self, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>) -> Result<OperateRst> {
        let proc_inst= &self.base.proc_inst;
        let current_exec = self.base.current_excution_ex()?;
        let element = &self.base.element;

        // create task
        let now = Some(get_now());
        let new_ru_task = NewApfRuTask {
            rev: 1,
            suspension_state: 0,
            create_time: now.clone(),
            execution_id: current_exec.read().unwrap().id.clone(),
            proc_inst_id: current_exec.read().unwrap().proc_inst_id()?,
            proc_def_id: current_exec.read().unwrap().proc_def_id.clone(),
            element_id: Some(current_exec.read().unwrap().element_id()?),
            element_name: element.get_element_name(),
            element_type: Some(element.get_element_type()),
            business_key: proc_inst.business_key.clone(),
            description: element.get_description(),
            start_user_id: operator_ctx.user_id.clone(),
            form_key: element.get_from_key(),
        };

        let task_dao = ApfRuTaskDao::new(tran);
        let task = task_dao.create(&new_ru_task).await?;

        // create task history
        let hi_task_dao = ApfHiTaskinstDao::new(tran);
        hi_task_dao.create_from_task(&task).await?;

        // create execution history
        self.base.create_hi_actinst(Some(task.id.clone()), tran).await?;

        // handle candidate users and groups
        if let BpmnElement::Node(node) = &self.base.element {
            let ru_ident_dao = ApfRuIdentitylinkDao::new(tran);
            let hi_ident_dao = ApfHiIdentitylinkDao::new(tran);

            match node.get_node_type() {
                NodeType::UserTask | NodeType::ServiceTask => {
                    for group in node.candidate_groups().iter() {
                        let new_ru_ident = NewApfRuIdentitylink {
                            ident_type: IdentType::group,
                            group_id: Some(group.to_owned()),
                            user_id: None,
                            task_id: Some(task.id.clone()),
                            proc_inst_id: Some(task.proc_inst_id.clone()),
                            proc_def_id: Some(task.proc_def_id.clone()),
                        };
                        let ru_ident = ru_ident_dao.create(&new_ru_ident).await?;
                        hi_ident_dao.create_from_ident_link(&ru_ident).await?;
                    }

                    for user in node.candidate_users().iter() {
                        let new_ru_ident = NewApfRuIdentitylink {
                            ident_type: IdentType::group,
                            group_id: None,
                            user_id: Some(user.to_owned()),
                            task_id: Some(task.id.clone()),
                            proc_inst_id: Some(task.proc_inst_id.clone()),
                            proc_def_id: Some(task.proc_def_id.clone()),
                        };
                        let ru_ident = ru_ident_dao.create(&new_ru_ident).await?;
                        hi_ident_dao.create_from_ident_link(&ru_ident).await?;
                    }
                }
                _ => {}
            }
        }

        // continue to handle service task
        if let BpmnElement::Node(node) = &self.base.element {
            if node.get_node_type() == NodeType::ServiceTask {
                let continue_operator = CompleteTaskCmd::new(
                    self.base.element.clone(),
                    proc_inst.clone(),
                    self.base.current_exec(),
                    Some(Arc::new(task))
                );

                operator_ctx.queue.write().unwrap().push(Operator::CompleteTaskCmd(continue_operator));
            }
        }

        Ok(OperateRst::default())
    }

}