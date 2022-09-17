use serde::Serialize;
use super::IdentType;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper)]
#[pg_mapper(table="apf_hi_identitylink")]
#[derive(Debug, Serialize, PartialEq, Default)]
pub struct ApfHiIdentitylink {
    pub id: String,
    pub rev: i32,
    pub ident_type: IdentType,
    pub group_id: Option<String>,
    pub user_id: Option<String>,
    pub task_id: Option<String>,
    pub proc_inst_id: Option<String>,
    pub proc_def_id: Option<String>,
}

#[derive(Debug, Default)]
pub struct NewApfHiIdentitylink {
    pub id: String,
    pub ident_type: IdentType,
    pub group_id: Option<String>,
    pub user_id: Option<String>,
    pub task_id: Option<String>,
    pub proc_inst_id: Option<String>,
    pub proc_def_id: Option<String>,
}