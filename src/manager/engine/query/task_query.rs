use std::any::Any;

use color_eyre::Result;
use rstring_builder::StringBuilder;
use sqlx::{Postgres, Transaction};

use crate::dao::{ApfRuTaskDao, BaseDao, SqlFragment as SF};
use crate::model::ApfRuTask;

#[derive(Debug, Default)]
pub struct TaskQuery {
    id: Option<String>,
    execution_id: Option<String>,
    proc_inst_id: Option<String>,
    candidate_group: Option<String>,
    candidate_user: Option<String>,
    business_key: Option<String>,
    process_definition_key: Option<String>,
    order_by: Option<String>,
    count: Option<String>,
}

#[allow(unused)]
impl TaskQuery {
    pub const SELECT_FIELD:&'static str = r#"
        t1.id, t1.rev, t1.execution_id, t1.proc_inst_id, t1.proc_def_id,
        t1.element_id, t1.element_name, t1.element_type, t1.business_key,
        t1.description, t1.start_user_id, t1.create_time,
        t1.suspension_state, t1.form_key 
    "#;

    const FROM_TABLE:&'static str = " from apf_ru_task ";

    pub fn new() -> Self {
        Self {
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

    pub async fn list<'a>(&self, tran: &mut Transaction<'a, Postgres>) -> Result<Vec<ApfRuTask>> {
        let mut params: Vec<Box<dyn Any>> = Vec::new();
        let sql= self.build_sql(&mut params);

        let rst = BaseDao::find_by_crieria(&sql, &params, tran).await?;

        Ok(rst)
    }

    pub async fn fetch_one<'a>(&self, tran: &mut Transaction<'a, Postgres>) -> Result<ApfRuTask> {
        let mut params: Vec<Box<dyn Any>> = Vec::new();
        let sql= self.build_sql(&mut params);

        let ru_task_dao = ApfRuTaskDao::new();
        let rst = BaseDao::fetch_one_by_crieria(&sql, &params, tran).await?;

        Ok(rst)
    }

    pub async fn fetch_count<'a>(&self, tran: &mut Transaction<'a, Postgres>) -> Result<i64> {
        let mut params: Vec<Box<dyn Any>> = Vec::new();
        let sql= self.build_sql(&mut params);

        let ru_task_dao = ApfRuTaskDao::new();
        let rst = BaseDao::fetch_scalar_by_crieria(&sql, &params, tran).await?;

        Ok(rst)
    }

    fn build_sql(&self, params: &mut Vec<Box<dyn Any>>) -> String {
        let mut sql_builder = StringBuilder::new();
        sql_builder.append(SF::SELECT);

        if let Some(v) = &self.count {
            sql_builder.append(SF::COUNT(v.to_owned()));
        } else {
            sql_builder.append(SF::DISTINCT)
                .append(SF::FIELD(TaskQuery::SELECT_FIELD.to_owned()));

        }

        sql_builder.append(SF::FROM("apf_ru_task t1".to_owned()));

        if let Some(_) = &self.process_definition_key {
            sql_builder.append(SF::JION("apf_re_procdef t2 on t2.id = t1.proc_def_id".to_owned()));
        }

        if self.candidate_group != None || self.candidate_user != None {
            sql_builder.append(SF::JION("apf_ru_identitylink t3 on t3.task_id = t1.id".to_owned()));
        }

        sql_builder.append(SF::WHERE);

        let mut idx = 0;
        if let Some(v) = &self.execution_id {
            idx += 1;
            sql_builder.append(SF::AND(format!("t1.execution_id = ${}", idx)));
            params.push(Box::new(v.clone()));
        }

        if let Some(v) = &self.proc_inst_id {
            idx += 1;
            sql_builder.append(SF::AND(format!("t1.proc_inst_id = ${}", idx)));
            params.push(Box::new(v.clone()));
        }

        if let Some(v) = &self.business_key {
            idx += 1;
            sql_builder.append(SF::AND(format!("t1.business_key = ${}", idx)));
            params.push(Box::new(v.to_owned()));
        }

        if let Some(v) = &self.candidate_group {
            idx += 1;
            let param_in_str = BaseDao::split_params(v);

            sql_builder.append(SF::AND(format!("t3.group_id in (${})", idx)));
            params.push(Box::new(param_in_str));
        }

        if let Some(v) = &self.candidate_user {
            idx += 1;
            sql_builder.append(SF::AND(format!("t3.user_id = ${}", idx)));
            params.push(Box::new(v.to_owned()));
        }

        if let Some(v) = &self.process_definition_key {
            idx += 1;
            sql_builder.append(SF::AND(format!("t2.key = ${}", idx)));
            params.push(Box::new(v.to_owned()));
        }

        if let Some(v) = &self.order_by {
            sql_builder.append(SF::ORDER_BY(v.to_owned()));
        }

        let sql = sql_builder.string();
        sql
    }
}

#[cfg(test)]
pub mod tests {
    use sqlx::Acquire;
    use crate::boot::db;
    use crate::dao::apf_ru_execution_dao::tests::create_test_procinst;
    use crate::dao::apf_ru_task_dao::tests::create_test_task;
    use crate::manager::engine::tests::create_test_deploy;
    use super::*;

    #[actix_rt::test]
    async fn test_list() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &mut tran).await;
        let proc_inst = create_test_procinst(&procdef, &mut tran).await;

        let task1 = create_test_task(&proc_inst, &mut tran).await;

        let tasks = TaskQuery::new()
            .execution_id("test_exec_id_1")
            .candidate_user(Some("test_user_id_1".to_owned()))
            .candidate_group(Some("test_group_id_1, test_group_id_2".to_owned()))
            .process_definition_key("test_process_definition_key")
            .list(&mut tran)
            .await
            .unwrap();
        assert_eq!(tasks.len(), 0);

        let task2 = TaskQuery::new()
            .execution_id(&task1.execution_id)
            .fetch_one(&mut tran)
            .await
            .unwrap();
        assert_eq!(task2.id, task1.id);

        let rst = TaskQuery::new()
            .execution_id(&task1.execution_id)
            .count("id")
            .fetch_count(&mut tran)
            .await
            .unwrap();
        assert_eq!(rst, 1);
    }

}