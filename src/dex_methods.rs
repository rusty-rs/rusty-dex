use std::io::{Seek, SeekFrom};
use std::cmp::Ordering;

use crate::dex_reader::DexReader;
use crate::dex_types::DexTypes;
use crate::dex_protos::DexProtos;
use crate::dex_strings::DexStrings;

#[derive(Debug)]
pub struct MethodIdItem {
    class_idx: u16,
    proto_idx: u16,
    name_idx: u32,
    decoded: String
}

#[derive(Debug)]
pub struct DexMethods {
    pub items: Vec<String>
}

impl DexMethods {
    fn sort(a: &MethodIdItem, b: &MethodIdItem) -> Ordering {
        // First sort by defining type
        let mut order = a.class_idx.cmp(&b.class_idx);

        if order == Ordering::Equal {
            // If that fails, sort by method name
            order = a.name_idx.cmp(&b.name_idx);
        }

        if order == Ordering::Equal {
            // If that fails too, sort by prototype
            order = a.proto_idx.cmp(&b.proto_idx);
        }

        order
    }

    pub fn build(dex_reader: &mut DexReader,
                 offset: u32,
                 size: u32,
                 types_list: &DexTypes,
                 protos_list: &DexProtos,
                 strings_list: &DexStrings) -> Self {
        dex_reader.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        let mut methods = Vec::new();

        for _ in 0..size {
            let class_idx = dex_reader.read_u16().unwrap();
            let proto_idx = dex_reader.read_u16().unwrap();
            let name_idx = dex_reader.read_u32().unwrap();

            let mut decoded = String::new();
            decoded.push_str(types_list.items.get(class_idx as usize).unwrap());
            decoded.push_str("->");
            decoded.push_str(&strings_list.strings.get(name_idx as usize).unwrap().string);
            decoded.push_str(protos_list.items.get(proto_idx as usize).unwrap());

            methods.push(MethodIdItem { 
                class_idx,
                proto_idx,
                name_idx,
                decoded
            });
        }

        methods.sort_by(DexMethods::sort);

        let mut items = Vec::new();
        for dex_method in methods.iter() {
            items.push(dex_method.decoded.clone());
        }
        items.dedup();

        DexMethods { items }
    }
}
