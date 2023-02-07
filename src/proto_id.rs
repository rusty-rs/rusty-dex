use std::io::{Seek, SeekFrom};
use std::collections::HashMap;

use crate::dex_reader::DexReader;
use crate::strings::StringData;

#[derive(Debug)]
pub struct ProtoIdItem {
    shorty_idx: u32,
    return_type_idx: u32,
    parameters_off: u32,
}

#[derive(Debug)]
pub struct ProtoIdList {
    pub items: HashMap<u32, ProtoIdItem>
}

impl ProtoIdList {
    pub fn build(dex_reader: &mut DexReader,
                 offset: u32,
                 size: u32,
                 strings_list: &StringData) -> Self {
        dex_reader.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        let mut protos = HashMap::new();

        for i in 0..size {
            let proto_offset = dex_reader.read_u32().unwrap();
            let current_offset = dex_reader.bytes.position();

            dex_reader.bytes.seek(SeekFrom::Start(proto_offset.into())).unwrap();

            let shorty_idx = dex_reader.read_u32().unwrap();
            let return_type_idx = dex_reader.read_u32().unwrap();
            let parameters_off = dex_reader.read_u32().unwrap();
            protos.insert(proto_offset,
                          ProtoIdItem { shorty_idx,
                                        return_type_idx,
                                        parameters_off });
                          // strings_list.strings[offset as usize].string.clone());

            dex_reader.bytes.seek(SeekFrom::Start(current_offset)).unwrap();
        }

        ProtoIdList { items: protos }
    }
}
