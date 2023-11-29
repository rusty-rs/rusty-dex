use crate::dex_strings::DexStrings;
use crate::dex_types::DexTypes;
use crate::dex_fields::DexFields;
use crate::dex_methods::DexMethods;

use crate::instructions_new::*;
use crate::opcodes::OpCode;
use crate::error;

pub fn disasm_ins(ins: &Instructions,
                  strings: &DexStrings,
                  types: &DexTypes,
                  fields: &DexFields,
                  methods: &DexMethods) -> String {
    "foo".to_string()
    /*
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
                let string = &strings.strings[ins.b(ins.bytes()).unwrap() as usize];
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

        OpCode::INVOKE_DIRECT_RANGE        | OpCode::INVOKE_INTERFACE_RANGE
            | OpCode::INVOKE_STATIC_RANGE  | OpCode::INVOKE_SUPER_RANGE
            | OpCode::INVOKE_VIRTUAL_RANGE | OpCode::FILLED_NEW_ARRAY_RANGE
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

        OpCode::INVOKE_POLYMORPHIC => todo!("{}", ins.opcode()),

        OpCode::INVOKE_POLYMORPHIC_RANGE => todo!("{}", ins.opcode()),

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

        /* Present in DEX files from version 038 onwards */
        OpCode::INVOKE_CUSTOM
            => todo!("{}", ins.opcode()),

        OpCode::INVOKE_CUSTOM_RANGE
            => todo!("{}", ins.opcode()),

        /* Present in DEX files from version 039 onwards */
        OpCode::CONST_METHOD_HANDLE
            => todo!("{}", ins.opcode()),

        OpCode::CONST_METHOD_TYPE
            => todo!("{}", ins.opcode()),

    }
    */
}


fn disasm_ins_10t(ins: &Instruction10t) -> String {
    format!("{} +{}", ins.opcode, ins.a())
}

fn disasm_ins_10x(ins: &Instruction10x) -> String {
    format!("{}", ins.opcode)
}

fn disasm_ins_11n(ins: &Instruction11n) -> String {
    format!("{} v{} #+{}", ins.opcode, ins.a(), ins.b())
}

fn disasm_ins_11x(ins: &Instruction11x) -> String {
    format!("{} v{}", ins.opcode, ins.a())
}

fn disasm_ins_12x(ins: &Instruction12x) -> String {
    format!("{} v{} v{}", ins.opcode, ins.a(), ins.b())
}

fn disasm_ins_20t(ins: &Instruction20t) -> String {
    format!("{} +{}", ins.opcode, ins.a())
}

fn disasm_ins_21c(ins: &Instruction21c,
                  strings: &DexStrings,
                  types: &DexTypes,
                  fields: &DexFields) -> String {
    match ins.opcode {
        OpCode::CHECK_CAST
            | OpCode::CONST_CLASS
            | OpCode::NEW_INSTANCE => {
            let class_name = &types.items[ins.b() as usize];
            format!("{} v{} {}", ins.opcode, ins.a(), class_name)
        },
        OpCode::CONST_METHOD_HANDLE => todo!("{}", ins.opcode),
        OpCode::CONST_METHOD_TYPE => todo!("{}", ins.opcode),
        OpCode::CONST_STRING => {
            let string = &strings.strings[ins.b() as usize];
            format!("{} v{} \"{}\"", ins.opcode, ins.a(), string)
        },

        OpCode::SGET
            | OpCode::SGET_WIDE
            | OpCode::SGET_OBJECT
            | OpCode::SGET_BOOLEAN
            | OpCode::SGET_BYTE
            | OpCode::SGET_CHAR
            | OpCode::SGET_SHORT
            | OpCode::SPUT
            | OpCode::SPUT_WIDE
            | OpCode::SPUT_OBJECT
            | OpCode::SPUT_BOOLEAN
            | OpCode::SPUT_BYTE
            | OpCode::SPUT_CHAR
            | OpCode::SPUT_SHORT => {
            let field = &fields.items[ins.b() as usize];
            format!("{} v{} \"{}\"", ins.opcode, ins.a(), field)
        },

        _ => {
            panic!("Invalid opcode for instruction 21c: {}", ins.opcode);
        }
    }
}

fn disasm_ins_21h(ins: &Instruction21h) -> String {
    match ins.opcode {
        OpCode::CONST_HIGH16 => {
            format!("{} v{} #+{}0000", ins.opcode, ins.a(), ins.b())
        },
        OpCode::CONST_WIDE_HIGH16 => {
            format!("{} v{} #+{}000000000000", ins.opcode, ins.a(), ins.b())
        },
        _ => {
            panic!("Invalid opcode for instruction 21h: {}", ins.opcode);
        }
    }
}

fn disasm_ins_21s(ins: &Instruction21s) -> String {
    format!("{} v{} #+{}", ins.opcode, ins.a(), ins.b())
}

fn disasm_ins_21t(ins: &Instruction21t) -> String {
    format!("{} v{} +{}", ins.opcode, ins.a(), ins.b())
}

fn disasm_ins_22b(ins: &Instruction22b) -> String {
    format!("{} v{} v{} #+{}", ins.opcode, ins.a(), ins.b(), ins.c())
}

fn disasm_ins_22c(ins: &Instruction22c,
                  types: &DexTypes,
                  fields: &DexFields) -> String {
    match ins.opcode {
        OpCode::IGET
            | OpCode::IGET_WIDE
            | OpCode::IGET_OBJECT
            | OpCode::IGET_BOOLEAN
            | OpCode::IGET_BYTE
            | OpCode::IGET_CHAR
            | OpCode::IGET_SHORT
            | OpCode::IPUT
            | OpCode::IPUT_WIDE
            | OpCode::IPUT_OBJECT
            | OpCode::IPUT_BOOLEAN
            | OpCode::IPUT_BYTE
            | OpCode::IPUT_CHAR
            | OpCode::IPUT_SHORT => {
            let field = &fields.items[ins.b() as usize];
            format!("{} v{} {}", ins.opcode, ins.a(), field)
        },
        OpCode::INSTANCE_OF | OpCode::NEW_ARRAY => {
            let class_name = &types.items[ins.b() as usize];
            format!("{} v{} {}", ins.opcode, ins.a(), class_name)
        }
        _ => {
            panic!("Invalid opcode for instruction 22c: {}", ins.opcode);
        }
    }
}

fn disasm_ins_22s(ins: &Instruction22s) -> String {
    format!("{} v{} v{} #+{}", ins.opcode, ins.a(), ins.b(), ins.c())
}

fn disasm_ins_22t(ins: &Instruction22t) -> String {
    format!("{} v{} v{} +{}", ins.opcode, ins.a(), ins.b(), ins.c())
}

fn disasm_ins_22x(ins: &Instruction22x) -> String {
    format!("{} v{} v{}", ins.opcode, ins.a(), ins.b())
}

fn disasm_ins_23x(ins: &Instruction23x) -> String {
    format!("{} v{} v{} v{}", ins.opcode, ins.a(), ins.b(), ins.c())
}

fn disasm_ins_30t(ins: &Instruction30t) -> String {
    format!("{} +{}", ins.opcode, ins.a())
}

fn disasm_ins_31c(ins: &Instruction31c, strings: &DexStrings) -> String {
    let string = &strings.strings[ins.b() as usize];
    format!("{} v{} \"{}\"", ins.opcode, ins.a(), string)
}

fn disasm_ins_31i(ins: &Instruction31i) -> String {
    format!("{} v{} #+{}", ins.opcode, ins.a(), ins.b())
}

fn disasm_ins_31t(ins: &Instruction31t) -> String {
    format!("{} v{} +{}", ins.opcode, ins.a(), ins.b())
}

fn disasm_ins_32x(ins: &Instruction32x) -> String {
    format!("{} v{} v{}", ins.opcode, ins.a(), ins.b())
}

fn disasm_ins_35c(ins: &Instruction35c,
                  types: &DexTypes,
                  methods: &DexMethods) -> String {
    let mut arg_regs = Vec::new();
    if ins.a() >= 1 {
        arg_regs.push(format!("v{}", ins.c()));
    }
    if ins.a() >= 2 {
        arg_regs.push(format!("v{}", ins.d()));
    }
    if ins.a() >= 3 {
        arg_regs.push(format!("v{}", ins.e()));
    }
    if ins.a() >= 4 {
        arg_regs.push(format!("v{}", ins.f()));
    }
    if ins.a() == 5 {
        arg_regs.push(format!("v{}", ins.g()));
    }

    match ins.opcode {
        OpCode::FILLED_NEW_ARRAY => {
            let type_name = &types.items[ins.b() as usize];
            format!("{} {{{}}} {}", ins.opcode, arg_regs.join(","), type_name)
        },
        OpCode::INVOKE_VIRTUAL
            | OpCode::INVOKE_SUPER
            | OpCode::INVOKE_DIRECT
            | OpCode::INVOKE_STATIC
            | OpCode::INVOKE_INTERFACE => {
            let meth_name = &methods.items[ins.b() as usize];
            format!("{} {{{}}} {}", ins.opcode, arg_regs.join(","), meth_name)
        },
        OpCode::INVOKE_CUSTOM => {
            todo!("{}", ins.opcode)
        },
        _ => {
            panic!("Invalid opcode for instruction 35c: {}", ins.opcode);
        }
    }
}

fn disasm_ins_3rc(ins: &Instruction3rc,
                  types: &DexTypes,
                  methods: &DexMethods) -> String {
    let mut arg_regs = Vec::new();
    for i in ins.c()..(ins.a() as u16 + ins.c() - 1) {
        arg_regs.push(format!("v{}", ins.c() + i));
    }

    match ins.opcode {
        OpCode::FILLED_NEW_ARRAY_RANGE => {
            let type_name = &types.items[ins.b() as usize];
            format!("{} {{{}}} {}", ins.opcode, arg_regs.join(","), type_name)
        },
        OpCode::INVOKE_VIRTUAL_RANGE
            | OpCode::INVOKE_SUPER_RANGE
            | OpCode::INVOKE_DIRECT_RANGE
            | OpCode::INVOKE_STATIC_RANGE
            | OpCode::INVOKE_INTERFACE_RANGE => {
            let method = &methods.items[ins.b() as usize];
            format!("{} {{{}}} {}", ins.opcode, arg_regs.join(","), method)
        },
        OpCode::INVOKE_CUSTOM_RANGE => {
            todo!("{}", ins.opcode)
        },
        _ => {
            panic!("Invalid opcode for instruction 3rc: {}", ins.opcode);
        }
    }
}

fn disasm_ins_45cc(ins: &Instruction45cc) -> String {
    todo!("{}", ins.opcode)
}

fn disasm_ins_4rcc(ins: &Instruction4rcc) -> String {
    todo!("{}", ins.opcode)
}

fn disasm_ins_51l(ins: &Instruction51l) -> String {
    format!("{} v{} #+{}", ins.opcode, ins.a(), ins.b())
}

pub fn disasm_ins_new(instructions: &Instructions,
                      strings: &DexStrings,
                      types: &DexTypes,
                      fields: &DexFields,
                      methods: &DexMethods) -> String {
    // for ins in instructions.iter() {
        match instructions {
            Instructions::Instruction10t(ins) => disasm_ins_10t(&ins),
            Instructions::Instruction10x(ins) => disasm_ins_10x(&ins),
            Instructions::Instruction11n(ins) => disasm_ins_11n(&ins),
            Instructions::Instruction11x(ins) => disasm_ins_11x(&ins),
            Instructions::Instruction12x(ins) => disasm_ins_12x(&ins),
            Instructions::Instruction20t(ins) => disasm_ins_20t(&ins),
            Instructions::Instruction21c(ins) => disasm_ins_21c(&ins, &strings, &types, &fields),
            Instructions::Instruction21h(ins) => disasm_ins_21h(&ins),
            Instructions::Instruction21s(ins) => disasm_ins_21s(&ins),
            Instructions::Instruction21t(ins) => disasm_ins_21t(&ins),
            Instructions::Instruction22b(ins) => disasm_ins_22b(&ins),
            Instructions::Instruction22c(ins) => disasm_ins_22c(&ins, &types, &fields),
            Instructions::Instruction22s(ins) => disasm_ins_22s(&ins),
            Instructions::Instruction22t(ins) => disasm_ins_22t(&ins),
            Instructions::Instruction22x(ins) => disasm_ins_22x(&ins),
            Instructions::Instruction23x(ins) => disasm_ins_23x(&ins),
            Instructions::Instruction30t(ins) => disasm_ins_30t(&ins),
            Instructions::Instruction31c(ins) => disasm_ins_31c(&ins, &strings),
            Instructions::Instruction31i(ins) => disasm_ins_31i(&ins),
            Instructions::Instruction31t(ins) => disasm_ins_31t(&ins),
            Instructions::Instruction32x(ins) => disasm_ins_32x(&ins),
            Instructions::Instruction35c(ins) => disasm_ins_35c(&ins, &types, &methods),
            Instructions::Instruction3rc(ins) => disasm_ins_3rc(&ins, &types, &methods),
            Instructions::Instruction45cc(ins) => disasm_ins_45cc(&ins),
            Instructions::Instruction4rcc(ins) => disasm_ins_4rcc(&ins),
            Instructions::Instruction51l(ins) => disasm_ins_51l(&ins),
            Instructions::PackedSwitchPayload(inst) => String::from("TODO PACKED_SWITCH_PAYLOAD"),
            Instructions::SparseSwitchPayload(inst) => String::from("TODO SPARSE_SWITCH_PAYLOAD"),
            Instructions::FillArrayDataPayload(inst) => String::from("TODO FILL_ARRAY_DATA_PAYLOAD"),
        }
    // }
}
