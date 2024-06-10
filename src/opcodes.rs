use std::fmt;
use log::warn;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
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
    PACKED_SWITCH_PAYLOAD,
    SPARSE_SWITCH_PAYLOAD,
    FILL_ARRAY_DATA_PAYLOAD,
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
                warn!("use of unused opcode {}", value);
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
                warn!("use of unused opcode {}", value);
                None
            },

            0x74 => Some(OpCode::INVOKE_VIRTUAL_RANGE),       // Instruction 3rc
            0x75 => Some(OpCode::INVOKE_SUPER_RANGE),         // Instruction 3rc
            0x76 => Some(OpCode::INVOKE_DIRECT_RANGE),        // Instruction 3rc
            0x77 => Some(OpCode::INVOKE_STATIC_RANGE),        // Instruction 3rc
            0x78 => Some(OpCode::INVOKE_INTERFACE_RANGE),     // Instruction 3rc

            /* Unused */
            0x79 | 0x7a => {
                warn!("use of unused opcode {}", value);
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
                warn!("use of unused opcode {}", value);
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
            OpCode::PACKED_SWITCH_PAYLOAD => Some("packed-switch-payload"),
            OpCode::SPARSE_SWITCH_PAYLOAD => Some("sparse-switch-payload"),
            OpCode::FILL_ARRAY_DATA_PAYLOAD => Some("fill-array-data-payload"),
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::NOP => write!(f, "nop"),
            OpCode::MOVE => write!(f, "move"),
            OpCode::MOVE_FROM16 => write!(f, "move-from16"),
            OpCode::MOVE_16 => write!(f, "move-16"),
            OpCode::MOVE_WIDE => write!(f, "move-wide"),
            OpCode::MOVE_WIDE_FROM16 => write!(f, "move-wide/from16"),
            OpCode::MOVE_WIDE_16 => write!(f, "move-wide/16"),
            OpCode::MOVE_OBJECT => write!(f, "move-object"),
            OpCode::MOVE_OBJECT_FROM16 => write!(f, "move-object/from16"),
            OpCode::MOVE_OBJECT_16 => write!(f, "move-object/16"),
            OpCode::MOVE_RESULT => write!(f, "move-result"),
            OpCode::MOVE_RESULT_WIDE => write!(f, "move-result-wide"),
            OpCode::MOVE_RESULT_OBJECT => write!(f, "move-result-object"),
            OpCode::MOVE_EXCEPTION => write!(f, "move-exception"),
            OpCode::RETURN_VOID => write!(f, "return-void"),
            OpCode::RETURN => write!(f, "return"),
            OpCode::RETURN_WIDE => write!(f, "return-wide"),
            OpCode::RETURN_OBJECT => write!(f, "return-object"),
            OpCode::CONST_4 => write!(f, "const/4"),
            OpCode::CONST_16 => write!(f, "const/16"),
            OpCode::CONST => write!(f, "const"),
            OpCode::CONST_HIGH16 => write!(f, "const-high16"),
            OpCode::CONST_WIDE_16 => write!(f, "const-wide/16"),
            OpCode::CONST_WIDE_32 => write!(f, "const-wide/32"),
            OpCode::CONST_WIDE => write!(f, "const-wide"),
            OpCode::CONST_WIDE_HIGH16 => write!(f, "const-wide/high16"),
            OpCode::CONST_STRING => write!(f, "const-string"),
            OpCode::CONST_STRING_JUMBO => write!(f, "const-string/jumbo"),
            OpCode::CONST_CLASS => write!(f, "const-class"),
            OpCode::MONITOR_ENTER => write!(f, "monitor-enter"),
            OpCode::MONITOR_EXIT => write!(f, "monitor-exit"),
            OpCode::CHECK_CAST => write!(f, "check-cast"),
            OpCode::INSTANCE_OF => write!(f, "instance-of"),
            OpCode::ARRAY_LENGTH => write!(f, "array-length"),
            OpCode::NEW_INSTANCE => write!(f, "new-instance"),
            OpCode::NEW_ARRAY => write!(f, "new-array"),
            OpCode::FILLED_NEW_ARRAY => write!(f, "filled-new-array"),
            OpCode::FILLED_NEW_ARRAY_RANGE => write!(f, "filled-new-array/range"),
            OpCode::FILL_ARRAY_DATA => write!(f, "fill-array-data"),
            OpCode::THROW => write!(f, "throw"),
            OpCode::GOTO => write!(f, "goto"),
            OpCode::GOTO_16 => write!(f, "goto-16"),
            OpCode::GOTO_32 => write!(f, "goto-32"),
            OpCode::PACKED_SWITCH => write!(f, "packed-switch"),
            OpCode::SPARSE_SWITCH => write!(f, "sparse-switch"),
            OpCode::CMPL_FLOAT => write!(f, "cmpl-float"),
            OpCode::CMPG_FLOAT => write!(f, "cmpg-float"),
            OpCode::CMPL_DOUBLE => write!(f, "cmpl-double"),
            OpCode::CMPG_DOUBLE => write!(f, "cmpg-double"),
            OpCode::CMP_LONG => write!(f, "cmp-long"),
            OpCode::IF_EQ => write!(f, "if-eq"),
            OpCode::IF_NE => write!(f, "if-ne"),
            OpCode::IF_LT => write!(f, "if-lt"),
            OpCode::IF_GE => write!(f, "if-ge"),
            OpCode::IF_GT => write!(f, "if-gt"),
            OpCode::IF_LE => write!(f, "if-le"),
            OpCode::IF_EQZ => write!(f, "if-eqz"),
            OpCode::IF_NEZ => write!(f, "if-nez"),
            OpCode::IF_LTZ => write!(f, "if-ltz"),
            OpCode::IF_GEZ => write!(f, "if-gez"),
            OpCode::IF_GTZ => write!(f, "if-gtz"),
            OpCode::IF_LEZ => write!(f, "if-lez"),
            OpCode::AGET => write!(f, "aget"),
            OpCode::AGET_WIDE => write!(f, "aget-wide"),
            OpCode::AGET_OBJECT => write!(f, "aget-object"),
            OpCode::AGET_BOOLEAN => write!(f, "aget-boolean"),
            OpCode::AGET_BYTE => write!(f, "aget-byte"),
            OpCode::AGET_CHAR => write!(f, "aget-char"),
            OpCode::AGET_SHORT => write!(f, "aget-short"),
            OpCode::APUT => write!(f, "aput"),
            OpCode::APUT_WIDE => write!(f, "aput-wide"),
            OpCode::APUT_OBJECT => write!(f, "aput-object"),
            OpCode::APUT_BOOLEAN => write!(f, "aput-boolean"),
            OpCode::APUT_BYTE => write!(f, "aput-byte"),
            OpCode::APUT_CHAR => write!(f, "aput-char"),
            OpCode::APUT_SHORT => write!(f, "aput-short"),
            OpCode::IGET => write!(f, "iget"),
            OpCode::IGET_WIDE => write!(f, "iget-wide"),
            OpCode::IGET_OBJECT => write!(f, "iget-object"),
            OpCode::IGET_BOOLEAN => write!(f, "iget-boolean"),
            OpCode::IGET_BYTE => write!(f, "iget-byte"),
            OpCode::IGET_CHAR => write!(f, "iget-char"),
            OpCode::IGET_SHORT => write!(f, "iget-short"),
            OpCode::IPUT => write!(f, "iput"),
            OpCode::IPUT_WIDE => write!(f, "iput-wide"),
            OpCode::IPUT_OBJECT => write!(f, "iput-object"),
            OpCode::IPUT_BOOLEAN => write!(f, "iput-boolean"),
            OpCode::IPUT_BYTE => write!(f, "iput-byte"),
            OpCode::IPUT_CHAR => write!(f, "iput-char"),
            OpCode::IPUT_SHORT => write!(f, "iput-short"),
            OpCode::SGET => write!(f, "sget"),
            OpCode::SGET_WIDE => write!(f, "sget-wide"),
            OpCode::SGET_OBJECT => write!(f, "sget-object"),
            OpCode::SGET_BOOLEAN => write!(f, "sget-boolean"),
            OpCode::SGET_BYTE => write!(f, "sget-byte"),
            OpCode::SGET_CHAR => write!(f, "sget-char"),
            OpCode::SGET_SHORT => write!(f, "sget-short"),
            OpCode::SPUT => write!(f, "sput"),
            OpCode::SPUT_WIDE => write!(f, "sput-wide"),
            OpCode::SPUT_OBJECT => write!(f, "sput-object"),
            OpCode::SPUT_BOOLEAN => write!(f, "sput-boolean"),
            OpCode::SPUT_BYTE => write!(f, "sput-byte"),
            OpCode::SPUT_CHAR => write!(f, "sput-char"),
            OpCode::SPUT_SHORT => write!(f, "sput-short"),
            OpCode::INVOKE_VIRTUAL => write!(f, "invoke-virtual"),
            OpCode::INVOKE_SUPER => write!(f, "invoke-super"),
            OpCode::INVOKE_DIRECT => write!(f, "invoke-direct"),
            OpCode::INVOKE_STATIC => write!(f, "invoke-static"),
            OpCode::INVOKE_INTERFACE => write!(f, "invoke-interface"),
            OpCode::INVOKE_VIRTUAL_RANGE => write!(f, "invoke-virtual/range"),
            OpCode::INVOKE_SUPER_RANGE => write!(f, "invoke-super/range"),
            OpCode::INVOKE_DIRECT_RANGE => write!(f, "invoke-direct/range"),
            OpCode::INVOKE_STATIC_RANGE => write!(f, "invoke-static/range"),
            OpCode::INVOKE_INTERFACE_RANGE => write!(f, "invoke-interface/range"),
            OpCode::NEG_INT => write!(f, "neg-int"),
            OpCode::NOT_INT => write!(f, "not-int"),
            OpCode::NEG_LONG => write!(f, "neg-long"),
            OpCode::NOT_LONG => write!(f, "not-long"),
            OpCode::NEG_FLOAT => write!(f, "neg-float"),
            OpCode::NEG_DOUBLE => write!(f, "neg-double"),
            OpCode::INT_TO_LONG => write!(f, "int-to-long"),
            OpCode::INT_TO_FLOAT => write!(f, "int-to-float"),
            OpCode::INT_TO_DOUBLE => write!(f, "int-to-double"),
            OpCode::LONG_TO_INT => write!(f, "long-to-int"),
            OpCode::LONG_TO_FLOAT => write!(f, "long-to-float"),
            OpCode::LONG_TO_DOUBLE => write!(f, "long-to-double"),
            OpCode::FLOAT_TO_INT => write!(f, "float-to-int"),
            OpCode::FLOAT_TO_LONG => write!(f, "float-to-long"),
            OpCode::FLOAT_TO_DOUBLE => write!(f, "float-to-double"),
            OpCode::DOUBLE_TO_INT => write!(f, "double-to-int"),
            OpCode::DOUBLE_TO_LONG => write!(f, "double-to-long"),
            OpCode::DOUBLE_TO_FLOAT => write!(f, "double-to-float"),
            OpCode::INT_TO_BYTE => write!(f, "int-to-byte"),
            OpCode::INT_TO_CHAR => write!(f, "int-to-char"),
            OpCode::INT_TO_SHORT => write!(f, "int-to-short"),
            OpCode::ADD_INT => write!(f, "add-int"),
            OpCode::SUB_INT => write!(f, "sub-int"),
            OpCode::MUL_INT => write!(f, "mul-int"),
            OpCode::DIV_INT => write!(f, "div-int"),
            OpCode::REM_INT => write!(f, "rem-int"),
            OpCode::AND_INT => write!(f, "and-int"),
            OpCode::OR_INT => write!(f, "or-int"),
            OpCode::XOR_INT => write!(f, "xor-int"),
            OpCode::SHL_INT => write!(f, "shl-int"),
            OpCode::SHR_INT => write!(f, "shr-int"),
            OpCode::USHR_INT => write!(f, "ushr-int"),
            OpCode::ADD_LONG => write!(f, "add-long"),
            OpCode::SUB_LONG => write!(f, "sub-long"),
            OpCode::MUL_LONG => write!(f, "mul-long"),
            OpCode::DIV_LONG => write!(f, "div-long"),
            OpCode::REM_LONG => write!(f, "rem-long"),
            OpCode::AND_LONG => write!(f, "and-long"),
            OpCode::OR_LONG => write!(f, "or-long"),
            OpCode::XOR_LONG => write!(f, "xor-long"),
            OpCode::SHL_LONG => write!(f, "shl-long"),
            OpCode::SHR_LONG => write!(f, "shr-long"),
            OpCode::USHR_LONG => write!(f, "ushr-long"),
            OpCode::ADD_FLOAT => write!(f, "add-float"),
            OpCode::SUB_FLOAT => write!(f, "sub-float"),
            OpCode::MUL_FLOAT => write!(f, "mul-float"),
            OpCode::DIV_FLOAT => write!(f, "div-float"),
            OpCode::REM_FLOAT => write!(f, "rem-float"),
            OpCode::ADD_DOUBLE => write!(f, "add-double"),
            OpCode::SUB_DOUBLE => write!(f, "sub-double"),
            OpCode::MUL_DOUBLE => write!(f, "mul-double"),
            OpCode::DIV_DOUBLE => write!(f, "div-double"),
            OpCode::REM_DOUBLE => write!(f, "rem-double"),
            OpCode::ADD_INT_2ADDR => write!(f, "add-int/2addr"),
            OpCode::SUB_INT_2ADDR => write!(f, "sub-int/2addr"),
            OpCode::MUL_INT_2ADDR => write!(f, "mul-int/2addr"),
            OpCode::DIV_INT_2ADDR => write!(f, "div-int/2addr"),
            OpCode::REM_INT_2ADDR => write!(f, "rem-int/2addr"),
            OpCode::AND_INT_2ADDR => write!(f, "and-int/2addr"),
            OpCode::OR_INT_2ADDR => write!(f, "or-int/2addr"),
            OpCode::XOR_INT_2ADDR => write!(f, "xor-int/2addr"),
            OpCode::SHL_INT_2ADDR => write!(f, "shl-int/2addr"),
            OpCode::SHR_INT_2ADDR => write!(f, "shr-int/2addr"),
            OpCode::USHR_INT_2ADDR => write!(f, "ushr-int/2addr"),
            OpCode::ADD_LONG_2ADDR => write!(f, "add-long/2addr"),
            OpCode::SUB_LONG_2ADDR => write!(f, "sub-long/2addr"),
            OpCode::MUL_LONG_2ADDR => write!(f, "mul-long/2addr"),
            OpCode::DIV_LONG_2ADDR => write!(f, "div-long/2addr"),
            OpCode::REM_LONG_2ADDR => write!(f, "rem-long/2addr"),
            OpCode::AND_LONG_2ADDR => write!(f, "and-long/2addr"),
            OpCode::OR_LONG_2ADDR => write!(f, "or-long/2addr"),
            OpCode::XOR_LONG_2ADDR => write!(f, "xor-long/2addr"),
            OpCode::SHL_LONG_2ADDR => write!(f, "shl-long/2addr"),
            OpCode::SHR_LONG_2ADDR => write!(f, "shr-long/2addr"),
            OpCode::USHR_LONG_2ADDR => write!(f, "ushr-long/2addr"),
            OpCode::ADD_FLOAT_2ADDR => write!(f, "add-float/2addr"),
            OpCode::SUB_FLOAT_2ADDR => write!(f, "sub-float/2addr"),
            OpCode::MUL_FLOAT_2ADDR => write!(f, "mul-float/2addr"),
            OpCode::DIV_FLOAT_2ADDR => write!(f, "div-float/2addr"),
            OpCode::REM_FLOAT_2ADDR => write!(f, "rem-float/2addr"),
            OpCode::ADD_DOUBLE_2ADDR => write!(f, "add-double/2addr"),
            OpCode::SUB_DOUBLE_2ADDR => write!(f, "sub-double/2addr"),
            OpCode::MUL_DOUBLE_2ADDR => write!(f, "mul-double/2addr"),
            OpCode::DIV_DOUBLE_2ADDR => write!(f, "div-double/2addr"),
            OpCode::REM_DOUBLE_2ADDR => write!(f, "rem-double/2addr"),
            OpCode::ADD_INT_LIT16 => write!(f, "add-int/lit16"),
            OpCode::RSUB_INT => write!(f, "rsub-int"),
            OpCode::MUL_INT_LIT16 => write!(f, "mul-int/lit16"),
            OpCode::DIV_INT_LIT16 => write!(f, "div-int/lit16"),
            OpCode::REM_INT_LIT16 => write!(f, "rem-int/lit16"),
            OpCode::AND_INT_LIT16 => write!(f, "and-int/lit16"),
            OpCode::OR_INT_LIT16 => write!(f, "or-int/lit16"),
            OpCode::XOR_INT_LIT16 => write!(f, "xor-int/lit16"),
            OpCode::ADD_INT_LIT8 => write!(f, "add-int/lit8"),
            OpCode::RSUB_INT_LIT8 => write!(f, "rsub-int/lit8"),
            OpCode::MUL_INT_LIT8 => write!(f, "mul-int/lit8"),
            OpCode::DIV_INT_LIT8 => write!(f, "div-int/lit8"),
            OpCode::REM_INT_LIT8 => write!(f, "rem-int/lit8"),
            OpCode::AND_INT_LIT8 => write!(f, "and-int/lit8"),
            OpCode::OR_INT_LIT8 => write!(f, "or-int/lit8"),
            OpCode::XOR_INT_LIT8 => write!(f, "xor-int/lit8"),
            OpCode::SHL_INT_LIT8 => write!(f, "shl-int/lit8"),
            OpCode::SHR_INT_LIT8 => write!(f, "shr-int/lit8"),
            OpCode::USHR_INT_LIT8 => write!(f, "ushr-int/lit8"),
            OpCode::INVOKE_POLYMORPHIC => write!(f, "invoke-polymorphic"),
            OpCode::INVOKE_POLYMORPHIC_RANGE => write!(f, "invoke-polymorphic/range"),
            OpCode::INVOKE_CUSTOM => write!(f, "invoke-custom"),
            OpCode::INVOKE_CUSTOM_RANGE => write!(f, "invoke-custom/range"),
            OpCode::CONST_METHOD_HANDLE => write!(f, "const-method-handle"),
            OpCode::CONST_METHOD_TYPE => write!(f, "const-method-type"),
            OpCode::PACKED_SWITCH_PAYLOAD => write!(f, "packed-switch-payload"),
            OpCode::SPARSE_SWITCH_PAYLOAD => write!(f, "sparse-switch-payload"),
            OpCode::FILL_ARRAY_DATA_PAYLOAD => write!(f, "fill-array-data-payload"),
        }
    }
}
