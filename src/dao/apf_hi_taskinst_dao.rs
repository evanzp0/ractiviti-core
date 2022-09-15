use sqlx::{Postgres, Transaction};
use crate::model::{ApfHiTaskinst, ApfRuTask, NewApfHiTaskinst};
use color_eyre::Result;
use crate::error::{AppError, ErrorCode};
use crate::get_now;

pub struct ApfHiTaskinstDao {

}

impl ApfHiTaskinstDao {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create_from_task(&self, task: &ApfRuTask, tran: &mut Transaction<'_, Postgres>) -> Result<ApfHiTaskinst> {
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

        let rst = self.create(&new_hi_task, tran).await?;
        Ok(rst)
    }

    pub async fn create(&self, obj: &NewApfHiTaskinst, tran: &mut Transaction<'_, Postgres>) -> Result<ApfHiTaskinst> {
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

        let rst = sqlx::query_as::<_, ApfHiTaskinst>(sql)
            .bind(&obj.rev)
            .bind(&obj.execution_id)
            .bind(&obj.proc_inst_id)
            .bind(&obj.proc_def_id) //
            .bind(&obj.element_id)
            .bind(&obj.element_name)
            .bind(&obj.element_type)
            .bind(&obj.business_key)
            .bind(&obj.description)
            .bind(&obj.start_user_id)
            .bind(&obj.start_time)
            .bind(&obj.suspension_state)
            .bind(&obj.form_key)
            .bind(&obj.end_time)
            .bind(&obj.duration)
            .bind(&obj.id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn mark_end(&self, task_id: &str, end_user_id: Option<String>, tran: &mut Transaction<'_, Postgres>) -> Result<()> {
        let hi_task = self.get_by_id(task_id, tran).await?;
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
        let rst = sqlx::query(sql)
            .bind(end_time)
            .bind(duration)
            .bind(end_user_id)
            .bind(task_id)
            .bind(hi_task.rev)
            .execute(&mut *tran)
            .await?;

        if rst.rows_affected() != 1 {
            Err(
                AppError::new(
                    ErrorCode::InternalError,
                    Some(
                        &format!(
                            "apf_hi_taskinst({}) is not updated correctly, affects({}) != 1", 
                            task_id, 
                            rst.rows_affected()
                        )
                    ), 
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?
        }

        Ok(())
    }

    pub async fn get_by_id(&self, id: &str, tran: &mut Transaction<'_, Postgres>) -> Result<ApfHiTaskinst> {
        let sql = r#"
            select id, rev, execution_id, proc_inst_id, proc_def_id,
                element_id, element_name, element_type, business_key,
                description, start_user_id, end_user_id, start_time,
                suspension_state, form_key, end_time, duration
                from apf_hi_taskinst
            where id = $1
        "#;

        let rst = sqlx::query_as::<_, ApfHiTaskinst>(sql)
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
    use crate::dao::apf_ru_task_dao::tests::create_test_task;
    use crate::manager::engine::tests::create_test_deploy;
    use crate::model::ApfRuTask;
    use super::*;

    #[actix_rt::test]
    async fn test_create_and_end_hi_actinst() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &mut tran).await;
        let proc_inst = create_test_procinst(&procdef, &mut tran).await;

        let task = create_test_task(&proc_inst, &mut tran).await;
        let hi_task = create_test_hi_task(&task, &mut tran).await;
        assert_eq!(hi_task.id, task.id);

        let hi_task_dao = ApfHiTaskinstDao::new();
        hi_task_dao.mark_end(&task.id, Some("end_user_1".to_owned()), &mut tran).await.unwrap();
    }

    pub async fn create_test_hi_task(task: &ApfRuTask, mut tran: &mut Transaction<'_, Postgres>) -> ApfHiTaskinst {
        let hi_task_dao = ApfHiTaskinstDao::new();
        let hi_task = hi_task_dao.create_from_task(task, &mut tran).await.unwrap();

        hi_task
    }
}