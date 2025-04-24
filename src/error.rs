//! Collection of error types for DEX files parsing
//!
//! We rely on `thiserror` for the heavy lifting

use thiserror::Error;

#[derive(Error, Debug)]
/// All errors that can be returned by the parser
pub enum DexError {
    /// The checksum of the header does not match the one in the DEX header
    #[error("computed checksum does not match one in header")]
    InvalidChecksumError,
    /// The header of the file is too short to be a valid DEX header
    #[error("DEX header too short")]
    DexHeaderTooShortError,
    /// The endianness tag of the header is invalid
    #[error("invalid endianness tag")]
    InvalidEndianessTag,
    /// The stream ended abruptly
    #[error("no data left to read")]
    NoDataLeftError,
    /// The unsigned LEB128 value has more than 5 bytes
    #[error("too many bytes in unsigned LEB128 value")]
    InvalidUleb128Value,
    /// The signed LEB128 value has more than 5 bytes
    #[error("too many bytes in unsigned LEB128p1 value")]
    InvalidUleb128p1Value,
    /// The unsigned LEB128p1 value has more than 5 bytes
    #[error("too many bytes in signed LEB128 value")]
    InvalidSleb128Value,
    /// Attempted to move the cursor after the end of the stream
    #[error("cannot move reader to requested offset")]
    SeekError(#[from] std::io::Error),
    /// Requested type index is not in the list of types
    #[error("cannot find element in types list")]
    InvalidTypeIdx,
    /// Requested string index is not in the list of strings
    #[error("cannot find element in strings list")]
    InvalidStringIdx,
    /// Requested field index is not in the list of fields
    #[error("cannot find element in fields list")]
    InvalidFieldIdx,
    /// Requested method index is not in the list of methods
    #[error("cannot find element in methods list")]
    InvalidMethodIdx,
    /// Encountered an invalid or unused opcode
    #[error("cannot parse instruction opcode")]
    InvalidOpCode,
}
