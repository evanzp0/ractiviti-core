use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow, PartialEq, Default)]
pub struct ApfHiTaskinst {
    pub id: Uuid,
    pub rev: i32,
    pub execution_id: Uuid,
    pub proc_inst_id: Uuid,
    pub proc_def_id: Uuid,
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
    pub id: Uuid,
    pub rev: i32,
    pub execution_id: Uuid,
    pub proc_inst_id: Uuid,
    pub proc_def_id: Uuid,
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
