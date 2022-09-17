pub mod apf_re_deployment;
pub mod apf_ge_bytearray;
pub mod apf_re_procdef;
pub mod apf_ru_execution;
pub mod apf_hi_procinst;
pub mod apf_ru_task;
pub mod apf_hi_actinst;
pub mod apf_hi_taskinst;
// pub mod apf_ru_identitylink;
// pub mod apf_hi_identitylink;
// pub mod apf_ru_variable;
// pub mod apf_hi_varinst;

pub use apf_re_deployment::*;
pub use apf_ge_bytearray::*;
pub use apf_re_procdef::*;
pub use apf_ru_execution::*;
pub use apf_hi_procinst::*;
pub use apf_ru_task::*;
pub use apf_hi_actinst::*;
pub use apf_hi_taskinst::*;
// pub use apf_ru_identitylink::*;
// pub use apf_hi_identitylink::*;
// pub use apf_ru_variable::*;
// pub use apf_hi_varinst::*;


use serde::Serialize;
use std::fmt::{Display, Formatter};
// use crate::manager::engine::TypeWrapper;

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, PartialEq, Clone)]
// #[derive(sqlx::Type)]
// #[sqlx(type_name = "varchar")]
pub enum IdentType {
    user,
    group,
}

impl Display for IdentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for IdentType {
    fn default() -> Self {
        IdentType::user
    }
}

// #[allow(non_camel_case_types)]
// #[derive(Debug, Serialize, PartialEq, Clone)]
// // #[derive(sqlx::Type)]
// // #[sqlx(type_name = "varchar")]
// pub enum VarType {
//     INT,
//     DOUBLE,
//     STRING,
//     BOOL,
// }

// impl Display for VarType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

// impl Default for VarType {
//     fn default() -> Self {
//         VarType::STRING
//     }
// }

// impl From<TypeWrapper> for VarType {
//     fn from(src: TypeWrapper) -> Self {
//         match src {
//             TypeWrapper::str(_) => Self::STRING,
//             TypeWrapper::i32(_) => Self::INT,
//             TypeWrapper::f32(_) => Self::DOUBLE,
//             TypeWrapper::f64(_) => Self::DOUBLE,
//             TypeWrapper::bool(_) => Self::BOOL,
//         }
//     }
// }

