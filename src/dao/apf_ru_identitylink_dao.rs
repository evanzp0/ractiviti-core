use color_eyre::Result;
use sqlx::{Postgres, Transaction};
use crate::{model::{ApfRuIdentitylink, NewApfRuIdentitylink}, gen_id};

pub struct ApfRuIdentitylinkDao {

}

impl ApfRuIdentitylinkDao {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create(&self, obj: &NewApfRuIdentitylink, tran: &mut Transaction<'_, Postgres>) -> Result<ApfRuIdentitylink> {
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

        let rst = sqlx::query_as::<_, ApfRuIdentitylink>(sql)
            .bind(1)
            .bind(&obj.ident_type)
            .bind(&obj.group_id)
            .bind(&obj.user_id)
            .bind(&obj.task_id)
            .bind(&obj.proc_inst_id)
            .bind(&obj.proc_def_id)
            .bind(new_id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn delete_by_task_id(&self, task_id: &str, tran: &mut Transaction<'_, Postgres>)
            -> Result<u64> {
        let sql = "delete from apf_ru_identitylink \
                        where task_id = $1";
        let rst = sqlx::query(sql)
            .bind(task_id)
            .execute(&mut *tran)
            .await?;

        Ok(rst.rows_affected())
    }

}

#[cfg(test)]
pub mod tests {
    use sqlx::Acquire;
    use crate::boot::db;
    use crate::dao::apf_ru_execution_dao::tests::create_test_procinst;
    use crate::dao::apf_ru_task_dao::tests::create_test_task;
    use crate::manager::engine::tests::create_test_deploy;
    use crate::model::{ApfRuTask, IdentType};

    use super::*;

    #[tokio::test]
    async fn test_create_and_delete() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let procdef = create_test_deploy("bpmn/process1.bpmn.xml", &mut tran).await;
        let proc_inst = create_test_procinst(&procdef, &mut tran).await;
        let task = create_test_task(&proc_inst, &mut tran).await;

        let ru_ident = create_test_apf_ru_identitylink(&task, &mut tran).await;
        assert_eq!(ru_ident.ident_type, IdentType::group);

        let ru_ident_dao = ApfRuIdentitylinkDao::new();
        let rst = ru_ident_dao.delete_by_task_id(&task.id, &mut tran).await.unwrap();
        assert_eq!(rst, 1);

        tran.rollback().await.unwrap();
    }

    pub async fn create_test_apf_ru_identitylink(task: &ApfRuTask, tran: &mut Transaction<'_, Postgres>) -> ApfRuIdentitylink {

        let obj = NewApfRuIdentitylink {
            ident_type: IdentType::group,
            group_id: Some("test_group_id".to_owned()),
            user_id: None,
            task_id: Some(task.id.clone()),
            proc_inst_id: Some(task.proc_inst_id.clone()),
            proc_def_id: Some(task.proc_def_id.clone()),
        };

        let ru_ident_dao = ApfRuIdentitylinkDao::new();
        let rst = ru_ident_dao.create(&obj, tran).await.unwrap();

        rst
    }
}
