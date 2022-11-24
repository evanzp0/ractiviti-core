use ramhorns::Content;
use serde::Deserialize;

#[derive(Debug, Deserialize, Content)]
pub struct ProcdefDto {
    pub id: Option<String>,
    pub name:  Option<String>,
    pub key: Option<String>,
    pub deployment_id: Option<String>,
    pub deployer_id: Option<String>,
    pub deployer_name: Option<String>,
    pub company_id: Option<String>,
    pub company_name: Option<String>,
}

impl ProcdefDto {
    pub fn trim(&mut self) {
        if let Some(id) = &self.id {
            self.id = Some(id.trim().to_owned())
        }
        if let Some(name) = &self.name {
            self.name = Some(name.trim().to_owned())
        }
        if let Some(key) = &self.key {
            self.key = Some(key.trim().to_owned())
        }
        if let Some(deployment_id) = &self.deployment_id {
            self.deployment_id = Some(deployment_id.trim().to_owned())
        }
        if let Some(deployer_id) = &self.deployer_id {
            self.deployer_id = Some(deployer_id.trim().to_owned())
        }
        if let Some(deployer_name) = &self.deployer_name {
            self.deployer_name = Some(deployer_name.trim().to_owned())
        }
        if let Some(company_id) = &self.company_id {
            self.company_id = Some(company_id.trim().to_owned())
        }
        if let Some(company_name) = &self.company_name {
            self.company_name = Some(company_name.trim().to_owned())
        }
    }
}