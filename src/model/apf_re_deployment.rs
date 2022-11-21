use tokio_pg_mapper_derive::PostgresMapper;
use validator::Validate;
use serde::{Serialize, Deserialize};

use super::NewApfGeBytearray;

#[derive(Debug, Serialize, PartialOrd, PartialEq, Default)]
#[derive(PostgresMapper)]
#[pg_mapper(table = "apf_re_deployment")]
pub struct ApfReDeployment {
    pub id: String,
    pub name: String,
    pub key: String,
    pub company_id: Option<String>,
    pub deployer_id: Option<String>,
    pub deploy_time: Option<i64>,
}

#[derive(Debug, Deserialize, Validate, Default)]
pub struct NewApfReDeployment {
    #[validate(length(max = 255, message = "name must be less than 255 chars."))]
    pub name: String,
    #[validate(length(max = 255, message = "key must be less than 255 chars."))]
    pub key: String,
    #[validate(length(max = 255, message = "organization must be less than 255 chars."))]
    pub company_id: String,
    #[validate(length(max = 255, message = "deployer must be less than 255 chars."))]
    pub deployer_id: String,
    #[serde(skip_serializing)]
    pub new_bytearray: NewApfGeBytearray,
    pub deploy_time: i64,
}

impl NewApfReDeployment {
    pub fn new() -> Self {
        Self {
            new_bytearray: NewApfGeBytearray::new(),
            ..Default::default()
        }
    }
}
