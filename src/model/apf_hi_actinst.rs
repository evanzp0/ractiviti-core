use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow, PartialEq, Default)]
pub struct ApfHiActinst {
    pub id: Uuid,
    pub rev: i32,
    pub proc_def_id: Uuid,
    pub proc_inst_id: Uuid,
    pub execution_id: Uuid,
    pub task_id: Option<Uuid>,
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
    pub proc_def_id: Uuid,
    pub proc_inst_id: Option<Uuid>,
    pub execution_id: Uuid,
    pub task_id: Option<Uuid>,
    pub element_id: Option<String>,
    pub element_name: Option<String>,
    pub element_type: Option<String>,
    pub start_user_id: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub duration: Option<i64>,
}
