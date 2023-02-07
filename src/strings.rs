#![allow(dead_code)]

use std::io::{Seek, SeekFrom};
use std::io::BufRead;

use crate::dex_reader::DexReader;

#[derive(Debug)]
pub struct StringDataItem {
    utf16_size: u32,
    offset: u32,
    is_raw: bool,  // sometimes decoding fails but we still need an entry
                   // in the list so we keep the raw bytes
    pub string: String
}

#[derive(Debug)]
pub struct StringData {
    pub strings: Vec<StringDataItem>
}

impl StringData {
    pub fn build(dex_reader: &mut DexReader, offset: u32, size: u32) -> Self {
        /* Move to start of map list */
        dex_reader.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        let mut strings = Vec::new();

        for _ in 0..size {
            let string_offset = dex_reader.read_u32().unwrap();
            let current_offset = dex_reader.bytes.position();

            dex_reader.bytes.seek(SeekFrom::Start(string_offset.into())).unwrap();

            let (utf16_size, _) = dex_reader.read_uleb128().unwrap();
            // TODO: make sure we are properly handling the case of U+0000
            if utf16_size > 0 {
                let mut raw_string = Vec::with_capacity(utf16_size as usize);
                dex_reader.bytes.read_until(0, &mut raw_string).unwrap();
                raw_string.pop();

                let (decoded, is_raw) = match String::from_utf8(raw_string) {
                    Ok(decoded) => (decoded, false),
                    Err(_) => (String::from(""), true)
                };

                strings.push(StringDataItem {
                    utf16_size,
                    offset: string_offset,
                    is_raw,
                    string: decoded,
                });
            }

            dex_reader.bytes.seek(SeekFrom::Start(current_offset)).unwrap();

        }

        StringData { strings }
    }
}
