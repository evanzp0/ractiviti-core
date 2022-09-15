use color_eyre::Result;
use sqlx::{Postgres, Transaction};
use crate::{model::{ApfRuTask, NewApfRuTask}, gen_id};

pub struct ApfRuTaskDao {
}

impl ApfRuTaskDao {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create(&self, obj: &NewApfRuTask, tran: &mut Transaction<'_, Postgres>) -> Result<ApfRuTask> {
        let sql = r#"
            insert into apf_ru_task (
                rev, execution_id, proc_inst_id, proc_def_id, element_id,
                element_name, element_type, business_key, description, start_user_id, 
                create_time, suspension_state, form_key, id
            ) values (
                $1, $2, $3, $4, $5, 
                $6, $7, $8, $9, $10, 
                $11, $12, $13, $14
            )
            returning *
        "#;
        let new_id = gen_id();

        let rst = sqlx::query_as::<_, ApfRuTask>(sql)
            .bind(obj.rev)
            .bind(&obj.execution_id)
            .bind(&obj.proc_inst_id)
            .bind(&obj.proc_def_id)
            .bind(&obj.element_id)
            .bind(&obj.element_name)
            .bind(&obj.element_type)
            .bind(&obj.business_key)
            .bind(&obj.description)
            .bind(&obj.start_user_id)
            .bind(&obj.create_time)
            .bind(&obj.suspension_state)
            .bind(&obj.form_key)
            .bind(new_id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn delete(&self, id: &str, tran: &mut Transaction<'_, Postgres>) -> Result<u64> {
        let sql = r#"delete from apf_ru_task where id = $1"#;
        let rst = sqlx::query(sql)
            .bind(id)
            .execute(&mut *tran)
            .await?;

        Ok(rst.rows_affected())
    }

    pub async fn get_by_id(&self, id: &str, tran: &mut Transaction<'_, Postgres>)
                -> Result<ApfRuTask> {
        let sql = r#"
            select id, rev, execution_id, proc_inst_id, proc_def_id, 
                element_id, element_name, element_type, business_key, description, 
                start_user_id, create_time, suspension_state, form_key
            from apf_ru_task
            where id = $1
        "#;
        let rst = sqlx::query_as::<_, ApfRuTask>(sql)
            .bind(id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }
}

#[cfg(test)]
pub mod tests {
    use sqlx::Connection;
    use crate::boot::db;
    use crate::dao::apf_ru_execution_dao::tests::create_test_procinst;
    use crate::get_now;
    use crate::manager::engine::tests::create_test_deploy;
    use crate::model::ApfRuExecution;
    use super::*;
    use std::any::Any;
    use crate::dao::BaseDao;

    #[tokio::test]
    async fn test_create_and_delete_hi_actinst() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &mut tran).await;
        let proc_inst = create_test_procinst(&procdef, &mut tran).await;

        let task = create_test_task(&proc_inst, &mut tran).await;
        assert_eq!(task.proc_inst_id, proc_inst.id);

        let task_dao = ApfRuTaskDao::new();
        // test get by id
        let task = task_dao.get_by_id(&task.id, &mut tran).await.unwrap();

        // test find by crieria
        let sql = "select * from apf_ru_task where id = $1 and element_id = $2 and suspension_state = $3";
        let params: Vec<Box<dyn Any>> = vec![
            Box::new(task.id.clone()),
            Box::new(task.element_id.clone()),
            Box::new(Some(task.suspension_state.clone()))
        ];
        let rst = BaseDao::find_by_crieria::<ApfRuTask>(sql, &params, &mut tran).await.unwrap();
        assert_eq!(rst.len(), 1);

        // test fetch one by crieria
        let rst = BaseDao::fetch_one_by_crieria::<ApfRuTask>(sql, &params, &mut tran).await.unwrap();
        assert_eq!(rst.id, task.id.clone());

        // test count
        let sql = "select count(*) from apf_ru_task where id = $1 and element_id = $2 and suspension_state = $3";
        let rst = BaseDao::fetch_scalar_by_crieria(sql, &params, &mut tran).await.unwrap();
        assert_eq!(rst, 1);

        // delete
        let rst = task_dao.delete(&task.id, &mut tran).await.unwrap();
        assert_eq!(rst, 1);
    }

    pub async fn create_test_task(proc_inst: &ApfRuExecution, mut tran: &mut Transaction<'_, Postgres>)
            -> ApfRuTask {
        let task_dao = ApfRuTaskDao::new();
        let now = Some(get_now());

        let new_ru_task = NewApfRuTask {
            rev: 1,
            suspension_state: 0,
            create_time: now.clone(),
            execution_id: proc_inst.id.clone(),
            proc_inst_id: proc_inst.id.clone(),
            proc_def_id: proc_inst.proc_def_id.clone(),
            element_id: Some("start_1".to_owned()),
            element_name: Some("start_it".to_owned()),
            element_type: Some("StartEvent".to_owned()),
            ..Default::default()
        };

        let task = task_dao.create(&new_ru_task, &mut tran).await.unwrap();
        task
    }
}