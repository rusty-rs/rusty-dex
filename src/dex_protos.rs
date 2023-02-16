use std::io::{Seek, SeekFrom};

use crate::dex_reader::DexReader;
use crate::dex_strings::DexStrings;
use crate::dex_types::DexTypes;

#[derive(Debug)]
pub struct ProtoIdItem {
    shorty_idx: u32,
    return_type_idx: u32,
    parameters_off: u32,
    pub proto: String,
}

#[derive(Debug)]
pub struct DexProtos {
    pub items: Vec<ProtoIdItem>
}

impl DexProtos {
    pub fn build(dex_reader: &mut DexReader,
                 offset: u32,
                 size: u32,
                 types_list: &DexTypes) -> Self {
        dex_reader.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        let mut protos = Vec::new();

        for _ in 0..size {
            let shorty_idx = dex_reader.read_u32().unwrap();
            let return_type_idx = dex_reader.read_u32().unwrap();
            let parameters_off = dex_reader.read_u32().unwrap();

            // Decode the prototype
            let mut proto = String::new();
            if parameters_off == 0 {
                // No parameters in this prototype
                proto.push_str("()");
            } else {
                // Save current stream position
                let current_pos = dex_reader.bytes.position();

                // Decode the parameters
                dex_reader.bytes.seek(SeekFrom::Start(parameters_off.into())).unwrap();

                proto.push('(');
                let params_size = dex_reader.read_u32().unwrap();
                for idx in 0..params_size {
                    let offset = dex_reader.read_u16().unwrap();
                    proto.push_str(types_list.items.get(offset as usize).unwrap());
                    if idx < params_size - 1 {
                        proto.push(' ');
                    }
                }
                proto.push(')');

                // Go back to the previous position
                dex_reader.bytes.seek(SeekFrom::Start(current_pos)).unwrap();
            }
            proto.push_str(types_list.items.get(return_type_idx as usize).unwrap());

            protos.push(ProtoIdItem {
                shorty_idx,
                return_type_idx,
                parameters_off,
                proto
            });
        }

        DexProtos { items: protos }
    }
}
