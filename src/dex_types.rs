use std::io::{Seek, SeekFrom};

use crate::dex_reader::DexReader;
use crate::dex_strings::DexStrings;

#[derive(Debug)]
pub struct DexTypes{
    pub items: Vec<String>
}

impl DexTypes {
    pub fn build(dex_reader: &mut DexReader,
                 offset: u32,
                 size: u32,
                 strings_list: &DexStrings) -> Self {
        dex_reader.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        let mut types = Vec::new();

        for _ in 0..size {
            let offset = dex_reader.read_u32().unwrap();
            types.push(strings_list.strings[offset as usize].string.clone());
        }
        types.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        DexTypes { items: types }
    }
}
