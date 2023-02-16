use std::io::{Seek, SeekFrom};

use crate::dex_reader::DexReader;
use crate::dex_strings::DexStrings;

#[derive(Debug)]
pub struct DexTypeItem {
    offset: u32,
    pub str_type: String,
}

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
            types.push(DexTypeItem {
                offset,
                str_type: strings_list.strings[offset as usize].string.clone()
            });
        }
        types.sort_by(|a, b| a.offset.cmp(&b.offset));
        let mut items = Vec::new();
        for dex_type in types.iter() {
            if ! items.contains(&dex_type.str_type) {
                items.push(dex_type.str_type.clone());
            }
        }
        DexTypes { items }
    }
}
