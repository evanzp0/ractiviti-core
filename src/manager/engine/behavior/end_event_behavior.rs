use std::sync::Arc;
use crate::manager::engine::{BaseOperator, BpmnElement, OperatorContext};
use color_eyre::Result;
use log4rs::debug;
use sqlx::{Postgres, Transaction};
use crate::{ArcRw, get_now};
use crate::dao::{ApfHiProcinstDao, ApfRuExecutionDao, ApfRuVariableDao};
use crate::model::{ApfRuExecution, ApfRuTask};

pub struct EndEventBehavior {
    base: BaseOperator,
}

impl EndEventBehavior {
    pub fn new(element: BpmnElement, terminate_element: Option<BpmnElement>, proc_inst: Arc<ApfRuExecution>, current_exec: Option<ArcRw<ApfRuExecution>>,
               current_task: Option<Arc<ApfRuTask>>) -> Self {
        Self {
            base: BaseOperator::new(proc_inst, current_exec, element, terminate_element, current_task),
        }
    }

    pub async fn execute<'a>(&self, operator_ctx: &mut OperatorContext, tran: &mut Transaction<'a, Postgres>)
                             -> Result<()> {
        debug!("EndEvent (process: {:?}, element: {}) is executed",
            self.base.proc_inst.id,
            self.base.element.get_element_id());

        if let None = self.base.terminate_element {
            self.base.create_hi_actinst(None, tran).await?;
        }

        self.leave(operator_ctx, tran).await
    }

    pub async fn leave<'a>(&self, operator_ctx: &mut OperatorContext, tran: &mut Transaction<'a, Postgres>)
                           -> Result<()> {
        // mark end of current execution
        if let None = self.base.terminate_element {
            self.base.mark_end_execution(operator_ctx, tran).await?;
        }

        let current_execution = self.base.current_excution_ex()?;
        let procinst_id = current_execution.read().unwrap().proc_inst_id()?;

        // delete current variable
        let var_dao = ApfRuVariableDao::new();
        var_dao.delete_by_proc_inst_id(&procinst_id, tran).await?;

        // delete current execution record
        let exec_dao = ApfRuExecutionDao::new();
        exec_dao.delete(&current_execution.read().unwrap().id, tran).await?;

        // mark end of proc_inst
        let hi_procinst_dao = ApfHiProcinstDao::new();
        let mut element_id = current_execution.read().unwrap().element_id()?;
        if let Some(el) = &self.base.terminate_element {
            element_id = el.get_element_id();
        }

        hi_procinst_dao.mark_end(&procinst_id, &element_id, get_now(), tran).await?;

        // delete proc_inst record
        exec_dao.delete(&procinst_id, tran).await?;

        Ok(())
    }
}