use std::io::{Seek, SeekFrom};

use crate::dex_reader::DexReader;

#[derive(Debug)]
pub struct MethodIdItem {
    class_idx: u16,
    proto_idx: u16,
    name_idx: u32,
}

#[derive(Debug)]
pub struct DexMethods {
    pub items: Vec<MethodIdItem>
}

impl DexMethods {
    pub fn build(dex_reader: &mut DexReader,
                 offset: u32,
                 size: u32) -> Self {
        dex_reader.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        let mut methods = Vec::new();

        for _ in 0..size {
            let class_idx = dex_reader.read_u16().unwrap();
            let proto_idx = dex_reader.read_u16().unwrap();
            let name_idx = dex_reader.read_u32().unwrap();

            methods.push(MethodIdItem { 
                class_idx,
                proto_idx,
                name_idx
            });
        }

        DexMethods { items: methods }
    }
}
