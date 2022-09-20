use color_eyre::Result;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Transaction;

use crate::model::{ApfHiIdentitylink, ApfRuIdentitylink, NewApfHiIdentitylink};

use super::{BaseDao, Dao};

pub struct ApfHiIdentitylinkDao<'a> {
    base_dao: BaseDao<'a>
}

impl<'a> Dao for ApfHiIdentitylinkDao<'a> {

    fn tran(&self) -> &Transaction {
        self.base_dao.tran()
    }
}

impl<'a> ApfHiIdentitylinkDao<'a> {

    pub fn new(tran: &'a Transaction<'a>) -> Self {
        Self {
            base_dao: BaseDao::new(tran)
        }
    }
    
    pub async fn create(&self, obj: &NewApfHiIdentitylink) -> Result<ApfHiIdentitylink> {
        let sql = r#"
            insert into apf_hi_identitylink
                (rev, id, ident_type, group_id, user_id, task_id, proc_inst_id, proc_def_id)
            values
                ($1, $2, $3, $4, $5, $6, $7,$8)
            returning *"#;
            let rev:i32 = 1;
            let stmt = self.tran().prepare(sql).await?;
            let row = self
                .tran()
                .query_one(
                    &stmt, 
                    &[
                        &rev,
                        &obj.id,
                        &obj.ident_type,
                        &obj.group_id,
                        &obj.user_id,
                        &obj.task_id,
                        &obj.proc_inst_id,
                        &obj.proc_def_id,
                    ]
                )
                .await?;
            let rst = ApfHiIdentitylink::from_row(row)?;
    
            Ok(rst)
    }

    pub async fn create_from_ident_link(&self, ident_link: &ApfRuIdentitylink) -> Result<ApfHiIdentitylink> {
        let new_hi_ident = NewApfHiIdentitylink {
            id: ident_link.id.clone(),
            ident_type: ident_link.ident_type.clone(),
            group_id: ident_link.group_id.clone(),
            user_id: ident_link.user_id.clone(),
            task_id: ident_link.task_id.clone(),
            proc_inst_id: ident_link.proc_inst_id.clone(),
            proc_def_id: ident_link.proc_def_id.clone(),
        };

        let rst = self.create(&new_hi_ident).await?;
        Ok(rst)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::common::db;
    use crate::dao::apf_ru_execution_dao::tests::create_test_procinst;
    use crate::dao::apf_ru_identitylink_dao::tests::create_test_apf_ru_identitylink;
    use crate::dao::apf_ru_task_dao::tests::create_test_task;
    use crate::service::engine::tests::create_test_deploy;
    use crate::model::IdentType;

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

        let hi_ident_dao = ApfHiIdentitylinkDao::new(&tran);
        let rst = hi_ident_dao.create_from_ident_link(&ru_ident).await.unwrap();
        assert_eq!(rst.id, ru_ident.id);

        tran.rollback().await.unwrap();
    }

}
