use color_eyre::Result;
use sqlx::{Postgres, Transaction};

use crate::model::{ApfHiIdentitylink, ApfRuIdentitylink, NewApfHiIdentitylink};

pub struct ApfHiIdentitylinkDao {

}

impl ApfHiIdentitylinkDao {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create(&self, obj: &NewApfHiIdentitylink, tran: &mut Transaction<'_, Postgres>) -> Result<ApfHiIdentitylink> {
        let sql = r#"
            insert into apf_hi_identitylink
                (rev, id, ident_type, group_id, user_id, task_id, proc_inst_id, proc_def_id)
            values
                ($1, $2, $3, $4, $5, $6, $7,$8)
            returning *"#;

        let rst = sqlx::query_as::<_, ApfHiIdentitylink>(sql)
            .bind(1)
            .bind(&obj.id)
            .bind(&obj.ident_type)
            .bind(&obj.group_id)
            .bind(&obj.user_id)
            .bind(&obj.task_id)
            .bind(&obj.proc_inst_id)
            .bind(&obj.proc_def_id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn create_from_ident_link(&self, ident_link: &ApfRuIdentitylink, tran: &mut Transaction<'_, Postgres>) -> Result<ApfHiIdentitylink> {
        let new_hi_ident = NewApfHiIdentitylink {
            id: ident_link.id.clone(),
            ident_type: ident_link.ident_type.clone(),
            group_id: ident_link.group_id.clone(),
            user_id: ident_link.user_id.clone(),
            task_id: ident_link.task_id.clone(),
            proc_inst_id: ident_link.proc_inst_id.clone(),
            proc_def_id: ident_link.proc_def_id.clone(),
        };

        let rst = self.create(&new_hi_ident, tran).await?;
        Ok(rst)
    }
}

#[cfg(test)]
pub mod tests {
    use sqlx::Acquire;
    use crate::boot::db;
    use crate::dao::apf_ru_execution_dao::tests::create_test_procinst;
    use crate::dao::apf_ru_identitylink_dao::tests::create_test_apf_ru_identitylink;
    use crate::dao::apf_ru_task_dao::tests::create_test_task;
    use crate::manager::engine::tests::create_test_deploy;
    use crate::model::IdentType;

    use super::*;

    #[actix_rt::test]
    async fn test_create_and_delete() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &mut tran).await;
        let proc_inst = create_test_procinst(&procdef, &mut tran).await;
        let task = create_test_task(&proc_inst, &mut tran).await;

        let ru_ident = create_test_apf_ru_identitylink(&task, &mut tran).await;
        assert_eq!(ru_ident.ident_type, IdentType::group);

        let hi_ident_dao = ApfHiIdentitylinkDao::new();
        let rst = hi_ident_dao.create_from_ident_link(&ru_ident, &mut tran).await.unwrap();
        assert_eq!(rst.id, ru_ident.id);

        tran.rollback().await.unwrap();
    }

}
