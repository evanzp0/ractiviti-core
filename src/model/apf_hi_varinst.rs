use serde::Serialize;
use tokio_pg_mapper_derive::PostgresMapper;

use super::WrappedValueType;

#[derive(Debug, Serialize, PartialEq, Default, Clone)]
#[derive(PostgresMapper)]
#[pg_mapper(table="apf_hi_varinst")]
pub struct ApfHiVarinst {
    pub id: String,
    pub rev: i32,
    pub var_type: WrappedValueType,
    pub name: String,
    pub value: String,
    pub proc_inst_id: String,
    pub execution_id: Option<String>,
    pub task_id: Option<String>,
    pub create_time: i64,
    pub last_updated_time: i64,
}

#[derive(Debug, Default)]
pub struct NewApfHiVarinst {
    pub id: String,
    pub var_type: WrappedValueType,
    pub name: String,
    pub value: String,
    pub proc_inst_id: String,
    pub execution_id: Option<String>,
    pub task_id: Option<String>,
    pub create_time: i64,
    pub last_updated_time: i64,
}