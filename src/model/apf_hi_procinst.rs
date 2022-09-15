use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow, PartialEq, Default)]
pub struct ApfHiProcinst {
    pub id: Uuid,
    pub rev: i32,
    pub proc_inst_id: Uuid,
    pub business_key: Option<String>,
    pub proc_def_id: Uuid,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub duration: i64,
    pub start_user: Option<String>,
    pub start_element_id: Option<String>,
    pub end_element_id: Option<String>,
}

#[derive(Debug, Default)]
pub struct NewApfHiProcinst {
    pub id: Uuid,
    pub proc_inst_id: Uuid,
    pub business_key: Option<String>,
    pub proc_def_id: Uuid,
    pub start_time: NaiveDateTime,
    pub start_user: Option<String>,
    pub start_element_id: Option<String>,
}
