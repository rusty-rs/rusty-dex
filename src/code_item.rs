use std::io::{Seek, SeekFrom};

use crate::dex_reader::DexReader;
use crate::dex_types::DexTypes;
use crate::instructions::Instruction;

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
    catch_all_addr: Option<u32>,
}

#[derive(Debug)]
pub struct EncodedTypeAddrPair {
    decoded_type: String,
    addr        : u32,
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
                 offset: u32,
                 types_list: &DexTypes) -> Self {
        /* Go to start of code item */
        dex_reader.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        /* Get the metadata */
        let registers_size = dex_reader.read_u16().unwrap();
        // println!("---> registers_size {registers_size}");
        let ins_size       = dex_reader.read_u16().unwrap();
        // println!("---> ins_size {ins_size}");
        let outs_size      = dex_reader.read_u16().unwrap();
        // println!("---> outs_size {outs_size}");
        let tries_size     = dex_reader.read_u16().unwrap();
        // println!("---> tries_size {tries_size}");
        let debug_info_off = dex_reader.read_u32().unwrap();
        // println!("---> debug_info_off {debug_info_off}");
        let insns_size     = dex_reader.read_u32().unwrap();
        // println!("---> insns_size {insns_size}");

        /* Get the actual bytecode */
        let mut insns = Vec::with_capacity(insns_size as usize);
        // FIXME: no pretty but i'm tired. doing it like this to make sure we use the right
        // endianess when reading the bytecode
        for _ in 0..insns_size {
            // insns.push(dex_reader.read_u16().unwrap());
            let ins = dex_reader.read_u16().unwrap();
            insns.push(ins);
            // TODO: create some kind of reader here that parses the
            // instructions with the right number of 16 bits words
        }

        /* Check if there is some padding */
        if tries_size != 0 && insns_size % 2 == 1 {
            _ = dex_reader.read_u16().unwrap();
        }

        let mut tries = Vec::<TryItem>::new();
        let mut handlers = Vec::<EncodedCatchHandler>::new();

        if tries_size != 0 {
            tries = Vec::with_capacity(tries_size as usize);
            for _ in 0..tries_size {
                let start_addr = dex_reader.read_u32().unwrap();
                let insn_count = dex_reader.read_u16().unwrap();
                let handler_off = dex_reader.read_u16().unwrap();

                tries.push(TryItem {
                    start_addr,
                    insn_count,
                    handler_off
                });
            }

            let (handlers_list_size, _) = dex_reader.read_uleb128().unwrap();
            handlers = Vec::with_capacity(handlers_list_size as usize);

            for _ in 0..handlers_list_size {
                let (handler_size, _) = dex_reader.read_sleb128().unwrap();
                let mut type_add_pairs = Vec::with_capacity(handler_size.unsigned_abs() as usize);

                for _ in 0..handler_size.abs() {
                    let (type_idx, _) = dex_reader.read_uleb128().unwrap();
                    let decoded_type = types_list.items.get(type_idx as usize)
                                                       .unwrap()
                                                       .to_string();
                    let (addr, _) = dex_reader.read_uleb128().unwrap();

                    type_add_pairs.push(EncodedTypeAddrPair {
                        decoded_type,
                        addr
                    });

                }

                if handler_size <= 0 {
                    let (catch_all_addr, _) = dex_reader.read_uleb128().unwrap();
                    handlers.push(EncodedCatchHandler {
                        size: handler_size,
                        handlers: type_add_pairs,
                        catch_all_addr: Some(catch_all_addr)
                    });
                } else {
                    handlers.push(EncodedCatchHandler {
                        size: handler_size,
                        handlers: type_add_pairs,
                        catch_all_addr: None
                    });
                }
            }
        }

        if tries_size != 0 {
            CodeItem {
                registers_size,
                ins_size,
                outs_size,
                tries_size,
                debug_info_off,
                insns_size,
                insns,
                tries: Some(tries),
                handlers: Some(handlers)
            }
        } else {
            CodeItem {
                registers_size,
                ins_size,
                outs_size,
                tries_size,
                debug_info_off,
                insns_size,
                insns,
                tries: None,
                handlers: None
            }
        }
    }
}