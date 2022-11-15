use ramhorns::Content;
use serde::Deserialize;

#[derive(Debug, Deserialize, Content)]
pub struct DeploymentDto {
    pub id: Option<String>,
    pub name: Option<String>,
    pub key: Option<String>,
    pub organization: Option<String>,
    pub deployer: Option<String>,
    pub deploy_time_start: Option<i64>,
    pub deploy_time_end: Option<i64>,
}