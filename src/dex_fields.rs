//! Representation of class fields
//!
//! This module decodes fields from a DEX file and returns them in the correct order. Fields must
//! be ordered by the class they belong to, then their name, and finally their type.
//! Each field can represent a static field initialized in the `<cinit>` pseudo-method or a class
//! field that is initialized when the class is instantiated.

use std::io::{Seek, SeekFrom};
use std::cmp::Ordering;

use crate::dex_reader::DexReader;
use crate::dex_types::DexTypes;
use crate::dex_strings::DexStrings;

/// Internal representation of a field. The index fields are read from the DEX file and are then
/// used to decode the field, which is what we actually return to the user.
#[derive(Debug)]
struct FieldIdItem {
    /// Index into the list of types representing the defining class of the field
    class_idx: u16,
    /// Index into the list of types representing the type of the field
    type_idx: u16,
    /// Index into the list of strings representing the name of the field
    name_idx: u32,
    /// Decoded field name
    decoded: String,
}

/// Representation of the fields in a DEX file. Only the decoded fields are present in the correct
/// order.
#[derive(Debug)]
pub struct DexFields {
    /// Vector of decoded field names
    pub items: Vec<String>
}

impl DexFields {
    /// Function to sort the field items in the order defined in the Dalvik documentation
    fn sort(a: &FieldIdItem, b: &FieldIdItem) -> Ordering {
        // First sort by defining type
        let mut order = a.class_idx.cmp(&b.class_idx);

        if order == Ordering::Equal {
            // If that fails, sort by field name
            order = a.name_idx.cmp(&b.name_idx);
        }

        if order == Ordering::Equal {
            // If that fails too, sort by type
            order = a.type_idx.cmp(&b.type_idx);
        }

        order
    }

    /// Parse the fields from the DEX file. This function returns a vector of decoded field names
    /// in the correct order.
    pub fn build(dex_reader: &mut DexReader,
                 offset: u32,
                 size: u32,
                 types_list: &DexTypes,
                 strings_list: &DexStrings) -> Self {
        dex_reader.bytes.seek(SeekFrom::Start(offset.into())).unwrap();

        let mut fields = Vec::new();

        for _ in 0..size {
            let class_idx = dex_reader.read_u16().unwrap();
            let type_idx = dex_reader.read_u16().unwrap();
            let name_idx = dex_reader.read_u32().unwrap();

            let mut decoded = String::new();
            decoded.push_str(types_list.items.get(class_idx as usize).unwrap());
            decoded.push_str("->");
            decoded.push_str(&strings_list.strings.get(name_idx as usize).unwrap());
            decoded.push(':');
            decoded.push_str(types_list.items.get(type_idx as usize).unwrap());

            fields.push(FieldIdItem {
                class_idx,
                type_idx,
                name_idx,
                decoded
            });
        }

        fields.sort_by(DexFields::sort);

        let mut items = Vec::new();
        for dex_field in fields.into_iter() {
            items.push(dex_field.decoded);
        }
        items.dedup();

        DexFields { items }
    }
}
