use crate::dex_reader::DexReader;
use crate::dex_header::DexHeader;
use crate::dex_strings::DexStrings;
use crate::dex_types::DexTypes;
use crate::dex_protos::DexProtos;
use crate::dex_fields::DexFields;
use crate::dex_methods::DexMethods;
use crate::dex_classes::DexClasses;
use crate::disasm;

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

    pub fn disasm(&self) {
        println!("{:#?}", self.strings);
        println!("-------------------------");
        for class in &self.classes.items {
            class.disasm(&self.strings,
                         &self.types,
                         &self.fields,
                         &self.protos,
                         &self.methods,
                         &self.classes);
            /*
            println!("{}", self.types.items[class.class_idx as usize]);
            if let Some(class_data) = &class.class_data {
                for method in &class_data.direct_methods {
                    println!("  --> {}", &method.proto);
                    if let Some(code) = &method.code_item {
                        let mut offset = 0;
                        if let Some(insns) = &code.insns {
                            for ins in insns {
                                println!("{} {}", offset * 2,
                                         disasm::disasm_ins(ins.as_ref(),
                                                            &self.strings,
                                                            &self.types,
                                                            &self.fields,
                                                            &self.protos,
                                                            &self.methods,
                                                            &self.classes));
                                offset += ins.length();
                            }
                        }
                    }
                    println!("");
                }
            }*/
        }
    }
}
