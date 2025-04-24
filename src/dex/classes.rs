//! Representation of a class and encoded methods

use std::io::{Seek, SeekFrom};
use lazy_static::lazy_static;
use regex::Regex;
use log::{ warn, debug };

use crate::dex::reader::DexReader;
use crate::dex::access_flags::{ AccessFlag, AccessFlagType };
use crate::dex::code_item::CodeItem;

use crate::dex::strings::DexStrings;
use crate::dex::types::DexTypes;
use crate::dex::fields::DexFields;
use crate::dex::methods::DexMethods;
use crate::error::DexError;

/// Constant to represent the absence of index
const NO_INDEX: u32 = 0xffffffff;

lazy_static!{
    /// Regex for method prototypes
    static ref METHOD_REGEX: Regex = Regex::new(r"(?x)
        (?P<class>L[a-zA-Z/$0-9]+;)
        (->)
        (?P<method><?[a-zA-Z0-9]+>?[\$\d+]*)
        (?P<args>\(.*\).*)
    ").unwrap();
}

/// Class definition item
///
/// This struct contains all the metadata of a class. The optional `class_data` item then contains
/// the list of fields and methods (with bytecode) in this class. Note that it is possible that a
/// class contains not fields or methods, in which case `class_data` will be `None`.
#[derive(Debug)]
pub struct ClassDefItem {
    class_str: String,
    access_flags: Vec<AccessFlag>,
    superclass_str: Option<String>,
    interfaces_off: u32,
    source_file_str: Option<String>,
    annotations_off: u32,
    class_data_off: u32,
    static_value_off: u32,
    class_data: Option<ClassDataItem>
}

/// Representation of an encoded field
#[derive(Debug)]
pub struct EncodedField {
    field: String,
    access_flags: Vec<AccessFlag>,
}

/// Representation of an encoded method
#[derive(Debug)]
pub struct EncodedMethod {
    pub proto: String,
    pub access_flags: Vec<AccessFlag>,
    pub code_item: Option<CodeItem>,
}

/// Class data item which contains all fields and methods of a class
#[derive(Debug)]
pub struct ClassDataItem {
    static_fields: Vec<EncodedField>,
    instance_fields: Vec<EncodedField>,
    direct_methods: Vec<EncodedMethod>,
    virtual_methods: Vec<EncodedMethod>,
}

/// List of all classes of a DEX file
#[derive(Debug)]
pub struct DexClasses {
    pub items: Vec<ClassDefItem>
}

impl DexClasses {
    /// Parse the DEX file to extract the classes and their content
    pub fn build(dex_reader: &mut DexReader,
                 offset: u32,
                 size: u32,
                 fields_list: &DexFields,
                 types_list: &DexTypes,
                 strings_list: &DexStrings,
                 methods_list: &DexMethods) -> Result<Self, DexError> {
        dex_reader.bytes.seek(SeekFrom::Start(offset.into()))?;

        let mut methods = Vec::new();

        for _ in 0..size {
            let class_idx = dex_reader.read_u32()?;
            let access_flags = dex_reader.read_u32()?;
            let access_flags_decoded = AccessFlag::parse(access_flags,
                                                         AccessFlagType::Class);

            let superclass_idx   = dex_reader.read_u32()?;
            let interfaces_off   = dex_reader.read_u32()?;
            let source_file_idx  = dex_reader.read_u32()?;
            let annotations_off  = dex_reader.read_u32()?;
            let class_data_off   = dex_reader.read_u32()?;
            let static_value_off = dex_reader.read_u32()?;

            // Convert indexs into human-readable strings
            let class_str = types_list.items
                                      .get(class_idx as usize)
                                      .ok_or(DexError::InvalidTypeIdx)?;

            let mut superclass_str = None;
            if superclass_idx != NO_INDEX {
                superclass_str = Some(types_list.items
                                                .get(superclass_idx as usize)
                                                .ok_or(DexError::InvalidTypeIdx)?);
            }

            let mut source_file_str = None;
            if source_file_idx != NO_INDEX {
                source_file_str = Some(strings_list.strings
                                                   .get(source_file_idx as usize)
                                                   .ok_or(DexError::InvalidStringIdx)?);
            }

            // If class_data_off == 0 then we have no class data
            let mut class_data = None;
            if class_data_off != 0 {
                // Start parse class data

                // Keep track of current stream position
                let current_offset = dex_reader.bytes.position();

                // Go to class data offset
                dex_reader.bytes.seek(SeekFrom::Start(class_data_off.into()))?;

                let (static_fields_size, _)   = dex_reader.read_uleb128()?;
                let (instance_fields_size, _) = dex_reader.read_uleb128()?;
                let (direct_methods_size, _)  = dex_reader.read_uleb128()?;
                let (virtual_methods_size, _) = dex_reader.read_uleb128()?;

                let mut static_fields   = Vec::<EncodedField>::with_capacity(static_fields_size as usize);
                let mut instance_fields = Vec::<EncodedField>::with_capacity(instance_fields_size as usize);
                let mut direct_methods  = Vec::<EncodedMethod>::with_capacity(direct_methods_size as usize);
                let mut virtual_methods = Vec::<EncodedMethod>::with_capacity(virtual_methods_size as usize);

                // Encoded fields
                let mut field_idx = 0;
                for _ in 0..static_fields_size {
                    let (idx, _) = dex_reader.read_uleb128()?;
                    let (access_flags, _) = dex_reader.read_uleb128()?;

                    field_idx += idx;

                    let decoded_field = fields_list.items.get(field_idx as usize)
                                                         .ok_or(DexError::InvalidFieldIdx)?;
                    let decoded_flags = AccessFlag::parse(access_flags,
                                                          AccessFlagType::Field);

                    static_fields.push(EncodedField {
                        field: decoded_field.to_string(),
                        access_flags: decoded_flags
                    });
                }

                let mut field_idx = 0;
                for _ in 0..instance_fields_size {
                    let (idx, _) = dex_reader.read_uleb128()?;
                    let (access_flags, _) = dex_reader.read_uleb128()?;

                    field_idx += idx;

                    let decoded_field = fields_list.items.get(field_idx as usize)
                                                         .ok_or(DexError::InvalidFieldIdx)?;
                    let decoded_flags = AccessFlag::parse(access_flags,
                                                          AccessFlagType::Field);

                    instance_fields.push(EncodedField {
                        field: decoded_field.to_string(),
                        access_flags: decoded_flags
                    });
                }

                // Encoded methods
                let mut method_idx = 0;
                for _ in 0..direct_methods_size {
                    let (idx, _) = dex_reader.read_uleb128()?;
                    let (access_flags, _) = dex_reader.read_uleb128()?;
                    let (code_offset, _) = dex_reader.read_uleb128()?;

                    method_idx += idx;

                    let proto = methods_list.items.get(method_idx as usize)
                                                         .ok_or(DexError::InvalidMethodIdx)?;
                    let decoded_flags = AccessFlag::parse(access_flags,
                                                          AccessFlagType::Method);

                    if code_offset == 0 {
                        // Abstract or native methods have no code
                        direct_methods.push(EncodedMethod {
                            proto: proto.to_string(),
                            access_flags: decoded_flags,
                            code_item: None
                        });
                    } else {
                        let current_offset = dex_reader.bytes.position();
                        let code_item = CodeItem::build(dex_reader,
                                                        code_offset,
                                                        types_list)?;
                        dex_reader.bytes.seek(SeekFrom::Start(current_offset))?;

                        direct_methods.push(EncodedMethod {
                            proto: proto.to_string(),
                            access_flags: decoded_flags,
                            code_item: Some(code_item),
                        });
                    }
                }

                let mut method_idx = 0;
                for _ in 0..virtual_methods_size {
                    let (idx, _) = dex_reader.read_uleb128()?;
                    let (access_flags, _) = dex_reader.read_uleb128()?;
                    let (code_offset, _) = dex_reader.read_uleb128()?;

                    method_idx += idx;

                    let proto = methods_list.items.get(method_idx as usize)
                                                  .ok_or(DexError::InvalidMethodIdx)?;
                    let decoded_flags = AccessFlag::parse(access_flags,
                                                          AccessFlagType::Method);

                    if code_offset == 0 {
                        // Abstract or native methods have no code
                        virtual_methods.push(EncodedMethod {
                            proto: proto.to_string(),
                            access_flags: decoded_flags,
                            code_item: None
                        });
                    } else {
                        let current_offset = dex_reader.bytes.position();
                        let code_item = CodeItem::build(dex_reader,
                                                        code_offset,
                                                        types_list)?;
                        dex_reader.bytes.seek(SeekFrom::Start(current_offset))?;

                        virtual_methods.push(EncodedMethod {
                            proto: proto.to_string(),
                            access_flags: decoded_flags,
                            code_item: Some(code_item),
                        });
                    }
                }

                // Go back to the previous offset
                dex_reader.bytes.seek(SeekFrom::Start(current_offset))?;

                class_data = Some(ClassDataItem {
                    static_fields,
                    instance_fields,
                    direct_methods,
                    virtual_methods,
                });
            }

            methods.push(ClassDefItem {
                class_str: class_str.to_string(),
                access_flags: access_flags_decoded,
                superclass_str: superclass_str.cloned(),
                interfaces_off,
                source_file_str: source_file_str.cloned(),
                annotations_off,
                class_data_off,
                static_value_off,
                class_data
            });
        }

        Ok(DexClasses { items: methods })
    }

    /// Get a class definition from the class name, if it exists
    pub fn get_class_def(&self, class_name: &String) -> Option<&ClassDefItem> {
        self.items.iter().find(|&item| &item.class_str == class_name)
    }
}

impl ClassDefItem {
    /// Get the name from a class definition
    pub fn get_class_name(&self) -> &String {
        &self.class_str
    }

    /// Get the access flags of a class definition
    pub fn get_access_flags(&self) -> String {
        AccessFlag::vec_to_string(&self.access_flags)
    }

    /// Get the methods of a class definition
    pub fn get_methods(&self) -> Vec<&EncodedMethod> {
        let mut methods = Vec::new();

        if let Some(class_data) = &self.class_data {
            methods.extend(&class_data.direct_methods);
            methods.extend(&class_data.virtual_methods);
        }

        methods
    }

    /// Get a method from a class definition using the method name
    pub fn get_encoded_method(&self, method_name: &String) -> Option<&EncodedMethod> {
        if let Some(class_data) = &self.class_data {
            for method in &class_data.direct_methods {
                if method.get_method_name() == method_name {
                    return Some(method);
                }
            }
            for method in &class_data.virtual_methods {
                if method.get_method_name() == method_name {
                    return Some(method);
                }
            }
        }
        None
    }
}

impl EncodedMethod {
    /// Get the prototype of a method
    pub fn get_proto(&self) -> &str {
        &self.proto
    }

    /// Get the name of a method
    pub fn get_method_name(&self) -> &str {
        let matches = METHOD_REGEX.captures(&self.proto);
        let method_name = match matches {
            Some(matched) => {
                match matched.name("method") {
                    Some(name) => name.as_str(),
                    None => ""
                }
            },
            None => ""
        };

        if method_name.is_empty() {
            warn!("Cannot retrieve method name from prototype");
            debug!("Prototype: {}", &self.proto);
        };

        method_name
    }

    /// Get the access flags of a method
    pub fn get_access_flags(&self) -> String {
        AccessFlag::vec_to_string(&self.access_flags)
    }
}
