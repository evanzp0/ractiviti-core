use color_eyre::Result;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Transaction;
use validator::Validate;

use crate::{model::{ApfGeBytearray, NewApfGeBytearray}, gen_id};
use super::{BaseDao, Dao};

pub struct ApfGeBytearrayDao<'a> {
    base_dao: BaseDao<'a>
}

impl<'a> Dao<'a> for ApfGeBytearrayDao<'a> {
    fn new(tran: &'a Transaction<'a>) -> Self {
        Self {
            base_dao: BaseDao::new(tran)
        }
    }

    fn tran(&self) -> &Transaction {
        self.base_dao.tran()
    }
}

impl<'a> ApfGeBytearrayDao<'a> {
    pub async fn get_by_id(&self, id: &str) -> Result<ApfGeBytearray> {
        let sql = r#"
            select id, name, deployment_id, bytes
            from apf_ge_bytearray
            where id = $1 
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let row = self.tran().query_one(&stmt, &[&id]).await?;
        let rst = ApfGeBytearray::from_row(row)?;

        Ok(rst)
    }

    pub async fn get_by_deployment_id(&self, deployment_id: &str) -> Result<ApfGeBytearray> {
        let sql = r#"
            select id, name, deployment_id, bytes
            from apf_ge_bytearray
            where deployment_id = $1
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let row = self.tran().query_one(&stmt, &[&deployment_id]).await?;
        let rst = ApfGeBytearray::from_row(row)?;

        Ok(rst)
    }

    pub async fn create(&self, obj: &NewApfGeBytearray) -> Result<ApfGeBytearray> {
        obj.validate()?;
        let sql = r#"
            insert into apf_ge_bytearray (id, name, deployment_id, bytes)
            values ($1, $2, $3, $4)
            returning *
        "#;
        let new_id = gen_id();

        let stmt = self.tran().prepare(sql).await?;
        let row = self.tran().query_one(
            &stmt, 
            &[
                &new_id,
                &obj.name,
                &obj.deployment_id,
                &obj.bytes
            ]
        )
        .await?;

        let rst = ApfGeBytearray::from_row(row)?;

        return Ok(rst)
    }
}

#[cfg(test)]
mod tests {
    use tokio_postgres::Transaction;

    use crate::boot::db;
    use crate::dao::ApfReDeploymentDao;
    use crate::model::{NewApfReDeployment};
    use super::*;

    #[tokio::test]
    async fn test_get_by_id() {
        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let obj1 = create_test_bytearray(&tran).await.unwrap();

        let dao = ApfGeBytearrayDao::new(&tran);
        dao.get_by_id(&obj1.id).await.unwrap();

        tran.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_by_deployment_id() {
        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let obj1 = create_test_bytearray(&tran).await.unwrap();

        let dao = ApfGeBytearrayDao::new(&tran);
        dao.get_by_deployment_id(&obj1.deployment_id).await.unwrap();

        tran.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_create_deployment() {
        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        create_test_bytearray(&tran).await.unwrap();

        tran.rollback().await.unwrap();
    }

    async fn create_test_bytearray(tran: &Transaction<'_>) -> Result<ApfGeBytearray> {
        let obj = NewApfReDeployment {
            name: Some("test1".to_string()),
            key: Some("key1".to_string()),
            organization: None,
            deployer: None,
            new_bytearray: NewApfGeBytearray::new(),
        };

        let dao = ApfReDeploymentDao::new(tran);
        let deployment = dao.create(&obj).await?;

        let obj = NewApfGeBytearray {
            name: Some("test1".to_string()),
            deployment_id: Some(deployment.id),
            bytes: Some(b"abc".to_vec()),
        };

        let dao = ApfGeBytearrayDao::new(&tran);
        let rst = dao.create(&obj).await?;
        
        Ok(rst)
    }
}