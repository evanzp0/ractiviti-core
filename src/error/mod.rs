use std::error::Error;
use int_enum::IntEnum;
use std::fmt::{Debug, Display, Formatter};
use serde::{Serialize, Serializer};

#[repr(u16)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, IntEnum)]
pub enum ErrorCode {
    InternalError = 1001,
    InvalidInput = 2001,
    NotSupportError = 2002,
    ResourceExist = 2003,
    InvalidCredentials = 3001,
    NotAuthorized = 3002,
    NotFound = 4001,
    SessionNotExist = 4002,
    FileSizeError = 5001,
    ParseError = 6001,
    UnexpectedError = 7001,
}

impl Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where S: Serializer {
        serializer.serialize_u16(self.int_value())
    }
}

impl ErrorCode {
    pub fn default_message(&self) -> String {
        match self {
            ErrorCode::InternalError => "Inernal error".to_string(),
            ErrorCode::InvalidInput => "Invalid input".to_string(),
            ErrorCode::NotSupportError => "Not support".to_string(),
            ErrorCode:: ResourceExist => "Resource exists".to_string(),
            ErrorCode::InvalidCredentials => "Invalid username or password provided".to_string(),
            ErrorCode::NotAuthorized => "Not authorized".to_string(),
            ErrorCode::SessionNotExist => "Session not exists".to_string(),
            ErrorCode::NotFound => "Not found".to_string(),
            ErrorCode::FileSizeError => "File size error".to_string(),
            ErrorCode::ParseError => "Parse error".to_string(),
            ErrorCode::UnexpectedError => "Unexpected error".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AppError {
    pub code: ErrorCode,
    pub msg: String,
    #[serde(skip_serializing)]
    pub location: String,
    #[serde(skip_serializing)]
    pub child_err: Option<Box<dyn Error>>,
}

unsafe impl Send for AppError {}

unsafe impl Sync for AppError {}

#[allow(unused)]
impl AppError {
    pub fn new(code: ErrorCode, msg: Option<&str>, location: &str, source: Option<Box<dyn Error>>) -> Self {
        let mut message = code.default_message();
        if let Some(m) = msg {
            message = m.to_owned();
        }

        Self {
            code,
            msg: message,
            location: location.to_owned(),
            child_err: source,
        }
    }

    pub fn unexpected_error(location: &str) -> Self {
        AppError::new(ErrorCode::UnexpectedError, None, location, None)
    }

    pub fn notfound_error(location: &str) -> Self {
        AppError::new(ErrorCode::NotFound, None, location, None)
    }

    pub fn internal_error(location: &str) -> Self {
        AppError::new(ErrorCode::InternalError, None, location, None)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "err_code: {:?}({}), msg: {}, location: {}, cause: {:?}",
            self.code, self.code.int_value(), self.msg, self.location, self.child_err)
    }
}

impl Error for AppError {
    fn cause(&self) -> Option<&dyn Error> {
        match &self.child_err {
            None => { None }
            Some(err) => {
                Some(&**err)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use log4rs_macros::debug;
    use super::*;

    #[test]
    fn test_some_error() {
        let err1 = AppError::new(ErrorCode::InternalError, None, concat!(file!(), ":", line!()), None);
        assert_eq!(err1.msg, "Inernal error");

        let err2 = AppError::new(ErrorCode::InternalError, None, concat!(file!(), ":", line!()), Some(Box::new(err1)));
        debug!(err2);
        debug!(serde_json::to_string(&err2));
    }
}