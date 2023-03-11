use crate::instructions::InstructionHandler;
use crate::opcodes::OpCode;

pub fn disasm(ins: &(impl InstructionHandler + ?Sized)) -> String {
    match ins.opcode() {
        OpCode::GOTO | OpCode::GOTO_16
            | OpCode::GOTO_32 => format!("{} +{}",
                                            ins.opcode(),
                                            ins.a(&ins.bytes()).unwrap() * 2),

        OpCode::NOP => format!(""),
        OpCode::RETURN_VOID => format!("{}", ins.opcode()),

        OpCode::CONST_4 => format!("{} v{} {}",
                                    ins.opcode(),
                                    ins.a(&ins.bytes()).unwrap(),
                                    ins.b(&ins.bytes()).unwrap()),

        OpCode::MONITOR_ENTER            | OpCode::MONITOR_EXIT
            | OpCode::MOVE_EXCEPTION     | OpCode::MOVE_RESULT
            | OpCode::MOVE_RESULT_OBJECT | OpCode::MOVE_RESULT_WIDE
            | OpCode::RETURN             | OpCode::RETURN_OBJECT
            | OpCode::RETURN_WIDE        | OpCode::THROW
            => format!("{} v{}",
                        ins.opcode(),
                        ins.a(&ins.bytes()).unwrap()),

        OpCode::ADD_DOUBLE_2ADDR      | OpCode::ADD_FLOAT_2ADDR
            | OpCode::ADD_INT_2ADDR   | OpCode::ADD_LONG_2ADDR
            | OpCode::AND_INT_2ADDR   | OpCode::AND_LONG_2ADDR
            | OpCode::ARRAY_LENGTH    | OpCode::DIV_DOUBLE_2ADDR
            | OpCode::DIV_FLOAT_2ADDR | OpCode::DIV_INT_2ADDR
            | OpCode::DIV_LONG_2ADDR  | OpCode::DOUBLE_TO_FLOAT
            | OpCode::DOUBLE_TO_INT   | OpCode::DOUBLE_TO_LONG
            | OpCode::FLOAT_TO_DOUBLE | OpCode::FLOAT_TO_INT
            | OpCode::FLOAT_TO_LONG   | OpCode::INT_TO_BYTE
            | OpCode::INT_TO_CHAR     | OpCode::INT_TO_DOUBLE
            | OpCode::INT_TO_FLOAT    | OpCode::INT_TO_LONG
            | OpCode::INT_TO_SHORT    | OpCode::LONG_TO_DOUBLE
            | OpCode::LONG_TO_FLOAT   | OpCode::LONG_TO_INT
            | OpCode::MOVE            | OpCode::MOVE_OBJECT
            | OpCode::MOVE_WIDE       | OpCode::MUL_DOUBLE_2ADDR
            | OpCode::MUL_FLOAT_2ADDR | OpCode::MUL_INT_2ADDR
            | OpCode::MUL_LONG_2ADDR  | OpCode::NEG_DOUBLE
            | OpCode::NEG_FLOAT       | OpCode::NEG_INT
            | OpCode::NEG_LONG        | OpCode::NOT_INT
            | OpCode::NOT_LONG        | OpCode::OR_INT_2ADDR
            | OpCode::OR_LONG_2ADDR   | OpCode::REM_DOUBLE_2ADDR
            | OpCode::REM_FLOAT_2ADDR | OpCode::REM_INT_2ADDR
            | OpCode::REM_LONG_2ADDR  | OpCode::SHL_INT_2ADDR
            | OpCode::SHL_LONG_2ADDR  | OpCode::SHR_INT_2ADDR
            | OpCode::SHR_LONG_2ADDR  | OpCode::SUB_DOUBLE_2ADDR
            | OpCode::SUB_FLOAT_2ADDR | OpCode::SUB_INT_2ADDR
            | OpCode::SUB_LONG_2ADDR  | OpCode::USHR_INT_2ADDR
            | OpCode::USHR_LONG_2ADDR | OpCode::XOR_INT_2ADDR
            | OpCode::XOR_LONG_2ADDR
            => format!("{} v{} v{}",
                        ins.opcode(),
                        ins.a(&ins.bytes()).unwrap(),
                        ins.b(&ins.bytes()).unwrap()),

        OpCode::CHECK_CAST                | OpCode::CONST_CLASS
            | OpCode::CONST_METHOD_HANDLE | OpCode::CONST_METHOD_TYPE
            | OpCode::CONST_STRING        | OpCode::NEW_INSTANCE
            | OpCode::SGET_BOOLEAN        | OpCode::SGET_BYTE
            | OpCode::SGET_CHAR           | OpCode::SGET
            | OpCode::SGET_OBJECT         | OpCode::SGET_SHORT
            | OpCode::SGET_WIDE           | OpCode::SPUT_BOOLEAN
            | OpCode::SPUT_BYTE           | OpCode::SPUT_CHAR
            | OpCode::SPUT                | OpCode::SPUT_OBJECT
            | OpCode::SPUT_SHORT          | OpCode::SPUT_WIDE
            => format!("TODO {}", ins.opcode()),

        OpCode::CONST_HIGH16 | OpCode::CONST_WIDE_HIGH16
            => format!("TODO {}", ins.opcode()),

        OpCode::CONST_16 | OpCode::CONST_WIDE_16
            => format!("TODO {}", ins.opcode()),

        OpCode::IF_EQZ       | OpCode::IF_GEZ
            | OpCode::IF_GTZ | OpCode::IF_LEZ
            | OpCode::IF_LTZ | OpCode::IF_NEZ
            => format!("{} v{} +{}",
                        ins.opcode(),
                        ins.a(&ins.bytes()).unwrap(),
                        ins.b(&ins.bytes()).unwrap() * 2),

        OpCode::ADD_INT_LIT8        | OpCode::AND_INT_LIT8
            | OpCode::DIV_INT_LIT8  | OpCode::MUL_INT_LIT8
            | OpCode::OR_INT_LIT8   | OpCode::REM_INT_LIT8
            | OpCode::RSUB_INT_LIT8 | OpCode::SHL_INT_LIT8
            | OpCode::SHR_INT_LIT8  | OpCode::USHR_INT_LIT8
            | OpCode::XOR_INT_LIT8
            => format!("TODO {}", ins.opcode()),

        OpCode::IGET_BOOLEAN       | OpCode::IGET_BYTE
            | OpCode::IGET_CHAR    | OpCode::IGET
            | OpCode::IGET_OBJECT  | OpCode::IGET_SHORT
            | OpCode::IGET_WIDE    | OpCode::INSTANCE_OF
            | OpCode::IPUT_BOOLEAN | OpCode::IPUT_BYTE
            | OpCode::IPUT_CHAR    | OpCode::IPUT
            | OpCode::IPUT_OBJECT  | OpCode::IPUT_SHORT
            | OpCode::IPUT_WIDE    | OpCode::NEW_ARRAY
            => format!("TODO {}", ins.opcode()),

        OpCode::ADD_INT_LIT16       | OpCode::AND_INT_LIT16
            | OpCode::DIV_INT_LIT16 | OpCode::MUL_INT_LIT16
            | OpCode::OR_INT_LIT16  | OpCode::REM_INT_LIT16
            | OpCode::RSUB_INT      | OpCode::XOR_INT_LIT16
            => format!("TODO {}", ins.opcode()),

        OpCode::IF_EQ       | OpCode::IF_GE
            | OpCode::IF_GT | OpCode::IF_LE
            | OpCode::IF_LT | OpCode::IF_NE
            => format!("{} v{} v{} +{}",
                        ins.opcode(),
                        ins.a(&ins.bytes()).unwrap(),
                        ins.b(&ins.bytes()).unwrap(),
                        ins.c(&ins.bytes()).unwrap() * 2),

        OpCode::MOVE_FROM16 | OpCode::MOVE_OBJECT_FROM16
            | OpCode::MOVE_WIDE_FROM16
            => format!("TODO {}", ins.opcode()),

        OpCode::ADD_DOUBLE         | OpCode::ADD_FLOAT
            | OpCode::ADD_INT      | OpCode::ADD_LONG
            | OpCode::AGET_BOOLEAN | OpCode::AGET_BYTE
            | OpCode::AGET_CHAR    | OpCode::AGET
            | OpCode::AGET_OBJECT  | OpCode::AGET_SHORT
            | OpCode::AGET_WIDE    | OpCode::AND_INT
            | OpCode::AND_LONG     | OpCode::APUT_BOOLEAN
            | OpCode::APUT_BYTE    | OpCode::APUT_CHAR
            | OpCode::APUT         | OpCode::APUT_OBJECT
            | OpCode::APUT_SHORT   | OpCode::APUT_WIDE
            | OpCode::CMPG_DOUBLE  | OpCode::CMPG_FLOAT
            | OpCode::CMPL_DOUBLE  | OpCode::CMPL_FLOAT
            | OpCode::CMP_LONG     | OpCode::DIV_DOUBLE
            | OpCode::DIV_FLOAT    | OpCode::DIV_INT
            | OpCode::DIV_LONG     | OpCode::MUL_DOUBLE
            | OpCode::MUL_FLOAT    | OpCode::MUL_INT
            | OpCode::MUL_LONG     | OpCode::OR_INT
            | OpCode::OR_LONG      | OpCode::REM_DOUBLE
            | OpCode::REM_FLOAT    | OpCode::REM_INT
            | OpCode::REM_LONG     | OpCode::SHL_INT
            | OpCode::SHL_LONG     | OpCode::SHR_INT
            | OpCode::SHR_LONG     | OpCode::SUB_DOUBLE
            | OpCode::SUB_FLOAT    | OpCode::SUB_INT
            | OpCode::SUB_LONG     | OpCode::USHR_INT
            | OpCode::USHR_LONG    | OpCode::XOR_INT
            | OpCode::XOR_LONG
            => format!("TODO {}", ins.opcode()),

        OpCode::CONST_STRING_JUMBO => format!("TODO {}", ins.opcode()),

        OpCode::CONST | OpCode::CONST_WIDE_32
            => format!("TODO {}", ins.opcode()),

        OpCode::FILL_ARRAY_DATA | OpCode::PACKED_SWITCH
            | OpCode::SPARSE_SWITCH => format!("TODO {}", ins.opcode()),

        OpCode::MOVE_16 | OpCode::MOVE_OBJECT_16
            | OpCode::MOVE_WIDE_16 => format!("TODO {}", ins.opcode()),

        OpCode::FILLED_NEW_ARRAY    | OpCode::INVOKE_CUSTOM
            | OpCode::INVOKE_DIRECT | OpCode::INVOKE_INTERFACE
            | OpCode::INVOKE_STATIC | OpCode::INVOKE_SUPER
            | OpCode::INVOKE_VIRTUAL
            => format!("TODO {}", ins.opcode()),

        OpCode::FILLED_NEW_ARRAY_RANGE    | OpCode::INVOKE_CUSTOM_RANGE
            | OpCode::INVOKE_DIRECT_RANGE | OpCode::INVOKE_INTERFACE_RANGE
            | OpCode::INVOKE_STATIC_RANGE | OpCode::INVOKE_SUPER_RANGE
            | OpCode::INVOKE_VIRTUAL_RANGE
            => format!("TODO {}", ins.opcode()),

        OpCode::INVOKE_POLYMORPHIC => format!("TODO {}", ins.opcode()),

        OpCode::INVOKE_POLYMORPHIC_RANGE => format!("TODO {}", ins.opcode()),

        OpCode::CONST_WIDE => format!("TODO {}", ins.opcode()),

        OpCode::PACKED_SWITCH_PAYLOAD
            | OpCode::SPARSE_SWITCH_PAYLOAD
            | OpCode::FILL_ARRAY_DATA_PAYLOAD => format!("TODO {}", ins.opcode())
    }
}
