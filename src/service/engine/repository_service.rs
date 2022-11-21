use color_eyre::Result;
use dysql::{PageDto, Pagination};
use tokio_postgres::Transaction;

use crate::common::{db, md5};
use crate::dao::{ApfGeBytearrayDao, ApfReDeploymentDao, ApfReProcdefDao};
use crate::dto::DeploymentDto;
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

    pub async fn create_procdef(bpmn_name: &str, deployer_id: &str, company_id: &str, bytes: Vec<u8>) -> Result<ApfReProcdef> {
        let mut conn = db::get_connect().await?;
        let tran = conn.transaction().await?;

        let procdef_dao = ApfReProcdefDao::new(&tran);
        let procdef_key = md5(bpmn_name);
        
        let lastest_procdef = procdef_dao.get_lastest_by_key(&procdef_key, company_id).await;
        if let Err(error) = lastest_procdef {
            let err = error.downcast_ref::<AppError>()
                .ok_or(AppError::internal_error(&format!("{} : {} , {}", file!(), line!(), error.to_string())))?; 

            if ErrorCode::NotFound == err.code {
                let builder = DeploymentBuilder::new();
                let deployment = builder
                    .name(bpmn_name)
                    .deployer_id(deployer_id)
                    .company_id(company_id)
                    .bytes(bytes)?
                    .deploy_with_tran(&tran)
                    .await?;

                let procdef = procdef_dao.get_by_deplyment_id(&deployment.id).await;

                return procdef;
            }
        }

        Err(AppError::new(ErrorCode::ResourceExist, Some("流程名称已存在无法创建"), concat!(file!(), ":", line!()), None))?
    } 
}

#[cfg(test)]
mod tests {

}
