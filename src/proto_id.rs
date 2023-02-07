use std::io::{Seek, SeekFrom};

use crate::dex_reader::DexReader;

#[derive(Debug)]
pub struct ProtoIdItem {
    shorty_idx: u32,
    return_type_idx: u32,
    parameters_off: u32,
}

#[derive(Debug)]
pub struct ProtoIdList {
    pub items: Vec<ProtoIdItem>
}

impl ProtoIdList {
    pub fn build(dex_reader: &mut DexReader,
                 offset: u32,
                 size: u32) -> Self {
        dex_reader.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        let mut protos = Vec::new();

        for _ in 0..size {
            let shorty_idx = dex_reader.read_u32().unwrap();
            let return_type_idx = dex_reader.read_u32().unwrap();
            let parameters_off = dex_reader.read_u32().unwrap();

            protos.push(ProtoIdItem {
                shorty_idx,
                return_type_idx,
                parameters_off
            });
        }

        ProtoIdList { items: protos }
    }
}
