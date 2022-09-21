use std::rc::Rc;

use color_eyre::Result;
use log4rs_macros::debug;
use tokio_postgres::Transaction;

use crate::{RcRefCell, get_now};
use crate::service::engine::{BaseOperator, BpmnElement, OperatorContext};
use crate::model::{ApfRuExecution, ApfRuTask};

pub struct StartEventBehavior {
    base: BaseOperator,
}

impl StartEventBehavior {
    pub fn new(
        element: BpmnElement, 
        proc_inst: Rc<ApfRuExecution>, 
        current_exec: Option<RcRefCell<ApfRuExecution>>, 
        current_task: Option<Rc<ApfRuTask>>
    ) -> Self {
        Self {
            base: BaseOperator::new(proc_inst, current_exec, element, None, current_task),
        }
    }

    pub async fn execute<'a>(&mut self, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>) -> Result<()> {
        // create current execution
        let start_time= get_now();
        let start_user = operator_ctx.user_id.clone();
        let current_execution = self.base
            .create_current_execution(
                &self.base.element.get_element_id(), 
                start_time, 
                start_user, 
                tran
            )
            .await?;

        // update current execution in the base
        self.base.set_current_exec(current_execution.clone());

        // create execution history
        self.base.create_hi_actinst(None, tran).await?;

        // #[cfg(debug_assertions)]
        debug!("StartEvent (process: {:?}, element: {}) is executed", self.base.proc_inst.id, self.base.element.get_element_id());

        self.leave(operator_ctx, tran).await
    }

    pub async fn leave<'a>(&self, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>) -> Result<()>  {
        self.base.mark_end_execution(operator_ctx, tran).await?;
        self.base.continue_outflow(operator_ctx, tran).await?;

        Ok(())
    }
}