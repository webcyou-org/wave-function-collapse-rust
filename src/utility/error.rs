use serde_json::{Error as JsonError, Result as JsonResult};
use std::fmt;
use std::io::{self, Error as IoError};

#[derive(Debug)]
pub enum MyError {
    Io(IoError),
    Json(JsonError),
    Other(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO error: {}", e),
            MyError::Json(e) => write!(f, "JSON error: {}", e),
            MyError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl From<IoError> for MyError {
    fn from(error: IoError) -> MyError {
        MyError::Io(error)
    }
}

impl From<JsonError> for MyError {
    fn from(error: JsonError) -> MyError {
        MyError::Json(error)
    }
}

impl From<String> for MyError {
    fn from(error: String) -> MyError {
        MyError::Other(error)
    }
}
