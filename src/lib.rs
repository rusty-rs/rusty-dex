#![allow(dead_code)]

use crate::dex::reader::DexReader;
use crate::dex::file::DexFile;
use crate::dex::instructions::Instructions;

// pub mod logging;
pub mod dex;
pub mod error;
pub mod adler32;
pub mod mutf8;

/* Actually unused for now but there should
 * be more options as things progress */
pub struct Config {
    pub log_level: u8,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            log_level: 0    // only show error messages
        }
    }
}

pub fn parse(filepath: &str) -> DexFile {
    let readers = DexReader::build_from_file(filepath);
    DexFile::merge(readers)
}

pub fn get_qualified_method_names(dex: &DexFile) -> Vec<String> {
    let mut methods = Vec::new();

    let class_names = dex.get_classes_names();
    for class in class_names.iter() {
        if let Some(class_def) = dex.classes.get_class_def(class) {
            for method in class_def.get_methods() {
                let name = method.get_method_name();
                methods.push(format!("{class}->{name}"));
            }
        }
    }

    methods
}

pub fn get_bytecode_for_method(dex: &DexFile,
                               class_name: &String,
                               method_name: &String) -> Option<Vec<Instructions>> {
    if let Some(class_def) = dex.get_class_def(class_name) {
        if let Some(encoded_method) = class_def.get_encoded_method(method_name) {
            if let Some(code_item) = &encoded_method.code_item {
                return code_item.insns.clone();
            }
        }
    }

    None
}
