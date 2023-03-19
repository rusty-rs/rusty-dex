use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DexError {
    pub message: String,
}

impl DexError {
    pub fn new(msg: &str) -> DexError {
        DexError { message: msg.to_string() }
    }
}

impl fmt::Display for DexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.message)
    }
}

impl Error for DexError {
    fn description(&self) -> &str {
        &self.message
    }
}

