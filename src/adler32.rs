use crate::error::DexError;

const MOD_ADLER: u32 = 65521;

/* Each DEX header contains an Adler-32 checksum of the file, minus the first
 * 11 bytes, which correspond to the space taken by the magic and the checksum.
 * This function computes the checksum of the file, and compares it to the one
 * found in the header.
 */
pub fn verify_from_bytes(bytes: &[u8], checksum: u32) -> Result<bool, DexError> {

    /* Define variable for checksum computation */
    let mut a: u32 = 1;
    let mut b: u32 = 0;

    /* Main computation
     * We must ignore the first 11 bytes of the file (which
     * correspond to the magic number and the checksum). */
    for byte in bytes {
        a = (a + *byte as u32) % MOD_ADLER;
        b = (b + a) % MOD_ADLER;
    }

    /* Concatenating A and B */
    let computed_checksum = (b << 16) | a;

    /* Verification of the checksum read from the DEX header */
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
        let bytes: [u8; 16] = [0x44, 0x45, 0x58, 0x0a,
                               0x30, 0x33, 0x35, 0x00,
                               0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00];
        let checksum: u32 = 0x14300184;
        assert!(verify_from_bytes(&bytes, checksum).unwrap());
    }

    #[test]
    fn test_verify_invalid_from_bytes() {
        // Test data with invalid checksum
        let bytes: [u8; 16] = [0x44, 0x45, 0x58, 0x0a,
                               0x30, 0x33, 0x35, 0x00,
                               0x00, 0x00, 0x00, 0x00,
                               0x00, 0x00, 0x00, 0x00];
        let checksum: u32 = 0xcafebabe;
        assert_eq!(verify_from_bytes(&bytes, checksum).unwrap_err().to_string(),
                   "[adler32] error: computed checksum does not match one in header");
    }
}
