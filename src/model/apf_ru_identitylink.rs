use uuid::Uuid;
use serde::Serialize;
use sqlx::FromRow;
use super::IdentType;

#[derive(Debug, Serialize, FromRow, PartialEq, Default, Clone)]
pub struct ApfRuIdentitylink {
    pub id: Uuid,
    pub rev: i32,
    pub ident_type: IdentType,
    pub group_id: Option<String>,
    pub user_id: Option<String>,
    pub task_id: Option<Uuid>,
    pub proc_inst_id: Option<Uuid>,
    pub proc_def_id: Option<Uuid>,
}

#[derive(Debug, PartialEq)]
pub struct NewApfRuIdentitylink {
    pub ident_type: IdentType,
    pub group_id: Option<String>,
    pub user_id: Option<String>,
    pub task_id: Option<Uuid>,
    pub proc_inst_id: Option<Uuid>,
    pub proc_def_id: Option<Uuid>,
}
