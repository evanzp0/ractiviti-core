use chrono::NaiveDateTime;
use tokio_pg_mapper_derive::PostgresMapper;
use validator::Validate;
use serde::{Serialize, Deserialize};
use color_eyre::Result;

use crate::error::AppError;
use super::NewApfGeBytearray;

#[derive(Debug, Serialize, PartialOrd, PartialEq, Default)]
#[derive(PostgresMapper)]
#[pg_mapper(table = "apf_re_deployment")]
pub struct ApfReDeployment {
    pub id: String,
    pub name: String,
    pub key: Option<String>,
    pub organization: Option<String>,
    pub deployer: Option<String>,
    pub deploy_time: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Validate, Default)]
pub struct NewApfReDeployment {
    #[validate(length(max = 255, message = "name must be less than 255 chars."))]
    pub name: Option<String>,
    #[validate(length(max = 255, message = "key must be less than 255 chars."))]
    pub key: Option<String>,
    #[validate(length(max = 255, message = "organization must be less than 255 chars."))]
    pub organization: Option<String>,
    #[validate(length(max = 255, message = "deployer must be less than 255 chars."))]
    pub deployer: Option<String>,
    #[serde(skip_serializing)]
    pub new_bytearray: NewApfGeBytearray,
}

impl NewApfReDeployment {
    pub fn new() -> Self {
        Self {
            new_bytearray: NewApfGeBytearray::new(),
            ..Default::default()
        }
    }
}

impl ApfReDeployment {
    pub fn key_ex(&self) -> Result<String> {
        let key = self.key.clone().ok_or(AppError::notfound_error(concat!(file!(), ":", line!())))?;

        Ok(key)
    }
}