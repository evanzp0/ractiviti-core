use color_eyre::Result;
use dysql::{Pagination, PageDto};
use dysql_macro::page;
use tokio_pg_mapper::FromTokioPostgresRow;
use validator::Validate;
use tokio_postgres::Transaction;

use crate::{model::{ApfReDeployment, NewApfReDeployment}, gen_id, dto::DeploymentDto};
use super::base_dao::{BaseDao, Dao};

pub struct ApfReDeploymentDao<'a> {
    base_dao: BaseDao<'a>
}

impl<'a> Dao for ApfReDeploymentDao<'a> {

    fn tran(&self) -> &Transaction {
        self.base_dao.tran()
    }
}

impl<'a> ApfReDeploymentDao<'a> {

    pub fn new(tran: &'a Transaction<'a>) -> Self {
        let base_dao = BaseDao::new(tran);

        Self {
            base_dao
        }
    }

    pub async fn get_by_id(&self, id: &str) -> Result<ApfReDeployment> {
        let sql = r#"
            select id, name, key, organization, deployer, deploy_time 
            from apf_re_deployment
            where id = $1
        "#;

        let stmt = self.tran().prepare(sql).await?;
        let row = self.tran().query_one(&stmt, &[&id]).await?;
        let rst = ApfReDeployment::from_row(row)?;

        Ok(rst)
    }

    pub async fn create(&self, obj: &NewApfReDeployment) -> Result<ApfReDeployment> {
        obj.validate()?;
        let sql = r#"insert into apf_re_deployment (
                id, name, key, organization, deployer, deploy_time
            ) values (
                $1, $2, $3, $4, $5, $6
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
                    &new_id,
                    &obj.name,
                    &obj.key,
                    &obj.organization,
                    &obj.deployer,
                    &obj.deploy_time,
                ]
            )
            .await?;

        let rst = ApfReDeployment::from_row(row)?;

        Ok(rst)
    }

    pub async fn query_by_page(&self, pg_dto: &PageDto<DeploymentDto>) -> Result<Pagination<ApfReDeployment>> {
        let tran = self.tran();
        let pg_deployment = page!(|pg_dto, tran| -> ApfReDeployment {
            "SELECT id, name, key, organization, deployer, deploy_time 
            FROM apf_re_deployment
            WHERE 1 = 1
            {{#data}}
                {{#id}}and id = :data.id{{/id}}
                {{#name}}and name like '%' || :data.name || '%'{{/name}}
                {{#key}}and key = :data.key{{/key}}
                {{#organization}}and organization = :data.organization{{/organization}}
                {{#deployer}}and deployer = :data.deployer{{/deployer}}
                {{#deploy_time_from}}and deploy_time >= :data.deploy_time_from{{/deploy_time_from}}
                {{#deploy_time_to}}and deploy_time <= :data.deploy_time_to{{/deploy_time_to}}
            {{/data}}"
        })?;

        Ok(pg_deployment)
    }
}

#[cfg(test)]
mod tests {
    use log4rs_macros::debug;
    use crate::common::db;
    use crate::get_now;
    use crate::model::NewApfGeBytearray;
    use super::*;

    #[tokio::test]
    async fn test_create_and_get_by_id() {
        let mut conn =  db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let rst = create_test_deployment(&tran).await;

        if let Err(e) = rst {
            tran.rollback().await.unwrap();
            panic!("{:?}", e);
        }

        let deployment1 = rst.unwrap();
        let dao = ApfReDeploymentDao::new(&tran);
        let deployment2 = dao.get_by_id(&deployment1.id).await.unwrap();
        assert_eq!(deployment1, deployment2);

        debug!("{:?}", deployment2);

        tran.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_query_by_page() {
        let mut conn =  db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();
        let dao = ApfReDeploymentDao::new(&tran);
        let d_dto = DeploymentDto {
            id: Some("".to_owned()),
            name: Some("".to_owned()),
            key: Some("".to_owned()),
            organization: Some("".to_owned()),
            deployer: Some("".to_owned()),
            deploy_time_from: Some(1),
            deploy_time_to: Some(1),
        };
        let pg_dto = PageDto::new(2, 1, d_dto);
        let rst = dao.query_by_page(&pg_dto).await.unwrap();
        println!("{:?}", rst);

        tran.rollback().await.unwrap();
    }

    async fn create_test_deployment(tran: &Transaction<'_>)
            -> Result<ApfReDeployment> {
        let obj = NewApfReDeployment {
            name: Some("test1".to_string()),
            key: Some("key1".to_string()),
            organization: Some("test_comp_1".to_owned()),
            deployer: Some("test_user_1".to_owned()),
            new_bytearray: NewApfGeBytearray::new(),
            deploy_time: get_now(),
        };

        let dao = ApfReDeploymentDao::new(tran);
        let rst = dao.create(&obj).await?;

        Ok(rst)
    }
}