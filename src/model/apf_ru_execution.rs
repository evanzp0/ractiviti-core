use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use color_eyre::Result;

use crate::error::{AppError, ErrorCode};

#[derive(Debug, Serialize, FromRow, PartialEq, Clone, Default)]
pub struct ApfRuExecution {
    pub id: String,
    pub rev: i32,
    pub proc_inst_id: Option<String>,
    pub business_key: Option<String>,
    pub parent_id: Option<String>,
    pub proc_def_id: String,
    pub root_proc_inst_id: Option<String>,
    pub element_id: Option<String>,
    pub is_active: i32,
    pub start_time: NaiveDateTime,
    pub start_user: Option<String>,
}

impl ApfRuExecution {
    pub fn proc_inst_id(&self) -> Result<String> {
        let proc_inst_id = self.proc_inst_id
            .clone()
            .ok_or(
                AppError::new(
                    ErrorCode::NotFound, 
                    Some("not found proc_inst_id in current execution"), 
                    concat!(file!(), ":", line!()), None
                )
            )?;

        Ok(proc_inst_id)
    }

    pub fn element_id(&self) -> Result<String> {
        let element_id = self.element_id
            .clone().ok_or(
                AppError::new(
                    ErrorCode::NotFound,
                    Some("not found element_id in current execution"),
                    concat!(file!(), ":", line!()), 
                    None
                )
            )?;

        Ok(element_id)
    }
}

#[derive(Default)]
pub struct NewApfRuExecution {
    pub proc_inst_id: Option<String>,
    pub business_key: Option<String>,
    pub parent_id: Option<String>,
    pub proc_def_id: String,
    pub root_proc_inst_id: Option<String>,
    pub element_id: Option<String>,
    pub is_active: i32,
    pub start_time:NaiveDateTime,
    pub start_user: Option<String>,
}
