#![allow(dead_code)]

use std::io::Read;

use crate::error::DexError;
use crate::adler32;
use crate::dex::reader::DexReader;

#[derive(Debug)]
pub struct DexHeader {
    pub version: [u8; 3],
    pub checksum: u32,
    pub signature: [u8; 20],
    pub file_size: u32,
    pub header_size: u32,
    pub endian_tag: u32,
    pub link_size: u32,
    pub link_off: u32,
    pub map_off: u32,
    pub string_ids_size: u32,
    pub string_ids_off: u32,
    pub type_ids_size: u32,
    pub type_ids_off: u32,
    pub proto_ids_size: u32,
    pub proto_ids_off: u32,
    pub fields_ids_size: u32,
    pub fields_ids_off: u32,
    pub method_ids_size: u32,
    pub method_ids_off: u32,
    pub class_defs_size: u32,
    pub class_defs_off: u32,
    pub data_size: u32,
    pub data_off: u32
}

impl DexHeader {
    pub fn new(dex_cursor: &mut DexReader) -> Result<DexHeader, DexError> {
        /* DEX version */
        let mut magic = [0; 8];
        dex_cursor.bytes.read_exact(&mut magic).unwrap();
        let mut version = [0; 3];
        version[0] = magic[4];
        version[1] = magic[5];
        version[2] = magic[6];

        let checksum = dex_cursor.read_u32().unwrap();
        match adler32::verify_from_bytes(&dex_cursor.bytes, checksum) {
            Ok(_) => { },
            Err(err) => {panic!("{}", err);},
        }

        let mut signature = [0; 20];
        dex_cursor.bytes.read_exact(&mut signature).unwrap();

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
                version,
                checksum,
                signature,
                file_size,
                header_size,
                endian_tag,
                link_size,
                link_off,
                map_off,
                string_ids_size,
                string_ids_off,
                type_ids_size,
                type_ids_off,
                proto_ids_size,
                proto_ids_off,
                fields_ids_size,
                fields_ids_off,
                method_ids_size,
                method_ids_off,
                class_defs_size,
                class_defs_off,
                data_size,
                data_off
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEX_DATA: [u8; 128] = [
        0x64, 0x65, 0x78, 0x0a, 0x30, 0x33, 0x35, 0x00,
        0x81, 0xc4, 0x10, 0x8e, 0x1c, 0xd2, 0x09, 0x16,
        0x05, 0x09, 0xc2, 0xb6, 0xe6, 0xbf, 0xfb, 0x47,
        0x1c, 0xda, 0xa3, 0xab, 0xfb, 0x5d, 0xd1, 0x4a,
        0x10, 0x85, 0x1b, 0x00, 0x70, 0x00, 0x00, 0x00,
        0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x34, 0x84, 0x1b, 0x00,
        0x44, 0x42, 0x00, 0x00, 0x70, 0x00, 0x00, 0x00,
        0x41, 0x09, 0x00, 0x00, 0x80, 0x09, 0x01, 0x00,
        0xf6, 0x0d, 0x00, 0x00, 0x84, 0x2e, 0x01, 0x00,
        0x5d, 0x20, 0x00, 0x00, 0x0c, 0xd6, 0x01, 0x00,
        0xab, 0x37, 0x00, 0x00, 0xf4, 0xd8, 0x02, 0x00,
        0xef, 0x05, 0x00, 0x00, 0x4c, 0x96, 0x04, 0x00,
        0xe4, 0x30, 0x16, 0x00, 0x2c, 0x54, 0x05, 0x00,
        0xc2, 0x15, 0x12, 0x00, 0xc4, 0x15, 0x12, 0x00,
        0xf7, 0x15, 0x12, 0x00, 0x36, 0x16, 0x12, 0x00
    ];

    #[test]
    fn test_build() {
        let mut dex_reader = DexReader::build(DEX_DATA.to_vec());
        let dex_header = DexHeader::new(&mut dex_reader).unwrap();

        assert_eq!(dex_header.version, [0x30, 0x33, 0x35]);
        assert_eq!(dex_header.checksum, 0x8e10c481);
        assert_eq!(dex_header.signature, [0x1c, 0xd2, 0x09, 0x16,
                                          0x05, 0x09, 0xc2, 0xb6,
                                          0xe6, 0xbf, 0xfb, 0x47,
                                          0x1c, 0xda, 0xa3, 0xab,
                                          0xfb, 0x5d, 0xd1, 0x4a]);
        assert_eq!(dex_header.file_size,       0x001b8510);
        assert_eq!(dex_header.header_size,     0x00000070);
        assert_eq!(dex_header.endian_tag,      0x12345678);
        assert_eq!(dex_header.link_size,       0x00000000);
        assert_eq!(dex_header.link_off,        0x00000000);
        assert_eq!(dex_header.map_off,         0x001b8434);
        assert_eq!(dex_header.string_ids_size, 0x00004244);
        assert_eq!(dex_header.string_ids_off,  0x00000070);
        assert_eq!(dex_header.type_ids_size,   0x00000941);
        assert_eq!(dex_header.type_ids_off,    0x00010980);
        assert_eq!(dex_header.proto_ids_size,  0x00000df6);
        assert_eq!(dex_header.proto_ids_off,   0x00012e84);
        assert_eq!(dex_header.fields_ids_size, 0x0000205d);
        assert_eq!(dex_header.fields_ids_off,  0x0001d60c);
        assert_eq!(dex_header.method_ids_size, 0x000037ab);
        assert_eq!(dex_header.method_ids_off,  0x0002d8f4);
        assert_eq!(dex_header.class_defs_size, 0x000005ef);
        assert_eq!(dex_header.class_defs_off,  0x0004964c);
        assert_eq!(dex_header.data_size,       0x001630e4);
        assert_eq!(dex_header.data_off,        0x0005542c);
    }
}
