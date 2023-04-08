use crate::info;

use crate::dex_reader::DexReader;
use crate::dex_header::DexHeader;
use crate::dex_strings::DexStrings;
use crate::dex_types::DexTypes;
use crate::dex_protos::DexProtos;
use crate::dex_fields::DexFields;
use crate::dex_methods::DexMethods;
use crate::dex_classes::DexClasses;

#[derive(Debug)]
pub struct DexFile {
    pub header: DexHeader,
    pub strings: DexStrings,
    pub types: DexTypes,
    pub protos: DexProtos,
    pub fields: DexFields,
    pub methods: DexMethods,
    pub classes: DexClasses,
}

impl DexFile {
    pub fn build(mut dex_reader: DexReader) -> Self {
        let dex_header = DexHeader::new(&mut dex_reader).unwrap();

        let strings_list = DexStrings::build(&mut dex_reader,
                                             dex_header.string_ids_off,
                                             dex_header.string_ids_size);

        let type_ids_list = DexTypes::build(&mut dex_reader,
                                            dex_header.type_ids_off,
                                            dex_header.type_ids_size,
                                            &strings_list);

        let proto_ids_list = DexProtos::build(&mut dex_reader,
                                              dex_header.proto_ids_off,
                                              dex_header.proto_ids_size,
                                              &type_ids_list);

        let field_ids_list = DexFields::build(&mut dex_reader,
                                              dex_header.fields_ids_off,
                                              dex_header.fields_ids_size,
                                              &type_ids_list,
                                              &strings_list);

        let method_ids_list = DexMethods::build(&mut dex_reader,
                                                dex_header.method_ids_off,
                                                dex_header.method_ids_size,
                                                &type_ids_list,
                                                &proto_ids_list,
                                                &strings_list);

        let class_defs_list = DexClasses::build(&mut dex_reader,
                                                dex_header.class_defs_off,
                                                dex_header.class_defs_size,
                                                &field_ids_list,
                                                &type_ids_list,
                                                &strings_list,
                                                &method_ids_list);

        DexFile {
            header: dex_header,
            strings: strings_list,
            types: type_ids_list,
            protos: proto_ids_list,
            fields: field_ids_list,
            methods: method_ids_list,
            classes: class_defs_list,
        }
    }

    pub fn merge(readers: Vec<DexReader>) -> Self {
        let mut strings_list = Vec::new();
        let mut type_ids_list = Vec::new();
        let mut proto_ids_list = Vec::new();
        let mut field_ids_list = Vec::new();
        let mut method_ids_list = Vec::new();
        let mut class_defs_list = Vec::new();

        info!("start merging");
        for reader in readers.into_iter() {
            let current_dex_file = DexFile::build(reader);

            info!("  [+] merging strings");
            for string in current_dex_file.strings.strings.into_iter() {
                strings_list.push(string);
            }

            info!("  [+] merging types");
            for type_id in current_dex_file.types.items.into_iter() {
                type_ids_list.push(type_id);
            }

            info!("  [+] merging protos");
            for proto_id in current_dex_file.protos.items.into_iter() {
                proto_ids_list.push(proto_id);
            }

            info!("  [+] merging fields");
            for field_id in current_dex_file.fields.items.into_iter() {
                field_ids_list.push(field_id);
            }

            info!("  [+] merging methods");
            for method_id in current_dex_file.methods.items.into_iter() {
                method_ids_list.push(method_id);
            }

            info!("  [+] merging classes");
            for class_def_id in current_dex_file.classes.items.into_iter() {
                class_defs_list.push(class_def_id);
            }
        }

        info!("removing strings duplicates and storing strings");
        strings_list.dedup();
        strings_list.sort();
        info!("removing strings duplicates and storing types");
        type_ids_list.dedup();
        type_ids_list.sort();
        info!("removing strings duplicates and storing protos");
        proto_ids_list.dedup();
        proto_ids_list.sort();
        info!("removing strings duplicates and storing fields");
        field_ids_list.dedup();
        field_ids_list.sort();
        info!("removing strings duplicates and storing methods");
        method_ids_list.dedup();
        method_ids_list.sort();
        // class_defs_list.dedup();
        // class_defs_list.sort();

        info!("done");

        let header = DexHeader {
            version: [0x00; 3],
            checksum: 0x0,
            signature: [0x00; 20],
            file_size: 0x00,
            header_size: 0x00,
            endian_tag: 0x00,
            link_size: 0x00,
            link_off: 0x00,
            map_off: 0x00,
            string_ids_size: strings_list.len() as u32,
            string_ids_off: 0x00,
            type_ids_size: type_ids_list.len() as u32,
            type_ids_off: 0x00,
            proto_ids_size: proto_ids_list.len() as u32,
            proto_ids_off: 0x00,
            fields_ids_size: field_ids_list.len() as u32,
            fields_ids_off: 0x00,
            method_ids_size: method_ids_list.len() as u32,
            method_ids_off: 0x00,
            class_defs_size: class_defs_list.len() as u32,
            class_defs_off: 0x00,
            data_size: 0x00,
            data_off: 0x00
        };

        DexFile {
            header: header,
            strings: DexStrings { strings: strings_list },
            types: DexTypes { items: type_ids_list },
            protos: DexProtos { items: proto_ids_list },
            fields: DexFields { items: field_ids_list },
            methods: DexMethods { items: method_ids_list },
            classes: DexClasses { items: class_defs_list },
        }
    }

    pub fn disasm(&self) {
        for class in &self.classes.items {
            class.disasm(&self.strings,
                         &self.types,
                         &self.fields,
                         &self.methods);
        }
    }
}
