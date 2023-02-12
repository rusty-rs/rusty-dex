use std::io::{Seek, SeekFrom};
use std::collections::HashMap;

use crate::dex_reader::DexReader;
use crate::dex_strings::DexStrings;

#[derive(Debug)]
pub struct DexTypes{
    pub items: HashMap<u32, String>
}

impl DexTypes {
    pub fn build(dex_reader: &mut DexReader,
                 offset: u32,
                 size: u32,
                 strings_list: &DexStrings) -> Self {
        dex_reader.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        let mut types = HashMap::new();

        for _ in 0..size {
            let offset = dex_reader.read_u32().unwrap();
            types.insert(offset,
                         strings_list.strings[offset as usize].string.clone());
        }
        DexTypes { items: types }
    }
}
