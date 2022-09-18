use color_eyre::Result;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Transaction;
use crate::{model::{ApfRuIdentitylink, NewApfRuIdentitylink}, gen_id};

use super::{BaseDao, Dao};

pub struct ApfRuIdentitylinkDao<'a> {
    base_dao: BaseDao<'a>
}

impl<'a> Dao for ApfRuIdentitylinkDao<'a> {

    fn tran(&self) -> &Transaction {
        self.base_dao.tran()
    }
}

impl<'a> ApfRuIdentitylinkDao<'a> {

    pub fn new(tran: &'a Transaction<'a>) -> Self {
        Self {
            base_dao: BaseDao::new(tran)
        }
    }

    pub async fn create(&self, obj: &NewApfRuIdentitylink) -> Result<ApfRuIdentitylink> {
        let sql = r#"
            insert into apf_ru_identitylink (
                rev, ident_type, group_id, user_id, task_id, 
                proc_inst_id, proc_def_id, id
            ) values (
                $1, $2, $3, $4, $5, 
                $6, $7, $8
            )
            returning *
        "#;

        let new_id = gen_id();
        let rev:i32 = 1;
        let stmt = self.tran().prepare(sql).await?;
        let row = self
            .tran()
            .query_one(
                &stmt, 
                &[
                    &rev,
                    &obj.ident_type,
                    &obj.group_id,
                    &obj.user_id,
                    &obj.task_id,
                    &obj.proc_inst_id,
                    &obj.proc_def_id,
                    &new_id,
                ]
            )
            .await?;
        let rst = ApfRuIdentitylink::from_row(row)?;

        Ok(rst)
    }

    pub async fn delete_by_task_id(&self, task_id: &str)
            -> Result<u64> {
        let sql = r#" delete from apf_ru_identitylink where task_id = $1"#;
        let stmt = self.tran().prepare(sql).await?;
        let r = self.tran().execute(&stmt, &[&task_id]).await?;

        Ok(r)
    }

}

#[cfg(test)]
pub mod tests {
    use crate::common::db;
    use crate::dao::apf_ru_execution_dao::tests::create_test_procinst;
    use crate::dao::apf_ru_task_dao::tests::create_test_task;
    use crate::manager::engine::tests::create_test_deploy;
    use crate::model::{ApfRuTask, IdentType};

    use super::*;

    #[tokio::test]
    async fn test_create_and_delete() {
        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &tran).await;
        let proc_inst = create_test_procinst(&procdef, &tran).await;
        let task = create_test_task(&proc_inst, &tran).await;

        let ru_ident = create_test_apf_ru_identitylink(&task, &tran).await;
        assert_eq!(ru_ident.ident_type, IdentType::group);

        let ru_ident_dao = ApfRuIdentitylinkDao::new(&tran);
        let rst = ru_ident_dao.delete_by_task_id(&task.id).await.unwrap();
        assert_eq!(rst, 1);

        tran.rollback().await.unwrap();
    }

    pub async fn create_test_apf_ru_identitylink(task: &ApfRuTask, tran: &Transaction<'_>) -> ApfRuIdentitylink {

        let obj = NewApfRuIdentitylink {
            ident_type: IdentType::group,
            group_id: Some("test_group_id".to_owned()),
            user_id: None,
            task_id: Some(task.id.clone()),
            proc_inst_id: Some(task.proc_inst_id.clone()),
            proc_def_id: Some(task.proc_def_id.clone()),
        };

        let ru_ident_dao = ApfRuIdentitylinkDao::new(tran);
        let rst = ru_ident_dao.create(&obj).await.unwrap();

        rst
    }
}
