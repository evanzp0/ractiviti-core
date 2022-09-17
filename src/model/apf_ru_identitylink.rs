use serde::Serialize;
use tokio_pg_mapper_derive::PostgresMapper;

use super::IdentType;

#[derive(Debug, Serialize, PartialEq, Default, Clone)]
#[derive(PostgresMapper)]
#[pg_mapper(table="apf_ru_identitylink")]
pub struct ApfRuIdentitylink {
    pub id: String,
    pub rev: i32,
    pub ident_type: IdentType,
    pub group_id: Option<String>,
    pub user_id: Option<String>,
    pub task_id: Option<String>,
    pub proc_inst_id: Option<String>,
    pub proc_def_id: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct NewApfRuIdentitylink {
    pub ident_type: IdentType,
    pub group_id: Option<String>,
    pub user_id: Option<String>,
    pub task_id: Option<String>,
    pub proc_inst_id: Option<String>,
    pub proc_def_id: Option<String>,
}
