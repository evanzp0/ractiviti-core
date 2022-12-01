use color_eyre::Result;
use dysql::{PageDto, Pagination};
use tokio_postgres::Transaction;

use crate::common::{db, md5};
use crate::dao::{ApfGeBytearrayDao, ApfReDeploymentDao, ApfReProcdefDao};
use crate::dto::{DeploymentDto, BpmnResultDto, ProcdefDto};
use crate::error::{AppError, ErrorCode};
use crate::model::{ApfReDeployment, ApfReProcdef};
use crate::service::engine::{BpmnManager, BpmnProcess};
use super::DeploymentBuilder;

#[derive(Debug)]
pub struct RepositoryService{

}

#[allow(unused)]
impl RepositoryService {

    pub fn new() -> Self {
        Self {}
    }

    pub fn create_deployment_builder(&self) -> DeploymentBuilder {
        DeploymentBuilder::new()
    }

    pub async fn load_bpmn_by_deployment(&self, deployment_id: &str, tran: &Transaction<'_>) -> Result<BpmnProcess> {
        let bytearray_dao = ApfGeBytearrayDao::new(tran);
        let ge_byte = bytearray_dao.get_by_deployment_id(deployment_id).await?;

        let bpmn_definitions = BpmnManager::new()
            .parse_from_bytes(ge_byte.bytes.unwrap_or(Vec::new()))?;
        let bpmn_process = bpmn_definitions.process;

        Ok(bpmn_process)
    }

    pub async fn query_deployment_by_page(&self, pg_dto: &mut PageDto<DeploymentDto>) -> Result<Pagination<ApfReDeployment>> {
        let mut conn = db::get_connect().await?;
        let tran = conn.transaction().await?;

        let deployment_dao = ApfReDeploymentDao::new(&tran);
        let pg_deployment = deployment_dao.query_by_page(pg_dto).await?;

        Ok(pg_deployment)
    }

    pub async fn create_procdef(&self, bpmn_name: &str, deployer_id: &str, deployer_name: &str, company_id: &str, company_name: &str, bpmn_xml: &str) -> Result<ApfReProcdef> {
        let mut conn = db::get_connect().await?;
        let tran = conn.transaction().await?;

        let procdef_dao = ApfReProcdefDao::new(&tran);
        let procdef_key = md5(bpmn_name.to_lowercase());
        let bytes = bpmn_xml.as_bytes().to_vec();
        
        let lastest_procdef = procdef_dao.get_lastest_by_key(&procdef_key, company_id).await;
        if let Err(error) = lastest_procdef {
            let err = error.downcast_ref::<AppError>()
                .ok_or(AppError::internal_error(&format!("{} : {} , {}", file!(), line!(), error.to_string())))?; 

            if ErrorCode::NotFound == err.code {
                let builder = DeploymentBuilder::new();
                let deployment = builder
                    .name(bpmn_name)
                    .key(&procdef_key)
                    .deployer_id(deployer_id)
                    .deployer_name(deployer_name)
                    .company_id(company_id)
                    .company_name(company_name)
                    .bytes(bytes)?
                    .deploy_with_tran(&tran)
                    .await?;

                let procdef = procdef_dao.get_by_deplyment_id(&deployment.id).await?;
                tran.commit().await?;

                return Ok(procdef);
            }
        }

        tran.rollback().await?;
        Err(AppError::new_for_input_err(Some("流程名称已存在无法创建"), "bpmn_name"))?
    } 

    pub async fn publish_procdef(&self, procdef: &ApfReProcdef, deployer_id: &str, deployer_name: &str, bpmn_xml: &str) -> Result<ApfReProcdef> {
        let mut conn = db::get_connect().await?;
        let tran = conn.transaction().await?;

        let procdef_dao = ApfReProcdefDao::new(&tran);
        let bytes = bpmn_xml.as_bytes().to_vec();
        

        let builder = DeploymentBuilder::new();
        let deployment = builder
            .name(&procdef.name)
            .key(&procdef.key)
            .deployer_id(deployer_id)
            .deployer_name(deployer_name)
            .company_id(&procdef.company_id)
            .company_name(&procdef.company_name)
            .bytes(bytes)?
            .deploy_with_tran(&tran)
            .await?;

        let procdef = procdef_dao.get_by_deplyment_id(&deployment.id).await?;
        tran.commit().await?;

        return Ok(procdef);
    } 

    pub async fn get_bpmn_by_procdef_id(&self, procdef_id: &str) -> Result<BpmnResultDto> {
        let mut conn = db::get_connect().await?;
        let tran = conn.transaction().await?;

        let procdef_dao = ApfReProcdefDao::new(&tran);
        let procdef = procdef_dao.get_by_id(procdef_id).await?;

        let bytearray_dao = ApfGeBytearrayDao::new(&tran);
        let bytearray = bytearray_dao.get_by_deployment_id(&procdef.deployment_id).await?;

        let bytes = if let Some(b) = bytearray.bytes {
            b
        } else {
            Err(
                AppError::new(
                    ErrorCode::InternalError,
                    Some(
                        &format!(
                            "apf_ge_bytearray({}) bytes is null", 
                            bytearray.id
                        )
                    ), 
                    concat!(file!(), ":", line!()),
                    None
                )
            )?
        };
        
        let xml = String::from_utf8(bytes)?;
        let dto = BpmnResultDto {
            bpmn_id: procdef.id,
            bpmn_key: procdef.key,
            bpmn_name: procdef.name,
            xml: Some(xml),
        };

        Ok(dto)
    }

    pub async fn get_procdef_by_id(&self, procdef_id: &str) -> Result<ApfReProcdef> {
        let mut conn = db::get_connect().await?;
        let tran = conn.transaction().await?;

        let procdef_dao = ApfReProcdefDao::new(&tran);
        let procdef = procdef_dao.get_by_id(procdef_id).await?;

        Ok(procdef)
    }

    pub async fn query_procdef_by_page(&self, pg_dto: &mut PageDto<ProcdefDto>) -> Result<Pagination<ApfReProcdef>> {
        let mut conn = db::get_connect().await?;
        let tran = conn.transaction().await?;

        let procdef_dao = ApfReProcdefDao::new(&tran);
        let pg_deployment = procdef_dao.query_by_page(pg_dto).await?;

        Ok(pg_deployment)
    }

    pub async fn delete_procdef_by_id(&self, procdef_id: &str, user_id: &str) -> Result<()> {
        let mut conn = db::get_connect().await?;
        let tran = conn.transaction().await?;

        let procdef_dao = ApfReProcdefDao::new(&tran);
        procdef_dao.delete_by_id(procdef_id, user_id).await?;

        tran.commit().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

}
