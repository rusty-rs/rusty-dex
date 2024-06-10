use log::{error, debug};

pub fn decode(raw: &Vec<u8>) -> Result<String, &'static str> {
    let raw_len = raw.len();
    let mut decoded: Vec<u32> = Vec::new();
    let mut idx = 0;

    while idx < raw_len {
        /* All characters in the range '\u0001' to '\u007F'
         * are represented by a single byte */
        if (raw[idx] >> 7) == 0 {
            decoded.push(raw[idx] as u32);

            idx += 1;
            continue;
        }

        /* The null character '\u0000' and characters
         * in the range '\u0080' to '\u07FF' are
         * represented by a pair of bytes */
        if (raw[idx] >> 5) == 0b110 {
            if idx + 1 >= raw_len {
                error!("[MUTF-8] error: two bytes code point detected but not enough bytes");
                return Err("[MUTF-8] two bytes code point detected but not enough bytes");
            }

            let x = raw[idx] as u32;
            let y = raw[idx + 1] as u32;

            let code_point = ((x & 0b0001_1111) << 6)
                            + (y & 0b0011_1111);
            decoded.push(code_point as u32);

            idx += 2;
            continue;
        }

        /* char values in the range '\u0800' to '\uFFFF'
         * are represented by three bytes */
        if (raw[idx] >> 4) == 0b1110 {
            if idx + 2 >= raw_len {
                error!("[MUTF-8] error: three bytes code point detected but not enough bytes");
                return Err("[MUTF-8] three bytes code point detected but not enough bytes");
            }

            /* Decoding the code point.
             * We have to first convert the bytes in u32 to avoid overflow.*/
            let x = raw[idx] as u32;
            let y = raw[idx + 1] as u32;
            let z = raw[idx + 2] as u32;

            let code_point = ((x & 0b0000_1111) << 12)
                           + ((y & 0b0011_1111) << 6)
                           + (z & 0b0011_1111);

            /* dealing with surrogate pairs */
            if code_point >= 0xd800 && code_point <= 0xdbff {
                if idx + 3 >= raw_len {
                    error!("[MUTF-8] error: truncated surrogate pair");
                    return Err("[MUTF-8] truncated surrogate pair");
                }

                /* checking second part of the surrogate pair */
                let next_x = raw[idx + 3] as u32;
                let next_y = raw[idx + 4] as u32;
                let next_z = raw[idx + 5] as u32;

                let next_code_point = ((next_x & 0b0000_1111) << 12)
                                    + ((next_y & 0b0011_1111) << 6)
                                    + (next_z & 0b0011_1111);

                if next_code_point >= 0xdc00 && next_code_point <= 0xdfff {
                    let final_code_point = ((code_point - 0xd800) << 10
                                         | (next_code_point - 0xdc00)) + 0x10000;
                    decoded.push(final_code_point);

                    idx += 6;
                    continue
                } else {
                    error!("[MUTF-8] error: invalid surrogate pair");
                    return Err("[MUTF-8] invalid surrogate pair");
                }
            } else {
                /* regular three-bytes code point */
                decoded.push(code_point as u32);
                idx += 3;
                continue;
            }
        }
    }

    /* Converting the decoding code points (which are u32)
     * into chars, which we just append to the decoded string */
    let mut decoded_str = String::new();
    for byte in decoded.iter() {
        match char::from_u32(*byte) {
            Some(c) => decoded_str.push(c),
            None  => debug!("no valid representation for {byte}"),
        }
    }

    Ok(decoded_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_single_byte() {
        let input = vec![0x41]; // 'A'
        let expected = "A";
        assert_eq!(decode(&input).unwrap(), expected);
    }

    #[test]
    fn test_decode_two_bytes() {
        let input = vec![0xc3, 0xa9]; // 'é'
        let expected = "é";
        assert_eq!(decode(&input).unwrap(), expected);
    }

    #[test]
    fn test_decode_three_bytes() {
        let input = vec![0xe6, 0xb0, 0xb4]; // '水'
        let expected = "水";
        assert_eq!(decode(&input).unwrap(), expected);
    }

    #[test]
    fn test_decode_error_two_bytes() {
        let input = vec![0xc3]; // Incomplete two-byte sequence
        let result = decode(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_error_three_bytes() {
        let input = vec![0xe6, 0xb0]; // Incomplete three-byte sequence
        let result = decode(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_surrogate_pairs() {
        let raw = vec![0xED, 0xA0, 0x80, 0xED, 0xB0, 0x80];
        let expected_result = "\u{10000}";

        assert_eq!(decode(&raw), Ok(expected_result.to_string()));
    }

    #[test]
    fn test_decode_invalid_surrogate_pair() {
        let raw = vec![0xED, 0xA0, 0x80, 0xED, 0xA0, 0x00]; // Invalid surrogate pair
        let result = decode(&raw);

        assert_eq!(result.unwrap_err().to_string(),
                   "[MUTF-8] invalid surrogate pair");
    }

    #[test]
    fn test_decode_truncated_surrogate_pair() {
        let raw = vec![0xED, 0xA0, 0x80]; // Truncated surrogate pair
        let result = decode(&raw);

        assert_eq!(result.unwrap_err().to_string(),
                   "[MUTF-8] truncated surrogate pair");
    }
}
