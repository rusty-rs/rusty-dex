use std::io::{Seek, SeekFrom};

use crate::dex_reader::DexReader;

#[derive(Debug)]
pub struct ClassDefItem {
    class_idx: u32,
    access_flags: u32,
    superclass_idx: u32,
    interfaces_off: u32,
    source_file_idx: u32,
    annotations_off: u32,
    class_data_off: u32,
    static_value_off: u32
}

#[derive(Debug)]
pub struct DexClasses {
    pub items: Vec<ClassDefItem>
}

impl DexClasses {
    pub fn build(dex_reader: &mut DexReader,
                 offset: u32,
                 size: u32) -> Self {
        dex_reader.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        let mut methods = Vec::new();

        for _ in 0..size {
            let class_idx = dex_reader.read_u32().unwrap();
            let access_flags = dex_reader.read_u32().unwrap();
            let superclass_idx = dex_reader.read_u32().unwrap();
            let interfaces_off = dex_reader.read_u32().unwrap();
            let source_file_idx = dex_reader.read_u32().unwrap();
            let annotations_off = dex_reader.read_u32().unwrap();
            let class_data_off = dex_reader.read_u32().unwrap();
            let static_value_off = dex_reader.read_u32().unwrap();

            methods.push(ClassDefItem { 
                class_idx,
                access_flags,
                superclass_idx,
                interfaces_off,
                source_file_idx,
                annotations_off,
                class_data_off,
                static_value_off
            });
        }

        DexClasses { items: methods }
    }
}
