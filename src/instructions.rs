use core::fmt::Debug;

use crate::error;
use crate::opcodes::OpCode;
use crate::dex_reader::DexEndianness;

#[derive(Debug)]
pub struct Instruction
{
    pub bytes: Vec<u16>,
    pub opcode: OpCode,
    pub handler: Box<dyn InstructionHandler>,
}

// TODO we might want to create a cursor over the bytes here, that
// would simplify the reading for the special opcodes at the end
impl Instruction
{
    pub fn parse(raw_bytes: &[u16],
                 offset: usize,
                 endianness: &DexEndianness) -> Self
    {
        let bytes = raw_bytes.to_vec();

        // TODO: make this prettier
        assert!(offset <= bytes.len());

        let opcode = match OpCode::parse((bytes[offset] & 0xff).try_into().unwrap()) {
            // Deal with the special cases of fill-array-data-payload,
            // packed-switch-payload, and sparse-switch-payload
            // Some(OpCode::NOP) => match bytes[offset] & 0xff00 {
            Some(OpCode::NOP) => match bytes[offset] >> 8 {
                0x01 => OpCode::PACKED_SWITCH_PAYLOAD,
                0x02 => OpCode::SPARSE_SWITCH_PAYLOAD,
                0x03 => OpCode::FILL_ARRAY_DATA_PAYLOAD,
                _    => OpCode::NOP
            },
            Some(code) => code,
            None => panic!("Cannot parse instruction from: 0x{:X?}", bytes[offset] & 0xff)
        };

        let handler: Box<dyn InstructionHandler> = match opcode {
            OpCode::GOTO => Box::new(Instruction10t{ }),

            OpCode::NOP | OpCode::RETURN_VOID => Box::new(Instruction10x{ }),

            OpCode::CONST_4 => Box::new(Instruction11n{ }),

            OpCode::MONITOR_ENTER            | OpCode::MONITOR_EXIT
                | OpCode::MOVE_EXCEPTION     | OpCode::MOVE_RESULT 
                | OpCode::MOVE_RESULT_OBJECT | OpCode::MOVE_RESULT_WIDE
                | OpCode::RETURN             | OpCode::RETURN_OBJECT
                | OpCode::RETURN_WIDE        | OpCode::THROW
                => Box::new(Instruction11x{ }),

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
                => Box::new(Instruction12x{ }),

            OpCode::GOTO_16 => Box::new(Instruction20t{ }),

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
                => Box::new(Instruction21c{ }),

            OpCode::CONST_HIGH16 | OpCode::CONST_WIDE_HIGH16
                => Box::new(Instruction21h{ }),

            OpCode::CONST_16 | OpCode::CONST_WIDE_16
                => Box::new(Instruction21s{ }),

            OpCode::IF_EQZ       | OpCode::IF_GEZ
                | OpCode::IF_GTZ | OpCode::IF_LEZ
                | OpCode::IF_LTZ | OpCode::IF_NEZ
                => Box::new(Instruction21t{ }),

            OpCode::ADD_INT_LIT8        | OpCode::AND_INT_LIT8
                | OpCode::DIV_INT_LIT8  | OpCode::MUL_INT_LIT8
                | OpCode::OR_INT_LIT8   | OpCode::REM_INT_LIT8
                | OpCode::RSUB_INT_LIT8 | OpCode::SHL_INT_LIT8
                | OpCode::SHR_INT_LIT8  | OpCode::USHR_INT_LIT8
                | OpCode::XOR_INT_LIT8
                => Box::new(Instruction22b{ }),

            OpCode::IGET_BOOLEAN       | OpCode::IGET_BYTE
                | OpCode::IGET_CHAR    | OpCode::IGET
                | OpCode::IGET_OBJECT  | OpCode::IGET_SHORT
                | OpCode::IGET_WIDE    | OpCode::INSTANCE_OF
                | OpCode::IPUT_BOOLEAN | OpCode::IPUT_BYTE
                | OpCode::IPUT_CHAR    | OpCode::IPUT
                | OpCode::IPUT_OBJECT  | OpCode::IPUT_SHORT
                | OpCode::IPUT_WIDE    | OpCode::NEW_ARRAY
                => Box::new(Instruction22c{ }),

            OpCode::ADD_INT_LIT16       | OpCode::AND_INT_LIT16
                | OpCode::DIV_INT_LIT16 | OpCode::MUL_INT_LIT16
                | OpCode::OR_INT_LIT16  | OpCode::REM_INT_LIT16
                | OpCode::RSUB_INT      | OpCode::XOR_INT_LIT16
                => Box::new(Instruction22s{ }),

            OpCode::IF_EQ       | OpCode::IF_GE
                | OpCode::IF_GT | OpCode::IF_LE
                | OpCode::IF_LT | OpCode::IF_NE
                => Box::new(Instruction22t{ }),

            OpCode::MOVE_FROM16 | OpCode::MOVE_OBJECT_FROM16
                | OpCode::MOVE_WIDE_FROM16
                => Box::new(Instruction22x{ }),

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
                => Box::new(Instruction23x{ }),

            OpCode::GOTO_32 => Box::new(Instruction30t{ }),

            OpCode::CONST_STRING_JUMBO => Box::new(Instruction31c{ }),

            OpCode::CONST | OpCode::CONST_WIDE_32
                => Box::new(Instruction31i{ }),

            OpCode::FILL_ARRAY_DATA | OpCode::PACKED_SWITCH
                | OpCode::SPARSE_SWITCH => Box::new(Instruction31t{ }),

            OpCode::MOVE_16 | OpCode::MOVE_OBJECT_16
                | OpCode::MOVE_WIDE_16 => Box::new(Instruction32x{ }),

            OpCode::FILLED_NEW_ARRAY    | OpCode::INVOKE_CUSTOM
                | OpCode::INVOKE_DIRECT | OpCode::INVOKE_INTERFACE
                | OpCode::INVOKE_STATIC | OpCode::INVOKE_SUPER
                | OpCode::INVOKE_VIRTUAL
                => Box::new(Instruction35c{ }),

            OpCode::FILLED_NEW_ARRAY_RANGE    | OpCode::INVOKE_CUSTOM_RANGE
                | OpCode::INVOKE_DIRECT_RANGE | OpCode::INVOKE_INTERFACE_RANGE
                | OpCode::INVOKE_STATIC_RANGE | OpCode::INVOKE_SUPER_RANGE
                | OpCode::INVOKE_VIRTUAL_RANGE
                => Box::new(Instruction3rc{ }),

            OpCode::INVOKE_POLYMORPHIC => Box::new(Instruction45cc{ }),

            OpCode::INVOKE_POLYMORPHIC_RANGE => Box::new(Instruction4rcc{ }),

            OpCode::CONST_WIDE => Box::new(Instruction51l{ }),

            // TODO: refactor this shit
            OpCode::PACKED_SWITCH_PAYLOAD
                => Box::new(PackedSwitchPayload::build(&bytes, offset, endianness)),

            OpCode::SPARSE_SWITCH_PAYLOAD
                => Box::new(SparseSwitchPayload::build(&bytes, offset, endianness)),

            OpCode::FILL_ARRAY_DATA_PAYLOAD
                => Box::new(FillArrayDataPayload::build(&bytes, offset, endianness)),
        };

        let ins_bytes = Vec::from(&bytes[offset..offset + handler.length()]);
        Instruction {
            bytes: ins_bytes,
            opcode,
            handler,
        }
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }
}

struct Instruction10t;
struct Instruction10x;
struct Instruction11n;
struct Instruction11x;
struct Instruction12x;
struct Instruction20t;
struct Instruction21c;
struct Instruction21h;
struct Instruction21s;
struct Instruction21t;
struct Instruction22b;
struct Instruction22c;
struct Instruction22s;
struct Instruction22t;
struct Instruction22x;
struct Instruction23x;
struct Instruction30t;
struct Instruction31c;
struct Instruction31i;
struct Instruction31t;
struct Instruction32x;
struct Instruction35c;
struct Instruction3rc;
struct Instruction45cc;
struct Instruction4rcc;
struct Instruction51l;

#[allow(unused_variables)]
pub trait InstructionHandler {
    fn length(&self) -> usize;
    fn inst_format(&self) -> &str;

    /* Getters for registers
     * Each getter has a default implementation that prints an error message
     * and returns None. This is because not all instructions use registers.
     * With a default implementation we do not have to rewrite the same
     * code for each instruction types. The getters are overriden in the
     * instructions' respective implementations, if needed.
     */
    // TODO: u64 is the size of the largest possible arg, but is only used
    // const-wide. We could use smaller uints for some of these methods.
    fn a(&self, data: &[u16]) -> Option<u64> {
        error!("Attempt to access register vA from {} instruction", self.inst_format());
        None
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        error!("Attempt to access register vB from {} instruction", self.inst_format());
        None
    }

    fn c(&self, data: &[u16]) -> Option<u64> {
        error!("Attempt to access register vC from {} instruction", self.inst_format());
        None
    }

    fn d(&self, data: &[u16]) -> Option<u64> {
        error!("Attempt to access register vD from {} instruction", self.inst_format());
        None
    }

    fn e(&self, data: &[u16]) -> Option<u64> {
        error!("Attempt to access register vE from {} instruction", self.inst_format());
        None
    }

    fn f(&self, data: &[u16]) -> Option<u64> {
        error!("Attempt to access register vF from {} instruction", self.inst_format());
        None
    }

    fn g(&self, data: &[u16]) -> Option<u64> {
        error!("Attempt to access register vG from {} instruction", self.inst_format());
        None
    }

    fn h(&self, data: &[u16]) -> Option<u64> {
        error!("Attempt to access register vH from {} instruction", self.inst_format());
        None
    }
}

impl Debug for dyn InstructionHandler {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "InstructionHandler{{{}}}", self.inst_format())
    }
}

/// 00|op
impl InstructionHandler for Instruction10x {
    fn length(&self) -> usize {
        1
    }

    fn inst_format(&self) -> &str {
        "Instruction10x"
    }
}

/// B|A|op
impl InstructionHandler for Instruction11n {
    fn length(&self) -> usize {
        1
    }

    fn inst_format(&self) -> &str {
        "Instruction11n"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] & 0x0f00) as u64)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] & 0xf000) as u64)
    }
}

impl InstructionHandler for Instruction12x {
    fn length(&self) -> usize {
        1
    }

    fn inst_format(&self) -> &str {
        "Instruction12x"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] & 0x0f00) as u64)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] & 0xf000) as u64)
    }
}

/// AA|op
impl InstructionHandler for Instruction11x {
    fn length(&self) -> usize {
        1
    }

    fn inst_format(&self) -> &str {
        "Instruction11x"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }
}

impl InstructionHandler for Instruction10t {
    fn length(&self) -> usize {
        1
    }

    fn inst_format(&self) -> &str {
        "Instruction10t"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }
}

/// 00|op
/// AAAA
impl InstructionHandler for Instruction20t {
    fn length(&self) -> usize {
        2
    }

    fn inst_format(&self) -> &str {
        "Instruction20t"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }
}

/// AA|op
/// BBBB
impl InstructionHandler for Instruction21c {
    fn length(&self) -> usize {
        2
    }

    fn inst_format(&self) -> &str {
        "Instruction21c"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }
}

impl InstructionHandler for Instruction21h {
    fn length(&self) -> usize {
        2
    }

    fn inst_format(&self) -> &str {
        "Instruction21h"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }
}

impl InstructionHandler for Instruction21s {
    fn length(&self) -> usize {
        2
    }

    fn inst_format(&self) -> &str {
        "Instruction21s"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }
}

impl InstructionHandler for Instruction21t {
    fn length(&self) -> usize {
        2
    }

    fn inst_format(&self) -> &str {
        "Instruction21t"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }
}

impl InstructionHandler for Instruction22x {
    fn length(&self) -> usize {
        2
    }

    fn inst_format(&self) -> &str {
        "Instruction22x"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }
}

/// AA|op
/// BB|CC
impl InstructionHandler for Instruction23x {
    fn length(&self) -> usize {
        2
    }

    fn inst_format(&self) -> &str {
        "Instruction23x"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some((data[1] as u64) >> 8)
    }

    fn c(&self, data: &[u16]) -> Option<u64> {
        Some((data[1] & 0xff) as u64)
    }
}

impl InstructionHandler for Instruction22b {
    fn length(&self) -> usize {
        2
    }

    fn inst_format(&self) -> &str {
        "Instruction22b"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some((data[1] as u64) >> 8)
    }

    fn c(&self, data: &[u16]) -> Option<u64> {
        Some((data[1] & 0xff) as u64)
    }
}

/// B|A|op
/// CCCC
impl InstructionHandler for Instruction22c {
    fn length(&self) -> usize {
        2
    }

    fn inst_format(&self) -> &str {
        "Instruction22c"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] & 0x0f00) as u64)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] & 0xf000) as u64)
    }

    fn c(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }
}

impl InstructionHandler for Instruction22s {
    fn length(&self) -> usize {
        2
    }

    fn inst_format(&self) -> &str {
        "Instruction22s"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] & 0x0f00) as u64)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] & 0xf000) as u64)
    }

    fn c(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }
}

impl InstructionHandler for Instruction22t {
    fn length(&self) -> usize {
        2
    }

    fn inst_format(&self) -> &str {
        "Instruction22t"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] & 0x0f00) as u64)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] & 0xf000) as u64)
    }

    fn c(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }
}

/// 00|op
/// AAAAlow
/// AAAAhigh
impl InstructionHandler for Instruction30t {
    fn length(&self) -> usize {
        3
    }

    fn inst_format(&self) -> &str {
        "Instruction30t"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64 + (data [2] as u64) << 16)
    }
}

/// AA|op
/// BBBBlow
/// BBBBhigh
impl InstructionHandler for Instruction31c {
    fn length(&self) -> usize {
        3
    }

    fn inst_format(&self) -> &str {
        "Instruction31c"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64 + (data [2] as u64) << 16)
    }
}

impl InstructionHandler for Instruction31i {
    fn length(&self) -> usize {
        3
    }

    fn inst_format(&self) -> &str {
        "Instruction31i"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64 + (data [2] as u64) << 16)
    }
}

impl InstructionHandler for Instruction31t {
    fn length(&self) -> usize {
        3
    }

    fn inst_format(&self) -> &str {
        "Instruction31t"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64 + (data [2] as u64) << 16)
    }
}

/// 00|op
/// AAAA
/// BBBB
impl InstructionHandler for Instruction32x {
    fn length(&self) -> usize {
        3
    }

    fn inst_format(&self) -> &str {
        "Instruction32x"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[2] as u64)
    }
}

/// A|G|op
/// BBBB
/// F|E|D|C
impl InstructionHandler for Instruction35c {
    fn length(&self) -> usize {
        3
    }

    fn inst_format(&self) -> &str {
        "Instruction35c"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] & 0xf000) as u64)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }

    fn c(&self, data: &[u16]) -> Option<u64> {
        Some((data[2] & 0x000f) as u64)
    }

    fn d(&self, data: &[u16]) -> Option<u64> {
        Some((data[2] & 0x00f0) as u64)
    }

    fn e(&self, data: &[u16]) -> Option<u64> {
        Some((data[2] & 0x0f00) as u64)
    }

    fn f(&self, data: &[u16]) -> Option<u64> {
        Some((data[2] & 0xf000) as u64)
    }

    fn g(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] & 0x0f00) as u64)
    }
}

/// AA|op
/// BBBB
/// CCCC
impl InstructionHandler for Instruction3rc {
    fn length(&self) -> usize {
        3
    }

    fn inst_format(&self) -> &str {
        "Instruction3rc"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }

    fn c(&self, data: &[u16]) -> Option<u64> {
        Some(data[2] as u64)
    }
}

/// A|G|op
/// BBBB
/// F|E|D|C
/// HHHH
impl InstructionHandler for Instruction45cc {
    fn length(&self) -> usize {
        4
    }

    fn inst_format(&self) -> &str {
        "Instruction45cc"
    }

    fn h(&self, data: &[u16]) -> Option<u64> {
        Some(data[3] as u64)
    }
}

/// AA|op
/// BBBB
/// CCCC
/// HHHH
impl InstructionHandler for Instruction4rcc {
    fn length(&self) -> usize {
        4
    }

    fn inst_format(&self) -> &str {
        "Instruction4rcc"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }

    fn c(&self, data: &[u16]) -> Option<u64> {
        Some(data[2] as u64)
    }

    fn h(&self, data: &[u16]) -> Option<u64> {
        Some(data[3] as u64)
    }
}

/// AA|op
/// BBBBlow
/// BBBB
/// BBBB
/// BBBBhigh
impl InstructionHandler for Instruction51l {
    fn length(&self) -> usize {
        5
    }

    fn inst_format(&self) -> &str {
        "Instruction51l"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64
             + (data[2] as u64) << 16
             + (data[3] as u64) << 32
             + (data[4] as u64) << 48)
    }
}

/// Utility functions to read i32 from [u16]
fn read_i32(bytes: &[u16],
            offset: usize,
            endianness: &DexEndianness) -> i32
{
    match endianness {
        DexEndianness::LittleEndian =>
            ((bytes[offset + 1] as i32) << 16) + bytes[offset] as i32,
        DexEndianness::BigEndian =>
            ((bytes[offset] as i32) << 16) + bytes[offset + 1] as i32,
    }
}

struct PackedSwitchPayload {
    size: u16,
    first_key: i32,
    targets: Vec<i32>
}

impl PackedSwitchPayload {
    fn build(bytes: &[u16],
             offset: usize,
             endianness: &DexEndianness) -> Self 
    {
        let size = bytes[offset + 1];
        let first_key = read_i32(&bytes, offset + 2, &endianness);
        let mut targets = Vec::new();
        let mut ioffset = 2;
        for _ in 0..size {
            targets.push(read_i32(&bytes, offset + ioffset, &endianness));
            ioffset += 2;
        }

        PackedSwitchPayload {
            size,
            first_key,
            targets
        }
    }
}

impl InstructionHandler for PackedSwitchPayload {
    fn length(&self) -> usize {
        // nb of entries in bytes + size of (opcode and size)
        ((self.size * 2) + 4).into()
    }

    fn inst_format(&self) -> &str {
        "PackedSwitchPayload"
    }
}

struct SparseSwitchPayload {
    size: u16,
    keys: Vec<i32>,
    targets: Vec<i32>
}

impl SparseSwitchPayload {
    fn build(bytes: &[u16],
             offset: usize,
             endianness: &DexEndianness) -> Self 
    {
        let size = bytes[offset + 1];

        let mut keys = Vec::new();
        let mut ioffset = 2;
        for _ in 0..size {
            keys.push(read_i32(&bytes, offset + ioffset, &endianness));
            ioffset += 2;
        }

        let mut targets = Vec::new();
        let mut ioffset = 2;
        for _ in 0..size {
            targets.push(read_i32(&bytes, offset + ioffset, &endianness));
            ioffset += 2;
        }

        SparseSwitchPayload {
            size,
            keys,
            targets
        }
    }
}

impl InstructionHandler for SparseSwitchPayload {
    fn length(&self) -> usize {
        ((self.size * 4) + 2).into()
    }

    fn inst_format(&self) -> &str {
        "SparseSwitchPayload"
    }
}

struct FillArrayDataPayload {
    element_width: u16,
    size: u32,
    data: Vec<u8>
}

impl FillArrayDataPayload {
    fn build(bytes: &[u16],
             offset: usize,
             endianness: &DexEndianness) -> Self 
    {
        let element_width = bytes[offset + 1];
        let size = match endianness {
            DexEndianness::LittleEndian =>
                ((bytes[offset + 3] as u32) << 16) + bytes[offset + 2] as u32,
            DexEndianness::BigEndian =>
                ((bytes[offset + 2] as u32) << 16) + bytes[offset + 3] as u32,
        };
        let mut data = Vec::new();
        let mut ioffset = 4;
        match endianness {
            DexEndianness::LittleEndian => 
                for i in 0..(size / element_width as u32) / 2 { 
                    let _b = bytes[offset + ioffset].to_le_bytes();
                    data.push(_b[0]);
                    data.push(_b[1]);
                    ioffset += 1;
                },
            DexEndianness::BigEndian => 
                for _ in 0..(size / element_width as u32) / 2 { 
                    let _b = bytes[offset + ioffset].to_be_bytes();
                    data.push(_b[0]);
                    data.push(_b[1]);
                    ioffset += 1;
                },
        };

        FillArrayDataPayload {
            element_width,
            size,
            data
        }
    }
}

impl InstructionHandler for FillArrayDataPayload {
    fn length(&self) -> usize {
        ((self.size * self.element_width as u32 + 1) / 2 + 4) as usize
    }

    fn inst_format(&self) -> &str {
        "FillArrayDataPayload"
    }
}

#[derive(Debug)]
pub struct InstructionsReader<'a>
{
    pub bytes: &'a [u16],
    offset: usize,
    length: usize,
    endianness: &'a DexEndianness,
}

impl<'a> InstructionsReader<'a> {
    pub fn new(bytes: &'a [u16], endianness: &'a DexEndianness) -> Self {
        InstructionsReader {
            bytes,
            offset: 0,
            length: bytes.len(),
            endianness
        }
    }

    fn update_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    /// Parses an instruction from the bytecode and move the cursor parser
    pub fn parse_instructions(&mut self) -> Option<Vec<Instruction>> {
        let mut instructions: Vec<Instruction> = Vec::new();

        while self.offset < self.length {
            let ins = Instruction::parse(self.bytes, self.offset, self.endianness);
            println!("{0} {ins:?}", 2 * self.offset);
            self.offset += ins.handler.length();
            instructions.push(ins);
        }

        // TODO: refactor this
        assert!(self.offset == self.length);

        match instructions.len() {
            0 => None,
            _ => Some(instructions)
        }
    }
}
