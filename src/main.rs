use std::fs::File;
use std::io::Read;
use zip::ZipArchive;
use clap::Parser;

extern crate dex_parser;

use dex_parser::logging;
use dex_parser::{info, die};

use dex_parser::dex_reader::DexReader;
use dex_parser::dex_file::DexFile;

fn main() {
    let cli_args = dex_parser::CliArgs::parse();
    logging::set_log_level(cli_args.log_level);
    info!("Set log level to {}", cli_args.log_level);

    info!("Loading classes.dex from {}", cli_args.apk);
    let raw_file = File::open(cli_args.apk)
        .unwrap_or_else(|err| die!("Could not open input file: {err}"));
    let mut zip_file = ZipArchive::new(raw_file)
        .unwrap_or_else(|err| die!("Error: cannot create ZipArchive object: {err}"));

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

    info!("Parsing DEX file");
    let dex_reader = DexReader::build(raw_dex);
    let dex_file = DexFile::build(dex_reader);
}
