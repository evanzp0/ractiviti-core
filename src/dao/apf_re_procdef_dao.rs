use color_eyre::Result;
use tokio_pg_mapper::{FromTokioPostgresRow};
use tokio_postgres::Transaction;

use crate::{model::{ApfReProcdef, NewApfReProcdef}, gen_id, error::{AppError, ErrorCode}};
use super::{BaseDao, Dao};

pub struct ApfReProcdefDao<'a> {
    base_dao: BaseDao<'a>
}

impl<'a> Dao for ApfReProcdefDao<'a> {

    fn tran(&self) -> &Transaction {
        self.base_dao.tran()
    }
}

impl<'a> ApfReProcdefDao<'a> {

    pub fn new(tran: &'a Transaction<'a>) -> Self {
        let base_dao = BaseDao::new(tran);

        Self {
            base_dao
        }
    }
    
    pub async fn get_by_id(&self, id: &str) -> Result<ApfReProcdef> {
        let sql = r#"
            select id, rev, name, key, version, deployment_id, resource_name,
                description, suspension_state
            from apf_re_procdef
            where id = $1 
        "#;

        let stmt = self.tran().prepare(sql).await?;
        let row = self.tran().query_one(&stmt, &[&id]).await?;
        let rst = ApfReProcdef::from_row(row)?;

        Ok(rst)
    }

    pub async fn get_by_deplyment_id(&self, deployment_id: &str) -> Result<ApfReProcdef> {
        let sql = r#"
            select id, rev, name, key, version, deployment_id, resource_name,
                description, suspension_state
            from apf_re_procdef
            where deployment_id = $1 
        "#;

        let stmt = self.tran().prepare(sql).await?;
        let row = self.tran().query_one(&stmt, &[&deployment_id]).await?;
        let rst = ApfReProcdef::from_row(row)?;

        Ok(rst)
    }

    pub async fn get_lastest_by_key(&self, key: &str) -> Result<ApfReProcdef> {
        let sql = r#"
            select id, rev, name, key, version, 
                deployment_id, resource_name, description, suspension_state
            from apf_re_procdef
            where key = $1
                and suspension_state = 0
            order by version desc
            limit 1
        "#;

        let stmt = self.tran().prepare(sql).await?;
        let row = self.tran().query(&stmt, &[&key]).await?;
        let rst = ApfReProcdef::from_row_ref(&row[0])?;

        Ok(rst)
    }

    pub async fn create(&self, obj: &NewApfReProcdef) -> Result<ApfReProcdef> {
        let sql = r#"
            select version 
            from apf_re_procdef 
            where key = $1 
            order by version desc limit 1
        "#;
        let stmt = self.tran().prepare(sql).await?;
        let rows = self.tran().query(&stmt, &[&obj.key]).await?;
        let mut version = 1;
        if rows.len() == 1 {
            let ver: i32 = rows[0].get(0);
            version = ver + 1;
        } else if rows.len() > 1 {
            Err(
                AppError::new(
                    ErrorCode::ParseError, 
                    Some(&format!("apf_re_procdef 中 key({}) 对应的记录数超过 1.", &obj.key)), 
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?
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
        let rev: i32 = 1;
        let stmt = self.tran().prepare(sql).await?;
        let row = self
            .tran()
            .query_one(
                &stmt, 
                &[
                    &obj.name,
                    &rev,
                    &obj.key,
                    &version,
                    &obj.deployment_id,
                    &obj.resource_name,
                    &obj.description,
                    &obj.suspension_state,
                    &new_id,
                ]
            )
            .await?;
        let rst = ApfReProcdef::from_row(row)?;
        
        Ok(rst)
    }
}

#[cfg(test)]
mod tests{
    use crate::common::db;
    use crate::dao::ApfReDeploymentDao;
    use crate::get_now;
    use crate::model::{NewApfGeBytearray, NewApfReDeployment, SuspensionState};
    use super::*;

    #[tokio::test]
    async fn test_create_and_get() {
        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let dpl_dao = ApfReDeploymentDao::new(&tran);
        let new_dpl1 = NewApfReDeployment {
            name: Some("test1".to_string()),
            key: Some("key1".to_string()),
            organization: Some("test_comp_1".to_owned()),
            deployer: Some("test_user_1".to_owned()),
            new_bytearray: NewApfGeBytearray::new(),
            deploy_time: get_now(),
        };

        let deployment1 = dpl_dao.create(&new_dpl1).await.unwrap();

        let prcdef_dao = ApfReProcdefDao::new(&tran);
        let new_prcdef1 = NewApfReProcdef {
            name: Some("test1".to_string()),
            key: "test1_key".to_string(),
            deployment_id: deployment1.id,
            resource_name: None,
            description: None,
            suspension_state: SuspensionState::FALSE
        };

        let procdef1 = prcdef_dao.create(&new_prcdef1).await.unwrap();
        let procdef2 = prcdef_dao.get_by_id(&procdef1.id).await.unwrap();
        assert_eq!(procdef1, procdef2);

        let deployment2 = dpl_dao.create(&new_dpl1).await.unwrap();
        let new_prcdef2 = NewApfReProcdef {
            name: Some("test1".to_string()),
            key: "test1_key".to_string(),
            deployment_id: deployment2.id,
            resource_name: None,
            description: None,
            suspension_state: SuspensionState::FALSE
        };

        let procdef3 = prcdef_dao.create(&new_prcdef2).await.unwrap();
        let procdef4 = prcdef_dao.get_lastest_by_key(&procdef1.key).await.unwrap();
        assert_eq!(procdef3, procdef4);
        assert_ne!(procdef1, procdef4);
        assert!(procdef2.version < procdef4.version);

        let procdef5 = prcdef_dao.get_by_deplyment_id(&procdef4.deployment_id).await.unwrap();
        assert_eq!(procdef4, procdef5);
        tran.rollback().await.unwrap();
    }
}