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
    #[error("cannot move reader to requested offset")]
    SeekError(#[from] std::io::Error),
    #[error("cannot find element in types list")]
    InvalidTypeIdx,
    #[error("cannot find element in strings list")]
    InvalidStringIdx,
    #[error("cannot find element in fields list")]
    InvalidFieldIdx,
    #[error("cannot find element in methods list")]
    InvalidMethodIdx,
}
