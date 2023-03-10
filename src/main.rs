use std::fs::File;
use std::io::{Read, Cursor, Seek, SeekFrom};
use zip::ZipArchive;
use clap::Parser;

extern crate dex_parser;

use dex_parser::logging;
use dex_parser::{info, die};

use dex_parser::dex_reader::DexReader;
use dex_parser::dex_file::DexFile;
use dex_parser::dex_header::DexHeader;
use dex_parser::map_list::MapList;
use dex_parser::dex_strings::DexStrings;
use dex_parser::dex_types::DexTypes;
use dex_parser::dex_protos::DexProtos;
use dex_parser::dex_fields::DexFields;
use dex_parser::dex_methods::DexMethods;
use dex_parser::dex_classes::DexClasses;
use dex_parser::method_handle::MethodHandleList;
use dex_parser::constants::MapItemType;

fn main() {
    let cli_args = dex_parser::CliArgs::parse();
    logging::set_log_level(cli_args.log_level);
    info!("Set log level to {}", cli_args.log_level);

    info!("Parsing {}", cli_args.apk);

    let raw_file = File::open(cli_args.apk)
        .unwrap_or_else(|err| die!("Could not open input file: {err}"));
    let mut zip_file = ZipArchive::new(raw_file)
        .unwrap_or_else(|err| die!("Error: cannot create ZipArchive object: {err}"));

    info!("Loading classes.dex from the APK");

    /* TODO: support merging of multiple DEX files
     * I dug around a little and it seems like this should be
     * pretty straightforward. Each classes.dex file in apps apps
     * with multiple DEX files are basically all valid and can be
     * parsed the same way. Then we can merge all the data into
     * one `DexFile` struct by eliminating duplicates and
     * re-sorting the list of values.
     **/
    let mut dex_entry = zip_file.by_name("classes.dex")
                                .unwrap_or_else(|_| die!("Error: cannot find classes.dex in the APK"));

    let mut raw_dex = Vec::new();
    dex_entry.read_to_end(&mut raw_dex)
             .unwrap_or_else(|err| die!("Could not read input file: {err}"));

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
                                            &field_ids_list,
                                            &type_ids_list,
                                            &method_ids_list);

    if let Some(map) = map_list.items.get(&MapItemType::METHOD_HANDLE_ITEM) {
        let _method_handles_list = MethodHandleList::build(&mut dex_cursor,
                                                          map.offset,
                                                          map.size);
    }

    let _dex_file = DexFile {
        header: dex_header,
        strings: strings_list,
        types: type_ids_list,
        protos: proto_ids_list,
        fields: field_ids_list,
        methods: method_ids_list,
        classes: class_defs_list,
    };
}
