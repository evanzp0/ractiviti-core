use std::sync::Arc;
use color_eyre::Result;
use log4rs::debug;
use sqlx::{Postgres, Transaction};
use crate::ArcRw;
use crate::manager::engine::{BaseOperator, BpmnElement, OperatorContext};
use crate::model::{ApfRuExecution, ApfRuTask};

pub struct ServiceTaskBehavior {
    base: BaseOperator,
}

impl ServiceTaskBehavior {
    pub fn new(element: BpmnElement, proc_inst: Arc<ApfRuExecution>, current_exec: Option<ArcRw<ApfRuExecution>>,
               current_task: Option<Arc<ApfRuTask>>) -> Self {
        Self {
            base: BaseOperator::new(proc_inst, current_exec, element, None, current_task),
        }
    }

    pub async fn execute<'a>(&self, operator_ctx: &mut OperatorContext, tran: &mut Transaction<'a, Postgres>)
            -> Result<()> {
        let task = self.base.current_task_ex()?;

        // #[cfg(debug_assertions)]
        debug!("ServiceTask (process: {:?}, element: {}, name: {}, fromKey: {}, task_id: {}) is executed",
            task.proc_inst_id,
            self.base.element.get_element_id(),
            task.element_name.clone().unwrap_or("?".to_owned()),
            task.form_key.clone().unwrap_or("?".to_owned()),
            task.id);

        self.leave(operator_ctx, tran).await
    }

    pub async fn leave<'a>(&self, operator_ctx: &mut OperatorContext, tran: &mut Transaction<'a, Postgres>)
                           -> Result<()>  {
        self.base.mark_end_execution(operator_ctx, tran).await
    }
}