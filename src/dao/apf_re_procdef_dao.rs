use color_eyre::Result;
use dysql::{PageDto, Pagination};
use dysql_macro::{fetch_all, page, sql};
use tokio_pg_mapper::{FromTokioPostgresRow};
use tokio_postgres::Transaction;

use crate::{model::{ApfReProcdef, NewApfReProcdef}, gen_id, error::{AppError, ErrorCode}, dto::ProcdefDto};
use super::{BaseDao, Dao};

const SELECT_FROM: &str = "select t1.id, t1.rev, t1.name, t1.key, t1.version, t1.deployment_id, t1.resource_name,
    t1.description, t1.suspension_state, t1.deployer_id, t1.deployer_name, t1.company_id, t1.company_name, t2.deploy_time
    from apf_re_procdef t1
        join apf_re_deployment t2 on t2.id = t1.deployment_id
    where t1.is_deleted = 0";

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
        let sql = format!("{} {}", SELECT_FROM, "and t1.id = $1");

        let stmt = self.tran().prepare(&sql).await?;
        let row = self.tran().query_one(&stmt, &[&id]).await?;
        let rst = ApfReProcdef::from_row(row)?;

        Ok(rst)
    }

    pub async fn get_by_deplyment_id(&self, deployment_id: &str) -> Result<ApfReProcdef> {
        let sql = format!("{} {}", SELECT_FROM, "and t1.deployment_id = $1");

        let stmt = self.tran().prepare(&sql).await?;
        let row = self.tran().query_one(&stmt, &[&deployment_id]).await?;
        let rst = ApfReProcdef::from_row(row)?;

        Ok(rst)
    }

    pub async fn get_lastest_by_key(&self, key: &str, company_id: &str) -> Result<ApfReProcdef> {
        let where_sql = "and t1.key = $1
            and t1.company_id = $2
            and t1.suspension_state = 0
            order by t1.version desc
            limit 1";

        let sql = format!("{} {}", SELECT_FROM, where_sql);

        let stmt = self.tran().prepare(&sql).await?;
        let rows = self.tran().query(&stmt, &[&key, &company_id]).await?;

        if rows.len() == 0 {
            Err(
                AppError::new(
                    ErrorCode::NotFound, 
                    Some(&format!("apf_re_procdef(key:{}, company_id:{}) is not exist", key, company_id)), 
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?
        }

        let rst = ApfReProcdef::from_row_ref(&rows[0])?;
        Ok(rst)
    }

    pub async fn create(&self, obj: &NewApfReProcdef) -> Result<ApfReProcdef> {
        let sql = r#"
            select version 
            from apf_re_procdef 
            where is_deleted = 0
            and key = $1 
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
                resource_name, description, suspension_state, id, deployer_id, 
                deployer_name, company_id, company_name
            ) values (
                $1, $2, $3, $4, $5, 
                $6, $7, $8, $9, $10, 
                $11, $12, $13
            )
            returning id
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
                    &obj.deployer_id,
                    &obj.deployer_name,
                    &obj.company_id,
                    &obj.company_name,
                ]
            )
            .await?;
        let id: String = row.get(0);
        let rst = self.get_by_id(&id).await?;
        
        Ok(rst)
    }

    sql!(
        "find_by_sql", 
        "select t1.id, t1.rev, t1.name, t1.key, t1.version, t1.deployment_id, t1.resource_name,
        t1.description, t1.suspension_state, t1.deployer_id, t1.deployer_name, t1.company_id, t1.company_name, t2.deploy_time
        from apf_re_procdef t1
        join apf_re_deployment t2 on t2.id = t1.deployment_id
        where t1.is_deleted = 0 "
    );

    pub async  fn find_by_dto(&self, proc_def_dto: &ProcdefDto) -> Result<Vec<ApfReProcdef>> {
        let tran = self.tran();
        let rst = fetch_all!(|proc_def_dto, tran| -> ApfReProcdef {
            find_by_sql + "
            {{#id}} and t1.id = :id {{/id}}
            {{#name}} and t1.name = :name {{/name}}
            {{#key}} and t1.key = :key {{/key}}
            {{#deployment_id}} and t1.deployment_id = :deployment_id {{/deployment_id}}
            {{#deployer_id}} and t1.deployer_id = :deployer_id {{/deployer_id}}
            {{#deployer_name}} and t1.deployer_name = :deployer_name {{/deployer_name}}
            {{#company_id}} and t1.company_id = :company_id {{/company_id}}
            {{#company_name}} and t1.company_name = :company_name {{/company_name}}
            "
        })?;

        Ok(rst)
    }

    pub async  fn query_by_page(&self, proc_def_dto: &mut PageDto<ProcdefDto>) -> Result<Pagination<ApfReProcdef>> {
        let tran = self.tran();
        let rst = page!(|proc_def_dto, tran| -> ApfReProcdef {
            find_by_sql + "
            {{#data}}
                {{#id}} and t1.id = :data.id {{/id}}
                {{#name}} and t1.name = :data.name {{/name}}
                {{#key}} and t1.key = :data.key {{/key}}
                {{#deployment_id}} and t1.deployment_id = :data.deployment_id {{/deployment_id}}
                {{#deployer_id}} and t1.deployer_id = :data.deployer_id {{/deployer_id}}
                {{#deployer_name}} and t1.deployer_name = :data.deployer_name {{/deployer_name}}
                {{#company_id}} and t1.company_id = :data.company_id {{/company_id}}
                {{#company_name}} and t1.company_name = :data.company_name {{/company_name}}
            {{/data}}
            {{#is_sort}}
                ORDER BY 
                    {{#sort_model}} {{field}} {{sort}}, {{/sort_model}}
                    ![B_DEL(,)]
            {{/is_sort}}
            "
        })?;

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
            name: "test1".to_string(),
            key: "key1".to_string(),
            company_id: "test_comp_1".to_owned(),
            company_name: "test_comp_1".to_owned(),
            deployer_id: "test_user_1".to_owned(),
            deployer_name: "test_user_1".to_owned(),
            new_bytearray: NewApfGeBytearray::new(),
            deploy_time: get_now(),
        };

        let deployment1 = dpl_dao.create(&new_dpl1).await.unwrap();

        let prcdef_dao = ApfReProcdefDao::new(&tran);
        let new_prcdef1 = NewApfReProcdef {
            name: "test1".to_string(),
            key: "test1_key".to_string(),
            deployment_id: deployment1.id,
            resource_name: None,
            description: None,
            suspension_state: SuspensionState::FALSE,
            deployer_id: "test_user1".to_owned(),
            deployer_name: "test_user1".to_owned(),
            company_id: "test_comp1".to_owned(),
            company_name: "test_comp1".to_owned(),
        };

        let procdef1 = prcdef_dao.create(&new_prcdef1).await.unwrap();
        let procdef2 = prcdef_dao.get_by_id(&procdef1.id).await.unwrap();
        assert_eq!(procdef1, procdef2);

        let deployment2 = dpl_dao.create(&new_dpl1).await.unwrap();
        let new_prcdef2 = NewApfReProcdef {
            name: "test1".to_string(),
            key: "test1_key".to_string(),
            deployment_id: deployment2.id,
            resource_name: None,
            description: None,
            suspension_state: SuspensionState::FALSE,
            deployer_id: "test_user1".to_owned(),
            deployer_name: "test_user1".to_owned(),
            company_id: "test_comp1".to_owned(),
            company_name: "test_comp1".to_owned(),
        };

        let procdef3 = prcdef_dao.create(&new_prcdef2).await.unwrap();
        let procdef4 = prcdef_dao.get_lastest_by_key(&procdef1.key, &procdef1.company_id).await.unwrap();
        assert_eq!(procdef3, procdef4);
        assert_ne!(procdef1, procdef4);
        assert!(procdef2.version < procdef4.version);

        let procdef5 = prcdef_dao.get_by_deplyment_id(&procdef4.deployment_id).await.unwrap();
        assert_eq!(procdef4, procdef5);
        tran.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_find_by_dto() {
        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let prcdef_dao = ApfReProcdefDao::new(&tran);
        let proc_def_dto = ProcdefDto {
            id: Some("1".to_owned()),
            name: Some("1".to_owned()),
            key: Some("1".to_owned()),
            deployment_id: Some("1".to_owned()),
            deployer_id: Some("1".to_owned()),
            deployer_name: Some("1".to_owned()),
            company_id: Some("1".to_owned()),
            company_name: Some("1".to_owned()),
        };
        let rst = prcdef_dao.find_by_dto(&proc_def_dto).await.unwrap();
        assert_eq!(0, rst.len());

        tran.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_query_by_page() {
        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        let prcdef_dao = ApfReProcdefDao::new(&tran);
        let proc_def_dto = ProcdefDto {
            id: Some("1".to_owned()),
            name: None,
            key: Some("1".to_owned()),
            deployment_id: Some("1".to_owned()),
            deployer_id: Some("1".to_owned()),
            deployer_name: Some("1".to_owned()),
            company_id: Some("1".to_owned()),
            company_name: Some("1".to_owned()),
        };
        let mut pg_dto = PageDto::new(2, 0, proc_def_dto);
        let rst = prcdef_dao.query_by_page(&mut pg_dto).await.unwrap();
        assert_eq!(0, rst.total);

        tran.rollback().await.unwrap();
    }
}