use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::Serialize;
use sqlx::FromRow;
use super::VarType;

#[derive(Debug, Serialize, FromRow, PartialEq, Default, Clone)]
pub struct ApfHiVarinst {
    pub id: Uuid,
    pub rev: i32,
    pub var_type: VarType,
    pub name: String,
    pub value: String,
    pub proc_inst_id: Uuid,
    pub execution_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub create_time: NaiveDateTime,
    pub last_updated_time: NaiveDateTime,
}

#[derive(Debug, Default)]
pub struct NewApfHiVarinst {
    pub id: Uuid,
    pub var_type: VarType,
    pub name: String,
    pub value: String,
    pub proc_inst_id: Uuid,
    pub execution_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub create_time: NaiveDateTime,
    pub last_updated_time: NaiveDateTime,
}