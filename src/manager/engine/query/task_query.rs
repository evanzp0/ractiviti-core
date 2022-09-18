use color_eyre::Result;
use log4rs::*;
use regex::Regex;
use tokio_postgres::Transaction;
use tokio_postgres::types::ToSql;

use crate::dao::{SqlFragment as SF, BaseDao};
use crate::error::AppError;
use crate::model::ApfRuTask;
use crate::common::StringBuilder;

#[derive(Default)]
pub struct TaskQuery<'a> {
    id: Option<String>,
    execution_id: Option<String>,
    proc_inst_id: Option<String>,
    candidate_group: Option<String>,
    candidate_user: Option<String>,
    business_key: Option<String>,
    process_definition_key: Option<String>,
    order_by: Option<String>,
    count: Option<String>,
    base_dao: Option<BaseDao<'a>>,
}

#[allow(unused)]
impl<'a> TaskQuery<'a> {
    pub const SELECT_FIELD:&'static str = r#"
        t1.id, t1.rev, t1.execution_id, t1.proc_inst_id, t1.proc_def_id,
        t1.element_id, t1.element_name, t1.element_type, t1.business_key,
        t1.description, t1.start_user_id, t1.create_time,
        t1.suspension_state, t1.form_key
    "#;

    const FROM_TABLE:&'static str = " from apf_ru_task ";

    pub fn new(tran: &'a Transaction<'a>) -> Self {
        Self {
            base_dao: Some(BaseDao::new(tran)),
            ..Default::default()
        }
    }

    pub fn count(mut self, field: &str) -> Self {
        self.count = Some(field.to_owned());
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_owned());
        self
    }

    pub fn execution_id(mut self, execution_id: &str) -> Self {
        self.execution_id = Some(execution_id.to_owned());
        self
    }

    pub fn proc_inst_id(mut self, proc_inst_id: &str) -> Self {
        self.proc_inst_id = Some(proc_inst_id.to_owned());
        self
    }

    pub fn candidate_user(mut self, candidate_user: Option<String>) -> Self {
        self.candidate_user = candidate_user;
        self
    }

    pub fn candidate_group(mut self, candidate_group: Option<String>) -> Self {
        self.candidate_group = candidate_group;
        self
    }

    pub fn business_key(mut self, business_key: &str) -> Self {
        self.business_key = Some(business_key.to_owned());
        self
    }

    pub fn process_definition_key(mut self, process_definition_key: &str) -> Self {
        self.process_definition_key = Some(process_definition_key.to_owned());
        self
    }

    pub async fn fetch_all(&self) -> Result<Vec<ApfRuTask>> {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let sql= self.build_sql(&mut params);
        let rst = match &self.base_dao {
            Some(base_dao) => base_dao.fetcth_all::<ApfRuTask>(&sql, &params).await?,
            None => Err(AppError::unexpected_error(concat!(file!(), ":", line!())))?,
        };

        Ok(rst)
    }

    pub async fn fetch_one(&self) -> Result<ApfRuTask> {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let sql= self.build_sql(&mut params);
        let rst = match &self.base_dao {
            Some(base_dao) => base_dao.fetch_one::<ApfRuTask>(&sql, &params).await?,
            None => Err(AppError::unexpected_error(concat!(file!(), ":", line!())))?,
        };

        Ok(rst)
    }

    pub async fn fetch_count(&self) -> Result<i64> {
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let sql= self.build_sql(&mut params);

        let rst = match &self.base_dao {
            Some(base_dao) => base_dao.fetch_i64(&sql, &params).await?,
            None => Err(AppError::unexpected_error(concat!(file!(), ":", line!())))?,
        };

        Ok(rst)
    }

    fn build_sql(&'a self, params: &mut Vec<&'a (dyn ToSql + Sync)>) -> String {
        let mut sql_builder = StringBuilder::new();
        sql_builder.append(SF::SELECT);

        if let Some(v) = &self.count {
            sql_builder.ltrim().append(SF::COUNT(v.to_owned()));
        } else {
            let field = TaskQuery::SELECT_FIELD.trim();
            let re = Regex::new(r"\n\s*").unwrap();
            let field = re.replace_all(field, " ");
            let field = field.trim();
            sql_builder.ltrim().append(SF::DISTINCT).append(SF::FIELD(field.to_owned()));
        }

        sql_builder.ltrim().append(SF::FROM("apf_ru_task t1".to_owned()));

        if let Some(_) = &self.process_definition_key {
            sql_builder.ltrim().append(SF::JOIN("apf_re_procdef t2 on t2.id = t1.proc_def_id".to_owned()));
        }

        if self.candidate_group != None || self.candidate_user != None {
            sql_builder.ltrim().append(SF::JOIN("apf_ru_identitylink t3 on t3.task_id = t1.id".to_owned()));
        }

        sql_builder.ltrim().append(SF::WHERE);

        let mut idx = 0;
        if let Some(v) = &self.execution_id {
            idx += 1;
            sql_builder.ltrim().append(SF::AND(format!("t1.execution_id = ${}", idx)));
            params.push(v);
        }

        if let Some(v) = &self.proc_inst_id {
            idx += 1;
            sql_builder.ltrim().append(SF::AND(format!("t1.proc_inst_id = ${}", idx)));
            params.push(v);
        }

        if let Some(v) = &self.business_key {
            idx += 1;
            sql_builder.ltrim().append(SF::AND(format!("t1.business_key = ${}", idx)));
            params.push(v);
        }

        if let Some(v) = &self.candidate_group {
            idx += 1;
            let param_in_str = BaseDao::split_params(v, ',');

            sql_builder.ltrim().append(SF::AND(format!("t3.group_id in (${})", idx)));
            params.push(v);
        }

        if let Some(v) = &self.candidate_user {
            idx += 1;
            sql_builder.ltrim().append(SF::AND(format!("t3.user_id = ${}", idx)));
            params.push(v);
        }

        if let Some(v) = &self.process_definition_key {
            idx += 1;
            sql_builder.ltrim().append(SF::AND(format!("t2.key = ${}", idx)));
            params.push(v);
        }

        if let Some(v) = &self.order_by {
            sql_builder.ltrim().append(SF::ORDER_BY(v.to_owned()));
        }

        let sql = sql_builder.string();

        debug!("{}", sql);

        sql
    }
}

#[cfg(test)]
pub mod tests {
    use crate::common::db;
    use crate::dao::apf_ru_execution_dao::tests::create_test_procinst;
    use crate::dao::apf_ru_task_dao::tests::create_test_task;
    use crate::manager::engine::tests::create_test_deploy;
    use super::*;

    #[tokio::test]
    async fn test_list() {
        log4rs::prepare_log();

        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &tran).await;
        let proc_inst = create_test_procinst(&procdef, &tran).await;

        let task1 = create_test_task(&proc_inst, &tran).await;

        let tasks = TaskQuery::new(&tran)
            .execution_id("test_exec_id_1")
            .candidate_user(Some("test_user_id_1".to_owned()))
            .candidate_group(Some("test_group_id_1, test_group_id_2".to_owned()))
            .process_definition_key("test_process_definition_key")
            .fetch_all()
            .await
            .unwrap();
        assert_eq!(tasks.len(), 0);

        let task2 = TaskQuery::new(&tran)
            .execution_id(&task1.execution_id)
            .fetch_one()
            .await
            .unwrap();
        assert_eq!(task2.id, task1.id);

        let rst = TaskQuery::new(&tran)
            .execution_id(&task1.execution_id)
            .count("id")
            .fetch_count()
            .await
            .unwrap();
        assert_eq!(rst, 1);
    }

}