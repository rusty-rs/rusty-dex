#![allow(dead_code)]

use std::io::{Seek, SeekFrom};
use std::collections::HashMap;

use crate::dex_reader::DexReader;

#[derive(Debug)]
pub struct MapListItem {
    item_type: u16,
    size: u32,
    pub offset: u32
}

#[derive(Debug)]
pub struct MapList {
    size: u32,
    pub items: HashMap<u16, MapListItem>
}

impl MapList {
    pub fn build(dex_cursor: &mut DexReader, offset: u32) -> Self {
        /* Move to start of map list */
        dex_cursor.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        let size = dex_cursor.read_u32().unwrap();
        let mut items = HashMap::new();

        for _ in 0..size {
            let item_type = dex_cursor.read_u16().unwrap();
            let _ = dex_cursor.read_u16().unwrap(); // unused for alignment
            let item_size = dex_cursor.read_u32().unwrap();
            let item_offset = dex_cursor.read_u32().unwrap();

            items.insert(item_type, MapListItem {
                item_type,
                size: item_size,
                offset: item_offset
            });
        }
        println!("{items:#?}");

        MapList { size, items }
    }
}
