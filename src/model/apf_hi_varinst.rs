use chrono::NaiveDateTime;
use serde::Serialize;
use super::VarType;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper)]
#[pg_mapper(table="apf_hi_varinst")]
#[derive(Debug, Serialize, PartialEq, Default, Clone)]
pub struct ApfHiVarinst {
    pub id: String,
    pub rev: i32,
    pub var_type: VarType,
    pub name: String,
    pub value: String,
    pub proc_inst_id: String,
    pub execution_id: Option<String>,
    pub task_id: Option<String>,
    pub create_time: NaiveDateTime,
    pub last_updated_time: NaiveDateTime,
}

#[derive(Debug, Default)]
pub struct NewApfHiVarinst {
    pub id: String,
    pub var_type: VarType,
    pub name: String,
    pub value: String,
    pub proc_inst_id: String,
    pub execution_id: Option<String>,
    pub task_id: Option<String>,
    pub create_time: NaiveDateTime,
    pub last_updated_time: NaiveDateTime,
}