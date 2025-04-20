//! Collection of error types for DEX files parsing
//!
//! Note: this mostly unused as of now but might be useful in the future.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DexError {
    #[error("computed checksum does not match one in header")]
    InvalidChecksumError,
    #[error("DEX header too short")]
    DexHeaderTooShortError,
    #[error("invalid endianness tag")]
    InvalidEndianessTag,
    #[error("no data left to read")]
    NoDataLeftError,
    #[error("too many bytes in unsigned LEB128 value")]
    InvalidUleb128Value,
    #[error("too many bytes in signed LEB128 value")]
    InvalidSleb128Value,
}
