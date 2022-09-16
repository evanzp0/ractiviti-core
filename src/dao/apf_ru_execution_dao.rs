use chrono::NaiveDateTime;
use color_eyre::Result;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Transaction;

use crate::error::{AppError, ErrorCode};
use crate::{model::{ApfRuExecution, NewApfRuExecution}, gen_id};
use super::{BaseDao, Dao};

pub struct ApfRuExecutionDao<'a> {
    base_dao: BaseDao<'a>
}

impl<'a> Dao<'a> for ApfRuExecutionDao<'a> {
    fn new(tran: &'a Transaction<'a>) -> Self {
        Self {
            base_dao: BaseDao::new(tran)
        }
    }

    fn tran(&self) -> &Transaction {
        self.base_dao.tran()
    }
}

impl<'a> ApfRuExecutionDao<'a> {
    pub async fn create(&self, obj: &NewApfRuExecution) -> Result<ApfRuExecution> {
        let sql = r#"
            insert into apf_ru_execution (
                rev, proc_inst_id, business_key, parent_id, proc_def_id, 
                root_proc_inst_id, element_id, is_active, start_time, start_user,
                id
            ) values (
                $1, $2, $3, $4, $5, 
                $6, $7, $8, $9, $10,
                $11
            )
            returning *
        "#;
        let new_id = gen_id();
        let rev:i32 = 1;
        let stmt = self.tran().prepare(sql).await?;
        let row = self.tran().query_one(
            &stmt, 
            &[
                &rev,
                &obj.proc_inst_id,
                &obj.business_key,
                &obj.parent_id,
                &obj.proc_def_id,
                &obj.root_proc_inst_id,
                &obj.element_id,
                &obj.is_active,
                &obj.start_time,
                &obj.start_user,
                &new_id,
            ]
        )
        .await?;
        let rst = ApfRuExecution::from_row(row)?;

        Ok(rst)
    }

    pub async fn create_proc_inst(&self, obj: &NewApfRuExecution) -> Result<ApfRuExecution> {
        let proc_inst = self.create(obj).await?;

        let sql = r#"
            update apf_ru_execution
            set proc_inst_id = $1,
                root_proc_inst_id = $2
            where id = $3
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let r = self.tran().execute(&stmt, &[&proc_inst.id, &proc_inst.id, &proc_inst.id]).await?;

        if r != 1 {
            Err(
                AppError::new(
                    ErrorCode::NotFound, 
                    Some(&format!("apf_ru_execution({}) is not updated", proc_inst.id)), 
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?
        }

        let rst = self.get_by_id(&proc_inst.id).await?;

        Ok(rst)
    }

    pub async fn mark_begin(
        &self,
        id: &str,
        element_id: &str,
        start_user: Option<String>,
        start_time: NaiveDateTime
    ) -> Result<()> {
        let ru_exection = self.get_by_id(id).await?;

        let sql = r#"
            update apf_ru_execution 
            set element_id = $1,
                start_time = $2,
                start_user = $3,
                rev = rev + 1
            where id = $4
                and rev = $5"#;
        let stmt = self.tran().prepare(sql).await?;
        let r = self.tran().execute(&stmt, &[&element_id, &start_time, &start_user, &id, &ru_exection.rev]).await?;
            
        if r != 1 {
            Err(
                AppError::new(
                    ErrorCode::InternalError, 
                    Some(
                        &format!("apf_ru_execution({}) is not updated correctly, affects ({}) != 1", ru_exection.id, r)
                    ), 
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?
        }

        Ok(())
    }

    pub async fn deactive_execution(&self, id: &str) -> Result<()> {
        let current_exec = self.get_by_id(id).await?;

        let sql = r#"
            update apf_ru_execution
            set is_active = 0,
                rev = $1
            where id = $2
                and rev = $3
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let r = self.tran().execute(&stmt, &[&(current_exec.rev + 1), &current_exec.id, &current_exec.rev]).await?;

        if r != 1 {
            Err(
                AppError::new(
                    ErrorCode::InternalError, 
                    Some(&format!("apf_ru_execution({}) is not updated correctly, affects ({}) != 1", current_exec.id, r)), 
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?
        }

        Ok(())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<ApfRuExecution> {
        let sql = r#"
            select id, rev, proc_inst_id, business_key, parent_id, 
                proc_def_id, root_proc_inst_id, element_id, is_active, start_time, 
                start_user
            from apf_ru_execution 
            where id = $1
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let row = self.tran().query_one(&stmt, &[&id]).await?;
        let rst = ApfRuExecution::from_row(row)?;

        Ok(rst)
    }

    pub async fn count_inactive_by_element(&self, proc_inst_id: &str, element_id: &str) -> Result<i64> {
        let sql = r#"
            select count(id) 
            from apf_ru_execution
            where proc_inst_id = $1
                and element_id = $2
                and is_active = 0
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let row = self.tran().query_one(&stmt, &[&proc_inst_id, &element_id]).await?;
        let rst: i64 = row.get(0);

        Ok(rst)
    }

    pub async fn del_inactive_by_element(&self, proc_inst_id: &str, element_id: &str) -> Result<u64> {
        let sql = r#"
            delete from apf_ru_execution
            where proc_inst_id = $1
                and element_id = $2
                and is_active = 0
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let r = self.tran().execute(&stmt, &[&proc_inst_id, &element_id]).await?;

        Ok(r)
    }

    pub async fn delete(&self, id: &str) -> Result<u64> {
        let sql = "delete from apf_ru_execution where id = $1 ";
        let stmt = self.tran().prepare(sql).await?;
        let r = self.tran().execute(&stmt, &[&id]).await?;

        Ok(r)
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::{Arc, RwLock};
    use crate::boot::db;
    use crate::get_now;
    use crate::manager::engine::tests::create_test_deploy;
    use crate::model::{ApfReProcdef};
    use super::*;

    #[tokio::test]
    async fn test_create_proc_inst() {
        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &tran).await;
        let proc_inst = create_test_procinst(&procdef, &tran).await;
        assert_eq!(proc_inst.id, proc_inst.proc_inst_id.clone().unwrap());
        assert_eq!(proc_inst.id, proc_inst.root_proc_inst_id.clone().unwrap());
        assert_eq!(proc_inst.parent_id, None);
        assert_eq!(proc_inst.is_active, 1);

        let exec_dao = ApfRuExecutionDao::new(&tran);
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

        exec_dao.mark_begin(&current_exec_id, element_id, start_user, start_time).await.unwrap();

        exec_dao.deactive_execution(&current_exec.read().unwrap().id).await.unwrap();

        let proc_inst_id = current_exec.read().unwrap().proc_inst_id().unwrap();
        let element_id = current_exec.read().unwrap().element_id().unwrap();
        let count = exec_dao.count_inactive_by_element(&proc_inst_id, &element_id).await.unwrap();
        assert_eq!(count, 1);

        let count = exec_dao.del_inactive_by_element(&proc_inst.id, &element_id).await.unwrap();
        assert_eq!(count, 1);

        let proc_inst = create_test_procinst(&procdef, &tran).await;
        let count = exec_dao.delete(&proc_inst.id).await.unwrap();
        assert_eq!(count, 1);

        tran.rollback().await.unwrap();
    }

    pub async fn create_test_procinst<'a>(procdef: &ApfReProcdef, tran: &Transaction<'a>) -> ApfRuExecution {
        let exec_dao = ApfRuExecutionDao::new(tran);
        let new_exec = NewApfRuExecution {
            proc_def_id: procdef.id.to_owned(),
            is_active: 1,
            start_time: get_now(),
            ..Default::default()
        };
        let proc_inst = exec_dao.create_proc_inst(&new_exec).await.unwrap();
        proc_inst
    }

}