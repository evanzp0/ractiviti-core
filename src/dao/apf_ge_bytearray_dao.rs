use sqlx::{Postgres, Transaction};
use color_eyre::Result;
use validator::Validate;

use crate::{model::{ApfGeBytearray, NewApfGeBytearray}, gen_id};

pub struct ApfGeBytearrayDao;
impl ApfGeBytearrayDao {

    pub fn new() -> Self {
        Self{}
    }

    pub async fn get_by_id(&self, id: &str, tran: &mut Transaction<'_, Postgres>) -> Result<ApfGeBytearray> {
        let sql = r#"
            select id, name, deployment_id, bytes
            from apf_ge_bytearray
            where id = $1 
        "#;
        let rst = sqlx::query_as::<_, ApfGeBytearray>(sql)
            .bind(id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn get_by_deployment_id(&self, deployment_id: &str, tran: &mut Transaction<'_, Postgres>) -> Result<ApfGeBytearray> {
        let sql = r#"
            select id, name, deployment_id, bytes
            from apf_ge_bytearray
            where deployment_id = $1
        "#;
        let rst = sqlx::query_as::<_, ApfGeBytearray>(sql)
            .bind(deployment_id)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }

    pub async fn create(&self, obj: &NewApfGeBytearray, tran: &mut Transaction<'_, Postgres>) -> Result<ApfGeBytearray> {
        obj.validate()?;
        let sql = r#"
            insert into apf_ge_bytearray (id, name, deployment_id, bytes)
            values ($1, $2, $3, $4)
            returning *
        "#;
        let new_id = gen_id();

        let rst = sqlx::query_as::<_, ApfGeBytearray>(sql)
            .bind(&new_id)
            .bind(&obj.name)
            .bind(&obj.deployment_id)
            .bind(&obj.bytes)
            .fetch_one(&mut *tran)
            .await?;

        Ok(rst)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::Connection;
    use crate::boot::db;
    use crate::dao::ApfReDeploymentDao;
    use crate::model::{NewApfReDeployment};
    use super::*;

    #[actix_rt::test]
    async fn test_get_by_id() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let obj1 = create_test_bytearray(&mut tran).await.unwrap();

        let dao = ApfGeBytearrayDao::new();
        dao.get_by_id(&obj1.id, &mut tran).await.unwrap();

        tran.rollback().await.unwrap();
    }

    #[actix_rt::test]
    async fn test_get_by_deployment_id() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        let obj1 = create_test_bytearray(&mut tran).await.unwrap();

        let dao = ApfGeBytearrayDao::new();
        dao.get_by_deployment_id(&obj1.deployment_id, &mut tran).await.unwrap();

        tran.rollback().await.unwrap();
    }

    #[actix_rt::test]
    async fn test_create_deployment() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        create_test_bytearray(&mut tran).await.unwrap();

        tran.rollback().await.unwrap();
    }

    async fn create_test_bytearray(tran: &mut Transaction<'_, Postgres>) -> Result<ApfGeBytearray> {
        let obj = NewApfReDeployment {
            name: Some("test1".to_string()),
            key: Some("key1".to_string()),
            organization: None,
            deployer: None,
            new_bytearray: NewApfGeBytearray::new(),
        };

        let dao = ApfReDeploymentDao::new();
        let deployment = dao.create(&obj, tran).await?;

        let obj = NewApfGeBytearray {
            name: Some("test1".to_string()),
            deployment_id: Some(deployment.id),
            bytes: Some(b"abc".to_vec()),
        };

        let dao = ApfGeBytearrayDao::new();
        let rst = dao.create(&obj, &mut *tran).await?;
        Ok(rst)
    }
}