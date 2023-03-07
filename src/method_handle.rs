use std::io::{Seek, SeekFrom};

use crate::dex_reader::DexReader;

// FIXME: untested

#[derive(Debug)]
pub struct MethodHandleItem{
    method_handle_type: u16,
    field_or_method_id: u16,
}

#[derive(Debug)]
pub struct MethodHandleList{
    pub items: Vec<MethodHandleItem>
}

impl MethodHandleList {
    pub fn build(dex_reader: &mut DexReader,
                 offset: u32,
                 size: u32) -> Self {
        dex_reader.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        let mut handles = Vec::new();

        for _ in 0..size {
            let method_handle_type = dex_reader.read_u16().unwrap();
            let _ = dex_reader.read_u16().unwrap();
            let field_or_method_id = dex_reader.read_u16().unwrap();
            let _ = dex_reader.read_u16().unwrap();

            handles.push(MethodHandleItem {
                method_handle_type,
                field_or_method_id
            });
        }

        MethodHandleList { items: handles }
    }
}
