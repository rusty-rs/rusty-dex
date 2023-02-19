use std::io::{Seek, SeekFrom};

use crate::dex_reader::DexReader;

#[derive(Debug)]
pub struct TryItem {
    start_addr : u32,
    insn_count : u16,
    handler_off: u16
}

#[derive(Debug)]
pub struct EncodedCatchHandler {
    size          : i32,
    handlers      : Vec<EncodedTypeAddrPair>,
    catch_all_addr: u32,
}

#[derive(Debug)]
pub struct EncodedTypeAddrPair {
    type_idx: u32,
    addr    : u32,
}

#[derive(Debug)]
pub struct CodeItem {
    registers_size: u16,
    ins_size      : u16,
    outs_size     : u16,
    tries_size    : u16,
    debug_info_off: u32,
    insns_size    : u32,
    insns         : Vec<u16>,
    tries         : Option<Vec<TryItem>>,
    handlers      : Option<Vec<EncodedCatchHandler>>
}

impl CodeItem {
    pub fn build(dex_reader: &mut DexReader,
                 offset: u32) -> Self {
        /* Go to start of code item */
        dex_reader.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        /* Get the metadata */
        let registers_size = dex_reader.read_u16().unwrap();
        println!("---> registers_size {registers_size}");
        let ins_size       = dex_reader.read_u16().unwrap();
        println!("---> ins_size {ins_size}");
        let outs_size      = dex_reader.read_u16().unwrap();
        println!("---> outs_size {outs_size}");
        let tries_size     = dex_reader.read_u16().unwrap();
        println!("---> tries_size {tries_size}");
        let debug_info_off = dex_reader.read_u32().unwrap();
        println!("---> debug_info_off {debug_info_off}");
        let insns_size     = dex_reader.read_u32().unwrap();
        println!("---> insns_size {insns_size}");

        /* Get the actual bytecode */
        let mut insns = Vec::with_capacity(insns_size as usize);
        // FIXME: no pretty but i'm tired. doing it like this to make sure we use the right
        // endianess when reading the bytecode
        for _ in 0..insns_size {
            insns.push(dex_reader.read_u16().unwrap());
        }

        /* Check if there is some padding */
        if tries_size != 0 && insns_size % 2 == 1 {
            _ = dex_reader.read_u16().unwrap();
        }

        if tries_size != 0 {
            todo!("parse tries");
            todo!("parse handlers");
        }

        CodeItem {
            registers_size,
            ins_size      ,
            outs_size     ,
            tries_size    ,
            debug_info_off,
            insns_size    ,
            insns         ,
            tries: None   ,
            handlers: None
        }
    }
}
