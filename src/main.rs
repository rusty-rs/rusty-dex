#[allow(dead_code)]

use std::fs::File;
use std::io::{Read, Cursor, Seek, SeekFrom};
use std::env;
use std::process::exit;
use zip::ZipArchive;

pub mod logging;

pub mod dex_header;
pub mod dex_file;
pub mod map_list;
pub mod error;
pub mod dex_reader;
pub mod adler32;
pub mod constants;
pub mod dex_strings;
pub mod dex_types;
pub mod dex_protos;
pub mod dex_fields;
pub mod dex_methods;
pub mod dex_classes;
pub mod access_flags;
pub mod method_handle;

use crate::dex_reader::DexReader;
use crate::dex_file::DexFile;
use crate::dex_header::DexHeader;
use crate::map_list::MapList;
use crate::dex_strings::DexStrings;
use crate::dex_types::DexTypes;
use crate::dex_protos::DexProtos;
use crate::dex_fields::DexFields;
use crate::dex_methods::DexMethods;
use crate::dex_classes::DexClasses;
use crate::method_handle::MethodHandleList;

use crate::constants::MapItemType;

fn main() {
    // TODO: use CLI arg
    logging::set_log_level(3);

    /* Check CLI arguments */
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("Usage: cargo run [APK]");
        exit(22);   /* Invalid arg */
    }

    let apk_path = &args[1];
    info!("Parsing {}", apk_path);

    let mut raw_file = File::open(apk_path)
        .unwrap_or_else(|err| panic!("Could not open input file: {err}"));
    let mut zip_file = ZipArchive::new(raw_file)
        .unwrap_or_else(|err| panic!("Error: cannot create ZipArchive object: {err}"));

    info!("Loading classes.dex from the APK");

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

    let map_list = MapList::build(&mut dex_cursor, dex_header.map_off);

    let strings_list = DexStrings::build(&mut dex_cursor,
                                         dex_header.string_ids_off,
                                         dex_header.string_ids_size);

    let type_ids_list = DexTypes::build(&mut dex_cursor,
                                          dex_header.type_ids_off,
                                          dex_header.type_ids_size,
                                          &strings_list);

    let proto_ids_list = DexProtos::build(&mut dex_cursor,
                                          dex_header.proto_ids_off,
                                          dex_header.proto_ids_size,
                                          &type_ids_list);

    let field_ids_list = DexFields::build(&mut dex_cursor,
                                          dex_header.fields_ids_off,
                                          dex_header.fields_ids_size,
                                          &type_ids_list,
                                          &strings_list);

    let method_ids_list = DexMethods::build(&mut dex_cursor,
                                            dex_header.method_ids_off,
                                            dex_header.method_ids_size,
                                            &type_ids_list,
                                            &proto_ids_list,
                                            &strings_list);

    let class_defs_list = DexClasses::build(&mut dex_cursor,
                                            dex_header.class_defs_off,
                                            dex_header.class_defs_size,
                                            &type_ids_list,
                                            &field_ids_list,
                                            &method_ids_list,
                                            &strings_list);

    if let Some(map) = map_list.items.get(&MapItemType::METHOD_HANDLE_ITEM) {
        let method_handles_list = MethodHandleList::build(&mut dex_cursor,
                                                          map.offset,
                                                          map.size);
    }

    let dex_file = DexFile {
        header: dex_header,
        strings: strings_list,
        types: type_ids_list,
        protos: proto_ids_list,
        fields: field_ids_list,
        methods: method_ids_list,
        classes: class_defs_list,
    };
}
