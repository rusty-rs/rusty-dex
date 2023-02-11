use crate::dex_header::DexHeader;
use crate::map_list::MapList;
use crate::strings::DexStrings;
use crate::type_id::DexTypes;
use crate::proto_id::DexProtos;
use crate::field_id::DexFields;
use crate::method_id::DexMethods;
use crate::class_def::DexClasses;

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
