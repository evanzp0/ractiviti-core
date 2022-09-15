use std::collections::HashMap;
use std::sync::Arc;
use sqlx::{Connection, Postgres, Transaction};
use crate::boot::db;
use crate::manager::engine::{CreateAndStartProcessInstanceCmd, Operator, OperatorContext, OperatorExecutor, ProcessEngine, TypeWrapper};
use crate::model::ApfRuExecution;
use color_eyre::Result;
use crate::ArcRw;
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

    pub async fn start_process_instance_by_key<'a>(&self, process_definition_key: &str,
                                                   business_key: Option<String>,
                                                   variables: Option<ArcRw<HashMap<String, TypeWrapper>>>,
                                               user_id: Option<String>,
                                               group_id: Option<String>)
            -> Result<Arc<ApfRuExecution>> {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let mut operator_ctx = OperatorContext::new(group_id, user_id, variables);

        let rst = self._start_process_instance_by_key(
            process_definition_key, business_key, &mut operator_ctx, &mut tran).await?;

        tran.commit().await?;

        Ok(rst)
    }

    pub async fn _start_process_instance_by_key<'a>(&self, process_definition_key: &str,
                                                    business_key: Option<String>,
                                                    operator_ctx: &mut OperatorContext,
                                                    tran: &mut Transaction<'a, Postgres>)
                                                    -> Result<Arc<ApfRuExecution>>  {
        let procdef_dao = ApfReProcdefDao::new();
        let re_def = procdef_dao.get_lastest_by_key(process_definition_key, tran).await?;

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
                Err(AppError::new(ErrorCode::NotFound,
                                  Some("process instance not found"),
                                  concat!(file!(), ":", line!()),
                                  None))?
            },
            Some(p) => {
                Ok(p)
            },
        }
    }

}


#[cfg(test)]
mod tests {
    use sqlx::Connection;
    use crate::boot::db;
    use crate::manager::engine::tests::create_test_deploy;
    use super::*;

    #[actix_rt::test]
    async fn test_execute() {
        log4rs::prepare_log();

        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &mut tran).await;

        let mut operator_ctx = OperatorContext::default();
        let var_1 = TypeWrapper::bool(true);
        operator_ctx.variables.write().unwrap().insert("approval".to_owned(), var_1);
        let rt_service = RuntimeService::new();
        let procinst = rt_service._start_process_instance_by_key(&procdef.key,
                                                                 Some("process_biz_key".to_owned()),
                                                                 &mut operator_ctx,
                                                                 &mut tran).await.unwrap();

        assert_eq!(procinst.proc_def_id, procdef.id);

        tran.rollback().await.unwrap();
    }

}