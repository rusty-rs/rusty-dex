use crate::warning;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum OpCode {
    NOP,
    MOVE,
    MOVE_FROM16,
    MOVE_16,
    MOVE_WIDE,
    MOVE_WIDE_FROM16,
    MOVE_WIDE_16,
    MOVE_OBJECT,
    MOVE_OBJECT_FROM16,
    MOVE_OBJECT_16,
    MOVE_RESULT,
    MOVE_RESULT_WIDE,
    MOVE_RESULT_OBJECT,
    MOVE_EXCEPTION,
    RETURN_VOID,
    RETURN,
    RETURN_WIDE,
    RETURN_OBJECT,
    CONST_4,
    CONST_16,
    CONST,
    CONST_HIGH16,
    CONST_WIDE_16,
    CONST_WIDE_32,
    CONST_WIDE,
    CONST_WIDE_HIGH16,
    CONST_STRING,
    CONST_STRING_JUMBO,
    CONST_CLASS,
    MONITOR_ENTER,
    MONITOR_EXIT,
    CHECK_CAST,
    INSTANCE_OF,
    ARRAY_LENGTH,
    NEW_INSTANCE,
    NEW_ARRAY,
    FILLED_NEW_ARRAY,
    FILLED_NEW_ARRAY_RANGE,
    FILL_ARRAY_DATA,
    THROW,
    GOTO,
    GOTO_16,
    GOTO_32,
    PACKED_SWITCH,
    SPARSE_SWITCH,
    CMPL_FLOAT,
    CMPG_FLOAT,
    CMPL_DOUBLE,
    CMPG_DOUBLE,
    CMP_LONG,
    IF_EQ,
    IF_NE,
    IF_LT,
    IF_GE,
    IF_GT,
    IF_LE,
    IF_EQZ,
    IF_NEZ,
    IF_LTZ,
    IF_GEZ,
    IF_GTZ,
    IF_LEZ,
    AGET,
    AGET_WIDE,
    AGET_OBJECT,
    AGET_BOOLEAN,
    AGET_BYTE,
    AGET_CHAR,
    AGET_SHORT,
    APUT,
    APUT_WIDE,
    APUT_OBJECT,
    APUT_BOOLEAN,
    APUT_BYTE,
    APUT_CHAR,
    APUT_SHORT,
    IGET,
    IGET_WIDE,
    IGET_OBJECT,
    IGET_BOOLEAN,
    IGET_BYTE,
    IGET_CHAR,
    IGET_SHORT,
    IPUT,
    IPUT_WIDE,
    IPUT_OBJECT,
    IPUT_BOOLEAN,
    IPUT_BYTE,
    IPUT_CHAR,
    IPUT_SHORT,
    SGET,
    SGET_WIDE,
    SGET_OBJECT,
    SGET_BOOLEAN,
    SGET_BYTE,
    SGET_CHAR,
    SGET_SHORT,
    SPUT,
    SPUT_WIDE,
    SPUT_OBJECT,
    SPUT_BOOLEAN,
    SPUT_BYTE,
    SPUT_CHAR,
    SPUT_SHORT,
    INVOKE_VIRTUAL,
    INVOKE_SUPER,
    INVOKE_DIRECT,
    INVOKE_STATIC,
    INVOKE_INTERFACE,
    INVOKE_VIRTUAL_RANGE,
    INVOKE_SUPER_RANGE,
    INVOKE_DIRECT_RANGE,
    INVOKE_STATIC_RANGE,
    INVOKE_INTERFACE_RANGE,
    NEG_INT,
    NOT_INT,
    NEG_LONG,
    NOT_LONG,
    NEG_FLOAT,
    NEG_DOUBLE,
    INT_TO_LONG,
    INT_TO_FLOAT,
    INT_TO_DOUBLE,
    LONG_TO_INT,
    LONG_TO_FLOAT,
    LONG_TO_DOUBLE,
    FLOAT_TO_INT,
    FLOAT_TO_LONG,
    FLOAT_TO_DOUBLE,
    DOUBLE_TO_INT,
    DOUBLE_TO_LONG,
    DOUBLE_TO_FLOAT,
    INT_TO_BYTE,
    INT_TO_CHAR,
    INT_TO_SHORT,
    ADD_INT,
    SUB_INT,
    MUL_INT,
    DIV_INT,
    REM_INT,
    AND_INT,
    OR_INT,
    XOR_INT,
    SHL_INT,
    SHR_INT,
    USHR_INT,
    ADD_LONG,
    SUB_LONG,
    MUL_LONG,
    DIV_LONG,
    REM_LONG,
    AND_LONG,
    OR_LONG,
    XOR_LONG,
    SHL_LONG,
    SHR_LONG,
    USHR_LONG,
    ADD_FLOAT,
    SUB_FLOAT,
    MUL_FLOAT,
    DIV_FLOAT,
    REM_FLOAT,
    ADD_DOUBLE,
    SUB_DOUBLE,
    MUL_DOUBLE,
    DIV_DOUBLE,
    REM_DOUBLE,
    ADD_INT_2ADDR,
    SUB_INT_2ADDR,
    MUL_INT_2ADDR,
    DIV_INT_2ADDR,
    REM_INT_2ADDR,
    AND_INT_2ADDR,
    OR_INT_2ADDR,
    XOR_INT_2ADDR,
    SHL_INT_2ADDR,
    SHR_INT_2ADDR,
    USHR_INT_2ADDR,
    ADD_LONG_2ADDR,
    SUB_LONG_2ADDR,
    MUL_LONG_2ADDR,
    DIV_LONG_2ADDR,
    REM_LONG_2ADDR,
    AND_LONG_2ADDR,
    OR_LONG_2ADDR,
    XOR_LONG_2ADDR,
    SHL_LONG_2ADDR,
    SHR_LONG_2ADDR,
    USHR_LONG_2ADDR,
    ADD_FLOAT_2ADDR,
    SUB_FLOAT_2ADDR,
    MUL_FLOAT_2ADDR,
    DIV_FLOAT_2ADDR,
    REM_FLOAT_2ADDR,
    ADD_DOUBLE_2ADDR,
    SUB_DOUBLE_2ADDR,
    MUL_DOUBLE_2ADDR,
    DIV_DOUBLE_2ADDR,
    REM_DOUBLE_2ADDR,
    ADD_INT_LIT16,
    RSUB_INT,
    MUL_INT_LIT16,
    DIV_INT_LIT16,
    REM_INT_LIT16,
    AND_INT_LIT16,
    OR_INT_LIT16,
    XOR_INT_LIT16,
    ADD_INT_LIT8,
    RSUB_INT_LIT8,
    MUL_INT_LIT8,
    DIV_INT_LIT8,
    REM_INT_LIT8,
    AND_INT_LIT8,
    OR_INT_LIT8,
    XOR_INT_LIT8,
    SHL_INT_LIT8,
    SHR_INT_LIT8,
    USHR_INT_LIT8,
    INVOKE_POLYMORPHIC,
    INVOKE_POLYMORPHIC_RANGE,
    INVOKE_CUSTOM,
    INVOKE_CUSTOM_RANGE,
    CONST_METHOD_HANDLE,
    CONST_METHOD_TYPE,
}


impl OpCode {
    pub fn parse(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(OpCode::NOP),                        // Instruction 10x
            0x01 => Some(OpCode::MOVE),                       // Instruction 12x
            0x02 => Some(OpCode::MOVE_FROM16),                // Instruction 22x
            0x03 => Some(OpCode::MOVE_16),                    // Instruction 32x
            0x04 => Some(OpCode::MOVE_WIDE),                  // Instruction 12x
            0x05 => Some(OpCode::MOVE_WIDE_FROM16),           // Instruction 22x
            0x06 => Some(OpCode::MOVE_WIDE_16),               // Instruction 32x
            0x07 => Some(OpCode::MOVE_OBJECT),                // Instruction 12x
            0x08 => Some(OpCode::MOVE_OBJECT_FROM16),         // Instruction 22x
            0x09 => Some(OpCode::MOVE_OBJECT_16),             // Instruction 32x
            0x0a => Some(OpCode::MOVE_RESULT),                // Instruction 11x
            0x0b => Some(OpCode::MOVE_RESULT_WIDE),           // Instruction 11x
            0x0c => Some(OpCode::MOVE_RESULT_OBJECT),         // Instruction 11x
            0x0d => Some(OpCode::MOVE_EXCEPTION),             // Instruction 11x
            0x0e => Some(OpCode::RETURN_VOID),                // Instruction 10x
            0x0f => Some(OpCode::RETURN),                     // Instruction 11x
            0x10 => Some(OpCode::RETURN_WIDE),                // Instruction 11x
            0x11 => Some(OpCode::RETURN_OBJECT),              // Instruction 11x
            0x12 => Some(OpCode::CONST_4),                    // Instruction 11n
            0x13 => Some(OpCode::CONST_16),                   // Instruction 21s
            0x14 => Some(OpCode::CONST),                      // Instruction 31i
            0x15 => Some(OpCode::CONST_HIGH16),               // Instruction 21h
            0x16 => Some(OpCode::CONST_WIDE_16),              // Instruction 21s
            0x17 => Some(OpCode::CONST_WIDE_32),              // Instruction 31i
            0x18 => Some(OpCode::CONST_WIDE),                 // Instruction 51l
            0x19 => Some(OpCode::CONST_WIDE_HIGH16),          // Instruction 21h
            0x1a => Some(OpCode::CONST_STRING),               // Instruction 21c
            0x1b => Some(OpCode::CONST_STRING_JUMBO),         // Instruction 31c
            0x1c => Some(OpCode::CONST_CLASS),                // Instruction 21c
            0x1d => Some(OpCode::MONITOR_ENTER),              // Instruction 11x
            0x1e => Some(OpCode::MONITOR_EXIT),               // Instruction 11x
            0x1f => Some(OpCode::CHECK_CAST),                 // Instruction 21c
            0x20 => Some(OpCode::INSTANCE_OF),                // Instruction 22c
            0x21 => Some(OpCode::ARRAY_LENGTH),               // Instruction 12x
            0x22 => Some(OpCode::NEW_INSTANCE),               // Instruction 21c
            0x23 => Some(OpCode::NEW_ARRAY),                  // Instruction 22c
            0x24 => Some(OpCode::FILLED_NEW_ARRAY),           // Instruction 35c
            0x25 => Some(OpCode::FILLED_NEW_ARRAY_RANGE),     // Instruction 3rc
            0x26 => Some(OpCode::FILL_ARRAY_DATA),            // Instruction 31t
            0x27 => Some(OpCode::THROW),                      // Instruction 11x
            0x28 => Some(OpCode::GOTO),                       // Instruction 10t
            0x29 => Some(OpCode::GOTO_16),                    // Instruction 20t
            0x2a => Some(OpCode::GOTO_32),                    // Instruction 30t
            0x2b => Some(OpCode::PACKED_SWITCH),              // Instruction 31t
            0x2c => Some(OpCode::SPARSE_SWITCH),              // Instruction 31t
            0x2d => Some(OpCode::CMPL_FLOAT),                 // Instruction 23x
            0x2e => Some(OpCode::CMPG_FLOAT),                 // Instruction 23x
            0x2f => Some(OpCode::CMPL_DOUBLE),                // Instruction 23x
            0x30 => Some(OpCode::CMPG_DOUBLE),                // Instruction 23x
            0x31 => Some(OpCode::CMP_LONG),                   // Instruction 23x
            0x32 => Some(OpCode::IF_EQ),                      // Instruction 22t
            0x33 => Some(OpCode::IF_NE),                      // Instruction 22t
            0x34 => Some(OpCode::IF_LT),                      // Instruction 22t
            0x35 => Some(OpCode::IF_GE),                      // Instruction 22t
            0x36 => Some(OpCode::IF_GT),                      // Instruction 22t
            0x37 => Some(OpCode::IF_LE),                      // Instruction 22t
            0x38 => Some(OpCode::IF_EQZ),                     // Instruction 21t
            0x39 => Some(OpCode::IF_NEZ),                     // Instruction 21t
            0x3a => Some(OpCode::IF_LTZ),                     // Instruction 21t
            0x3b => Some(OpCode::IF_GEZ),                     // Instruction 21t
            0x3c => Some(OpCode::IF_GTZ),                     // Instruction 21t
            0x3d => Some(OpCode::IF_LEZ),                     // Instruction 21t

            /* Unused */
            0x3e..=0x43 => {
                warning!("use of unused opcode {}", value);
                None
            },

            0x44 => Some(OpCode::AGET),                       // Instruction 23x
            0x45 => Some(OpCode::AGET_WIDE),                  // Instruction 23x
            0x46 => Some(OpCode::AGET_OBJECT),                // Instruction 23x
            0x47 => Some(OpCode::AGET_BOOLEAN),               // Instruction 23x
            0x48 => Some(OpCode::AGET_BYTE),                  // Instruction 23x
            0x49 => Some(OpCode::AGET_CHAR),                  // Instruction 23x
            0x4a => Some(OpCode::AGET_SHORT),                 // Instruction 23x
            0x4b => Some(OpCode::APUT),                       // Instruction 23x
            0x4c => Some(OpCode::APUT_WIDE),                  // Instruction 23x
            0x4d => Some(OpCode::APUT_OBJECT),                // Instruction 23x
            0x4e => Some(OpCode::APUT_BOOLEAN),               // Instruction 23x
            0x4f => Some(OpCode::APUT_BYTE),                  // Instruction 23x
            0x50 => Some(OpCode::APUT_CHAR),                  // Instruction 23x
            0x51 => Some(OpCode::APUT_SHORT),                 // Instruction 23x
            0x52 => Some(OpCode::IGET),                       // Instruction 22c
            0x53 => Some(OpCode::IGET_WIDE),                  // Instruction 22c
            0x54 => Some(OpCode::IGET_OBJECT),                // Instruction 22c
            0x55 => Some(OpCode::IGET_BOOLEAN),               // Instruction 22c
            0x56 => Some(OpCode::IGET_BYTE),                  // Instruction 22c
            0x57 => Some(OpCode::IGET_CHAR),                  // Instruction 22c
            0x58 => Some(OpCode::IGET_SHORT),                 // Instruction 22c
            0x59 => Some(OpCode::IPUT),                       // Instruction 22c
            0x5a => Some(OpCode::IPUT_WIDE),                  // Instruction 22c
            0x5b => Some(OpCode::IPUT_OBJECT),                // Instruction 22c
            0x5c => Some(OpCode::IPUT_BOOLEAN),               // Instruction 22c
            0x5d => Some(OpCode::IPUT_BYTE),                  // Instruction 22c
            0x5e => Some(OpCode::IPUT_CHAR),                  // Instruction 22c
            0x5f => Some(OpCode::IPUT_SHORT),                 // Instruction 22c
            0x60 => Some(OpCode::SGET),                       // Instruction 21c
            0x61 => Some(OpCode::SGET_WIDE),                  // Instruction 21c
            0x62 => Some(OpCode::SGET_OBJECT),                // Instruction 21c
            0x63 => Some(OpCode::SGET_BOOLEAN),               // Instruction 21c
            0x64 => Some(OpCode::SGET_BYTE),                  // Instruction 21c
            0x65 => Some(OpCode::SGET_CHAR),                  // Instruction 21c
            0x66 => Some(OpCode::SGET_SHORT),                 // Instruction 21c
            0x67 => Some(OpCode::SPUT),                       // Instruction 21c
            0x68 => Some(OpCode::SPUT_WIDE),                  // Instruction 21c
            0x69 => Some(OpCode::SPUT_OBJECT),                // Instruction 21c
            0x6a => Some(OpCode::SPUT_BOOLEAN),               // Instruction 21c
            0x6b => Some(OpCode::SPUT_BYTE),                  // Instruction 21c
            0x6c => Some(OpCode::SPUT_CHAR),                  // Instruction 21c
            0x6d => Some(OpCode::SPUT_SHORT),                 // Instruction 21c
            0x6e => Some(OpCode::INVOKE_VIRTUAL),             // Instruction 35c
            0x6f => Some(OpCode::INVOKE_SUPER),               // Instruction 35c
            0x70 => Some(OpCode::INVOKE_DIRECT),              // Instruction 35c
            0x71 => Some(OpCode::INVOKE_STATIC),              // Instruction 35c
            0x72 => Some(OpCode::INVOKE_INTERFACE),           // Instruction 35c

            /* Unused */
            0x73 => {
                warning!("use of unused opcode {}", value);
                None
            },

            0x74 => Some(OpCode::INVOKE_VIRTUAL_RANGE),       // Instruction 3rc
            0x75 => Some(OpCode::INVOKE_SUPER_RANGE),         // Instruction 3rc
            0x76 => Some(OpCode::INVOKE_DIRECT_RANGE),        // Instruction 3rc
            0x77 => Some(OpCode::INVOKE_STATIC_RANGE),        // Instruction 3rc
            0x78 => Some(OpCode::INVOKE_INTERFACE_RANGE),     // Instruction 3rc

            /* Unused */
            0x79 | 0x7a => {
                warning!("use of unused opcode {}", value);
                None
            },

            0x7b => Some(OpCode::NEG_INT),                    // Instruction 12x
            0x7c => Some(OpCode::NOT_INT),                    // Instruction 12x
            0x7d => Some(OpCode::NEG_LONG),                   // Instruction 12x
            0x7e => Some(OpCode::NOT_LONG),                   // Instruction 12x
            0x7f => Some(OpCode::NEG_FLOAT),                  // Instruction 12x
            0x80 => Some(OpCode::NEG_DOUBLE),                 // Instruction 12x
            0x81 => Some(OpCode::INT_TO_LONG),                // Instruction 12x
            0x82 => Some(OpCode::INT_TO_FLOAT),               // Instruction 12x
            0x83 => Some(OpCode::INT_TO_DOUBLE),              // Instruction 12x
            0x84 => Some(OpCode::LONG_TO_INT),                // Instruction 12x
            0x85 => Some(OpCode::LONG_TO_FLOAT),              // Instruction 12x
            0x86 => Some(OpCode::LONG_TO_DOUBLE),             // Instruction 12x
            0x87 => Some(OpCode::FLOAT_TO_INT),               // Instruction 12x
            0x88 => Some(OpCode::FLOAT_TO_LONG),              // Instruction 12x
            0x89 => Some(OpCode::FLOAT_TO_DOUBLE),            // Instruction 12x
            0x8a => Some(OpCode::DOUBLE_TO_INT),              // Instruction 12x
            0x8b => Some(OpCode::DOUBLE_TO_LONG),             // Instruction 12x
            0x8c => Some(OpCode::DOUBLE_TO_FLOAT),            // Instruction 12x
            0x8d => Some(OpCode::INT_TO_BYTE),                // Instruction 12x
            0x8e => Some(OpCode::INT_TO_CHAR),                // Instruction 12x
            0x8f => Some(OpCode::INT_TO_SHORT),               // Instruction 12x
            0x90 => Some(OpCode::ADD_INT),                    // Instruction 23x
            0x91 => Some(OpCode::SUB_INT),                    // Instruction 23x
            0x92 => Some(OpCode::MUL_INT),                    // Instruction 23x
            0x93 => Some(OpCode::DIV_INT),                    // Instruction 23x
            0x94 => Some(OpCode::REM_INT),                    // Instruction 23x
            0x95 => Some(OpCode::AND_INT),                    // Instruction 23x
            0x96 => Some(OpCode::OR_INT),                     // Instruction 23x
            0x97 => Some(OpCode::XOR_INT),                    // Instruction 23x
            0x98 => Some(OpCode::SHL_INT),                    // Instruction 23x
            0x99 => Some(OpCode::SHR_INT),                    // Instruction 23x
            0x9a => Some(OpCode::USHR_INT),                   // Instruction 23x
            0x9b => Some(OpCode::ADD_LONG),                   // Instruction 23x
            0x9c => Some(OpCode::SUB_LONG),                   // Instruction 23x
            0x9d => Some(OpCode::MUL_LONG),                   // Instruction 23x
            0x9e => Some(OpCode::DIV_LONG),                   // Instruction 23x
            0x9f => Some(OpCode::REM_LONG),                   // Instruction 23x
            0xa0 => Some(OpCode::AND_LONG),                   // Instruction 23x
            0xa1 => Some(OpCode::OR_LONG),                    // Instruction 23x
            0xa2 => Some(OpCode::XOR_LONG),                   // Instruction 23x
            0xa3 => Some(OpCode::SHL_LONG),                   // Instruction 23x
            0xa4 => Some(OpCode::SHR_LONG),                   // Instruction 23x
            0xa5 => Some(OpCode::USHR_LONG),                  // Instruction 23x
            0xa6 => Some(OpCode::ADD_FLOAT),                  // Instruction 23x
            0xa7 => Some(OpCode::SUB_FLOAT),                  // Instruction 23x
            0xa8 => Some(OpCode::MUL_FLOAT),                  // Instruction 23x
            0xa9 => Some(OpCode::DIV_FLOAT),                  // Instruction 23x
            0xaa => Some(OpCode::REM_FLOAT),                  // Instruction 23x
            0xab => Some(OpCode::ADD_DOUBLE),                 // Instruction 23x
            0xac => Some(OpCode::SUB_DOUBLE),                 // Instruction 23x
            0xad => Some(OpCode::MUL_DOUBLE),                 // Instruction 23x
            0xae => Some(OpCode::DIV_DOUBLE),                 // Instruction 23x
            0xaf => Some(OpCode::REM_DOUBLE),                 // Instruction 23x
            0xb0 => Some(OpCode::ADD_INT_2ADDR),              // Instruction 12x
            0xb1 => Some(OpCode::SUB_INT_2ADDR),              // Instruction 12x
            0xb2 => Some(OpCode::MUL_INT_2ADDR),              // Instruction 12x
            0xb3 => Some(OpCode::DIV_INT_2ADDR),              // Instruction 12x
            0xb4 => Some(OpCode::REM_INT_2ADDR),              // Instruction 12x
            0xb5 => Some(OpCode::AND_INT_2ADDR),              // Instruction 12x
            0xb6 => Some(OpCode::OR_INT_2ADDR),               // Instruction 12x
            0xb7 => Some(OpCode::XOR_INT_2ADDR),              // Instruction 12x
            0xb8 => Some(OpCode::SHL_INT_2ADDR),              // Instruction 12x
            0xb9 => Some(OpCode::SHR_INT_2ADDR),              // Instruction 12x
            0xba => Some(OpCode::USHR_INT_2ADDR),             // Instruction 12x
            0xbb => Some(OpCode::ADD_LONG_2ADDR),             // Instruction 12x
            0xbc => Some(OpCode::SUB_LONG_2ADDR),             // Instruction 12x
            0xbd => Some(OpCode::MUL_LONG_2ADDR),             // Instruction 12x
            0xbe => Some(OpCode::DIV_LONG_2ADDR),             // Instruction 12x
            0xbf => Some(OpCode::REM_LONG_2ADDR),             // Instruction 12x
            0xc0 => Some(OpCode::AND_LONG_2ADDR),             // Instruction 12x
            0xc1 => Some(OpCode::OR_LONG_2ADDR),              // Instruction 12x
            0xc2 => Some(OpCode::XOR_LONG_2ADDR),             // Instruction 12x
            0xc3 => Some(OpCode::SHL_LONG_2ADDR),             // Instruction 12x
            0xc4 => Some(OpCode::SHR_LONG_2ADDR),             // Instruction 12x
            0xc5 => Some(OpCode::USHR_LONG_2ADDR),            // Instruction 12x
            0xc6 => Some(OpCode::ADD_FLOAT_2ADDR),            // Instruction 12x
            0xc7 => Some(OpCode::SUB_FLOAT_2ADDR),            // Instruction 12x
            0xc8 => Some(OpCode::MUL_FLOAT_2ADDR),            // Instruction 12x
            0xc9 => Some(OpCode::DIV_FLOAT_2ADDR),            // Instruction 12x
            0xca => Some(OpCode::REM_FLOAT_2ADDR),            // Instruction 12x
            0xcb => Some(OpCode::ADD_DOUBLE_2ADDR),           // Instruction 12x
            0xcc => Some(OpCode::SUB_DOUBLE_2ADDR),           // Instruction 12x
            0xcd => Some(OpCode::MUL_DOUBLE_2ADDR),           // Instruction 12x
            0xce => Some(OpCode::DIV_DOUBLE_2ADDR),           // Instruction 12x
            0xcf => Some(OpCode::REM_DOUBLE_2ADDR),           // Instruction 12x
            0xd0 => Some(OpCode::ADD_INT_LIT16),              // Instruction 22s
            0xd1 => Some(OpCode::RSUB_INT),                   // Instruction 22s
            0xd2 => Some(OpCode::MUL_INT_LIT16),              // Instruction 22s
            0xd3 => Some(OpCode::DIV_INT_LIT16),              // Instruction 22s
            0xd4 => Some(OpCode::REM_INT_LIT16),              // Instruction 22s
            0xd5 => Some(OpCode::AND_INT_LIT16),              // Instruction 22s
            0xd6 => Some(OpCode::OR_INT_LIT16),               // Instruction 22s
            0xd7 => Some(OpCode::XOR_INT_LIT16),              // Instruction 22s
            0xd8 => Some(OpCode::ADD_INT_LIT8),               // Instruction 22b
            0xd9 => Some(OpCode::RSUB_INT_LIT8),              // Instruction 22b
            0xda => Some(OpCode::MUL_INT_LIT8),               // Instruction 22b
            0xdb => Some(OpCode::DIV_INT_LIT8),               // Instruction 22b
            0xdc => Some(OpCode::REM_INT_LIT8),               // Instruction 22b
            0xdd => Some(OpCode::AND_INT_LIT8),               // Instruction 22b
            0xde => Some(OpCode::OR_INT_LIT8),                // Instruction 22b
            0xdf => Some(OpCode::XOR_INT_LIT8),               // Instruction 22b
            0xe0 => Some(OpCode::SHL_INT_LIT8),               // Instruction 22b
            0xe1 => Some(OpCode::SHR_INT_LIT8),               // Instruction 22b
            0xe2 => Some(OpCode::USHR_INT_LIT8),              // Instruction 22b

            /* Unused */
            0xe3..=0xf9 => {
                warning!("use of unused opcode {}", value);
                None
            },

            0xfa => Some(OpCode::INVOKE_POLYMORPHIC),         // Instruction 45cc,
            0xfb => Some(OpCode::INVOKE_POLYMORPHIC_RANGE),   // Instruction 4rcc,
            0xfc => Some(OpCode::INVOKE_CUSTOM),              // Instruction 35c,
            0xfd => Some(OpCode::INVOKE_CUSTOM_RANGE),        // Instruction 3rc,
            0xfe => Some(OpCode::CONST_METHOD_HANDLE),        // Instruction 21c,
            0xff => Some(OpCode::CONST_METHOD_TYPE),          // Instruction 21c,
        }
    }

    pub fn get_inst_format(&self) -> Option<&str> {
        match self {
            OpCode::NOP => Some("Instruction10x"),
            OpCode::MOVE => Some("Instruction12x"),
            OpCode::MOVE_FROM16 => Some("Instruction22x"),
            OpCode::MOVE_16 => Some("Instruction32x"),
            OpCode::MOVE_WIDE => Some("Instruction12x"),
            OpCode::MOVE_WIDE_FROM16 => Some("Instruction22x"),
            OpCode::MOVE_WIDE_16 => Some("Instruction32x"),
            OpCode::MOVE_OBJECT => Some("Instruction12x"),
            OpCode::MOVE_OBJECT_FROM16 => Some("Instruction22x"),
            OpCode::MOVE_OBJECT_16 => Some("Instruction32x"),
            OpCode::MOVE_RESULT => Some("Instruction11x"),
            OpCode::MOVE_RESULT_WIDE => Some("Instruction11x"),
            OpCode::MOVE_RESULT_OBJECT => Some("Instruction11x"),
            OpCode::MOVE_EXCEPTION => Some("Instruction11x"),
            OpCode::RETURN_VOID => Some("Instruction10x"),
            OpCode::RETURN => Some("Instruction11x"),
            OpCode::RETURN_WIDE => Some("Instruction11x"),
            OpCode::RETURN_OBJECT => Some("Instruction11x"),
            OpCode::CONST_4 => Some("Instruction11n"),
            OpCode::CONST_16 => Some("Instruction21s"),
            OpCode::CONST => Some("Instruction31i"),
            OpCode::CONST_HIGH16 => Some("Instruction21h"),
            OpCode::CONST_WIDE_16 => Some("Instruction21s"),
            OpCode::CONST_WIDE_32 => Some("Instruction31i"),
            OpCode::CONST_WIDE => Some("Instruction51l"),
            OpCode::CONST_WIDE_HIGH16 => Some("Instruction21h"),
            OpCode::CONST_STRING => Some("Instruction21c"),
            OpCode::CONST_STRING_JUMBO => Some("Instruction31c"),
            OpCode::CONST_CLASS => Some("Instruction21c"),
            OpCode::MONITOR_ENTER => Some("Instruction11x"),
            OpCode::MONITOR_EXIT => Some("Instruction11x"),
            OpCode::CHECK_CAST => Some("Instruction21c"),
            OpCode::INSTANCE_OF => Some("Instruction22c"),
            OpCode::ARRAY_LENGTH => Some("Instruction12x"),
            OpCode::NEW_INSTANCE => Some("Instruction21c"),
            OpCode::NEW_ARRAY => Some("Instruction22c"),
            OpCode::FILLED_NEW_ARRAY => Some("Instruction35c"),
            OpCode::FILLED_NEW_ARRAY_RANGE => Some("Instruction3rc"),
            OpCode::FILL_ARRAY_DATA => Some("Instruction31t"),
            OpCode::THROW => Some("Instruction11x"),
            OpCode::GOTO => Some("Instruction10t"),
            OpCode::GOTO_16 => Some("Instruction20t"),
            OpCode::GOTO_32 => Some("Instruction30t"),
            OpCode::PACKED_SWITCH => Some("Instruction31t"),
            OpCode::SPARSE_SWITCH => Some("Instruction31t"),
            OpCode::CMPL_FLOAT => Some("Instruction23x"),
            OpCode::CMPG_FLOAT => Some("Instruction23x"),
            OpCode::CMPL_DOUBLE => Some("Instruction23x"),
            OpCode::CMPG_DOUBLE => Some("Instruction23x"),
            OpCode::CMP_LONG => Some("Instruction23x"),
            OpCode::IF_EQ => Some("Instruction22t"),
            OpCode::IF_NE => Some("Instruction22t"),
            OpCode::IF_LT => Some("Instruction22t"),
            OpCode::IF_GE => Some("Instruction22t"),
            OpCode::IF_GT => Some("Instruction22t"),
            OpCode::IF_LE => Some("Instruction22t"),
            OpCode::IF_EQZ => Some("Instruction21t"),
            OpCode::IF_NEZ => Some("Instruction21t"),
            OpCode::IF_LTZ => Some("Instruction21t"),
            OpCode::IF_GEZ => Some("Instruction21t"),
            OpCode::IF_GTZ => Some("Instruction21t"),
            OpCode::IF_LEZ => Some("Instruction21t"),
            OpCode::AGET => Some("Instruction23x"),
            OpCode::AGET_WIDE => Some("Instruction23x"),
            OpCode::AGET_OBJECT => Some("Instruction23x"),
            OpCode::AGET_BOOLEAN => Some("Instruction23x"),
            OpCode::AGET_BYTE => Some("Instruction23x"),
            OpCode::AGET_CHAR => Some("Instruction23x"),
            OpCode::AGET_SHORT => Some("Instruction23x"),
            OpCode::APUT => Some("Instruction23x"),
            OpCode::APUT_WIDE => Some("Instruction23x"),
            OpCode::APUT_OBJECT => Some("Instruction23x"),
            OpCode::APUT_BOOLEAN => Some("Instruction23x"),
            OpCode::APUT_BYTE => Some("Instruction23x"),
            OpCode::APUT_CHAR => Some("Instruction23x"),
            OpCode::APUT_SHORT => Some("Instruction23x"),
            OpCode::IGET => Some("Instruction22c"),
            OpCode::IGET_WIDE => Some("Instruction22c"),
            OpCode::IGET_OBJECT => Some("Instruction22c"),
            OpCode::IGET_BOOLEAN => Some("Instruction22c"),
            OpCode::IGET_BYTE => Some("Instruction22c"),
            OpCode::IGET_CHAR => Some("Instruction22c"),
            OpCode::IGET_SHORT => Some("Instruction22c"),
            OpCode::IPUT => Some("Instruction22c"),
            OpCode::IPUT_WIDE => Some("Instruction22c"),
            OpCode::IPUT_OBJECT => Some("Instruction22c"),
            OpCode::IPUT_BOOLEAN => Some("Instruction22c"),
            OpCode::IPUT_BYTE => Some("Instruction22c"),
            OpCode::IPUT_CHAR => Some("Instruction22c"),
            OpCode::IPUT_SHORT => Some("Instruction22c"),
            OpCode::SGET => Some("Instruction21c"),
            OpCode::SGET_WIDE => Some("Instruction21c"),
            OpCode::SGET_OBJECT => Some("Instruction21c"),
            OpCode::SGET_BOOLEAN => Some("Instruction21c"),
            OpCode::SGET_BYTE => Some("Instruction21c"),
            OpCode::SGET_CHAR => Some("Instruction21c"),
            OpCode::SGET_SHORT => Some("Instruction21c"),
            OpCode::SPUT => Some("Instruction21c"),
            OpCode::SPUT_WIDE => Some("Instruction21c"),
            OpCode::SPUT_OBJECT => Some("Instruction21c"),
            OpCode::SPUT_BOOLEAN => Some("Instruction21c"),
            OpCode::SPUT_BYTE => Some("Instruction21c"),
            OpCode::SPUT_CHAR => Some("Instruction21c"),
            OpCode::SPUT_SHORT => Some("Instruction21c"),
            OpCode::INVOKE_VIRTUAL => Some("Instruction35c"),
            OpCode::INVOKE_SUPER => Some("Instruction35c"),
            OpCode::INVOKE_DIRECT => Some("Instruction35c"),
            OpCode::INVOKE_STATIC => Some("Instruction35c"),
            OpCode::INVOKE_INTERFACE => Some("Instruction35c"),
            OpCode::INVOKE_VIRTUAL_RANGE => Some("Instruction3rc"),
            OpCode::INVOKE_SUPER_RANGE => Some("Instruction3rc"),
            OpCode::INVOKE_DIRECT_RANGE => Some("Instruction3rc"),
            OpCode::INVOKE_STATIC_RANGE => Some("Instruction3rc"),
            OpCode::INVOKE_INTERFACE_RANGE => Some("Instruction3rc"),
            OpCode::NEG_INT => Some("Instruction12x"),
            OpCode::NOT_INT => Some("Instruction12x"),
            OpCode::NEG_LONG => Some("Instruction12x"),
            OpCode::NOT_LONG => Some("Instruction12x"),
            OpCode::NEG_FLOAT => Some("Instruction12x"),
            OpCode::NEG_DOUBLE => Some("Instruction12x"),
            OpCode::INT_TO_LONG => Some("Instruction12x"),
            OpCode::INT_TO_FLOAT => Some("Instruction12x"),
            OpCode::INT_TO_DOUBLE => Some("Instruction12x"),
            OpCode::LONG_TO_INT => Some("Instruction12x"),
            OpCode::LONG_TO_FLOAT => Some("Instruction12x"),
            OpCode::LONG_TO_DOUBLE => Some("Instruction12x"),
            OpCode::FLOAT_TO_INT => Some("Instruction12x"),
            OpCode::FLOAT_TO_LONG => Some("Instruction12x"),
            OpCode::FLOAT_TO_DOUBLE => Some("Instruction12x"),
            OpCode::DOUBLE_TO_INT => Some("Instruction12x"),
            OpCode::DOUBLE_TO_LONG => Some("Instruction12x"),
            OpCode::DOUBLE_TO_FLOAT => Some("Instruction12x"),
            OpCode::INT_TO_BYTE => Some("Instruction12x"),
            OpCode::INT_TO_CHAR => Some("Instruction12x"),
            OpCode::INT_TO_SHORT => Some("Instruction12x"),
            OpCode::ADD_INT => Some("Instruction23x"),
            OpCode::SUB_INT => Some("Instruction23x"),
            OpCode::MUL_INT => Some("Instruction23x"),
            OpCode::DIV_INT => Some("Instruction23x"),
            OpCode::REM_INT => Some("Instruction23x"),
            OpCode::AND_INT => Some("Instruction23x"),
            OpCode::OR_INT => Some("Instruction23x"),
            OpCode::XOR_INT => Some("Instruction23x"),
            OpCode::SHL_INT => Some("Instruction23x"),
            OpCode::SHR_INT => Some("Instruction23x"),
            OpCode::USHR_INT => Some("Instruction23x"),
            OpCode::ADD_LONG => Some("Instruction23x"),
            OpCode::SUB_LONG => Some("Instruction23x"),
            OpCode::MUL_LONG => Some("Instruction23x"),
            OpCode::DIV_LONG => Some("Instruction23x"),
            OpCode::REM_LONG => Some("Instruction23x"),
            OpCode::AND_LONG => Some("Instruction23x"),
            OpCode::OR_LONG => Some("Instruction23x"),
            OpCode::XOR_LONG => Some("Instruction23x"),
            OpCode::SHL_LONG => Some("Instruction23x"),
            OpCode::SHR_LONG => Some("Instruction23x"),
            OpCode::USHR_LONG => Some("Instruction23x"),
            OpCode::ADD_FLOAT => Some("Instruction23x"),
            OpCode::SUB_FLOAT => Some("Instruction23x"),
            OpCode::MUL_FLOAT => Some("Instruction23x"),
            OpCode::DIV_FLOAT => Some("Instruction23x"),
            OpCode::REM_FLOAT => Some("Instruction23x"),
            OpCode::ADD_DOUBLE => Some("Instruction23x"),
            OpCode::SUB_DOUBLE => Some("Instruction23x"),
            OpCode::MUL_DOUBLE => Some("Instruction23x"),
            OpCode::DIV_DOUBLE => Some("Instruction23x"),
            OpCode::REM_DOUBLE => Some("Instruction23x"),
            OpCode::ADD_INT_2ADDR => Some("Instruction12x"),
            OpCode::SUB_INT_2ADDR => Some("Instruction12x"),
            OpCode::MUL_INT_2ADDR => Some("Instruction12x"),
            OpCode::DIV_INT_2ADDR => Some("Instruction12x"),
            OpCode::REM_INT_2ADDR => Some("Instruction12x"),
            OpCode::AND_INT_2ADDR => Some("Instruction12x"),
            OpCode::OR_INT_2ADDR => Some("Instruction12x"),
            OpCode::XOR_INT_2ADDR => Some("Instruction12x"),
            OpCode::SHL_INT_2ADDR => Some("Instruction12x"),
            OpCode::SHR_INT_2ADDR => Some("Instruction12x"),
            OpCode::USHR_INT_2ADDR => Some("Instruction12x"),
            OpCode::ADD_LONG_2ADDR => Some("Instruction12x"),
            OpCode::SUB_LONG_2ADDR => Some("Instruction12x"),
            OpCode::MUL_LONG_2ADDR => Some("Instruction12x"),
            OpCode::DIV_LONG_2ADDR => Some("Instruction12x"),
            OpCode::REM_LONG_2ADDR => Some("Instruction12x"),
            OpCode::AND_LONG_2ADDR => Some("Instruction12x"),
            OpCode::OR_LONG_2ADDR => Some("Instruction12x"),
            OpCode::XOR_LONG_2ADDR => Some("Instruction12x"),
            OpCode::SHL_LONG_2ADDR => Some("Instruction12x"),
            OpCode::SHR_LONG_2ADDR => Some("Instruction12x"),
            OpCode::USHR_LONG_2ADDR => Some("Instruction12x"),
            OpCode::ADD_FLOAT_2ADDR => Some("Instruction12x"),
            OpCode::SUB_FLOAT_2ADDR => Some("Instruction12x"),
            OpCode::MUL_FLOAT_2ADDR => Some("Instruction12x"),
            OpCode::DIV_FLOAT_2ADDR => Some("Instruction12x"),
            OpCode::REM_FLOAT_2ADDR => Some("Instruction12x"),
            OpCode::ADD_DOUBLE_2ADDR => Some("Instruction12x"),
            OpCode::SUB_DOUBLE_2ADDR => Some("Instruction12x"),
            OpCode::MUL_DOUBLE_2ADDR => Some("Instruction12x"),
            OpCode::DIV_DOUBLE_2ADDR => Some("Instruction12x"),
            OpCode::REM_DOUBLE_2ADDR => Some("Instruction12x"),
            OpCode::ADD_INT_LIT16 => Some("Instruction22s"),
            OpCode::RSUB_INT => Some("Instruction22s"),
            OpCode::MUL_INT_LIT16 => Some("Instruction22s"),
            OpCode::DIV_INT_LIT16 => Some("Instruction22s"),
            OpCode::REM_INT_LIT16 => Some("Instruction22s"),
            OpCode::AND_INT_LIT16 => Some("Instruction22s"),
            OpCode::OR_INT_LIT16 => Some("Instruction22s"),
            OpCode::XOR_INT_LIT16 => Some("Instruction22s"),
            OpCode::ADD_INT_LIT8 => Some("Instruction22b"),
            OpCode::RSUB_INT_LIT8 => Some("Instruction22b"),
            OpCode::MUL_INT_LIT8 => Some("Instruction22b"),
            OpCode::DIV_INT_LIT8 => Some("Instruction22b"),
            OpCode::REM_INT_LIT8 => Some("Instruction22b"),
            OpCode::AND_INT_LIT8 => Some("Instruction22b"),
            OpCode::OR_INT_LIT8 => Some("Instruction22b"),
            OpCode::XOR_INT_LIT8 => Some("Instruction22b"),
            OpCode::SHL_INT_LIT8 => Some("Instruction22b"),
            OpCode::SHR_INT_LIT8 => Some("Instruction22b"),
            OpCode::USHR_INT_LIT8 => Some("Instruction22b"),
            OpCode::INVOKE_POLYMORPHIC => Some("Instruction45cc",),
            OpCode::INVOKE_POLYMORPHIC_RANGE => Some("Instruction4rcc",),
            OpCode::INVOKE_CUSTOM => Some("Instruction35c",),
            OpCode::INVOKE_CUSTOM_RANGE => Some("Instruction3rc",),
            OpCode::CONST_METHOD_HANDLE => Some("Instruction21c",),
            OpCode::CONST_METHOD_TYPE => Some("Instruction21c",),
        }
    }

    pub fn to_str(value: u8) -> Option<String> {
        match value {
            0x00 => Some(String::from("nop")),
            0x01 => Some(String::from("move")),
            0x02 => Some(String::from("move/from16")),
            0x03 => Some(String::from("move/16")),
            0x04 => Some(String::from("move-wide")),
            0x05 => Some(String::from("move-wide/from16")),
            0x06 => Some(String::from("move-wide/16")),
            0x07 => Some(String::from("move-object")),
            0x08 => Some(String::from("move-object/from16")),
            0x09 => Some(String::from("move-object/16")),
            0x0a => Some(String::from("move-result")),
            0x0b => Some(String::from("move-result-wide")),
            0x0c => Some(String::from("move-result-object")),
            0x0d => Some(String::from("move-exception")),
            0x0e => Some(String::from("return-void")),
            0x0f => Some(String::from("return")),
            0x10 => Some(String::from("return-wide")),
            0x11 => Some(String::from("return-object")),
            0x12 => Some(String::from("const/4")),
            0x13 => Some(String::from("const/16")),
            0x14 => Some(String::from("const")),
            0x15 => Some(String::from("const/high16")),
            0x16 => Some(String::from("const-wide/16")),
            0x17 => Some(String::from("const-wide/32")),
            0x18 => Some(String::from("const-wide")),
            0x19 => Some(String::from("const-wide/high16")),
            0x1a => Some(String::from("const-string")),
            0x1b => Some(String::from("const-string/jumbo")),
            0x1c => Some(String::from("const-class")),
            0x1d => Some(String::from("monitor-enter")),
            0x1e => Some(String::from("monitor-exit")),
            0x1f => Some(String::from("check-cast")),
            0x20 => Some(String::from("instance-of")),
            0x21 => Some(String::from("array-length")),
            0x22 => Some(String::from("new-instance")),
            0x23 => Some(String::from("new-array")),
            0x24 => Some(String::from("filled-new-array")),
            0x25 => Some(String::from("filled-new-array/range")),
            0x26 => Some(String::from("fill-array-data")),
            0x27 => Some(String::from("throw")),
            0x28 => Some(String::from("goto")),
            0x29 => Some(String::from("goto/16")),
            0x2a => Some(String::from("goto/32")),
            0x2b => Some(String::from("packed-switch")),
            0x2c => Some(String::from("sparse-switch")),
            0x2d => Some(String::from("cmpl-float")),
            0x2e => Some(String::from("cmpg-float")),
            0x2f => Some(String::from("cmpl-double")),
            0x30 => Some(String::from("cmpg-double")),
            0x31 => Some(String::from("cmp-long")),
            0x32 => Some(String::from("if-eq")),
            0x33 => Some(String::from("if-ne")),
            0x34 => Some(String::from("if-lt")),
            0x35 => Some(String::from("if-ge")),
            0x36 => Some(String::from("if-gt")),
            0x37 => Some(String::from("if-le")),
            0x38 => Some(String::from("if-eqz")),
            0x39 => Some(String::from("if-nez")),
            0x3a => Some(String::from("if-ltz")),
            0x3b => Some(String::from("if-gez")),
            0x3c => Some(String::from("if-gtz")),
            0x3d => Some(String::from("if-lez")),

            /* Unused */
            0x3e..=0x43 => {
                warning!("use of unused opcode {}", value);
                None
            },

            0x44 => Some(String::from("aget")),
            0x45 => Some(String::from("aget-wide")),
            0x46 => Some(String::from("aget-object")),
            0x47 => Some(String::from("aget-boolean")),
            0x48 => Some(String::from("aget-byte")),
            0x49 => Some(String::from("aget-char")),
            0x4a => Some(String::from("aget-short")),
            0x4b => Some(String::from("aput")),
            0x4c => Some(String::from("aput-wide")),
            0x4d => Some(String::from("aput-object")),
            0x4e => Some(String::from("aput-boolean")),
            0x4f => Some(String::from("aput-byte")),
            0x50 => Some(String::from("aput-char")),
            0x51 => Some(String::from("aput-short")),
            0x52 => Some(String::from("iget")),
            0x53 => Some(String::from("iget-wide")),
            0x54 => Some(String::from("iget-object")),
            0x55 => Some(String::from("iget-boolean")),
            0x56 => Some(String::from("iget-byte")),
            0x57 => Some(String::from("iget-char")),
            0x58 => Some(String::from("iget-short")),
            0x59 => Some(String::from("iput")),
            0x5a => Some(String::from("iput-wide")),
            0x5b => Some(String::from("iput-object")),
            0x5c => Some(String::from("iput-boolean")),
            0x5d => Some(String::from("iput-byte")),
            0x5e => Some(String::from("iput-char")),
            0x5f => Some(String::from("iput-short")),
            0x60 => Some(String::from("sget")),
            0x61 => Some(String::from("sget-wide")),
            0x62 => Some(String::from("sget-object")),
            0x63 => Some(String::from("sget-boolean")),
            0x64 => Some(String::from("sget-byte")),
            0x65 => Some(String::from("sget-char")),
            0x66 => Some(String::from("sget-short")),
            0x67 => Some(String::from("sput")),
            0x68 => Some(String::from("sput-wide")),
            0x69 => Some(String::from("sput-object")),
            0x6a => Some(String::from("sput-boolean")),
            0x6b => Some(String::from("sput-byte")),
            0x6c => Some(String::from("sput-char")),
            0x6d => Some(String::from("sput-short")),
            0x6e => Some(String::from("invoke-virtual")),
            0x6f => Some(String::from("invoke-super")),
            0x70 => Some(String::from("invoke-direct")),
            0x71 => Some(String::from("invoke-static")),
            0x72 => Some(String::from("invoke-interface")),

            /* Unused */
            0x73 => { warning!("use of unused opcode {}", value); None },

            0x74 => Some(String::from("invoke-virtual/range")),
            0x75 => Some(String::from("invoke-super/range")),
            0x76 => Some(String::from("invoke-direct/range")),
            0x77 => Some(String::from("invoke-static/range")),
            0x78 => Some(String::from("invoke-interface/range")),

            /* Unused */
            0x79 | 0x7a => {
                warning!("use of unused opcode {}", value);
                None
            },

            0x7b => Some(String::from("neg-int")),
            0x7c => Some(String::from("not-int")),
            0x7d => Some(String::from("neg-long")),
            0x7e => Some(String::from("not-long")),
            0x7f => Some(String::from("neg-float")),
            0x80 => Some(String::from("neg-double")),
            0x81 => Some(String::from("int-to-long")),
            0x82 => Some(String::from("int-to-float")),
            0x83 => Some(String::from("int-to-double")),
            0x84 => Some(String::from("long-to-int")),
            0x85 => Some(String::from("long-to-float")),
            0x86 => Some(String::from("long-to-double")),
            0x87 => Some(String::from("float-to-int")),
            0x88 => Some(String::from("float-to-long")),
            0x89 => Some(String::from("float-to-double")),
            0x8a => Some(String::from("double-to-int")),
            0x8b => Some(String::from("double-to-long")),
            0x8c => Some(String::from("double-to-float")),
            0x8d => Some(String::from("int-to-byte")),
            0x8e => Some(String::from("int-to-char")),
            0x8f => Some(String::from("int-to-short")),
            0x90 => Some(String::from("add-int")),
            0x91 => Some(String::from("sub-int")),
            0x92 => Some(String::from("mul-int")),
            0x93 => Some(String::from("div-int")),
            0x94 => Some(String::from("rem-int")),
            0x95 => Some(String::from("and-int")),
            0x96 => Some(String::from("or-int")),
            0x97 => Some(String::from("xor-int")),
            0x98 => Some(String::from("shl-int")),
            0x99 => Some(String::from("shr-int")),
            0x9a => Some(String::from("ushr-int")),
            0x9b => Some(String::from("add-long")),
            0x9c => Some(String::from("sub-long")),
            0x9d => Some(String::from("mul-long")),
            0x9e => Some(String::from("div-long")),
            0x9f => Some(String::from("rem-long")),
            0xa0 => Some(String::from("and-long")),
            0xa1 => Some(String::from("or-long")),
            0xa2 => Some(String::from("xor-long")),
            0xa3 => Some(String::from("shl-long")),
            0xa4 => Some(String::from("shr-long")),
            0xa5 => Some(String::from("ushr-long")),
            0xa6 => Some(String::from("add-float")),
            0xa7 => Some(String::from("sub-float")),
            0xa8 => Some(String::from("mul-float")),
            0xa9 => Some(String::from("div-float")),
            0xaa => Some(String::from("rem-float")),
            0xab => Some(String::from("add-double")),
            0xac => Some(String::from("sub-double")),
            0xad => Some(String::from("mul-double")),
            0xae => Some(String::from("div-double")),
            0xaf => Some(String::from("rem-double")),
            0xb0 => Some(String::from("add-int/2addr")),
            0xb1 => Some(String::from("sub-int/2addr")),
            0xb2 => Some(String::from("mul-int/2addr")),
            0xb3 => Some(String::from("div-int/2addr")),
            0xb4 => Some(String::from("rem-int/2addr")),
            0xb5 => Some(String::from("and-int/2addr")),
            0xb6 => Some(String::from("or-int/2addr")),
            0xb7 => Some(String::from("xor-int/2addr")),
            0xb8 => Some(String::from("shl-int/2addr")),
            0xb9 => Some(String::from("shr-int/2addr")),
            0xba => Some(String::from("ushr-int/2addr")),
            0xbb => Some(String::from("add-long/2addr")),
            0xbc => Some(String::from("sub-long/2addr")),
            0xbd => Some(String::from("mul-long/2addr")),
            0xbe => Some(String::from("div-long/2addr")),
            0xbf => Some(String::from("rem-long/2addr")),
            0xc0 => Some(String::from("and-long/2addr")),
            0xc1 => Some(String::from("or-long/2addr")),
            0xc2 => Some(String::from("xor-long/2addr")),
            0xc3 => Some(String::from("shl-long/2addr")),
            0xc4 => Some(String::from("shr-long/2addr")),
            0xc5 => Some(String::from("ushr-long/2addr")),
            0xc6 => Some(String::from("add-float/2addr")),
            0xc7 => Some(String::from("sub-float/2addr")),
            0xc8 => Some(String::from("mul-float/2addr")),
            0xc9 => Some(String::from("div-float/2addr")),
            0xca => Some(String::from("rem-float/2addr")),
            0xcb => Some(String::from("add-double/2addr")),
            0xcc => Some(String::from("sub-double/2addr")),
            0xcd => Some(String::from("mul-double/2addr")),
            0xce => Some(String::from("div-double/2addr")),
            0xcf => Some(String::from("rem-double/2addr")),
            0xd0 => Some(String::from("add-int/lit16")),
            0xd1 => Some(String::from("rsub-int")),
            0xd2 => Some(String::from("mul-int/lit16")),
            0xd3 => Some(String::from("div-int/lit16")),
            0xd4 => Some(String::from("rem-int/lit16")),
            0xd5 => Some(String::from("and-int/lit16")),
            0xd6 => Some(String::from("or-int/lit16")),
            0xd7 => Some(String::from("xor-int/lit16")),
            0xd8 => Some(String::from("add-int/lit8")),
            0xd9 => Some(String::from("rsub-int/lit8")),
            0xda => Some(String::from("mul-int/lit8")),
            0xdb => Some(String::from("div-int/lit8")),
            0xdc => Some(String::from("rem-int/lit8")),
            0xdd => Some(String::from("and-int/lit8")),
            0xde => Some(String::from("or-int/lit8")),
            0xdf => Some(String::from("xor-int/lit8")),
            0xe0 => Some(String::from("shl-int/lit8")),
            0xe1 => Some(String::from("shr-int/lit8")),
            0xe2 => Some(String::from("ushr-int/lit8")),

            /* Unused */
            0xe3..=0xf9 => {
                warning!("use of unused opcode {}", value);
                None
            },

            0xfa => Some(String::from("invoke-polymorphic")),
            0xfb => Some(String::from("invoke-polymorphic/range")),
            0xfc => Some(String::from("invoke-custom")),
            0xfd => Some(String::from("invoke-custom/range")),
            0xfe => Some(String::from("const-method/handle")),
            0xff => Some(String::from("const-method/type")),
        }
    }
}
