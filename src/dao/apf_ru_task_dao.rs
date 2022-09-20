use color_eyre::Result;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Transaction;
use crate::{model::{ApfRuTask, NewApfRuTask}, gen_id};
use super::{BaseDao, Dao};

pub struct ApfRuTaskDao<'a> {
    base_dao: BaseDao<'a>
}

impl<'a> Dao for ApfRuTaskDao<'a> {

    fn tran(&self) -> &Transaction {
        self.base_dao.tran()
    }
}

impl<'a> ApfRuTaskDao<'a> {

    pub fn new(tran: &'a Transaction<'a>) -> Self {
        Self {
            base_dao: BaseDao::new(tran)
        }
    }

    pub async fn create(&self, obj: &NewApfRuTask) -> Result<ApfRuTask> {
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
        let stmt = self.tran().prepare(sql).await?;
        let row = self
            .tran()
            .query_one(
                &stmt, 
                &[
                    &obj.rev,
                    &obj.execution_id,
                    &obj.proc_inst_id,
                    &obj.proc_def_id,
                    &obj.element_id,
                    &obj.element_name,
                    &obj.element_type,
                    &obj.business_key,
                    &obj.description,
                    &obj.start_user_id,
                    &obj.create_time,
                    &obj.suspension_state,
                    &obj.form_key,
                    &new_id,
                ]
            )
            .await?;
        let rst = ApfRuTask::from_row(row)?;

        Ok(rst)
    }

    pub async fn delete(&self, id: &str) -> Result<u64> {
        let sql = r#"delete from apf_ru_task where id = $1"#;
        let stmt = self.tran().prepare(sql).await?;
        let r = self.tran().execute(&stmt, &[&id]).await?;
        Ok(r)
    }

    pub async fn get_by_id(&self, id: &str)
                -> Result<ApfRuTask> {
        let sql = r#"
            select id, rev, execution_id, proc_inst_id, proc_def_id, 
                element_id, element_name, element_type, business_key, description, 
                start_user_id, create_time, suspension_state, form_key
            from apf_ru_task
            where id = $1
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let row = self.tran().query_one(&stmt, &[&id]).await?;
        let rst = ApfRuTask::from_row(row)?;

        Ok(rst)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::common::db;
    use crate::dao::apf_ru_execution_dao::tests::create_test_procinst;
    use crate::get_now;
    use crate::service::engine::tests::create_test_deploy;
    use crate::model::ApfRuExecution;
    use super::*;

    #[tokio::test]
    async fn test_create_and_delete_hi_actinst() {
        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &tran).await;
        let proc_inst = create_test_procinst(&procdef, &tran).await;

        let task = create_test_task(&proc_inst, &tran).await;
        assert_eq!(task.proc_inst_id, proc_inst.id);

        let task_dao = ApfRuTaskDao::new(&tran);
        // test get by id
        let task = task_dao.get_by_id(&task.id).await.unwrap();

        // delete
        let rst = task_dao.delete(&task.id).await.unwrap();
        assert_eq!(rst, 1);
    }

    pub async fn create_test_task(proc_inst: &ApfRuExecution, tran: &Transaction<'_>)
            -> ApfRuTask {
        let task_dao = ApfRuTaskDao::new(tran);
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

        let task = task_dao.create(&new_ru_task).await.unwrap();
        task
    }
}