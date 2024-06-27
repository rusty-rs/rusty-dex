//! Collection of error types for DEX files parsing
//!
//! Note: this mostly unused as of now but might be useful in the future.

use std::error::Error;
use std::fmt;

/// A generic error type
#[derive(Debug)]
pub struct DexError {
    /// Error message
    pub message: String,
}

impl DexError {
    /// Create a new error with the given message
    pub fn new(msg: &str) -> DexError {
        DexError { message: msg.to_string() }
    }
}

/// Implement `Display` trait for `DexError`
impl fmt::Display for DexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.message)
    }
}

/// Implement `Error` trait for `DexError`
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
