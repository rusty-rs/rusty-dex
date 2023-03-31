use clap::Parser;

extern crate dex_parser;

use dex_parser::logging;
use dex_parser::{ info };

use dex_parser::dex_reader::DexReader;
use dex_parser::dex_file::DexFile;

fn main() {
    let cli_args = dex_parser::CliArgs::parse();
    logging::set_log_level(cli_args.log_level);
    info!("Set log level to {}", cli_args.log_level);

    info!("Loading classes.dex from {}", cli_args.apk);
    let dex_reader = DexReader::build(&cli_args.apk);

    info!("Parsing DEX file");
    let dex_file = DexFile::build(dex_reader);
    dex_file.disasm();
}
