use validator::Validate;
use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper)]
#[pg_mapper(table="apf_re_deployment")]
#[derive(Debug, Serialize, PartialEq, Default)]
pub struct ApfReProcdef {
    pub id: String,
    pub rev: i32,
    pub name:  Option<String>,
    pub key: String,
    pub version: i32,
    pub deployment_id: String,
    pub resource_name: Option<String>,
    pub description: Option<String>,
    pub suspension_state: i32,
}

#[derive(Debug)]
pub enum SuspensionState {}

#[allow(dead_code)]
impl SuspensionState {
    pub const TRUE : i32 = 1;
    pub const FALSE: i32 = 0;
}

#[derive(Debug, Deserialize, Validate, Default)]
pub struct NewApfReProcdef {
    pub name: Option<String>,
    pub key: String,
    pub deployment_id: String,
    pub resource_name: Option<String>,
    pub description: Option<String>,
    pub suspension_state: i32,
}