use chrono::NaiveDateTime;
use uuid::Uuid;
use validator::Validate;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use super::NewApfGeBytearray;
use color_eyre::Result;
use crate::error::AppError;

#[derive(Debug, Serialize, FromRow, PartialOrd, PartialEq, Default)]
pub struct ApfReDeployment {
    pub id: Uuid,
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