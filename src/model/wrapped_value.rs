use std::{fmt::{Display, Formatter}, error::Error};

use serde::Serialize;
use tokio_postgres::types::{FromSql, Type, ToSql, private::BytesMut, IsNull, to_sql_checked};

use crate::error::{AppError, ErrorCode};


#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum WrappedValue {
    Int(i32),
    Double(f64),
    Str(String),
    Bool(bool),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum WrappedValueType {
    INT,
    DOUBLE,
    STRING,
    BOOL,
}

impl Default for WrappedValue {
    fn default() -> Self {
        WrappedValue::Str("".to_owned())
    }
}

impl Default for WrappedValueType {
    fn default() -> Self {
        WrappedValueType::STRING
    }
}

impl WrappedValue {
    pub fn get_type(&self) -> WrappedValueType {
        match self {
            WrappedValue::Int(_) => WrappedValueType::INT,
            WrappedValue::Double(_) => WrappedValueType::DOUBLE,
            WrappedValue::Str(_) => WrappedValueType::STRING,
            WrappedValue::Bool(_) => WrappedValueType::BOOL,
        }
    }

    pub fn as_str(&self) -> String {
        match self {
            WrappedValue::Str(v) => v.to_owned(),
            WrappedValue::Int(v) => v.to_string(),
            WrappedValue::Double(v) => v.to_string(),
            WrappedValue::Bool(v) => if *v { 1.to_string() } else { 0.to_string() },
        }
    }

    pub fn as_i32(&self) -> i32 {
        let mut rst = 0;
        if let WrappedValue::Int(v) = self {
            rst = *v;
        } else if let WrappedValue::Bool(v) = self {
            if *v == true {
                rst = 1;
            } else {
                rst = 0;
            }
        }

        rst
    }

    pub fn as_f64(&self) -> f64 {
        let mut rst = 0_f64;
        if let WrappedValue::Double(v) = self {
            rst = *v;
        }

        rst
    }

    pub fn as_bool(&self) -> bool {
        let mut rst = false;
        if let WrappedValue::Bool(v) = self {
            rst = *v;
        }

        rst
    }
}


impl Display for WrappedValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> FromSql<'a> for WrappedValueType {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let s = String::from_utf8(raw.to_vec())?;
        if s == WrappedValueType::INT.to_string() {
            return Ok(WrappedValueType::INT);
        } else if s == WrappedValueType::DOUBLE.to_string() {
            return Ok(WrappedValueType::DOUBLE);
        } else if s == WrappedValueType::STRING.to_string() {
            return Ok(WrappedValueType::STRING);
        } else if s == WrappedValueType::BOOL.to_string() {
            return Ok(WrappedValueType::BOOL);
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

impl ToSql for WrappedValueType {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.to_string().to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool {
        <String as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}

