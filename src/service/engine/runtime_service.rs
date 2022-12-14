use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use color_eyre::Result;
use tokio_postgres::Transaction;

use crate::common::db;
use crate::service::engine::{CreateAndStartProcessInstanceCmd, Operator, OperatorContext, OperatorExecutor, ProcessEngine};
use crate::model::{ApfRuExecution, WrappedValue};
use crate::dao::ApfReProcdefDao;
use crate::error::{AppError, ErrorCode};

#[derive(Debug)]
pub struct RuntimeService {

}

#[allow(unused)]
impl RuntimeService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn start_process_instance_by_key<'a>(
        &self, 
        process_definition_key: &str,
        company_id: &str,
        business_key: Option<String>,
        variables: HashMap<String, WrappedValue>,
        user_id: Option<String>,
        group_id: Option<String>)
    -> Result<Rc<ApfRuExecution>> {
        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let mut operator_ctx = OperatorContext::new(group_id, user_id, variables);

        let rst = self._start_process_instance_by_key(
            process_definition_key, company_id, business_key, &mut operator_ctx, &tran).await?;

        tran.commit().await?;

        Ok(rst)
    }

    pub(crate) async fn _start_process_instance_by_key<'a>(
        &self, 
        process_definition_key: &str,
        company_id: &str,
        business_key: Option<String>,
        operator_ctx: &mut OperatorContext,
        tran: &Transaction<'_>)
    -> Result<Rc<ApfRuExecution>>  {
        let procdef_dao = ApfReProcdefDao::new(tran);
        let re_def = procdef_dao.get_lastest_by_key(process_definition_key, company_id).await?;

        let repository_service = ProcessEngine::new(ProcessEngine::DEFAULT_ENGINE).get_repository_service();
        let bpmn_process = repository_service.load_bpmn_by_deployment(&re_def.deployment_id, tran).await?;
        let bpmn_process = Arc::new(bpmn_process);
        operator_ctx.bpmn_process = Some(bpmn_process.clone());

        let re_def = Arc::new(re_def);
        let caspi_operator = CreateAndStartProcessInstanceCmd::new(re_def.clone(), business_key);

        let mut operator_exec = OperatorExecutor::new();
        let procinst = operator_exec.execute(
            Operator::CreateAndStartProcessInstanceCmd(caspi_operator), operator_ctx, tran).await?;

        match procinst.process_instantce {
            None => {
                Err(AppError::new(ErrorCode::NotFound, Some("process instance not found"), concat!(file!(), ":", line!()), None))?
            },
            Some(p) => {
                Ok(p)
            },
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::common::db;
    use crate::service::engine::tests::create_test_deploy;
    use super::*;

    #[tokio::test]
    async fn test_execute() {
        log4rs_macros::prepare_log();

        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &tran).await;

        let mut operator_ctx = OperatorContext::default();
        let var_1 = WrappedValue::Bool(true);
        operator_ctx.variables.insert("approval".to_owned(), var_1);
        let rt_service = RuntimeService::new();
        let procinst = rt_service._start_process_instance_by_key(
            &procdef.key,
            &procdef.company_id,
            Some("process_biz_key".to_owned()),
            &mut operator_ctx,
            &tran
        )
        .await
        .unwrap();

        assert_eq!(procinst.proc_def_id, procdef.id);

        tran.rollback().await.unwrap();
    }

}