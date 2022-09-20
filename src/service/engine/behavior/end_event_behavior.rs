use std::rc::Rc;

use color_eyre::Result;
use log4rs::debug;
use tokio_postgres::Transaction;

use crate::{RcRefCell, get_now};
use crate::service::engine::{BaseOperator, BpmnElement, OperatorContext};
use crate::dao::{ApfHiProcinstDao, ApfRuExecutionDao, ApfRuVariableDao};
use crate::model::{ApfRuExecution, ApfRuTask};

pub struct EndEventBehavior {
    base: BaseOperator,
}

impl EndEventBehavior {
    pub fn new(
        element: BpmnElement, 
        terminate_element: Option<BpmnElement>, 
        proc_inst: Rc<ApfRuExecution>, 
        current_exec: Option<RcRefCell<ApfRuExecution>>,
        current_task: Option<Rc<ApfRuTask>>
    ) -> Self {
        Self {
            base: BaseOperator::new(proc_inst, current_exec, element, terminate_element, current_task),
        }
    }

    pub async fn execute<'a>(&self, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>) -> Result<()> {
        debug!("EndEvent (process: {:?}, element: {}) is executed",
            self.base.proc_inst.id,
            self.base.element.get_element_id());

        if let None = self.base.terminate_element {
            self.base.create_hi_actinst(None, tran).await?;
        }

        self.leave(operator_ctx, tran).await
    }

    pub async fn leave<'a>(&self, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>) -> Result<()> {
        // mark end of current execution
        if let None = self.base.terminate_element {
            self.base.mark_end_execution(operator_ctx, tran).await?;
        }

        let current_execution = self.base.current_excution_ex()?;
        let procinst_id = current_execution.borrow().proc_inst_id()?;

        // delete current variable
        let var_dao = ApfRuVariableDao::new(tran);
        var_dao.delete_by_proc_inst_id(&procinst_id).await?;

        // delete current execution record
        let exec_dao = ApfRuExecutionDao::new(tran);
        exec_dao.delete(&current_execution.borrow().id).await?;

        // mark end of proc_inst
        let hi_procinst_dao = ApfHiProcinstDao::new(tran);
        let mut element_id = current_execution.borrow().element_id()?;
        if let Some(el) = &self.base.terminate_element {
            element_id = el.get_element_id();
        }

        hi_procinst_dao.mark_end(&procinst_id, &element_id, get_now()).await?;

        // delete proc_inst record
        exec_dao.delete(&procinst_id).await?;

        Ok(())
    }
}