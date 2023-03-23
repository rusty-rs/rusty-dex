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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let error = DexError::new("test message");
        assert_eq!(error.message, "test message");
    }

    #[test]
    fn test_display() {
        let error = DexError::new("test message");
        assert_eq!(error.to_string(), "test message");
    }

    #[test]
    fn test_description() {
        let error = DexError::new("test message");
        assert_eq!(error.description(), "test message");
    }
}
