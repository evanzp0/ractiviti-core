use chrono::NaiveDateTime;
use sqlx::{Postgres, Transaction};
use color_eyre::Result;

use crate::error::{AppError, ErrorCode};
use crate::gen_id;
use crate::model::{ApfHiActinst, NewApfHiActinst};

pub struct ApfHiActinstDao {
    
}

impl ApfHiActinstDao {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create(&self, obj: &NewApfHiActinst, tran: &mut Transaction<'_, Postgres>) -> Result<ApfHiActinst> {
        let sql = r#"
            insert into apf_hi_actinst (
                rev, proc_def_id, proc_inst_id, execution_id, task_id, 
                element_id, element_name, element_type, start_user_id, start_time, 
                end_time, duration, id
            ) values ( 
                $1, $2, $3, $4, $5, 
                $6, $7, $8, $9, $10,
                $11, $12, $13
            ) 
            returning *
        "#;
        let new_id = gen_id();
        
        let rst = sqlx::query_as::<_, ApfHiActinst>(sql)
            .bind(&obj.rev)
            .bind(&obj.proc_def_id)
            .bind(&obj.proc_inst_id)
            .bind(&obj.execution_id)
            .bind(&obj.task_id)
            .bind(&obj.element_id)
            .bind(&obj.element_name)
            .bind(&obj.element_type)
            .bind(&obj.start_user_id)
            .bind(&obj.start_time)
            .bind(&obj.end_time)
            .bind(&obj.duration)
            .bind(&new_id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn mark_end(
        &self, 
        execution_id: &str, 
        element_id: &str,
        end_time: NaiveDateTime, 
        end_user_id: Option<String>,
        tran: &mut Transaction<'_, Postgres>
    ) -> Result<()> {
        let hi_actinst = self.find_one_by_element_id(execution_id, element_id, tran).await?;
        let duration = (end_time - hi_actinst.start_time).num_milliseconds();

        let sql = r#"
            update apf_hi_actinst
            set rev = rev + 1,
                end_time = $1,
                duration = $2,
                end_user_id = $3
            where id = $4
                and rev = $5
        "#;
        let rst = sqlx::query(sql)
                .bind(end_time)
                .bind(duration)
                .bind(end_user_id)
                .bind(&hi_actinst.id)
                .bind(hi_actinst.rev)
                .execute(&mut *tran)
                .await?;

        if rst.rows_affected() != 1 {
            Err(
                AppError::new(
                    ErrorCode::InternalError,
                    Some(
                        &format!(
                            "apf_hi_actinst({}) is not updated correctly, affects ({}) != 1", 
                            hi_actinst.id, 
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

    #[allow(unused)]
    pub async fn get_by_id(&self, id: &str, tran: &mut Transaction<'_, Postgres>) -> Result<ApfHiActinst> {
        let sql = r#"
            select id, rev, proc_def_id, proc_inst_id, execution_id, 
                task_id, element_id, element_name, element_type,
                start_user_id, end_user_id, start_time, end_time, duration
            from apf_hi_actinst
            where id = $1
        "#;

        let rst = sqlx::query_as::<_, ApfHiActinst>(sql)
            .bind(id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn find_one_by_element_id(
        &self, 
        execution_id: &str, 
        element_id: &str, 
        tran: &mut Transaction<'_, Postgres>
    ) -> Result<ApfHiActinst> {
        let sql = r#"
            select id, rev, proc_def_id, proc_inst_id, execution_id,
                task_id, element_id, element_name, element_type,
                start_user_id, end_user_id, start_time, end_time, duration
            from apf_hi_actinst
            where execution_id = $1
                and element_id = $2 
        "#;

        // ptln!("{} , {}, {}", sql, execution_id, element_id);

        let rst = sqlx::query_as::<_, ApfHiActinst>(sql)
            .bind(execution_id)
            .bind(element_id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }
}

#[cfg(test)]
pub mod tests {
    use chrono::Local;
    use sqlx::Connection;
    use crate::boot::db;
    use crate::dao::apf_ru_execution_dao::tests::create_test_procinst;
    use crate::dao::apf_ru_task_dao::tests::create_test_task;
    use crate::manager::engine::tests::create_test_deploy;
    use crate::model::{ApfRuExecution, ApfRuTask};
    use super::*;

    #[tokio::test]
    async fn test_create_hi_actinst() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let procdef = create_test_deploy( "bpmn/process1.bpmn.xml", &mut tran).await;
        let proc_inst = create_test_procinst(&procdef, &mut tran).await;
        let task = create_test_task(&proc_inst, &mut tran).await;

        let hi_actinst = create_test_hi_actinst(&proc_inst, &task, &mut tran).await;
        assert_eq!(hi_actinst.task_id, Some(task.id));

        tran.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_find_hi_actinst_by_element() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &mut tran).await;
        let proc_inst = create_test_procinst(&procdef, &mut tran).await;
        let task = create_test_task(&proc_inst, &mut tran).await;

        let hi_actinst = create_test_hi_actinst(&proc_inst, &task, &mut tran).await;

        let hi_act_dao = ApfHiActinstDao::new();

        let hi_actinst2 = hi_act_dao
            .find_one_by_element_id(&proc_inst.id, &task.element_id.unwrap(), &mut tran)
            .await.unwrap();

        assert_eq!(hi_actinst, hi_actinst2);
        tran.rollback().await.unwrap();
    }

    async fn create_test_hi_actinst(proc_inst: &ApfRuExecution, task: &ApfRuTask, tran: &mut Transaction<'_, Postgres>)
            -> ApfHiActinst {

        let now = Some(Local::now().naive_local());
        let new_hi_actinst = NewApfHiActinst {
            rev: 1,
            proc_def_id: proc_inst.proc_def_id.clone(),
            proc_inst_id: proc_inst.proc_inst_id.clone(),
            execution_id: proc_inst.id.clone(),
            task_id: Some(task.id.clone()),
            element_id: task.element_id.clone(),
            element_name: task.element_name.clone(),
            element_type: task.element_type.clone(),
            start_user_id: task.start_user_id.clone(),
            start_time: task.create_time.clone(),
            end_time: now,
            duration: Some(1000),
        };
        let hi_act_dao = ApfHiActinstDao::new();
        let hi_actinst = hi_act_dao.create(&new_hi_actinst, tran).await.unwrap();
        hi_actinst
    }
}