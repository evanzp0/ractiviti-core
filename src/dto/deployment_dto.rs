use ramhorns::Content;
use serde::Deserialize;

#[derive(Debug, Deserialize, Content)]
pub struct DeploymentDto {
    pub id: Option<String>,
    pub name: Option<String>,
    pub key: Option<String>,
    pub company_id: Option<String>,
    pub deployer_id: Option<String>,
    pub deploy_time_from: Option<i64>,
    pub deploy_time_to: Option<i64>,
}