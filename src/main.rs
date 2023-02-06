use std::fs::File;
use std::io::Read;

pub mod dex_file;
pub mod error;
pub mod endianness;
pub mod adler32;
use crate::dex_file::DexHeader;

fn main() {
    // TODO: CLI arg
    let fpath = "classes.dex";
    println!("[+] loading file: {fpath}");
    let mut file = File::open(fpath)
        .unwrap_or_else(|err| panic!("Could not open input file: {err}"));

    let mut raw_dex = Vec::new();
    file.read_to_end(&mut raw_dex)
        .unwrap_or_else(|err| panic!("Could not read input file: {err}"));

    let dex_header = DexHeader::new(&raw_dex);
    println!("{dex_header:#?}");
}
