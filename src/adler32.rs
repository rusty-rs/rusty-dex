//! Module to compute and verify the Adler-32 checksum of a file
//!
//! DEX files use Adler-32 to detect file corruption. Each DEX file contains a checksum value to
//! compare against. The checksum must be computed from the whole file except the magic number and
//! the checksum field in the header.
//!
//! # Example
//!
//! ```
//! use dex_parser::adler32::verify_from_bytes;
//!
//! let bytes: [u8; 16] = [0x44, 0x45, 0x58, 0x0a,
//!                        0x30, 0x33, 0x35, 0x00,
//!                        0x00, 0x00, 0x00, 0x00,
//!                        0x00, 0x00, 0x00, 0x00];
//! assert!(verify_from_bytes(&bytes, 0x14300184).unwrap());
//! ```

use std::io::Cursor;

use crate::error::DexError;

/// Constant used in the checksum computation
const MOD_ADLER: u32 = 65521;

/// Each DEX header contains an Adler-32 checksum of the file, minus the first
/// 11 bytes, which correspond to the space taken by the magic and the checksum.
/// This function computes the checksum of the file, and compares it to the one
/// found in the header.
pub fn verify_from_bytes(bytes: &Cursor<Vec<u8>>, checksum: u32) -> Result<bool, DexError> {

    // Define variable for checksum computation
    let mut a: u32 = 1;
    let mut b: u32 = 0;

    // Main computation
    for byte in bytes.get_ref().iter().skip(12) {
        a = (a + *byte as u32) % MOD_ADLER;
        b = (b + a) % MOD_ADLER;
    }

    // Concatenating A and B
    let computed_checksum = (b << 16) | a;

    // Verification of the checksum read from the DEX header
    if computed_checksum == checksum {
        Ok(true)
    } else {
        Err(DexError::new("[adler32] error: computed checksum does not match one in header"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_valid_from_bytes() {
        // Test data with valid checksum
        let bytes = Cursor::new(vec![0x44, 0x45, 0x58, 0x0a,
                               0x30, 0x33, 0x35, 0x00,
                               0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00]);
        let checksum: u32 = 0x00040001;
        println!("{}", verify_from_bytes(&bytes, checksum).unwrap());
        assert!(verify_from_bytes(&bytes, checksum).unwrap());
    }

    #[test]
    fn test_verify_invalid_from_bytes() {
        // Test data with invalid checksum
        let bytes = Cursor::new(vec![0x44, 0x45, 0x58, 0x0a,
                               0x30, 0x33, 0x35, 0x00,
                               0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00]);
        let checksum: u32 = 0xcafebabe;
        assert_eq!(verify_from_bytes(&bytes, checksum).unwrap_err().to_string(),
                   "[adler32] error: computed checksum does not match one in header");
    }
}
