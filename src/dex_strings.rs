#![allow(dead_code)]

use std::io::{Seek, SeekFrom};
use std::io::BufRead;
use std::cmp::Ordering;

use crate::dex_reader::DexReader;

#[derive(Debug, PartialEq)]
pub struct DexStringsItem {
    utf16_size: u32,
    offset: u32,
    is_raw: bool,  // sometimes decoding fails but we still need an entry
                   // in the list so we keep the raw bytes
    string: String
}

#[derive(Debug)]
pub struct DexStrings {
    pub strings: Vec<String>
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

                // TODO: `mutf8::decode()` has some issues which leads to
                // string ordering issues. For now we use `String::from_utf8_lossy()`
                // which works as long as the app doesn't actually use UTF-16
                let decoded = String::from_utf8_lossy(&raw_string).to_string();
                /*
                let (decoded, is_raw) = match mutf8::decode(&raw_string) {
                    Ok(decoded) => (decoded, false),
                    Err(_) => {
                        error!("invalid MUTF-8 string");
                        (String::from(""), true)
                    }
                };
                */

                strings.push(DexStringsItem {
                    utf16_size,
                    offset: string_offset,
                    is_raw: true,
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
        let mut uniq_strings: Vec<String> = strings.into_iter()
                                                   .map(|x| x.string)
                                                   .collect();
        uniq_strings.dedup();

        DexStrings { strings: uniq_strings }
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
        assert_eq!(dex_strings.strings[0], String::from("Hello!"));
        assert_eq!(dex_strings.strings[1], String::from("This is a test. \"ABCD\" in MUTF-8"));
        assert_eq!(dex_strings.strings[2], String::from(""));
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
        assert_eq!(dex_strings.strings[0], String::from(""));
    }
}
