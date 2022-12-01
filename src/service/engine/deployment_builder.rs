use std::fs::File;
use std::io::BufRead;

use color_eyre::Result;
use tokio_postgres::Transaction;
use std::io::BufReader;

use crate::common::db;
use crate::dao::{ApfGeBytearrayDao, ApfReDeploymentDao, ApfReProcdefDao};
use crate::error::{AppError, ErrorCode};
use crate::get_now;
use crate::service::engine::BpmnManager;
use crate::model::{ApfReDeployment, NewApfGeBytearray, NewApfReDeployment, NewApfReProcdef, SuspensionState};

pub struct DeploymentBuilder {
    pub new_deployment: NewApfReDeployment,
}

impl DeploymentBuilder {
    pub fn new() -> Self {
        Self {
            new_deployment: NewApfReDeployment::new(),
        }
    }

    pub fn key(mut self, key: &str) -> DeploymentBuilder {
        self.new_deployment.key =key.to_string();
        self
    }

    pub fn name(mut self, name: &str) -> DeploymentBuilder {
        self.new_deployment.name =name.to_string();
        self
    }

    pub fn deployer_id(mut self, deployer_id: &str) -> DeploymentBuilder {
        self.new_deployment.deployer_id = deployer_id.to_string();
        self
    }

    pub fn company_id(mut self, company_id: &str) -> DeploymentBuilder {
        self.new_deployment.company_id = company_id.to_string();
        self
    }

    pub fn deployer_name(mut self, deployer_name: &str) -> DeploymentBuilder {
        self.new_deployment.deployer_name = deployer_name.to_string();
        self
    }

    pub fn company_name(mut self, company_name: &str) -> DeploymentBuilder {
        self.new_deployment.company_name = company_name.to_string();
        self
    }

    pub fn bytes(mut self, bytes: Vec<u8>) ->  Result<DeploymentBuilder> {

        if bytes.len() > 1024 * 1024 * 2 {
            Err(AppError::new(ErrorCode::InternalError, Some("文件大小不能超过 2M "), concat!(file!(), ":", line!()), None))?;
        } else if bytes.len() == 0 {
            Err(AppError::new(ErrorCode::InternalError, Some("文件大小不能为 0 "), concat!(file!(), ":", line!()), None))?;
        }

        let mut new_bytearray = NewApfGeBytearray::new();
        new_bytearray.bytes = Some(bytes);
            let mut byte_array = NewApfGeBytearray::new();
        byte_array.name = None;
        self.new_deployment.new_bytearray = new_bytearray;
        
        Ok(self)
    }


    pub fn add_file(mut self, path: &str) -> Result<DeploymentBuilder> {
        let f = File::open(path)?;
        let meta = f.metadata()?;
        if meta.len() > 1024 * 1024 * 2 {
            Err(AppError::new(ErrorCode::InternalError, Some(&format!("文件大小不能超过 2M ({})", path)), concat!(file!(), ":", line!()), None))?
        } else if meta.len() == 0 {
            Err(AppError::new(ErrorCode::InternalError, Some(&format!("文件大小不能为 0 ({})", path)), concat!(file!(), ":", line!()), None))?
        }

        let mut reader = BufReader::new(f);
        let buffer  = reader.fill_buf().unwrap();
        let buffer = buffer.to_vec();
        let mut byte_array = NewApfGeBytearray::new();
        byte_array.bytes = Some(buffer);
        byte_array.name = Some(path.to_string());

        self.new_deployment = NewApfReDeployment::new();
        self.new_deployment.new_bytearray = byte_array;

        Ok(self)
    }

    pub async fn deply<'a>(&mut self) -> Result<ApfReDeployment> {
        let mut conn = db::get_connect().await?;
        let tran = conn.transaction().await?;

        let deployment = self.deploy_with_tran(&tran).await?;

        tran.commit().await?;

        Ok(deployment)
    }

    pub async fn deploy_with_tran<'a>(&mut self, tran: &'a Transaction<'a>) -> Result<ApfReDeployment> {
        let bpmn_xml = String::from_utf8(self.new_deployment.new_bytearray.bytes.clone()
                .unwrap_or(Vec::new()))?;

        let bpmn_manager = BpmnManager::new();
        let bpmn_def = bpmn_manager.parse(bpmn_xml)?;
        let bpmn_proc = &bpmn_def.process;

        // create deployment
        let deployment_dao = ApfReDeploymentDao::new(tran);
        self.new_deployment.deploy_time = get_now();
        let deployment = deployment_dao.create(&self.new_deployment).await?;

        // create bytearray
        self.new_deployment.new_bytearray.deployment_id = Some(deployment.id.clone());
        let bytearray_dao = ApfGeBytearrayDao::new(tran);

        let _bytearray = bytearray_dao.create(&self.new_deployment.new_bytearray).await?;

        // create proc_def
        let new_procdef = NewApfReProcdef {
            key: self.new_deployment.key.clone(),
            name: self.new_deployment.name.clone(),
            deployment_id: deployment.id.clone(),
            suspension_state: SuspensionState::FALSE,
            resource_name: _bytearray.name.clone(),
            description: bpmn_proc.description.clone(),
            deployer_id: self.new_deployment.deployer_id.clone(),
            deployer_name: self.new_deployment.deployer_name.clone(),
            company_id: self.new_deployment.company_id.clone(),
            company_name: self.new_deployment.company_name.clone(),
            update_user_id: self.new_deployment.deployer_id.clone(),
            update_time:  self.new_deployment.deploy_time,
        };

        let procdef_dao = ApfReProcdefDao::new(tran);
        
        let _procdef = procdef_dao.create(&new_procdef).await?;

        Ok(deployment)
    }

}

#[cfg(test)]
pub mod tests {
    use crate::common::db;
    use crate::model::ApfReProcdef;
    use super::*;

    #[tokio::test]
    async fn test_deploy() {
        let mut conn = db::get_connect().await.unwrap();
        let tran = conn.transaction().await.unwrap();

        create_test_deploy("bpmn/process1.bpmn.xml", &tran).await;
    }

    pub async fn create_test_deploy<'a>(file: &str, tran: &'a Transaction<'a>) -> ApfReProcdef {
        let builder = DeploymentBuilder::new();
        let deployment = builder.add_file(file).unwrap()
            .name("test_deploy")
            .key("test_key")
            .deployer_id("test_user_1")
            .deployer_name("test_user_name")
            .company_id("test_comp_1")
            .company_name("test_comp_1")
            .deploy_with_tran(tran)
            .await
            .unwrap();

        let procdef_dao = ApfReProcdefDao::new(tran);
        let procdef = procdef_dao.get_by_deplyment_id(&deployment.id)
            .await
            .unwrap();

        procdef
    }
}