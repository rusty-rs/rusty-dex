#![allow(dead_code)]

use std::io::{Seek, SeekFrom};
use std::io::BufRead;
use std::cmp::Ordering;

use crate::mutf8;

use crate::error;
use crate::dex_reader::DexReader;

#[derive(Debug, PartialEq)]
pub struct DexStringsItem {
    utf16_size: u32,
    offset: u32,
    is_raw: bool,  // sometimes decoding fails but we still need an entry
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
