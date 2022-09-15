use sqlx::{Postgres, Transaction};
use color_eyre::Result;
use crate::model::{ApfReDeployment, NewApfReDeployment};
use uuid::Uuid;
use validator::Validate;

pub struct ApfReDeploymentDao {}

impl ApfReDeploymentDao {

    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_by_id(&self, id: &Uuid, tran: &mut Transaction<'_, Postgres>)
            -> Result<ApfReDeployment> {
        let sql = "select id, name, key, organization, deployer, deploy_time \
                        from apf_re_deployment \
                        where id = $1 ";
        let rst = sqlx::query_as::<_, ApfReDeployment>(sql)
            .bind(&id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn create(&self, obj: &NewApfReDeployment, tran: &mut Transaction<'_, Postgres>)
                -> Result<ApfReDeployment> {
        obj.validate()?;
        let sql = "insert into apf_re_deployment \
                        (name, key, organization, deployer) \
                        values \
                        ($1, $2, $3, $4) \
                        returning * ";
        let rst = sqlx::query_as::<_, ApfReDeployment>(sql)
            .bind(&obj.name)
            .bind(&obj.key)
            .bind(&obj.organization)
            .bind(&obj.deployer)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }
}

#[cfg(test)]
mod tests {
    use log4rs::debug;
    use sqlx::Connection;
    use crate::boot::db;
    use crate::model::NewApfGeBytearray;
    use super::*;

    #[actix_rt::test]
    async fn test_create_and_get_by_id() {
        let mut conn =  db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let rst = create_test_deployment(&mut tran).await;

        if let Err(e) = rst {
            tran.rollback().await.unwrap();
            panic!("{:?}", e);
        }

        let deployment1 = rst.unwrap();
        let dao = ApfReDeploymentDao::new();
        let deployment2 = dao.get_by_id(&deployment1.id, &mut tran).await.unwrap();
        assert_eq!(deployment1, deployment2);

        debug!("{:?}", deployment2);

        tran.rollback().await.unwrap();
    }

    async fn create_test_deployment(tran: &mut Transaction<'_, Postgres>)
            -> Result<ApfReDeployment> {
        let obj = NewApfReDeployment {
            name: Some("test1".to_string()),
            key: Some("key1".to_string()),
            organization: None,
            deployer: None,
            new_bytearray: NewApfGeBytearray::new()
        };

        let dao = ApfReDeploymentDao::new();
        let rst = dao.create(&obj, tran).await?;

        Ok(rst)
    }
}