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

    pub fn disasm(&self, class_names: Option<Vec<String>>) {
        let allowlist = match class_names {
            Some(names) => names,
            None => Vec::new()
        };

        for class in &self.classes.items {
            // FIXME no comment
            if allowlist.len() == 0 ||
                    allowlist.contains(class.get_class_name()) {
                class.disasm(&self.strings,
                            &self.types,
                            &self.fields,
                            &self.methods);
            }
        }
    }

    pub fn get_classes(&self, prefix: Option<String>) {
        let mut class_names = Vec::new();

        let regex = match prefix {
            Some(prefix) => prefix.clone(),
            None => String::new(),
        };

        for class in &self.classes.items {
            if ! class.get_class_name().starts_with(&regex) {
                continue;
            }

            class_names.push((class.get_class_name(),
                              class.get_access_flags()));
        }

        class_names.sort();

        for (class_name, access_flags) in class_names.iter() {
            if access_flags.is_empty() {
                println!("{}", class_name);
            } else {
                println!("{} ({})", class_name, access_flags);
            }
        }
    }

    pub fn get_methods(&self,
                       c_prefix: Option<String>,
                       m_prefix: Option<String>) {
        let mut method_names = Vec::new();

        let class_prefix = match c_prefix {
            Some(name) => name,
            None => String::new()
        };

        let mut method_prefix = String::from("->");
        match m_prefix {
            Some(prefix) => method_prefix.push_str(&prefix), // .clone()),
            None => ()
        };

        for class in &self.classes.items {
            if ! class.get_class_name().starts_with(&class_prefix) {
                continue;
            }

            for method in class.get_methods() {
                let method_proto = method.get_proto();

                if ! method_proto.contains(&method_prefix) {
                    continue;
                }

                method_names.push((method_proto,
                                   method.get_access_flags()));
            }
        }

        method_names.sort();

        for (method_name, access_flags) in method_names.iter() {
            if access_flags.is_empty() {
                println!("{}", method_name);
            } else {
                println!("{} ({})", method_name, access_flags);
            }
        }
    }
}
