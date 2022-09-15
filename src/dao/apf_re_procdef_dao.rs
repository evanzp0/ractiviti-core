use sqlx::{Error, Postgres, Transaction};
use color_eyre::Result;
use crate::{model::{ApfReProcdef, NewApfReProcdef}, gen_id};

pub struct ApfReProcdefDao {}

impl ApfReProcdefDao {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_by_id(&self, id: &str, tran: &mut Transaction<'_, Postgres>) -> Result<ApfReProcdef> {
        let sql = r#"
            select id, rev, name, key, version, deployment_id, resource_name,
                description, suspension_state
            from apf_re_procdef
            where id = $1 
        "#;
        let rst = sqlx::query_as::<_, ApfReProcdef>(sql)
            .bind(id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn get_by_deplyment_id(&self, deployment_id: &str, tran: &mut Transaction<'_, Postgres>) -> Result<ApfReProcdef> {
        let sql = r#"
            select id, rev, name, key, version, deployment_id, resource_name,
                description, suspension_state
            from apf_re_procdef
            where deployment_id = $1 
        "#;
        let rst = sqlx::query_as::<_, ApfReProcdef>(sql)
            .bind(deployment_id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn get_lastest_by_key(&self, key: &str, tran: &mut Transaction<'_, Postgres>) -> Result<ApfReProcdef> {
        let sql = r#"
            select id, rev, name, key, version, 
                deployment_id, resource_name, description, suspension_state
            from apf_re_procdef
            where key = $1
                and suspension_state = 0
            order by version desc
        "#;
        let rst = sqlx::query_as::<_, ApfReProcdef>(sql)
            .bind(key)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn create(&self, obj: &NewApfReProcdef, tran: &mut Transaction<'_, Postgres>) -> Result<ApfReProcdef> {
        let sql = r#"
            select version 
            from apf_re_procdef 
            where key = $1 
            order by version desc limit 1
        "#;
        let rst = sqlx::query_scalar::<_, i32>(sql)
            .bind(&obj.key)
            .fetch_one(&mut *tran)
            .await;

        let mut version = 1;
        match rst {
            Ok(ver) => {
                version = ver + 1;
            }
            Err(error) => {
                match error {
                    Error::RowNotFound => {}
                    _ => {
                        Err(error)?
                    }
                }
            }
        }

        let sql = r#"
            insert into apf_re_procdef (
                name, rev, key, version, deployment_id, 
                resource_name, description, suspension_state, id
            ) values (
                $1, $2, $3, $4, $5, 
                $6, $7, $8, $9
            )
            returning *
        "#;
        let new_id = gen_id();

        let rst = sqlx::query_as::<_, ApfReProcdef>(sql)
            .bind(&obj.name)
            .bind(1)
            .bind(&obj.key)
            .bind(version)
            .bind(&obj.deployment_id)
            .bind(&obj.resource_name)
            .bind(&obj.description)
            .bind(&obj.suspension_state)
            .bind(new_id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }
}

#[cfg(test)]
mod tests{
    use sqlx::Connection;
    use crate::boot::db;
    use crate::dao::ApfReDeploymentDao;
    use crate::model::{NewApfGeBytearray, NewApfReDeployment, SuspensionState};
    use super::*;

    #[tokio::test]
    async fn test_create_and_get() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let dpl_dao = ApfReDeploymentDao::new();
        let new_dpl1 = NewApfReDeployment {
            name: Some("test1".to_string()),
            key: Some("key1".to_string()),
            organization: None,
            deployer: None,
            new_bytearray: NewApfGeBytearray::new(),
        };

        let deployment1 = dpl_dao.create(&new_dpl1, &mut tran).await.unwrap();

        let prcdef_dao = ApfReProcdefDao::new();
        let new_prcdef1 = NewApfReProcdef {
            name: Some("test1".to_string()),
            key: "test1_key".to_string(),
            deployment_id: deployment1.id,
            resource_name: None,
            description: None,
            suspension_state: SuspensionState::FALSE
        };

        let procdef1 = prcdef_dao.create(&new_prcdef1, &mut tran).await.unwrap();
        let procdef2 = prcdef_dao.get_by_id(&procdef1.id, &mut tran).await.unwrap();
        assert_eq!(procdef1, procdef2);

        let deployment2 = dpl_dao.create(&new_dpl1, &mut tran).await.unwrap();
        let new_prcdef2 = NewApfReProcdef {
            name: Some("test1".to_string()),
            key: "test1_key".to_string(),
            deployment_id: deployment2.id,
            resource_name: None,
            description: None,
            suspension_state: SuspensionState::FALSE
        };

        let procdef3 = prcdef_dao.create(&new_prcdef2, &mut tran).await.unwrap();
        let procdef4 = prcdef_dao.get_lastest_by_key(&procdef1.key, &mut tran).await.unwrap();
        assert_eq!(procdef3, procdef4);
        assert_ne!(procdef1, procdef4);
        assert!(procdef2.version < procdef4.version);

        let procdef5 = prcdef_dao.get_by_deplyment_id(&procdef4.deployment_id, &mut tran).await.unwrap();
        assert_eq!(procdef4, procdef5);
        tran.rollback().await.unwrap();

    }
}