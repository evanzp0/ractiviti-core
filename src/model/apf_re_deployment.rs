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
    pub company_id: String,
    pub company_name: String,
    pub deployer_id: String,
    pub deployer_name: String,
    pub deploy_time: i64,
}

#[derive(Debug, Deserialize, Validate, Default)]
pub struct NewApfReDeployment {
    #[validate(length(max = 255, message = "name must be less than 255 chars."))]
    pub name: String,

    #[validate(length(max = 255, message = "key must be less than 255 chars."))]
    pub key: String,

    #[validate(length(max = 255, message = "company id must be less than 255 chars."))]
    pub company_id: String,

    #[validate(length(max = 100, message = "company name must be less than 100 chars."))]
    pub company_name: String,

    #[validate(length(max = 255, message = "deployer id must be less than 255 chars."))]
    pub deployer_id: String,

    #[validate(length(max = 50, message = "deployer name must be less than 50 chars."))]
    pub deployer_name: String,

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
