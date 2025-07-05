use std::error::Error;
use std::fmt;

use anyhow::Result;

pub type ZigAnyResult<T> = Result<T, anyhow::Error>;

#[derive(Debug)]
pub struct ZigError {
    pub message: String
}

impl ZigError {

    pub fn new(message: &str) -> ZigError {
        ZigError {
            message: message.to_string()
        }
    }

    pub fn any(message: &str) -> anyhow::Error {
        anyhow::Error::new(ZigError::new(message))
    }
}

impl fmt::Display for ZigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ZigError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<&str> for ZigError {

    fn from(message: &str) -> Self {
        ZigError::new(message)
    }
}

impl From<String> for ZigError {

    fn from(message: String) -> Self {
        ZigError::new(message.as_str())
    }
}
