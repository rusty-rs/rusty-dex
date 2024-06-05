//! Various constants defined in Dalvik files

#[allow(non_camel_case_types)]

/// Types of items present in the header map of a DEX file
pub struct MapItemType;
impl MapItemType {
    pub const HEADER_ITEM               : u16 = 0x0000;
    pub const STRING_ID_ITEM            : u16 = 0x0001;
    pub const TYPE_ID_ITEM              : u16 = 0x0002;
    pub const PROTO_ID_ITEM             : u16 = 0x0003;
    pub const FIELD_ID_ITEM             : u16 = 0x0004;
    pub const METHOD_ID_ITEM            : u16 = 0x0005;
    pub const CLASS_DEF_ITEM            : u16 = 0x0006;
    pub const CALL_SITE_ID_ITEM         : u16 = 0x0007;
    pub const METHOD_HANDLE_ITEM        : u16 = 0x0008;
    pub const MAP_LIST                  : u16 = 0x1000;
    pub const TYPE_LIST                 : u16 = 0x1001;
    pub const ANNOTATION_SET_REF_LIST   : u16 = 0x1002;
    pub const ANNOTATION_SET_ITEM       : u16 = 0x1003;
    pub const CLASS_DATA_ITEM           : u16 = 0x2000;
    pub const CODE_ITEM                 : u16 = 0x2001;
    pub const STRING_DATA_ITEM          : u16 = 0x2002;
    pub const DEBUG_INFO_ITEM           : u16 = 0x2003;
    pub const ANNOTATION_ITEM           : u16 = 0x2004;
    pub const ENCODED_ARRAY_ITEM        : u16 = 0x2005;
    pub const ANNOTATIONS_DIRECTORY_ITEM: u16 = 0x2006;
    pub const HIDDENAPI_CLASS_DATA_ITEM : u16 = 0xF000;
}

/// Representation of the different method types
pub struct MethodHandleType;
impl MethodHandleType {
    /// Method handle is a static field setter (accessor)
    pub const METHOD_HANDLE_TYPE_STATIC_PUT        : u16 = 0x00;
    /// Method handle is a static field getter (accessor)
    pub const METHOD_HANDLE_TYPE_STATIC_GET        : u16 = 0x01;
    /// Method handle is an instance field setter (accessor)
    pub const METHOD_HANDLE_TYPE_INSTANCE_PUT      : u16 = 0x02;
    /// Method handle is an instance field getter (accessor)
    pub const METHOD_HANDLE_TYPE_INSTANCE_GET      : u16 = 0x03;
    /// Method handle is a static method invoker
    pub const METHOD_HANDLE_TYPE_INVOKE_STATIC     : u16 = 0x04;
    /// Method handle is an instance method invoker
    pub const METHOD_HANDLE_TYPE_INVOKE_INSTANCE   : u16 = 0x05;
    /// Method handle is a constructor method invoker
    pub const METHOD_HANDLE_TYPE_INVOKE_CONSTRUCTOR: u16 = 0x06;
    /// Method handle is a direct method invoker
    pub const METHOD_HANDLE_TYPE_INVOKE_DIRECT     : u16 = 0x07;
    /// Method handle is an interface method invoker
    pub const METHOD_HANDLE_TYPE_INVOKE_INTERFACE  : u16 = 0x08;
}

/// An encoded value is an encoded piece of (nearly) arbitrary
/// hierarchically structured data. The encoding is meant to be
/// both compact and straightforward to parse.
pub struct EncodedValueFormat;
impl EncodedValueFormat {
    pub const VALUE_BYTE         : u8 = 0x00;
    pub const VALUE_SHORT        : u8 = 0x02;
    pub const VALUE_CHAR         : u8 = 0x03;
    pub const VALUE_INT          : u8 = 0x04;
    pub const VALUE_LONG         : u8 = 0x06;
    pub const VALUE_FLOAT        : u8 = 0x10;
    pub const VALUE_DOUBLE       : u8 = 0x11;
    pub const VALUE_METHOD_TYPE  : u8 = 0x15;
    pub const VALUE_METHOD_HANDLE: u8 = 0x16;
    pub const VALUE_STRING       : u8 = 0x17;
    pub const VALUE_TYPE         : u8 = 0x18;
    pub const VALUE_FIELD        : u8 = 0x19;
    pub const VALUE_METHOD       : u8 = 0x1a;
    pub const VALUE_ENUM         : u8 = 0x1b;
    pub const VALUE_ARRAY        : u8 = 0x1c;
    pub const VALUE_ANNOTATION   : u8 = 0x1d;
    pub const VALUE_NULL         : u8 = 0x1e;
    pub const VALUE_BOOLEAN      : u8 = 0x1f;
}
