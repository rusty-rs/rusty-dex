use std::fs::File;
use std::io::{Read, Cursor, Seek, SeekFrom};

pub mod dex_file;
pub mod map_list;
pub mod error;
pub mod dex_reader;
pub mod adler32;
pub mod constants;
pub mod strings;
pub mod type_id;
pub mod proto_id;
pub mod field_id;
pub mod method_id;
use crate::dex_reader::DexReader;
use crate::dex_file::DexHeader;
use crate::map_list::MapList;
use crate::strings::StringData;
use crate::type_id::TypeIdList;
use crate::proto_id::ProtoIdList;
use crate::field_id::FieldIdList;
use crate::method_id::MethodIdList;

fn main() {
    // TODO: CLI arg
    let fpath = "classes.dex";
    println!("[+] loading file: {fpath}");
    let mut file = File::open(fpath)
        .unwrap_or_else(|err| panic!("Could not open input file: {err}"));

    let mut raw_dex = Vec::new();
    file.read_to_end(&mut raw_dex)
        .unwrap_or_else(|err| panic!("Could not read input file: {err}"));

    /* First check endianness */
    let mut bytes = Cursor::new(&raw_dex);
    let bytes_len = bytes.seek(SeekFrom::End(0)).unwrap();
    bytes.rewind().unwrap();
    let endianness = DexReader::check_endianness(&raw_dex).unwrap();
    let mut dex_cursor = DexReader {
        bytes,
        bytes_len,
        endianness
    };

    let dex_header = DexHeader::new(&mut dex_cursor).unwrap();
    println!("{dex_header:#?}");

    let _map_list = MapList::build(&mut dex_cursor, dex_header.map_off);
    // println!("{map_list:#?}");

    let strings_list = StringData::build(&mut dex_cursor,
                                         dex_header.string_ids_off,
                                         dex_header.string_ids_size);
    // println!("{strings_list:#?}");

    let _type_ids_list = TypeIdList::build(&mut dex_cursor,
                                          dex_header.type_ids_off,
                                          dex_header.type_ids_size,
                                          &strings_list);
    // println!("{type_ids_list:#?}");

    let _proto_ids_list = ProtoIdList::build(&mut dex_cursor,
                                            dex_header.proto_ids_off,
                                            dex_header.proto_ids_size,
                                            &strings_list);
    // println!("{proto_ids_list:#?}");

    let _field_ids_list = FieldIdList::build(&mut dex_cursor,
                                            dex_header.fields_ids_off,
                                            dex_header.fields_ids_size);
    // println!("{_field_ids_list:#?}");

    let _method_ids_list = MethodIdList::build(&mut dex_cursor,
                                              dex_header.method_ids_off,
                                              dex_header.method_ids_size);
    // println!("{_method_ids_list:#?}");
}
