#![allow(dead_code)]

use std::io::{Read, Write};

use crate::error::DexError;
use crate::endianness::DexCursor;

#[derive(Debug)]
pub struct DexHeader {
    version: [u8; 3],
    checksum: u32,
    signature: [u8; 20],
    file_size: u32,
    header_size: u32,
    endian_tag: u32,
    link_size: u32,
    link_off: u32,
    map_off: u32,
    string_ids_size: u32,
    string_ids_off: u32,
    type_ids_size: u32,
    type_ids_off: u32,
    proto_ids_size: u32,
    proto_ids_off: u32,
    fields_ids_size: u32,
    fields_ids_off: u32,
    method_ids_size: u32,
    method_ids_off: u32,
    class_defs_size: u32,
    class_defs_off: u32,
    data_size: u32,
    data_off: u32
}

impl DexHeader {
    pub fn new(mut raw_data: &[u8]) -> Result<DexHeader, DexError> {
        /* First check endianness */
        let endianness = DexCursor::check_endianness(&raw_data).unwrap();
        println!("{:?}", endianness);
        let mut dex_cursor = DexCursor { bytes: raw_data, endianness: endianness };

        /* DEX version */
        let mut magic = [0; 8];
        let _ = dex_cursor.bytes.take(8).read(&mut magic);
        let mut version = [0; 3];
        version[0] = magic[4];
        version[1] = magic[5];
        version[2] = magic[6];

        let checksum = dex_cursor.read_u32().unwrap();
        println!("{:X?}", checksum);

        let mut signature = [0; 20];
        let _ = dex_cursor.bytes.take(20).read(&mut signature);

        /* TODO: this does not consume the data! */
        /* TODO: repalce with DexCursor or something that has endianness and cursor */
        let file_size = dex_cursor.read_u32().unwrap();
        let header_size = dex_cursor.read_u32().unwrap();
        let endian_tag = dex_cursor.read_u32().unwrap();

        let link_size = dex_cursor.read_u32().unwrap();
        let link_off = dex_cursor.read_u32().unwrap();
        let map_off = dex_cursor.read_u32().unwrap();
        let string_ids_size = dex_cursor.read_u32().unwrap();
        let string_ids_off = dex_cursor.read_u32().unwrap();
        let type_ids_size = dex_cursor.read_u32().unwrap();
        let type_ids_off = dex_cursor.read_u32().unwrap();
        let proto_ids_size = dex_cursor.read_u32().unwrap();
        let proto_ids_off = dex_cursor.read_u32().unwrap();
        let fields_ids_size = dex_cursor.read_u32().unwrap();
        let fields_ids_off = dex_cursor.read_u32().unwrap();
        let method_ids_size = dex_cursor.read_u32().unwrap();
        let method_ids_off = dex_cursor.read_u32().unwrap();
        let class_defs_size = dex_cursor.read_u32().unwrap();
        let class_defs_off = dex_cursor.read_u32().unwrap();
        let data_size = dex_cursor.read_u32().unwrap();
        let data_off = dex_cursor.read_u32().unwrap();

        Ok(DexHeader {
                version: version,
                checksum: checksum,
                signature: signature,
                file_size: file_size,
                header_size: header_size,
                endian_tag: endian_tag,
                link_size: link_size,
                link_off: link_off,
                map_off: map_off,
                string_ids_size: string_ids_size,
                string_ids_off: string_ids_off,
                type_ids_size: type_ids_size,
                type_ids_off: type_ids_off,
                proto_ids_size: proto_ids_size,
                proto_ids_off: proto_ids_off,
                fields_ids_size: fields_ids_size,
                fields_ids_off: fields_ids_off,
                method_ids_size: method_ids_size,
                method_ids_off: method_ids_off,
                class_defs_size: class_defs_size,
                class_defs_off: class_defs_off,
                data_size: data_size,
                data_off:data_off
        })
    }
}
