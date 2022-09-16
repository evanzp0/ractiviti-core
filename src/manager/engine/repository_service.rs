use color_eyre::Result;
use tokio_postgres::Transaction;

use crate::dao::ApfGeBytearrayDao;
use crate::manager::engine::{BpmnManager, BpmnProcess};
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


    pub async fn load_bpmn_by_deployment<'a>(&self, deployment_id: &str, tran: &'a Transaction<'a>) -> Result<BpmnProcess> {
        let bytearray_dao = ApfGeBytearrayDao::new(tran);
        let ge_byte = bytearray_dao.get_by_deployment_id(deployment_id, tran).await?;

        let bpmn_definitions = BpmnManager::new()
            .parse_from_bytes(ge_byte.bytes.unwrap_or(Vec::new()))?;
        let bpmn_process = bpmn_definitions.process;

        Ok(bpmn_process)
    }
}

#[cfg(test)]
mod tests {


}
