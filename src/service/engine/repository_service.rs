use color_eyre::Result;
use dysql::{PageDto, Pagination};
use tokio_postgres::Transaction;

use crate::common::db;
use crate::dao::{ApfGeBytearrayDao, ApfReDeploymentDao};
use crate::dto::DeploymentDto;
use crate::model::ApfReDeployment;
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

    pub async fn query_deployment_by_page(&self, pg_dto: &PageDto<DeploymentDto>) -> Result<Pagination<ApfReDeployment>> {
        let mut conn = db::get_connect().await?;
        let tran = conn.transaction().await?;

        let deployment_dao = ApfReDeploymentDao::new(&tran);
        let pg_deployment = deployment_dao.query_by_page(pg_dto).await?;

        Ok(pg_deployment)
    }
}

#[cfg(test)]
mod tests {

}
