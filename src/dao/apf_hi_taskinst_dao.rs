use crate::model::{ApfHiTaskinst, ApfRuTask, NewApfHiTaskinst};
use color_eyre::Result;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Transaction;
use crate::error::{AppError, ErrorCode};
use crate::get_now;

use super::{BaseDao, Dao};

pub struct ApfHiTaskinstDao<'a> {
    base_dao: BaseDao<'a>
}

impl<'a> Dao for ApfHiTaskinstDao<'a> {

    fn tran(&self) -> &Transaction {
        self.base_dao.tran()
    }
}

impl<'a> ApfHiTaskinstDao<'a> {

    pub fn new(tran: &'a Transaction<'a>) -> Self {
        Self {
            base_dao: BaseDao::new(tran)
        }
    }
    
    pub async fn create_from_task(&self, task: &ApfRuTask) -> Result<ApfHiTaskinst> {
        let new_hi_task = NewApfHiTaskinst {
            id: task.id.clone(),
            rev: 1,
            suspension_state: 0,
            start_time: task.create_time,
            end_time: None,
            duration: None,
            execution_id: task.execution_id.clone(),
            proc_inst_id: task.proc_inst_id.clone(),
            proc_def_id: task.proc_def_id.clone(),
            element_id: task.element_id.clone(),
            element_name: task.element_name.clone(),
            element_type:  task.element_type.clone(),
            business_key: task.business_key.clone(),
            description: task.description.clone(),
            start_user_id: task.start_user_id.clone(),
            form_key: task.form_key.clone(),
        };

        let rst = self.create(&new_hi_task).await?;
        Ok(rst)
    }

    pub async fn create(&self, obj: &NewApfHiTaskinst) -> Result<ApfHiTaskinst> {
        let sql = r#"insert into apf_hi_taskinst (
                rev, execution_id, proc_inst_id, proc_def_id,
                element_id, element_name, element_type, business_key,
                description, start_user_id, start_time,
                suspension_state, form_key, end_time, duration, id
            ) values (
                $1, $2, $3, $4,
                $5, $6, $7, $8,
                $9, $10, $11, $12,
                $13, $14, $15, $16
            )
            returning *
        "#;
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
                    &obj.start_time,
                    &obj.suspension_state,
                    &obj.form_key,
                    &obj.end_time,
                    &obj.duration,
                    &obj.id,
                ]
            )
            .await?;
        let rst = ApfHiTaskinst::from_row(row)?;

        Ok(rst)
    }

    pub async fn mark_end(&self, task_id: &str, end_user_id: Option<String>) -> Result<u64> {
        let hi_task = self.get_by_id(task_id).await?;
        let end_time = get_now();
        let duration = (end_time - hi_task.start_time).num_milliseconds();

        let sql = r#"
            update apf_hi_taskinst
            set end_time = $1,
                duration = $2,
                end_user_id = $3,
                rev = rev + 1
            where id = $4
            and rev = $5
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let r = self.tran().execute(&stmt, &[&end_time, &duration, &end_user_id, &task_id, &hi_task.rev]).await?;

        if r != 1 {
            Err(
                AppError::new(
                    ErrorCode::InternalError,
                    Some(
                        &format!(
                            "apf_hi_taskinst({}) is not updated correctly, affects({}) != 1", 
                            task_id, 
                            r
                        )
                    ), 
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?
        }

        Ok(r)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<ApfHiTaskinst> {
        let sql = r#"
            select id, rev, execution_id, proc_inst_id, proc_def_id,
                element_id, element_name, element_type, business_key,
                description, start_user_id, end_user_id, start_time,
                suspension_state, form_key, end_time, duration
                from apf_hi_taskinst
            where id = $1
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let row = self.tran().query_one(&stmt, &[&id]).await?;
        let rst = ApfHiTaskinst::from_row(row)?;

        Ok(rst)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::common::db;
    use crate::dao::apf_ru_execution_dao::tests::create_test_procinst;
    use crate::dao::apf_ru_task_dao::tests::create_test_task;
    use crate::service::engine::tests::create_test_deploy;
    use crate::model::ApfRuTask;
    use super::*;

    #[tokio::test]
    async fn test_create_and_end_hi_actinst() {
        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &tran).await;
        let proc_inst = create_test_procinst(&procdef, &tran).await;

        let task = create_test_task(&proc_inst, &tran).await;
        let hi_task = create_test_hi_task(&task, &tran).await;
        assert_eq!(hi_task.id, task.id);

        let hi_task_dao = ApfHiTaskinstDao::new(&tran);
        hi_task_dao.mark_end(&task.id, Some("end_user_1".to_owned())).await.unwrap();
    }

    pub async fn create_test_hi_task(task: &ApfRuTask, tran: &Transaction<'_>) -> ApfHiTaskinst {
        let hi_task_dao = ApfHiTaskinstDao::new(tran);
        let hi_task = hi_task_dao.create_from_task(task).await.unwrap();

        hi_task
    }
}