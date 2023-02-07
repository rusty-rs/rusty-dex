use std::io::{Seek, SeekFrom};

use crate::dex_reader::DexReader;

#[derive(Debug)]
pub struct FieldIdItem {
    class_idx: u16,
    type_idx: u16,
    name_idx: u32,
}

#[derive(Debug)]
pub struct FieldIdList {
    pub items: Vec<FieldIdItem>
}

impl FieldIdList {
    pub fn build(dex_reader: &mut DexReader,
                 offset: u32,
                 size: u32) -> Self {
        dex_reader.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        let mut fields = Vec::new();

        for _ in 0..size {
            let class_idx = dex_reader.read_u16().unwrap();
            let type_idx = dex_reader.read_u16().unwrap();
            let name_idx = dex_reader.read_u32().unwrap();

            fields.push(FieldIdItem { 
                class_idx,
                type_idx,
                name_idx
            });
        }

        FieldIdList { items: fields }
    }
}
