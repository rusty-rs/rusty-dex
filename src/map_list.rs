use std::io::{Seek, SeekFrom};
use std::fmt;

use crate::endianness::DexCursor;
use crate::constants::MapItemType;

#[derive(Debug)]
struct MapListItem {
    item_type: u16,
    size: u32,
    offset: u32
}

impl fmt::Display for MapListItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let item_type_str = match self.item_type {
            MapItemType::HEADER_ITEM => format!("HEADER_ITEM"),
            MapItemType::STRING_ID_ITEM => format!("STRING_ID_ITEM"),
            MapItemType::TYPE_ID_ITEM => format!("TYPE_ID_ITEM"),
            MapItemType::PROTO_ID_ITEM => format!("PROTO_ID_ITEM"),
            MapItemType::FIELD_ID_ITEM => format!("FIELD_ID_ITEM"),
            MapItemType::METHOD_ID_ITEM => format!("METHOD_ID_ITEM"),
            MapItemType::CLASS_DEF_ITEM => format!("CLASS_DEF_ITEM"),
            MapItemType::CALL_SITE_ID_ITEM => format!("CALL_SITE_ID_ITEM"),
            MapItemType::METHOD_HANDLE_ITEM => format!("METHOD_HANDLE_ITEM"),
            MapItemType::MAP_LIST => format!("MAP_LIST"),
            MapItemType::TYPE_LIST => format!("TYPE_LIST"),
            MapItemType::ANNOTATION_SET_REF_LIST => format!("ANNOTATION_SET_REF_LIST"),
            MapItemType::ANNOTATION_SET_ITEM => format!("ANNOTATION_SET_ITEM"),
            MapItemType::CLASS_DATA_ITEM => format!("CLASS_DATA_ITEM"),
            MapItemType::CODE_ITEM => format!("CODE_ITEM"),
            MapItemType::STRING_DATA_ITEM => format!("STRING_DATA_ITEM"),
            MapItemType::DEBUG_INFO_ITEM => format!("DEBUG_INFO_ITEM"),
            MapItemType::ANNOTATION_ITEM => format!("ANNOTATION_ITEM"),
            MapItemType::ENCODED_ARRAY_ITEM => format!("ENCODED_ARRAY_ITEM"),
            MapItemType::ANNOTATIONS_DIRECTORY_ITEM => format!("ANNOTATIONS_DIRECTORY_ITEM"),
            MapItemType::HIDDENAPI_CLASS_DATA_ITEM => format!("HIDDENAPI_CLASS_DATA_ITEM"),
            other => panic!("Error: unknown item type: {other:X}"),
        };
        write!(f,"{}", item_type_str)
    }
}

#[derive(Debug)]
pub struct MapList {
    size: u32,
    items: Vec<MapListItem>
}

impl MapList {
    pub fn build(dex_cursor: &mut DexCursor, offset: u32) -> Self {
        /* Move to start of map list */
        dex_cursor.bytes.seek(SeekFrom::Start(offset.into()));

        let size = dex_cursor.read_u32().unwrap();
        let mut items = Vec::new();

        for _ in 0..size {
            let item_type = dex_cursor.read_u16().unwrap();
            let _ = dex_cursor.read_u16().unwrap(); // unused for alignment
            let item_size = dex_cursor.read_u32().unwrap();
            let item_offset = dex_cursor.read_u32().unwrap();

            items.push(MapListItem {
                item_type,
                size: item_size,
                offset: item_offset
            });
        }

        MapList { size, items }
    }
}
