use std::sync::Arc;

use color_eyre::Result;
use log4rs::debug;
use tokio_postgres::Transaction;

use crate::ArcRw;
use crate::service::engine::{BaseOperator, BpmnElement, OperatorContext};
use crate::model::{ApfRuExecution, ApfRuTask};

pub struct UserTaskBehavior {
    base: BaseOperator,
}

impl UserTaskBehavior {
    pub fn new(element: BpmnElement, proc_inst: Arc<ApfRuExecution>, current_exec: Option<ArcRw<ApfRuExecution>>, current_task: Option<Arc<ApfRuTask>>) -> Self {
        Self {
            base: BaseOperator::new(proc_inst, current_exec, element, None, current_task),
        }
    }

    pub async fn execute(&self, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>) -> Result<()> {
        let task = self.base.current_task_ex()?;

        // #[cfg(debug_assertions)]
        debug!("UserTask (process: {:?}, element: {}, name: {}, fromKey: {}, task_id: {}) is executed",
            task.proc_inst_id,
            self.base.element.get_element_id(),
            task.element_name.clone().unwrap_or("?".to_owned()),
            task.form_key.clone().unwrap_or("?".to_owned()),
            task.id);

        self.leave(operator_ctx, tran).await
    }

    pub async fn leave(&self, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>) -> Result<()>  {
        self.base.mark_end_execution(operator_ctx, tran).await
    }
}