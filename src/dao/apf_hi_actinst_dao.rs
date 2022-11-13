use color_eyre::Result;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Transaction;

use crate::error::{AppError, ErrorCode};
use crate::gen_id;
use crate::model::{ApfHiActinst, NewApfHiActinst};

use super::{BaseDao, Dao};

pub struct ApfHiActinstDao<'a> {
    base_dao: BaseDao<'a>
}

impl<'a> Dao for ApfHiActinstDao<'a> {
    fn tran(&self) -> &Transaction {
        self.base_dao.tran()
    }
}

impl<'a> ApfHiActinstDao<'a> {

    pub fn new(tran: &'a Transaction<'a>) -> Self {
        Self {
            base_dao: BaseDao::new(tran)
        }
    }

    pub async fn create(&self, obj: &NewApfHiActinst) -> Result<ApfHiActinst> {
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

        let stmt = self.tran().prepare(sql).await?;
        let row = self
            .tran()
            .query_one(
                &stmt, 
                &[
                    &obj.rev,
                    &obj.proc_def_id,
                    &obj.proc_inst_id,
                    &obj.execution_id,
                    &obj.task_id,
                    &obj.element_id,
                    &obj.element_name,
                    &obj.element_type,
                    &obj.start_user_id,
                    &obj.start_time,
                    &obj.end_time,
                    &obj.duration,
                    &new_id
                ]
            )
            .await?;
        let rst = ApfHiActinst::from_row(row)?;

        Ok(rst)
    }

    pub async fn mark_end( &self, execution_id: &str, element_id: &str,end_time: i64, end_user_id: Option<String>) -> Result<u64> {
        let hi_actinst = self.find_one_by_element_id(execution_id, element_id).await?;
        let duration = end_time - hi_actinst.start_time;

        let sql = r#"
            update apf_hi_actinst
            set rev = rev + 1,
                end_time = $1,
                duration = $2,
                end_user_id = $3
            where id = $4
                and rev = $5
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let r = self.tran()
            .execute(
                &stmt, 
                &[
                    &end_time,
                    &duration,
                    &end_user_id,
                    &&hi_actinst.id,
                    &hi_actinst.rev
                ]
            )
            .await?;
        if r != 1 {
            Err(
                AppError::new(
                    ErrorCode::InternalError,
                    Some(
                        &format!(
                            "apf_hi_actinst({}) is not updated correctly, affects ({}) != 1", 
                            hi_actinst.id, 
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

    #[allow(unused)]
    pub async fn get_by_id(&self, id: &str) -> Result<ApfHiActinst> {
        let sql = r#"
            select id, rev, proc_def_id, proc_inst_id, execution_id, 
                task_id, element_id, element_name, element_type,
                start_user_id, end_user_id, start_time, end_time, duration
            from apf_hi_actinst
            where id = $1
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let row = self.tran().query_one(&stmt, &[&id]).await?;
        let rst = ApfHiActinst::from_row(row)?;

        Ok(rst)
    }

    pub async fn find_one_by_element_id( &self, execution_id: &str, element_id: &str) -> Result<ApfHiActinst> {
        let sql = r#"
            select id, rev, proc_def_id, proc_inst_id, execution_id,
                task_id, element_id, element_name, element_type,
                start_user_id, end_user_id, start_time, end_time, duration
            from apf_hi_actinst
            where execution_id = $1
                and element_id = $2 
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let row = self.tran().query_one(&stmt, &[&execution_id, &element_id]).await?;
        let rst = ApfHiActinst::from_row(row)?;

        Ok(rst)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::common::db;
    use crate::dao::apf_ru_execution_dao::tests::create_test_procinst;
    use crate::dao::apf_ru_task_dao::tests::create_test_task;
    use crate::get_now;
    use crate::service::engine::tests::create_test_deploy;
    use crate::model::{ApfRuExecution, ApfRuTask};
    use super::*;

    #[tokio::test]
    async fn test_create_hi_actinst() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.transaction().await.unwrap();

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
        let tran = conn.transaction().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &tran).await;
        let proc_inst = create_test_procinst(&procdef, &tran).await;
        let task = create_test_task(&proc_inst, &tran).await;

        let hi_actinst = create_test_hi_actinst(&proc_inst, &task, &tran).await;

        let hi_act_dao = ApfHiActinstDao::new(&tran);

        let hi_actinst2 = hi_act_dao
            .find_one_by_element_id(&proc_inst.id, &task.element_id.unwrap())
            .await.unwrap();

        assert_eq!(hi_actinst, hi_actinst2);
        tran.rollback().await.unwrap();
    }

    async fn create_test_hi_actinst(proc_inst: &ApfRuExecution, task: &ApfRuTask, tran: &Transaction<'_>)
            -> ApfHiActinst {

        let now = Some(get_now());
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
        let hi_act_dao = ApfHiActinstDao::new(tran);
        let hi_actinst = hi_act_dao.create(&new_hi_actinst).await.unwrap();
        hi_actinst
    }
}