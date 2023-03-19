use crate::dex_strings::DexStrings;
use crate::dex_types::DexTypes;
use crate::dex_protos::DexProtos;
use crate::dex_fields::DexFields;
use crate::dex_classes::DexClasses;
use crate::dex_methods::DexMethods;

use crate::instructions::InstructionHandler;
use crate::opcodes::OpCode;
use crate::warning;

pub fn disasm_ins(ins: &(impl InstructionHandler + ?Sized),
                  strings: &DexStrings,
                  types: &DexTypes,
                  fields: &DexFields,
                  protos: &DexProtos,
                  methods: &DexMethods,
                  class_defs: &DexClasses) -> String {
    match ins.opcode() {
        OpCode::GOTO | OpCode::GOTO_16
            | OpCode::GOTO_32 => format!("{} +{}",
                                            ins.opcode(),
                                            ins.a(ins.bytes()).unwrap() * 2),

        OpCode::NOP => String::new(),
        OpCode::RETURN_VOID => format!("{}", ins.opcode()),

        OpCode::CONST               | OpCode::CONST_4
            | OpCode::CONST_16      | OpCode::CONST_HIGH16
            | OpCode::CONST_WIDE    | OpCode::CONST_WIDE_16
            | OpCode::CONST_WIDE_32 | OpCode::CONST_WIDE_HIGH16
            => format!("{} v{} {}", ins.opcode(),
                                    ins.a(ins.bytes()).unwrap(),
                                    ins.b(ins.bytes()).unwrap()),

        OpCode::MONITOR_ENTER            | OpCode::MONITOR_EXIT
            | OpCode::MOVE_EXCEPTION     | OpCode::MOVE_RESULT
            | OpCode::MOVE_RESULT_OBJECT | OpCode::MOVE_RESULT_WIDE
            | OpCode::RETURN             | OpCode::RETURN_OBJECT
            | OpCode::RETURN_WIDE        | OpCode::THROW
            => format!("{} v{}",
                        ins.opcode(),
                        ins.a(ins.bytes()).unwrap()),

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
            | OpCode::MOVE_WIDE       | OpCode::MOVE_OBJECT_FROM16
            | OpCode::MOVE_FROM16     | OpCode::MOVE_WIDE_FROM16
            | OpCode::MOVE_16         | OpCode::MOVE_OBJECT_16
            | OpCode::MOVE_WIDE_16    | OpCode::MUL_DOUBLE_2ADDR
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
                        ins.a(ins.bytes()).unwrap(),
                        ins.b(ins.bytes()).unwrap()),

        OpCode::CHECK_CAST                | OpCode::CONST_CLASS
            | OpCode::NEW_INSTANCE => {
                let class_name = &types.items[ins.b(ins.bytes()).unwrap() as usize];
                format!("{} v{} {}",
                        ins.opcode(),
                        ins.a(ins.bytes()).unwrap(),
                        class_name)
            },

        OpCode::SGET_BOOLEAN              | OpCode::SGET_BYTE
            | OpCode::SGET_CHAR           | OpCode::SGET
            | OpCode::SGET_OBJECT         | OpCode::SGET_SHORT
            | OpCode::SGET_WIDE           | OpCode::SPUT_BOOLEAN
            | OpCode::SPUT_BYTE           | OpCode::SPUT_CHAR
            | OpCode::SPUT                | OpCode::SPUT_OBJECT
            | OpCode::SPUT_SHORT          | OpCode::SPUT_WIDE
            => {
                let field = &fields.items[ins.b(ins.bytes()).unwrap() as usize];
                format!("{} v{} {}",
                       ins.opcode(),
                       ins.a(ins.bytes()).unwrap(),
                       field)
            },

        OpCode::IF_EQZ       | OpCode::IF_GEZ
            | OpCode::IF_GTZ | OpCode::IF_LEZ
            | OpCode::IF_LTZ | OpCode::IF_NEZ
            => format!("{} v{} +{}",
                        ins.opcode(),
                        ins.a(ins.bytes()).unwrap(),
                        ins.b(ins.bytes()).unwrap() * 2),
                        // TODO: b() seems wrong, check shifts

        OpCode::ADD_INT_LIT8        | OpCode::AND_INT_LIT8
            | OpCode::DIV_INT_LIT8  | OpCode::MUL_INT_LIT8
            | OpCode::OR_INT_LIT8   | OpCode::REM_INT_LIT8
            | OpCode::RSUB_INT_LIT8 | OpCode::SHL_INT_LIT8
            | OpCode::SHR_INT_LIT8  | OpCode::USHR_INT_LIT8
            | OpCode::XOR_INT_LIT8  | OpCode::ADD_INT_LIT16
            | OpCode::AND_INT_LIT16 | OpCode::DIV_INT_LIT16
            | OpCode::MUL_INT_LIT16 | OpCode::OR_INT_LIT16
            | OpCode::REM_INT_LIT16 | OpCode::RSUB_INT
            | OpCode::XOR_INT_LIT16
            => format!("{} v{} v{} #+{}",
                       ins.opcode(),
                        ins.a(ins.bytes()).unwrap(),
                        ins.b(ins.bytes()).unwrap(),
                        ins.c(ins.bytes()).unwrap()),

        OpCode::IGET_BOOLEAN       | OpCode::IGET_BYTE
            | OpCode::IGET_CHAR    | OpCode::IGET
            | OpCode::IGET_OBJECT  | OpCode::IGET_SHORT
            | OpCode::IGET_WIDE    | OpCode::INSTANCE_OF
            | OpCode::IPUT_BOOLEAN | OpCode::IPUT_BYTE
            | OpCode::IPUT_CHAR    | OpCode::IPUT
            | OpCode::IPUT_OBJECT  | OpCode::IPUT_SHORT
            | OpCode::IPUT_WIDE
            => {
                let field = &fields.items[ins.c(ins.bytes()).unwrap() as usize];
                format!("{} v{} v{} {}",
                       ins.opcode(),
                       ins.a(ins.bytes()).unwrap(),
                       ins.b(ins.bytes()).unwrap(),
                       field)
            }

        OpCode::NEW_ARRAY
            => {
                let array_type = &types.items[ins.b(ins.bytes()).unwrap() as usize];
                format!("{} v{} v{} {}",
                        ins.opcode(),
                        ins.a(ins.bytes()).unwrap(),
                        ins.b(ins.bytes()).unwrap(),
                        array_type)
            },

        OpCode::IF_EQ       | OpCode::IF_GE
            | OpCode::IF_GT | OpCode::IF_LE
            | OpCode::IF_LT | OpCode::IF_NE
            => format!("{} v{} v{} +{}",
                        ins.opcode(),
                        ins.a(ins.bytes()).unwrap(),
                        ins.b(ins.bytes()).unwrap(),
                        ins.c(ins.bytes()).unwrap() * 2),

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
            => format!("{} v{} v{} v{}",
                       ins.opcode(),
                        ins.a(ins.bytes()).unwrap(),
                        ins.b(ins.bytes()).unwrap(),
                        ins.c(ins.bytes()).unwrap()),

        OpCode::CONST_STRING | OpCode::CONST_STRING_JUMBO
            => {
                let string = &strings.strings[ins.b(ins.bytes()).unwrap() as usize].string;
                format!("{} v{} \"{}\"",
                        ins.opcode(),
                        ins.a(ins.bytes()).unwrap(),
                        string)
            },

        OpCode::FILL_ARRAY_DATA | OpCode::PACKED_SWITCH
            | OpCode::SPARSE_SWITCH
            => format!("{} v{} +{}",
                       ins.opcode(),
                       ins.a(ins.bytes()).unwrap(),
                       ins.b(ins.bytes()).unwrap() * 2),

        OpCode::INVOKE_DIRECT       | OpCode::INVOKE_INTERFACE
            | OpCode::INVOKE_STATIC | OpCode::INVOKE_SUPER
            | OpCode::INVOKE_VIRTUAL
            => {
                let proto = &methods.items[ins.b(ins.bytes()).unwrap() as usize];
                let args = match ins.a(ins.bytes()).unwrap() {
                    0 => String::from(""),
                    1 => format!("v{}", ins.c(ins.bytes()).unwrap()),
                    2 => format!("v{} v{}",
                                 ins.c(ins.bytes()).unwrap(),
                                 ins.d(ins.bytes()).unwrap()),
                    3 => format!("v{} v{} v{}",
                                 ins.c(ins.bytes()).unwrap(),
                                 ins.d(ins.bytes()).unwrap(),
                                 ins.e(ins.bytes()).unwrap()),
                    4 => format!("v{} v{} v{} v{}",
                                 ins.c(ins.bytes()).unwrap(),
                                 ins.d(ins.bytes()).unwrap(),
                                 ins.e(ins.bytes()).unwrap(),
                                 ins.f(ins.bytes()).unwrap()),
                    5 => format!("v{} v{} v{} v{} v{}",
                                 ins.c(ins.bytes()).unwrap(),
                                 ins.d(ins.bytes()).unwrap(),
                                 ins.e(ins.bytes()).unwrap(),
                                 ins.f(ins.bytes()).unwrap(),
                                 ins.g(ins.bytes()).unwrap()),
                    _ => {
                        warning!("invalid args count in invoke-* instruction");
                        warning!("computed count is: {}", ins.a(ins.bytes()).unwrap());
                        warning!("defaulting to empty string");
                        String::from("")
                    }
                };

                format!("{} {} {}", ins.opcode(), args, proto)
            }

        OpCode::INVOKE_DIRECT_RANGE       | OpCode::INVOKE_INTERFACE_RANGE
            | OpCode::INVOKE_STATIC_RANGE | OpCode::INVOKE_SUPER_RANGE
            | OpCode::INVOKE_VIRTUAL_RANGE
            => {
                let proto = &methods.items[ins.b(ins.bytes()).unwrap() as usize];
                let mut first_reg = ins.c(ins.bytes()).unwrap();
                let mut args = format!("v{first_reg} ");
                for _ in 1..ins.a(ins.bytes()).unwrap() {
                    first_reg += 1;
                    args.push('v');
                    args.push_str(&first_reg.to_string());
                    args.push(' ');
                }
                format!("{} {}{}", ins.opcode(), args, proto)
            },

        OpCode::INVOKE_POLYMORPHIC => todo!("TODO {}", ins.opcode()),

        OpCode::INVOKE_POLYMORPHIC_RANGE => todo!("TODO {}", ins.opcode()),

        OpCode::PACKED_SWITCH_PAYLOAD
            | OpCode::SPARSE_SWITCH_PAYLOAD
            | OpCode::FILL_ARRAY_DATA_PAYLOAD => format!("TODO {}", ins.opcode()),

        OpCode::FILLED_NEW_ARRAY
            => {
                let array_type = &types.items[ins.b(ins.bytes()).unwrap() as usize];
                let args = match ins.a(ins.bytes()).unwrap() {
                    0 => String::from(""),
                    1 => format!("v{}", ins.c(ins.bytes()).unwrap()),
                    2 => format!("v{} v{}",
                                 ins.c(ins.bytes()).unwrap(),
                                 ins.d(ins.bytes()).unwrap()),
                    3 => format!("v{} v{} v{}",
                                 ins.c(ins.bytes()).unwrap(),
                                 ins.d(ins.bytes()).unwrap(),
                                 ins.e(ins.bytes()).unwrap()),
                    4 => format!("v{} v{} v{} v{}",
                                 ins.c(ins.bytes()).unwrap(),
                                 ins.d(ins.bytes()).unwrap(),
                                 ins.e(ins.bytes()).unwrap(),
                                 ins.f(ins.bytes()).unwrap()),
                    5 => format!("v{} v{} v{} v{} v{}",
                                 ins.c(ins.bytes()).unwrap(),
                                 ins.d(ins.bytes()).unwrap(),
                                 ins.e(ins.bytes()).unwrap(),
                                 ins.f(ins.bytes()).unwrap(),
                                 ins.g(ins.bytes()).unwrap()),
                    _ => {
                        warning!("invalid args count in invoke-* instruction");
                        warning!("computed count is: {}", ins.a(ins.bytes()).unwrap());
                        warning!("defaulting to empty string");
                        String::from("")
                    }
                };

                format!("{} {} {}", ins.opcode(), args, array_type)
            }

        OpCode::FILLED_NEW_ARRAY_RANGE
            => todo!("TODO {}", ins.opcode()),

        /* Present in DEX files from version 038 onwards */
        OpCode::INVOKE_CUSTOM
            => todo!("TODO {}", ins.opcode()),

        OpCode::INVOKE_CUSTOM_RANGE
            => todo!("TODO {}", ins.opcode()),

        /* Present in DEX files from version 039 onwards */
        OpCode::CONST_METHOD_HANDLE
            => todo!("TODO {}", ins.opcode()),

        OpCode::CONST_METHOD_TYPE
            => todo!("TODO {}", ins.opcode()),

    }
}
