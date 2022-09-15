use chrono::NaiveDateTime;
use color_eyre::Result;
use sqlx::{Postgres, Transaction};

use crate::error::{AppError, ErrorCode};
use crate::model::{ApfHiProcinst, NewApfHiProcinst};

pub struct ApfHiProcinstDao {

}

impl ApfHiProcinstDao {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create(&self, obj: &NewApfHiProcinst, tran: &mut Transaction<'_, Postgres>) -> Result<ApfHiProcinst> {
        let sql = r#"
            insert into apf_hi_procinst (
                id, rev, proc_inst_id, business_key,
                proc_def_id, start_time, start_user, start_element_id
            ) values (
                $1, 1, $2, $3,
                $4, $5, $6, $7
            ) 
            returning *
        "#;

        let rst = sqlx::query_as::<_, ApfHiProcinst>(sql)
            .bind(&obj.id)
            .bind(&obj.proc_inst_id)
            .bind(&obj.business_key)
            .bind(&obj.proc_def_id)
            .bind(&obj.start_time)
            .bind(&obj.start_user)
            .bind(&obj.start_element_id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn mark_end(&self, id: &str, end_element_id: &str, end_time: NaiveDateTime, tran: &mut Transaction<'_, Postgres>) -> Result<()> {
        let hi_procinst = self.get_by_id(id, tran).await?;
        let duration = (end_time - hi_procinst.start_time).num_milliseconds();

        let sql = r#"
            update apf_hi_procinst
            set rev = rev + 1,
                end_time = $1,
                duration = $2,
                end_element_id = $3
            where id = $4
                and rev = $5
        "#;

        let rst = sqlx::query(sql)
            .bind(end_time)
            .bind(duration)
            .bind(end_element_id)
            .bind(id)
            .bind(hi_procinst.rev)
            .execute(&mut *tran)
            .await?;

        if rst.rows_affected() != 1 {
            Err(
                AppError::new(
                    ErrorCode::InternalError, 
                    Some(
                        &format!(
                            "apf_hi_procinst({}) is not updated correctly, affects ({}) != 1", 
                            hi_procinst.id, rst.rows_affected()
                        )
                    ), 
                    concat!(file!(), ":", line!()), None))?
        }

        Ok(())
    }

    pub async fn get_by_id(&self, id: &str, tran: &mut Transaction<'_, Postgres>) -> Result<ApfHiProcinst> {
        let sql = r#"
            select id, rev, proc_inst_id, business_key,
                proc_def_id, start_time, start_user, start_element_id,
                end_time, duration, end_element_id
            from apf_hi_procinst
            where id = $1
        "#;

        let rst = sqlx::query_as::<_, ApfHiProcinst>(sql).bind(id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::Connection;
    use crate::boot::db;
    use crate::dao::ApfRuExecutionDao;
    use crate::get_now;
    use crate::manager::engine::tests::create_test_deploy;
    use crate::model::NewApfRuExecution;
    use super::*;

    #[tokio::test]
    async fn test_create() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &mut tran).await;
        let exec_dao = ApfRuExecutionDao::new();
        let new_exec = NewApfRuExecution {
            proc_def_id: procdef.id.to_owned(),
            is_active: 1,
            start_time: get_now(),
            ..Default::default()
        };
        let proc_inst = exec_dao.create_proc_inst(&new_exec,&mut tran).await.unwrap();
        let hi_procinst_dao = ApfHiProcinstDao::new();
        let hi_obj = NewApfHiProcinst {
            id: proc_inst.id.clone(),
            proc_inst_id: proc_inst.proc_def_id.clone(),
            business_key: proc_inst.business_key,
            proc_def_id: proc_inst.proc_def_id.clone(),
            start_time: proc_inst.start_time,
            start_user: proc_inst.start_user,
            start_element_id: proc_inst.element_id,
        };

        let hi_procinst = hi_procinst_dao.create(&hi_obj, &mut tran).await.unwrap();
        assert_eq!(hi_procinst.id, proc_inst.id.clone());

        hi_procinst_dao.mark_end(&proc_inst.id, "end_event_1", get_now(), &mut tran).await.unwrap();

        tran.rollback().await.unwrap();
    }

}