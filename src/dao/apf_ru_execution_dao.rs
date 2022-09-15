use chrono::NaiveDateTime;
use sqlx::{Postgres, Transaction};
use crate::model::{ApfRuExecution, NewApfRuExecution};
use color_eyre::Result;
use uuid::Uuid;
use crate::error::{AppError, ErrorCode};

pub struct ApfRuExecutionDao {

}

impl ApfRuExecutionDao {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create(&self, obj: &NewApfRuExecution, tran: &mut Transaction<'_, Postgres>)
            -> Result<ApfRuExecution> {
        let sql = "insert into apf_ru_execution \
                (rev, proc_inst_id, business_key, parent_id, proc_def_id, root_proc_inst_id, \
                element_id, is_active, start_time, start_user) \
            values \
                (1, $1, $2, $3, $4, $5, \
                $6, $7, $8, $9) \
            returning * ";

        let rst = sqlx::query_as::<_, ApfRuExecution>(sql)
            .bind(&obj.proc_inst_id)
            .bind(&obj.business_key)
            .bind(&obj.parent_id)
            .bind(&obj.proc_def_id)
            .bind(&obj.root_proc_inst_id)
            .bind(&obj.element_id)
            .bind(&obj.is_active)
            .bind(&obj.start_time)
            .bind(&obj.start_user)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn create_proc_inst(&self, obj: &NewApfRuExecution, tran: &mut Transaction<'_, Postgres>)
            -> Result<ApfRuExecution> {
        let proc_inst = self.create(obj, tran).await?;

        let sql = "update apf_ru_execution \
            set proc_inst_id = $1, \
                root_proc_inst_id = $2 \
            where id = $3";

        let r = sqlx::query(sql)
            .bind(&proc_inst.id)
            .bind(&proc_inst.id)
            .bind(&proc_inst.id)
            .execute(&mut *tran)
            .await?;

        if r.rows_affected() != 1 {
            Err(AppError::new(ErrorCode::NotFound,
                              Some(&format!("apf_ru_execution({}) is not updated", proc_inst.id)),
                              concat!(file!(), ":", line!()),
                              None))?
        }

        let rst = self.get_by_id(&proc_inst.id, tran).await?;

        Ok(rst)
    }

    pub async fn mark_begin(&self, id: &Uuid,
                                element_id: &str,
                                start_user: Option<String>,
                                start_time: NaiveDateTime,
                                tran: &mut Transaction<'_, Postgres>) -> Result<()> {
        let ru_exection = self.get_by_id(id, tran).await?;

        let sql = "update apf_ru_execution \
                        set element_id = $1,\
                            start_time = $2,\
                            start_user = $3,\
                            rev = rev + 1 \
                        where id = $4 \
                          and rev = $5 ";
        let rst = sqlx::query(sql)
            .bind(element_id)
            .bind(start_time)
            .bind(start_user)
            .bind(id)
            .bind(ru_exection.rev)
            .execute(&mut *tran)
            .await?;

        if rst.rows_affected() != 1 {
            Err(AppError::new(ErrorCode::InternalError,
                              Some(&format!("apf_ru_execution({}) is not updated correctly, affects ({}) != 1",
                                            ru_exection.id, rst.rows_affected())),
                              concat!(file!(), ":", line!()),
                              None))?
        }

        Ok(())
    }

    pub async fn deactive_execution(&self, id: &Uuid, tran: &mut Transaction<'_, Postgres>)
            -> Result<()> {
        let current_exec = self.get_by_id(id, tran).await?;

        let sql = "update apf_ru_execution \
                        set is_active = 0,\
                            rev = $1 \
                        where id = $2\
                          and rev = $3";

        let rst = sqlx::query(sql)
            .bind(current_exec.rev + 1)
            .bind(&current_exec.id)
            .bind(current_exec.rev)
            .execute(&mut *tran)
            .await?;

        if rst.rows_affected() != 1 {
            Err(AppError::new(ErrorCode::InternalError,
                              Some(&format!("apf_ru_execution({}) is not updated correctly, affects ({}) != 1",
                                            current_exec.id, rst.rows_affected())),
                              concat!(file!(), ":", line!()),
                              None))?
        }

        Ok(())
    }

    pub async fn get_by_id(&self, id: &Uuid, tran: &mut Transaction<'_, Postgres>)
            -> Result<ApfRuExecution> {
        let sql = "select id, rev, proc_inst_id, business_key, parent_id, proc_def_id, root_proc_inst_id, \
                element_id, is_active, start_time, start_user \
            from apf_ru_execution where id = $1";
        let rst = sqlx::query_as::<_, ApfRuExecution>(sql)
            .bind(id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn count_inactive_by_element(&self, proc_inst_id: &Uuid, element_id: &str, tran: &mut Transaction<'_, Postgres>)
            -> Result<i64> {
        let sql = "select count(id) from apf_ru_execution \
                        where proc_inst_id = $1 \
                          and element_id = $2\
                          and is_active = 0";
        let rst = sqlx::query_scalar::<_, i64>(sql)
            .bind(proc_inst_id)
            .bind(element_id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn del_inactive_by_element(&self, proc_inst_id: &Uuid, element_id: &str, tran: &mut Transaction<'_, Postgres>)
                                               -> Result<u64> {
        let sql = "delete from apf_ru_execution \
                        where proc_inst_id = $1 \
                          and element_id = $2\
                          and is_active = 0";
        let rst = sqlx::query(sql)
            .bind(proc_inst_id)
            .bind(element_id)
            .execute(&mut *tran)
            .await?;

        Ok(rst.rows_affected())
    }

    pub async fn delete(&self, id: &Uuid, tran: &mut Transaction<'_, Postgres>) -> Result<u64> {
        let sql = "delete from apf_ru_execution \
                         where id = $1 ";
        let rst = sqlx::query(sql)
            .bind(id)
            .execute(&mut *tran)
            .await?;

        Ok(rst.rows_affected())
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::{Arc, RwLock};
    use sqlx::Connection;
    use crate::boot::db;
    use crate::get_now;
    use crate::manager::engine::tests::create_test_deploy;
    use crate::model::{ApfReProcdef};
    use super::*;

    #[actix_rt::test]
    async fn test_create_proc_inst() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &mut tran).await;
        let proc_inst = create_test_procinst(&procdef, &mut tran).await;
        assert_eq!(proc_inst.id, proc_inst.proc_inst_id.unwrap());
        assert_eq!(proc_inst.id, proc_inst.root_proc_inst_id.unwrap());
        assert_eq!(proc_inst.parent_id, None);
        assert_eq!(proc_inst.is_active, 1);

        let exec_dao = ApfRuExecutionDao::new();
        let mut current_exec = proc_inst.clone();
        let current_exec_id = current_exec.id.clone();
        let element_id = "startEvent_1";
        let start_user = Some("test_user".to_owned());
        let start_time = get_now();
        current_exec.parent_id = Some(proc_inst.id.clone());
        current_exec.start_user = start_user.clone();
        current_exec.start_time = start_time;
        current_exec.element_id = Some(element_id.to_owned());
        let current_exec = Arc::new(RwLock::new(current_exec));

        exec_dao.mark_begin(&current_exec_id, element_id, start_user, start_time, &mut tran).await.unwrap();

        exec_dao.deactive_execution(&current_exec.read().unwrap().id, &mut tran).await.unwrap();

        let proc_inst_id = current_exec.read().unwrap().proc_inst_id().unwrap();
        let element_id = current_exec.read().unwrap().element_id().unwrap();
        let count = exec_dao.count_inactive_by_element(&proc_inst_id, &element_id, &mut tran).await.unwrap();
        assert_eq!(count, 1);

        let count = exec_dao.del_inactive_by_element(&proc_inst.id, &element_id, &mut tran).await.unwrap();
        assert_eq!(count, 1);

        let proc_inst = create_test_procinst(&procdef, &mut tran).await;
        let count = exec_dao.delete(&proc_inst.id, &mut tran).await.unwrap();
        assert_eq!(count, 1);

        tran.rollback().await.unwrap();
    }

    pub async fn create_test_procinst(procdef: &ApfReProcdef, tran: &mut Transaction<'_, Postgres>) -> ApfRuExecution {
        let exec_dao = ApfRuExecutionDao::new();
        let new_exec = NewApfRuExecution {
            proc_def_id: procdef.id.to_owned(),
            is_active: 1,
            start_time: get_now(),
            ..Default::default()
        };
        let proc_inst = exec_dao.create_proc_inst(&new_exec, tran).await.unwrap();
        proc_inst
    }
}