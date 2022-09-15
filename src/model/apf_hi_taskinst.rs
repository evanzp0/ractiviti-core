use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow, PartialEq, Default)]
pub struct ApfHiTaskinst {
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
    pub end_user_id: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub duration: Option<i64>,
    pub suspension_state: i32,
    pub form_key: Option<String>,
}

#[derive(Debug, Default)]
pub struct NewApfHiTaskinst {
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
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub duration: Option<i64>,
    pub suspension_state: i32,
    pub form_key: Option<String>,
}
