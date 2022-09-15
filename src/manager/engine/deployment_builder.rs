use std::fs::File;
use std::io::BufRead;

use color_eyre::Result;
use std::io::BufReader;
use sqlx::{Acquire, Postgres, Transaction};

use crate::boot::db;
use crate::dao::{ApfGeBytearrayDao, ApfReDeploymentDao, ApfReProcdefDao};
use crate::error::{AppError, ErrorCode};
use crate::manager::engine::BpmnManager;
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

    pub fn key(mut self, key: &str) -> Result<DeploymentBuilder> {
        self.new_deployment.key = Some(key.to_string());

        Ok(self)
    }

    pub fn name(mut self, name: &str) -> Result<DeploymentBuilder> {
        self.new_deployment.name = Some(name.to_string());

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
        let mut tran = conn.begin().await?;

        let deployment = self._deploy(&mut tran).await?;

        tran.commit().await?;

        Ok(deployment)
    }

    pub async fn _deploy<'a>(&mut self, tran: &mut Transaction<'a, Postgres>) -> Result<ApfReDeployment> {
        let bpmn_xml = String::from_utf8(self.new_deployment.new_bytearray.bytes.clone()
                .unwrap_or(Vec::new()))?;

        let bpmn_manager = BpmnManager::new();
        let bpmn_def = bpmn_manager.parse(bpmn_xml)?;
        let bpmn_proc = &bpmn_def.process;

        // create deployment
        let deployment_dao = ApfReDeploymentDao::new();
        let deployment = deployment_dao.create(&self.new_deployment, tran).await?;

        // create bytearray
        self.new_deployment.new_bytearray.deployment_id = Some(deployment.id.clone());
        let bytearray_dao = ApfGeBytearrayDao::new();
        let _bytearray = bytearray_dao.create(&self.new_deployment.new_bytearray, tran).await?;

        // create proc_def
        let new_procdef = NewApfReProcdef {
            key: bpmn_proc.id.clone(),
            name: bpmn_proc.name.clone(),
            deployment_id: deployment.id.clone(),
            suspension_state: SuspensionState::FALSE,
            resource_name: _bytearray.name.clone(),
            description: bpmn_proc.description.clone(),
        };

        let procdef_dao = ApfReProcdefDao::new();
        let _procdef = procdef_dao.create(&new_procdef, tran).await?;

        Ok(deployment)
    }

}

#[cfg(test)]
pub mod tests {
    use sqlx::Acquire;
    use crate::boot::db;
    use crate::model::ApfReProcdef;
    use super::*;

    #[tokio::test]
    async fn test_deploy() {
        let mut conn = db::get_connect().await.unwrap();
        let mut tran = conn.begin().await.unwrap();

        create_test_deploy("bpmn/process1.bpmn.xml", &mut tran).await;
    }

    pub async fn create_test_deploy<'a>(file: &str, tran: &mut Transaction<'a, Postgres>) -> ApfReProcdef {
        let builder = DeploymentBuilder::new();
        let deployment = builder.add_file(file).unwrap()
            .name("test_deploy").unwrap()
            .key("test_key").unwrap()
            ._deploy(tran)
            .await
            .unwrap();

        let procdef_dao = ApfReProcdefDao::new();
        let procdef = procdef_dao.get_by_deplyment_id(&deployment.id, tran)
            .await
            .unwrap();

        procdef
    }
}