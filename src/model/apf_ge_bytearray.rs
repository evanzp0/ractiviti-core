use serde::{Serialize, Deserialize};
use validator::Validate;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow, PartialEq, Default)]
pub struct ApfGeBytearray {
    pub id: String,
    pub name: Option<String>,
    pub deployment_id: String,
    pub bytes: Option<Vec<u8>>,
}

#[derive(Debug, Deserialize, Validate, Default)]
pub struct NewApfGeBytearray {
    #[validate(length(max = 50, message = "name must be less than 255 chars."))]
    pub name: Option<String>,
    pub deployment_id: Option<String>,
    pub bytes: Option<Vec<u8>>,
}

impl NewApfGeBytearray {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}