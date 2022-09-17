use serde::Serialize;
use tokio_postgres::types::{FromSql, Type, ToSql, private::BytesMut, IsNull, to_sql_checked};
use std::{fmt::{Display, Formatter}, error::Error};

use crate::error::{AppError, ErrorCode};

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, PartialEq, Clone)]
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

impl<'a> FromSql<'a> for IdentType {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let s = String::from_utf8(raw.to_vec())?;
        if s == IdentType::user.to_string() {
            return Ok(IdentType::user);
        } else if s == IdentType::group.to_string() {
            return Ok(IdentType::group);
        }

        Err(
            AppError::new(
                ErrorCode::InternalError, 
                Some(&format!("Value ({}) 无法映射为 IdentType", s)), 
                concat!(file!(), ":", line!()), 
                None
            )
        )?
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        *ty == Type::VARCHAR
    }

}

impl ToSql for IdentType {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.to_string().to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool {
        <String as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}