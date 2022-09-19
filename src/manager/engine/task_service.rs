use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use color_eyre::Result;
use tokio_postgres::Transaction;

use crate::{ArcRw, get_now};
use crate::common::db;
use crate::dao::{ApfHiVarinstDao, ApfReProcdefDao, ApfRuExecutionDao, ApfRuTaskDao, ApfRuVariableDao};
use crate::error::AppError;
use crate::manager::engine::{CompleteTaskCmd, Operator, OperatorContext, OperatorExecutor, ProcessEngine};
use crate::model::{ApfRuVariable, ApfRuVariableDto, WrappedValue};

#[derive(Debug)]
pub struct TaskService {

}

impl TaskService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn complete<'a>(
        &self, 
        task_id: &str,
        variables: Option<ArcRw<HashMap<String, WrappedValue>>>,
        user_id: Option<String>,
        group_id: Option<String>
    ) -> Result<()> {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.transaction().await.unwrap();

        let mut operator_ctx = OperatorContext::new(group_id, user_id, variables);
        self._complete(task_id, &mut operator_ctx, &mut tran).await?;
        tran.commit().await?;

        Ok(())
    }

    pub async fn _complete(&self, task_id: &str, operator_ctx: &mut OperatorContext, tran: &Transaction<'_>) -> Result<()> {
        let task_dao = ApfRuTaskDao::new(tran);
        let current_task = task_dao.get_by_id(task_id).await?;

        let procdef_dao = ApfReProcdefDao::new(tran);
        let re_def = procdef_dao.get_by_id(&current_task.proc_def_id).await?;
        let repository_service = ProcessEngine::new(ProcessEngine::DEFAULT_ENGINE).get_repository_service();
        let bpmn_process = repository_service.load_bpmn_by_deployment(&re_def.deployment_id, tran).await?;
        let bpmn_process = Arc::new(bpmn_process);
        let element = bpmn_process.element_map.get(&current_task.element_id_ex()?).ok_or(
            AppError::notfound_error(concat!(file!(), ":", line!())))?;

        operator_ctx.bpmn_process = Some(bpmn_process.clone());

        // merge variables
        let var_dao = ApfRuVariableDao::new(tran);
        let hi_var_dao = ApfHiVarinstDao::new(tran);
        let update_time = get_now();
        for (key, value) in operator_ctx.variables.read().unwrap().iter() {
            let dto = ApfRuVariableDto {
                var_type: value.get_type(),
                name: key.to_owned(),
                value: value.as_str(),
                proc_inst_id: current_task.proc_inst_id.clone(),
                execution_id: Some(current_task.execution_id.clone()),
                task_id: Some(current_task.id.clone()),
            };
            let variable = var_dao.create_or_update(&dto).await?;
            hi_var_dao.create_or_update_by_variable(&variable, update_time).await?;
        }

        let var_insts = var_dao.find_all_by_proc_inst(&current_task.proc_inst_id).await?;
        let vars_map = ApfRuVariable::convert_variables_to_map(&var_insts);
        operator_ctx.variables = Arc::new(RwLock::new(vars_map));

        // continue to handle operator
        let execution_dao = ApfRuExecutionDao::new(tran);
        let proc_inst = execution_dao.get_by_id(&current_task.proc_inst_id).await?;
        let current_execution = execution_dao.get_by_id(&current_task.execution_id).await?;

        let complete_task_cmd = CompleteTaskCmd::new(
            element.clone(), 
            Arc::new(proc_inst), 
            Some(Arc::new(RwLock::new(current_execution))), 
            Some(Arc::new(current_task))
        );

        let mut operator_exec = OperatorExecutor::new();
        operator_exec.execute(Operator::CompleteTaskCmd(complete_task_cmd), operator_ctx, tran).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::manager::engine::query::TaskQuery;
    use crate::manager::engine::tests::create_test_deploy;
    use super::*;

    #[tokio::test]
    async fn test_complete() {
        log4rs::prepare_log();

        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.transaction().await.unwrap();

        let procdef = create_test_deploy("bpmn/process_2.bpmn.xml", &mut tran).await;
        let rt_service = ProcessEngine::new(ProcessEngine::DEFAULT_ENGINE).get_runtime_service();
        let mut operator_ctx = OperatorContext::default();
        let procinst = rt_service._start_process_instance_by_key(
            &procdef.key,
            Some("process_biz_key_2".to_owned()),
            &mut operator_ctx,
            &mut tran).await.unwrap();

        let task = TaskQuery::new(&tran)
            .proc_inst_id(&procinst.id)
            .candidate_user(Some("user_1".to_owned()))
            .fetch_one()
            .await.unwrap();

        let mut variables = HashMap::new();
        variables.insert("approval_pass".to_owned(), WrappedValue::Bool(false));
        let variables = Arc::new(RwLock::new(variables));

        let mut operator_ctx = OperatorContext::new(
            None,
            Some("user_1".to_owned()),
            Some(variables));

        let task_service = TaskService::new();
        task_service._complete(&task.id, &mut operator_ctx, &mut tran).await.unwrap();

        tran.rollback().await.unwrap();
    }
}