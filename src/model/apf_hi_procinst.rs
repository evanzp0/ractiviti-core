use chrono::NaiveDateTime;
use serde::Serialize;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper)]
#[pg_mapper(table="apf_hi_procinst")]
#[derive(Debug, Serialize, PartialEq, Default)]
pub struct ApfHiProcinst {
    pub id: String,
    pub rev: i32,
    pub proc_inst_id: String,
    pub business_key: Option<String>,
    pub proc_def_id: String,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub duration: i64,
    pub start_user: Option<String>,
    pub start_element_id: Option<String>,
    pub end_element_id: Option<String>,
}

#[derive(Debug, Default)]
pub struct NewApfHiProcinst {
    pub id: String,
    pub proc_inst_id: String,
    pub business_key: Option<String>,
    pub proc_def_id: String,
    pub start_time: NaiveDateTime,
    pub start_user: Option<String>,
    pub start_element_id: Option<String>,
}
