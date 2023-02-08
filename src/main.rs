use std::fs::File;
use std::io::{Read, Cursor, Seek, SeekFrom};
use std::env;
use std::process::exit;
use zip::ZipArchive;

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
pub mod class_def;
use crate::dex_reader::DexReader;
use crate::dex_file::DexHeader;
use crate::map_list::MapList;
use crate::strings::StringData;
use crate::type_id::TypeIdList;
use crate::proto_id::ProtoIdList;
use crate::field_id::FieldIdList;
use crate::method_id::MethodIdList;
use crate::class_def::ClassDefList;
use crate::call_site::CallSiteList;

use crate::constants::MapItemType;

fn main() {
    /* Check CLI arguments */
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./{:} [APK]", args[0]);
        exit(22);   /* Invalid arg */
    }

    let apk_path = &args[1];
    println!("[+] Parsing {}", apk_path);

    let mut raw_file = File::open(apk_path)
        .unwrap_or_else(|err| panic!("Could not open input file: {err}"));
    let mut zip_file = ZipArchive::new(raw_file)
        .unwrap_or_else(|err| panic!("Error: cannot create ZipArchive object: {err}"));

    println!("[+] Loading classes.dex from the APK");

    /* TODO: support merging of multiple DEX files */
    let mut dex_entry = zip_file.by_name("classes.dex")
                                .expect("Error: cannot find classes.dex in the APK");

    let mut raw_dex = Vec::new();
    dex_entry.read_to_end(&mut raw_dex)
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
                                            dex_header.proto_ids_size);
    // println!("{proto_ids_list:#?}");

    let _field_ids_list = FieldIdList::build(&mut dex_cursor,
                                            dex_header.fields_ids_off,
                                            dex_header.fields_ids_size);
    // println!("{_field_ids_list:#?}");

    let _method_ids_list = MethodIdList::build(&mut dex_cursor,
                                              dex_header.method_ids_off,
                                              dex_header.method_ids_size);
    // println!("{_method_ids_list:#?}");

    let _class_defs_list = ClassDefList::build(&mut dex_cursor,
                                              dex_header.class_defs_off,
                                              dex_header.class_defs_size);
    // println!("{_class_defs_list:#?}");
}
