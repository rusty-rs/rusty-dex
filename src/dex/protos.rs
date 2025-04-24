//! Representation of method prototypes
//!
//! This module contains the logic to decode method prototypes from a DEX file.

use std::io::{Seek, SeekFrom};
use std::cmp::Ordering;

use crate::dex::reader::DexReader;
use crate::dex::types::DexTypes;
use crate::error::DexError;

/// Internal representation of a prototype index
#[derive(Debug)]
struct ProtoIdItem {
    shorty_idx: u32,
    return_type_idx: u32,
    parameters_off: u32,
    parameters_off_list: Vec<u16>,  // used to sort items
    proto: String,
}

/// List of decoded prototypes in the DEX files
#[derive(Debug)]
pub struct DexProtos {
    pub items: Vec<String>
}

impl DexProtos {
    /// Sorting method for prototypes
    fn sort(a: &ProtoIdItem, b: &ProtoIdItem) -> Ordering {
        // First sort by return type
        let sort_return = a.return_type_idx.cmp(&b.return_type_idx);

        if sort_return == Ordering::Equal {
            // Same return type, sort by params offsets
            return a.parameters_off_list.cmp(&b.parameters_off_list);
        }

        sort_return
    }

    /// Parse the prototypes from the reader
    pub fn build(dex_reader: &mut DexReader,
                 offset: u32,
                 size: u32,
                 types_list: &DexTypes) -> Result<Self, DexError> {
        dex_reader.bytes.seek(SeekFrom::Start(offset.into()))?;

        let mut protos = Vec::new();

        for _ in 0..size {
            let shorty_idx = dex_reader.read_u32()?;
            let return_type_idx = dex_reader.read_u32()?;
            let parameters_off = dex_reader.read_u32()?;

            // Decode the prototype
            let mut proto = String::new();
            let mut parameters_off_list = Vec::new();
            if parameters_off == 0 {
                // No parameters in this prototype
                proto.push_str("()");
            } else {
                // Save current stream position
                let current_pos = dex_reader.bytes.position();

                // Decode the parameters
                dex_reader.bytes.seek(SeekFrom::Start(parameters_off.into()))?;

                proto.push('(');
                let params_size = dex_reader.read_u32()?;
                for idx in 0..params_size {
                    let offset = dex_reader.read_u16()?;
                    parameters_off_list.push(offset);

                    proto.push_str(types_list.items.get(offset as usize).ok_or(DexError::InvalidTypeIdx)?);
                    if idx < params_size - 1 {
                        proto.push(' ');
                    }
                }
                proto.push(')');

                // Go back to the previous position
                dex_reader.bytes.seek(SeekFrom::Start(current_pos))?;
            }
            proto.push_str(types_list.items.get(return_type_idx as usize).ok_or(DexError::InvalidTypeIdx)?);

            protos.push(ProtoIdItem {
                shorty_idx,
                return_type_idx,
                parameters_off,
                parameters_off_list,
                proto
            });
        }


        protos.sort_by(DexProtos::sort);

        let mut items = Vec::new();
        for dex_proto in protos.into_iter() {
            items.push(dex_proto.proto);
        }
        items.dedup();

        Ok(DexProtos { items })
    }
}
