use std::sync::Arc;

use color_eyre::Result;
use tokio_postgres::Transaction;

use crate::service::engine::{BaseOperator, ContinueProcessOperator, OperateRst, Operator, OperatorContext};
use crate::dao::{ApfHiProcinstDao, ApfRuExecutionDao};
use crate::get_now;
use crate::model::{ApfReProcdef, NewApfHiProcinst, NewApfRuExecution};

#[derive(Debug)]
pub struct CreateAndStartProcessInstanceCmd {
    pub procdef: Arc<ApfReProcdef>,
    pub business_key: Option<String>,
}

impl CreateAndStartProcessInstanceCmd {
    pub fn new(procdef: Arc<ApfReProcdef>, business_key: Option<String>) -> Self {
        Self{
            procdef,
            business_key
        }
    }

    pub async fn execute<'a> (&self, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>) -> Result<OperateRst> {
        let start_event = operator_ctx.bpmn_process_ex()?.get_start_event()?;

        // create process instance
        let new_exec = NewApfRuExecution {
            proc_def_id: self.procdef.id.clone(),
            business_key: self.business_key.clone(),
            is_active: 1,
            start_time: get_now(),
            element_id: Some(start_event.get_element_id()),
            ..Default::default()
        };
        let exec_dao = ApfRuExecutionDao::new(tran);
        let proc_inst = exec_dao.create_proc_inst(&new_exec).await?;

        // create process instance history
        let proc_inst = Arc::new(proc_inst);
        let hi_procinst_dao = ApfHiProcinstDao::new(tran);
        let new_hi_procinst = NewApfHiProcinst {
            id: proc_inst.id.to_owned(),
            proc_inst_id: proc_inst.id.clone(),
            business_key: proc_inst.business_key.to_owned(),
            proc_def_id: proc_inst.proc_def_id.to_owned(),
            start_time: proc_inst.start_time,
            start_user: proc_inst.start_user.to_owned(),
            start_element_id: proc_inst.element_id.to_owned(),
        };
        hi_procinst_dao.create(&new_hi_procinst).await?;



        // create or update variables
        let base_operator = BaseOperator::new(proc_inst.clone(), None, start_event.clone(), None, None);
        base_operator.create_or_update_variables(&mut operator_ctx.variables, tran).await?;

        // continue to handle start event operator
        let continue_operator = ContinueProcessOperator::new(
            start_event.clone(),
            None,
            proc_inst.clone(),
            None,
            None);

        operator_ctx.queue.push(Operator::ContinueProcessOperator(continue_operator));

        let rst = OperateRst {
            process_instantce: Some(proc_inst)
        };
        
        Ok(rst)
    }
}

