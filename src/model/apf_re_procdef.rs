use validator::Validate;
use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper)]
#[pg_mapper(table="apf_re_procdef")]
#[derive(Debug, Serialize, PartialEq, Default)]
pub struct ApfReProcdef {
    pub id: String,
    pub rev: i32,
    pub name:  String,
    pub key: String, // key = md5(self.name)
    pub version: i32,
    pub deployment_id: String,
    pub resource_name: Option<String>,
    pub description: Option<String>,
    pub suspension_state: i32,
    pub company_id: String,
    pub company_name: String,
    pub deployer_id: String,
    pub deployer_name: String,
    pub deploy_time: i64,
    pub update_user_id: String,
    pub update_time: i64,
}

#[derive(Debug)]
pub enum SuspensionState {}

#[allow(dead_code)]
impl SuspensionState {
    pub const TRUE : i32 = 1;
    pub const FALSE: i32 = 0;
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewApfReProcdef {
    pub name: String,
    pub key: String,
    pub deployment_id: String,
    pub resource_name: Option<String>,
    pub description: Option<String>,
    pub suspension_state: i32,
    pub company_id: String,
    pub company_name: String,
    pub deployer_id: String,
    pub deployer_name: String,
    pub update_user_id: String,
    pub update_time: i64,
}