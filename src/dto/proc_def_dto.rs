use ramhorns::Content;
use serde::Deserialize;

#[derive(Debug, Deserialize, Content)]
pub struct ProcdefDto {
    pub id: Option<String>,
    pub name:  Option<String>,
    pub key: Option<String>,
    pub deployment_id: Option<String>,
    pub deployer_id: Option<String>,
    pub company_id: Option<String>,
}