use crate::{error, debug};

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

        /* TODO: deal with surrogate pairs */
        if raw[idx] as u16 >= 0xd800 && raw[idx] as u16 <= 0xdbff {
            println!("-------------------");
            println!("{:08X} {:010b}", raw[idx], raw[idx]);
            println!("{:08X} {:010b}", raw[idx + 1], raw[idx + 1]);
            panic!();
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
            decoded.push(code_point as u32);

            idx += 3;
            continue;
        }

        panic!("WTF: {:?}", raw[idx]);
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
