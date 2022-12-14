use serde::Serialize;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper)]
#[pg_mapper(table="apf_hi_actinst")]
#[derive(Debug, Serialize, PartialEq, Default)]
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
    pub start_time: i64,
    pub end_time: Option<i64>,
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
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub duration: Option<i64>,
}
