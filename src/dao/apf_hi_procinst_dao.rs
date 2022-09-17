use chrono::NaiveDateTime;
use color_eyre::Result;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Transaction;

use crate::error::{AppError, ErrorCode};
use crate::model::{ApfHiProcinst, NewApfHiProcinst};
use super::{BaseDao, Dao};

pub struct ApfHiProcinstDao<'a> {
    base_dao: BaseDao<'a>
}

impl<'a> Dao for ApfHiProcinstDao<'a> {
    
    fn tran(&self) -> &Transaction {
        self.base_dao.tran()
    }
}

impl<'a> ApfHiProcinstDao<'a> {

    pub fn new(tran: &'a Transaction<'a>) -> Self {
        Self {
            base_dao: BaseDao::new(tran)
        }
    }

    pub async fn create(&self, obj: &NewApfHiProcinst) -> Result<ApfHiProcinst> {
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
        let stmt = self.tran().prepare(sql).await?;
        let row = self
            .tran()
            .query_one(
                &stmt, 
                &[
                    &obj.id,
                    &obj.proc_inst_id,
                    &obj.business_key,
                    &obj.proc_def_id,
                    &obj.start_time,
                    &obj.start_user,
                    &obj.start_element_id,
                ]
            )
            .await?;
        let rst = ApfHiProcinst::from_row(row)?;

        Ok(rst)
    }

    pub async fn mark_end(&self, id: &str, end_element_id: &str, end_time: NaiveDateTime) -> Result<()> {
        let hi_procinst = self.get_by_id(id).await?;
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
        let stmt = self.tran().prepare(sql).await?;
        let r = self
            .tran()
            .execute(
                &stmt, 
                &[
                    &end_time,
                    &duration,
                    &end_element_id,
                    &id,
                    &hi_procinst.rev,
                ]
            )
            .await?;

        if r != 1 {
            Err(
                AppError::new(
                    ErrorCode::InternalError, 
                    Some(&format!("apf_hi_procinst({}) is not updated correctly, affects ({}) != 1", hi_procinst.id, r)), 
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?
        }

        Ok(())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<ApfHiProcinst> {
        let sql = r#"
            select id, rev, proc_inst_id, business_key,
                proc_def_id, start_time, start_user, start_element_id,
                end_time, duration, end_element_id
            from apf_hi_procinst
            where id = $1
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let row = self.tran().query_one( &stmt, &[&id]).await?;
        let rst = ApfHiProcinst::from_row(row)?;

        Ok(rst)
    }
}

#[cfg(test)]
mod tests {
    use crate::boot::db;
    use crate::dao::ApfRuExecutionDao;
    use crate::get_now;
    use crate::manager::engine::tests::create_test_deploy;
    use crate::model::NewApfRuExecution;
    use super::*;

    #[tokio::test]
    async fn test_create() {
        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &tran).await;
        let exec_dao = ApfRuExecutionDao::new(&tran);
        let new_exec = NewApfRuExecution {
            proc_def_id: procdef.id.to_owned(),
            is_active: 1,
            start_time: get_now(),
            ..Default::default()
        };
        let proc_inst = exec_dao.create_proc_inst(&new_exec).await.unwrap();
        let hi_procinst_dao = ApfHiProcinstDao::new(&tran);
        let hi_obj = NewApfHiProcinst {
            id: proc_inst.id.clone(),
            proc_inst_id: proc_inst.proc_def_id.clone(),
            business_key: proc_inst.business_key,
            proc_def_id: proc_inst.proc_def_id.clone(),
            start_time: proc_inst.start_time,
            start_user: proc_inst.start_user,
            start_element_id: proc_inst.element_id,
        };

        let hi_procinst = hi_procinst_dao.create(&hi_obj).await.unwrap();
        assert_eq!(hi_procinst.id, proc_inst.id.clone());

        hi_procinst_dao.mark_end(&proc_inst.id, "end_event_1", get_now()).await.unwrap();

        tran.rollback().await.unwrap();
    }

}