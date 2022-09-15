use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use crate::error::AppError;
use color_eyre::Result;

#[derive(Debug, Serialize, FromRow, PartialEq, Default)]
pub struct ApfRuTask {
    pub id: String,
    pub rev: i32,
    pub execution_id: String,
    pub proc_inst_id: String,
    pub proc_def_id: String,
    pub element_id: Option<String>,
    pub element_name: Option<String>,
    pub element_type: Option<String>,
    pub business_key: Option<String>,
    pub description: Option<String>,
    pub start_user_id: Option<String>,
    pub create_time: NaiveDateTime,
    pub suspension_state: i32,
    pub form_key: Option<String>,
}

#[derive(Debug, Default)]
pub struct NewApfRuTask {
    pub rev: i32,
    pub execution_id: String,
    pub proc_inst_id: String,
    pub proc_def_id: String,
    pub element_id: Option<String>,
    pub element_name: Option<String>,
    pub element_type: Option<String>,
    pub business_key: Option<String>,
    pub description: Option<String>,
    pub start_user_id: Option<String>,
    pub create_time: Option<NaiveDateTime>,
    pub suspension_state: i32,
    pub form_key: Option<String>,
}

impl ApfRuTask {
    pub fn element_id_ex(&self) -> Result<String> {
        let rst = self.element_id
            .clone()
            .ok_or(AppError::notfound_error(concat!(file!(), ":", line!())))?;

        Ok(rst)
    }
}