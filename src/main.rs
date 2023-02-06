use std::fs::File;
use std::io::Read;

pub mod dex_file;
pub mod error;
pub mod endianness;
use crate::dex_file::DexHeader;
use crate::endianness::DexEndianness;

fn main() {
    // TODO: CLI arg
    let fpath = "classes.dex";
    println!("[+] loading file: {fpath}");
    let mut file = File::open(fpath)
        .unwrap_or_else(|err| panic!("Could not open input file: {}", err));

    let mut raw_dex = Vec::new();
    file.read_to_end(&mut raw_dex)
        .unwrap_or_else(|err| panic!("Could not read input file: {}", err));

    println!("{:?}", &raw_dex[0..100]);
    println!("{}", raw_dex.len());
    let dex_file = DexHeader::new(&mut raw_dex);
    println!("{:#?}", dex_file);
}
