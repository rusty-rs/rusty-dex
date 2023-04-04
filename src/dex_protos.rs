use std::io::{Seek, SeekFrom};
use std::cmp::Ordering;

use crate::dex_reader::DexReader;
use crate::dex_types::DexTypes;

#[derive(Debug)]
struct ProtoIdItem {
    shorty_idx: u32,
    return_type_idx: u32,
    parameters_off: u32,
    parameters_off_list: Vec<u16>,  // used to sort items
    proto: String,
}

#[derive(Debug)]
pub struct DexProtos {
    pub items: Vec<String>
}

impl DexProtos {
    fn sort(a: &ProtoIdItem, b: &ProtoIdItem) -> Ordering {
        // First sort by return type
        let sort_return = a.return_type_idx.cmp(&b.return_type_idx);

        if sort_return == Ordering::Equal {
            // Same return type, sort by params offsets
            return a.parameters_off_list.cmp(&b.parameters_off_list);
        }

        sort_return
    }

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
            let mut parameters_off_list = Vec::new();
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
                    parameters_off_list.push(offset);

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
                parameters_off_list,
                proto
            });
        }


        protos.sort_by(DexProtos::sort);

        let mut items = Vec::new();
        for dex_proto in protos.iter() {
            items.push(dex_proto.proto.clone());
        }
        items.dedup();

        DexProtos { items }
    }
}
