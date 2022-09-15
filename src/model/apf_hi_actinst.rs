use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow, PartialEq, Default)]
pub struct ApfHiActinst {
    pub id: String,
    pub rev: i32,
    pub proc_def_id: String,
    pub proc_inst_id: String,
    pub execution_id: String,
    pub task_id: Option<String>,
    pub element_id: Option<String>,
    pub element_name: Option<String>,
    pub element_type: Option<String>,
    pub start_user_id: Option<String>,
    pub end_user_id: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub duration: Option<i64>,
}

#[derive(Debug, Default)]
pub struct NewApfHiActinst {
    pub rev: i32,
    pub proc_def_id: String,
    pub proc_inst_id: Option<String>,
    pub execution_id: String,
    pub task_id: Option<String>,
    pub element_id: Option<String>,
    pub element_name: Option<String>,
    pub element_type: Option<String>,
    pub start_user_id: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub duration: Option<i64>,
}
