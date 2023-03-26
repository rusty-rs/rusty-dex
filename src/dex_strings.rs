#![allow(dead_code)]

use std::io::{Seek, SeekFrom};
use std::io::BufRead;
use std::cmp::Ordering;

use crate::mutf8;

use crate::error;
use crate::dex_reader::DexReader;

#[derive(Debug, PartialEq)]
pub struct DexStringsItem {
    pub utf16_size: u32,
    pub offset: u32,
    pub is_raw: bool,  // sometimes decoding fails but we still need an entry
                   // in the list so we keep the raw bytes
    pub string: String
}

#[derive(Debug)]
pub struct DexStrings {
    pub strings: Vec<DexStringsItem>
}

impl DexStrings {
    fn sort(a: &DexStringsItem, b: &DexStringsItem) -> Ordering {
        // TODO: this seems to work (tested on app for which I have
        // the Java source code) but this is not following the
        // documentation. The documentation says:
        //
        // "This list must be sorted by string contents, using UTF-16
        // code point values (not in a locale-sensitive manner)"
        //
        // However for some reason this is giving me issues later on
        // when decoding e.g., prototypes.
        a.offset.cmp(&b.offset)
    }

    pub fn build(dex_reader: &mut DexReader, offset: u32, size: u32) -> Self {
        /* Move to start of map list */
        dex_reader.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        let mut strings = Vec::new();

        for _ in 0..size {
            let string_offset = dex_reader.read_u32().unwrap();
            let current_offset = dex_reader.bytes.position();

            dex_reader.bytes.seek(SeekFrom::Start(string_offset.into())).unwrap();

            let (utf16_size, _) = dex_reader.read_uleb128().unwrap();
            if utf16_size > 0 {
                let mut raw_string = Vec::with_capacity(utf16_size as usize);
                dex_reader.bytes.read_until(0, &mut raw_string).unwrap();
                raw_string.pop();

                let (decoded, is_raw) = match mutf8::decode(&raw_string) {
                    Ok(decoded) => (decoded, false),
                    Err(_) => {
                        error!("invalid MUTF-8 string");
                        (String::from(""), true)
                    }
                };

                strings.push(DexStringsItem {
                    utf16_size,
                    offset: string_offset,
                    is_raw,
                    string: decoded,
                });
            } else {
                strings.push(DexStringsItem {
                    utf16_size,
                    offset: string_offset,
                    is_raw: false,
                    string: String::new(),
                });
            }

            dex_reader.bytes.seek(SeekFrom::Start(current_offset)).unwrap();

        }

        strings.sort_by(DexStrings::sort);
        strings.dedup();

        DexStrings { strings }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_with_empty_strings() {
        let data = vec![
            0x64, 0x65, 0x78, 0x0a, 0x30, 0x33, 0x35, 0x00, 0x00, 0x00,  // DEX magic
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // endianness tag
        ];
        let mut dex_reader = DexReader::build(data);
        let dex_strings = DexStrings::build(&mut dex_reader, 44, 0);

        assert_eq!(dex_strings.strings.len(), 0);
    }

    #[test]
    fn test_build_with_non_empty_strings() {
        let data = vec![
            0x64, 0x65, 0x78, 0x0a, 0x30, 0x33, 0x35, 0x00, 0x00, 0x00,  // DEX magic
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // endianness tag
            // offsets
            0x3E, 0x00, 0x00, 0x00,
            0x46, 0x00, 0x00, 0x00,
            0x68, 0x00, 0x00, 0x00,

            // strings size and data
            0x0C,
            b'H', b'e', b'l', b'l', b'o', b'!', 0x00, // string #0 value
            0x48,
            b'T', b'h', b'i', b's', b' ', b'i', b's', b' ', b'a', b' ', b't', b'e', b's', b't', b'.', b' ', b'\"', b'A', b'B', b'C', b'D', b'\"', b' ', b'i', b'n', b' ', b'M', b'U', b'T', b'F', b'-', b'8', 0x00, // string #1 value
            0x00,
        ];

        let mut dex_reader = DexReader::build(data);
        let dex_strings = DexStrings::build(&mut dex_reader, 50, 3);

        assert_eq!(dex_strings.strings.len(), 3);
        assert_eq!(dex_strings.strings[0],
                   DexStringsItem {
                       utf16_size: 12,
                       offset: 0x3E,
                       is_raw: false,
                       string: String::from("Hello!")
                   });
        assert_eq!(dex_strings.strings[1],
                   DexStringsItem {
                       utf16_size: 72,
                       offset: 0x46,
                       is_raw: false,
                       string: String::from("This is a test. \"ABCD\" in MUTF-8")
                   });
        assert_eq!(dex_strings.strings[2],
                   DexStringsItem {
                       utf16_size: 0,
                       offset: 0x68,
                       is_raw: false,
                       string: String::from(""),
                   });
    }

    #[test]
    fn test_build_with_invalid_string() {
        let data = vec![
            0x64, 0x65, 0x78, 0x0a, 0x30, 0x33, 0x35, 0x00, 0x00, 0x00,  // DEX magic
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // nothing
            0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // endianness tag
            // offsets
            0x36, 0x00, 0x00, 0x00,

            // string size and data
            0x02,
            0xc3, 0x00    // incomplete MUTF-8 two-byte sequence
        ];

        let mut dex_reader = DexReader::build(data);
        let dex_strings = DexStrings::build(&mut dex_reader, 50, 1);

        assert_eq!(dex_strings.strings.len(), 1);
        assert_eq!(dex_strings.strings[0],
                   DexStringsItem {
                       utf16_size: 2,
                       offset: 54,
                       is_raw: true,
                       string: String::from(""),
                   });
    }
}
