use core::fmt::Debug;
use std::any::Any;

use crate::error;
use crate::opcodes::OpCode;
use crate::dex_reader::DexEndianness;

pub fn parse(raw_bytes: &[u16],
                offset: usize,
                endianness: &DexEndianness) -> Box<dyn InstructionHandler>
{
    let bytes = raw_bytes.to_vec();

    // TODO: make this prettier
    assert!(offset <= bytes.len());

    let opcode = match OpCode::parse((bytes[offset] & 0xff).try_into().unwrap()) {
        // Deal with the special cases of fill-array-data-payload,
        // packed-switch-payload, and sparse-switch-payload
        Some(OpCode::NOP) => match bytes[offset] >> 8 {
            0x01 => OpCode::PACKED_SWITCH_PAYLOAD,
            0x02 => OpCode::SPARSE_SWITCH_PAYLOAD,
            0x03 => OpCode::FILL_ARRAY_DATA_PAYLOAD,
            _    => OpCode::NOP
        },
        Some(code) => code,
        None => panic!("Cannot parse instruction from: 0x{:X?}", bytes[offset] & 0xff)
    };

    match opcode {
        OpCode::GOTO => Box::new(Instruction10t{
            opcode,
            bytes: Vec::from(&bytes[offset..offset + 1]),
            length: 1
        }),

        OpCode::NOP | OpCode::RETURN_VOID => Box::new(Instruction10x{
            opcode,
            bytes: Vec::from(&bytes[offset..offset + 1]),
            length: 1
        }),

        OpCode::CONST_4 => Box::new(Instruction11n{
            opcode,
            bytes: Vec::from(&bytes[offset..offset + 1]),
            length: 1
        }),

        OpCode::MONITOR_ENTER            | OpCode::MONITOR_EXIT
            | OpCode::MOVE_EXCEPTION     | OpCode::MOVE_RESULT
            | OpCode::MOVE_RESULT_OBJECT | OpCode::MOVE_RESULT_WIDE
            | OpCode::RETURN             | OpCode::RETURN_OBJECT
            | OpCode::RETURN_WIDE        | OpCode::THROW
            => Box::new(Instruction11x{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 1]),
                length: 1
            }),

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
            => Box::new(Instruction12x{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 1]),
                length: 1
            }),

        OpCode::GOTO_16 => Box::new(Instruction20t{
            opcode,
            bytes: Vec::from(&bytes[offset..offset + 2]),
            length: 2
        }),

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
            => Box::new(Instruction21c{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 2]),
                length: 2
            }),

        OpCode::CONST_HIGH16 | OpCode::CONST_WIDE_HIGH16
            => Box::new(Instruction21h{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 2]),
                length: 2
            }),

        OpCode::CONST_16 | OpCode::CONST_WIDE_16
            => Box::new(Instruction21s{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 2]),
                length: 2
            }),

        OpCode::IF_EQZ       | OpCode::IF_GEZ
            | OpCode::IF_GTZ | OpCode::IF_LEZ
            | OpCode::IF_LTZ | OpCode::IF_NEZ
            => Box::new(Instruction21t{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 2]),
                length: 2
            }),

        OpCode::ADD_INT_LIT8        | OpCode::AND_INT_LIT8
            | OpCode::DIV_INT_LIT8  | OpCode::MUL_INT_LIT8
            | OpCode::OR_INT_LIT8   | OpCode::REM_INT_LIT8
            | OpCode::RSUB_INT_LIT8 | OpCode::SHL_INT_LIT8
            | OpCode::SHR_INT_LIT8  | OpCode::USHR_INT_LIT8
            | OpCode::XOR_INT_LIT8
            => Box::new(Instruction22b{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 2]),
                length: 2
            }),

        OpCode::IGET_BOOLEAN       | OpCode::IGET_BYTE
            | OpCode::IGET_CHAR    | OpCode::IGET
            | OpCode::IGET_OBJECT  | OpCode::IGET_SHORT
            | OpCode::IGET_WIDE    | OpCode::INSTANCE_OF
            | OpCode::IPUT_BOOLEAN | OpCode::IPUT_BYTE
            | OpCode::IPUT_CHAR    | OpCode::IPUT
            | OpCode::IPUT_OBJECT  | OpCode::IPUT_SHORT
            | OpCode::IPUT_WIDE    | OpCode::NEW_ARRAY
            => Box::new(Instruction22c{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 2]),
                length: 2
            }),

        OpCode::ADD_INT_LIT16       | OpCode::AND_INT_LIT16
            | OpCode::DIV_INT_LIT16 | OpCode::MUL_INT_LIT16
            | OpCode::OR_INT_LIT16  | OpCode::REM_INT_LIT16
            | OpCode::RSUB_INT      | OpCode::XOR_INT_LIT16
            => Box::new(Instruction22s{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 2]),
                length: 2
            }),

        OpCode::IF_EQ       | OpCode::IF_GE
            | OpCode::IF_GT | OpCode::IF_LE
            | OpCode::IF_LT | OpCode::IF_NE
            => Box::new(Instruction22t{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 2]),
                length: 2
            }),

        OpCode::MOVE_FROM16 | OpCode::MOVE_OBJECT_FROM16
            | OpCode::MOVE_WIDE_FROM16
            => Box::new(Instruction22x{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 2]),
                length: 2
            }),

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
            => Box::new(Instruction23x{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 2]),
                length: 2
            }),

        OpCode::GOTO_32 => Box::new(Instruction30t{
            opcode,
            bytes: Vec::from(&bytes[offset..offset + 3]),
            length: 3
        }),

        OpCode::CONST_STRING_JUMBO => Box::new(Instruction31c{
            opcode,
            bytes: Vec::from(&bytes[offset..offset + 3]),
            length: 3
        }),

        OpCode::CONST | OpCode::CONST_WIDE_32
            => Box::new(Instruction31i{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 3]),
                length: 3
            }),

        OpCode::FILL_ARRAY_DATA | OpCode::PACKED_SWITCH
            | OpCode::SPARSE_SWITCH => Box::new(Instruction31t{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 3]),
                length: 3
            }),

        OpCode::MOVE_16 | OpCode::MOVE_OBJECT_16
            | OpCode::MOVE_WIDE_16 => Box::new(Instruction32x{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 3]),
                length: 3
            }),

        OpCode::FILLED_NEW_ARRAY    | OpCode::INVOKE_CUSTOM
            | OpCode::INVOKE_DIRECT | OpCode::INVOKE_INTERFACE
            | OpCode::INVOKE_STATIC | OpCode::INVOKE_SUPER
            | OpCode::INVOKE_VIRTUAL
            => Box::new(Instruction35c{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 3]),
                length: 3
            }),

        OpCode::FILLED_NEW_ARRAY_RANGE    | OpCode::INVOKE_CUSTOM_RANGE
            | OpCode::INVOKE_DIRECT_RANGE | OpCode::INVOKE_INTERFACE_RANGE
            | OpCode::INVOKE_STATIC_RANGE | OpCode::INVOKE_SUPER_RANGE
            | OpCode::INVOKE_VIRTUAL_RANGE
            => Box::new(Instruction3rc{
                opcode,
                bytes: Vec::from(&bytes[offset..offset + 3]),
                length: 3
            }),

        OpCode::INVOKE_POLYMORPHIC => Box::new(Instruction45cc{
            opcode,
            bytes: Vec::from(&bytes[offset..offset + 4]),
            length: 4
        }),

        OpCode::INVOKE_POLYMORPHIC_RANGE => Box::new(Instruction4rcc{
            opcode,
            bytes: Vec::from(&bytes[offset..offset + 4]),
            length: 4
        }),

        OpCode::CONST_WIDE => Box::new(Instruction51l{
            opcode,
            bytes: Vec::from(&bytes[offset..offset + 5]),
            length: 5
        }),

        // TODO: refactor this shit
        OpCode::PACKED_SWITCH_PAYLOAD
            => Box::new(PackedSwitchPayload::build(&bytes, offset, endianness)),

        OpCode::SPARSE_SWITCH_PAYLOAD
            => Box::new(SparseSwitchPayload::build(&bytes, offset, endianness)),

        OpCode::FILL_ARRAY_DATA_PAYLOAD
            => Box::new(FillArrayDataPayload::build(&bytes, offset, endianness)),
    }
}

// TODO: not sure we need to derive debug everywhere, and this
// will lead to larger binary in the end. Keeping it for now.
#[derive(Debug)]
struct Instruction10t  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction10x  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction11n  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction11x  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction12x  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction20t  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction21c  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction21h  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction21s  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction21t  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction22b  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction22c  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction22s  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction22t  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction22x  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction23x  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction30t  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction31c  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction31i  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction31t  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction32x  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction35c  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction3rc  { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction45cc { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction4rcc { opcode: OpCode, length: usize, bytes: Vec<u16> }
#[derive(Debug)]
struct Instruction51l  { opcode: OpCode, length: usize, bytes: Vec<u16> }

pub trait AToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[allow(unused_variables)]
pub trait InstructionHandler: Debug + Any + AToAny {
    /* Getters for the instructions metadata */
    fn length(&self) -> usize;
    fn opcode(&self) -> OpCode;
    /* TODO: should we use a slice here instead? We know
     * the size in advance for a given instruction type. */
    fn bytes(&self) -> &[u16]; // Vec<u16>;
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

/// 00|op
impl InstructionHandler for Instruction10x {
    fn length(&self) -> usize {
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
    }

    fn inst_format(&self) -> &str {
        "Instruction10x"
    }
}

/// B|A|op
impl InstructionHandler for Instruction11n {
    fn length(&self) -> usize {
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
    }

    fn inst_format(&self) -> &str {
        "Instruction11n"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some(((data[0] & 0x0f00) >> 8) as u64)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(((data[0] & 0xf000) >> 12) as u64)
    }
}

impl InstructionHandler for Instruction12x {
    fn length(&self) -> usize {
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
    }

    fn inst_format(&self) -> &str {
        "Instruction12x"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some(((data[0] & 0x0f00) >> 8) as u64)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(((data[0] & 0xf000) >> 12) as u64)
    }
}

/// AA|op
impl InstructionHandler for Instruction11x {
    fn length(&self) -> usize {
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
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
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
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
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
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
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
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
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
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
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
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
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
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
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
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
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
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
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
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
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
    }

    fn inst_format(&self) -> &str {
        "Instruction22c"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some(((data[0] & 0x0f00) >> 8) as u64)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(((data[0] & 0xf000) >> 12) as u64)
    }

    fn c(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }
}

impl InstructionHandler for Instruction22s {
    fn length(&self) -> usize {
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
    }

    fn inst_format(&self) -> &str {
        "Instruction22s"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some(((data[0] & 0x0f00) >> 8) as u64)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(((data[0] & 0xf000) >> 12) as u64)
    }

    fn c(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }
}

impl InstructionHandler for Instruction22t {
    fn length(&self) -> usize {
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
    }

    fn inst_format(&self) -> &str {
        "Instruction22t"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some(((data[0] & 0x0f00) >> 8) as u64)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(((data[0] & 0xf000) >> 12) as u64)
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
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
    }

    fn inst_format(&self) -> &str {
        "Instruction30t"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64 + ((data [2] as u64) << 16))
    }
}

/// AA|op
/// BBBBlow
/// BBBBhigh
impl InstructionHandler for Instruction31c {
    fn length(&self) -> usize {
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
    }

    fn inst_format(&self) -> &str {
        "Instruction31c"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64 + ((data [2] as u64) << 16))
    }
}

impl InstructionHandler for Instruction31i {
    fn length(&self) -> usize {
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
    }

    fn inst_format(&self) -> &str {
        "Instruction31i"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64 + ((data [2] as u64) << 16))
    }
}

impl InstructionHandler for Instruction31t {
    fn length(&self) -> usize {
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
    }

    fn inst_format(&self) -> &str {
        "Instruction31t"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64 + ((data [2] as u64) << 16))
    }
}

/// 00|op
/// AAAA
/// BBBB
impl InstructionHandler for Instruction32x {
    fn length(&self) -> usize {
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
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
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
    }

    fn inst_format(&self) -> &str {
        "Instruction35c"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some(((data[0] & 0xf000) >> 12) as u64)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }

    fn c(&self, data: &[u16]) -> Option<u64> {
        Some((data[2] & 0x000f) as u64)
    }

    fn d(&self, data: &[u16]) -> Option<u64> {
        Some(((data[2] & 0x00f0) >> 4) as u64)
    }

    fn e(&self, data: &[u16]) -> Option<u64> {
        Some(((data[2] & 0x0f00) >> 8) as u64)
    }

    fn f(&self, data: &[u16]) -> Option<u64> {
        Some(((data[2] & 0xf000) >> 12) as u64)
    }

    fn g(&self, data: &[u16]) -> Option<u64> {
        Some(((data[0] & 0x0f00) >> 8) as u64)
    }
}

/// AA|op
/// BBBB
/// CCCC
impl InstructionHandler for Instruction3rc {
    fn length(&self) -> usize {
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
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
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
    }

    fn inst_format(&self) -> &str {
        "Instruction45cc"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some(((data[0] & 0xf000) >> 12) as u64)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64)
    }

    fn c(&self, data: &[u16]) -> Option<u64> {
        Some((data[2] & 0x000f) as u64)
    }

    fn d(&self, data: &[u16]) -> Option<u64> {
        Some(((data[2] & 0x00f0) >> 4) as u64)
    }

    fn e(&self, data: &[u16]) -> Option<u64> {
        Some(((data[2] & 0x0f00) >> 8) as u64)
    }

    fn f(&self, data: &[u16]) -> Option<u64> {
        Some(((data[2] & 0xf000) >> 12) as u64)
    }

    fn g(&self, data: &[u16]) -> Option<u64> {
        Some(((data[0] & 0x0f00) >> 8) as u64)
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
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
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
        self.length
    }

    fn opcode(&self) -> OpCode {
        self.opcode
    }

    fn bytes(&self) -> &[u16] {
        &self.bytes
    }

    fn inst_format(&self) -> &str {
        "Instruction51l"
    }

    fn a(&self, data: &[u16]) -> Option<u64> {
        Some((data[0] as u64) >> 8)
    }

    fn b(&self, data: &[u16]) -> Option<u64> {
        Some(data[1] as u64
             + ((data[2] as u64) << 16)
             + ((data[3] as u64) << 32)
             + ((data[4] as u64) << 48))
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

#[derive(Debug)]
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
        let first_key = read_i32(bytes, offset + 2, endianness);
        let mut targets = Vec::new();
        let mut ioffset = 4;
        for _ in 0..size {
            targets.push(read_i32(bytes, offset + ioffset, endianness));
            ioffset += 2;
        }

        PackedSwitchPayload {
            size,
            first_key,
            targets
        }
    }

    fn get_size(&self) -> usize {
        self.size as usize
    }

    fn get_first_key(&self) -> i32 {
        self.first_key
    }

    fn get_targets(&self) -> &[i32] {
        &self.targets
    }
}

impl InstructionHandler for PackedSwitchPayload {
    fn length(&self) -> usize {
        // nb of entries in bytes + size of (opcode and size)
        ((self.size * 2) + 4).into()
    }

    fn opcode(&self) -> OpCode {
        OpCode::PACKED_SWITCH_PAYLOAD
    }

    // TODO: find a better fallback. This instruction
    // does not have "bytes" like the others but we
    // need to implement this function as part of the
    // trait implementation.
    fn bytes(&self) -> &[u16] {
        &[]
    }

    fn inst_format(&self) -> &str {
        "PackedSwitchPayload"
    }
}

#[derive(Debug)]
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
            keys.push(read_i32(bytes, offset + ioffset, endianness));
            ioffset += 2;
        }

        let mut targets = Vec::new();
        let mut ioffset = 2 + (size as usize) * 2;
        for _ in 0..size {
            targets.push(read_i32(bytes, offset + ioffset, endianness));
            ioffset += 2;
        }

        SparseSwitchPayload {
            size,
            keys,
            targets
        }
    }

    fn get_size(&self) -> usize {
        self.size as usize
    }

    fn get_keys(&self) -> &[i32] {
        &self.keys
    }

    fn get_targets(&self) -> &[i32] {
        &self.targets
    }
}

impl InstructionHandler for SparseSwitchPayload {
    fn length(&self) -> usize {
        ((self.size * 4) + 2).into()
    }

    fn opcode(&self) -> OpCode {
        OpCode::SPARSE_SWITCH_PAYLOAD
    }

    // TODO: find a better fallback. This instruction
    // does not have "bytes" like the others but we
    // need to implement this function as part of the
    // trait implementation.
    fn bytes(&self) -> &[u16] {
        &[]
    }

    fn inst_format(&self) -> &str {
        "SparseSwitchPayload"
    }
}

#[derive(Debug)]
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
                for _ in 0..(size / element_width as u32) / 2 {
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

    fn opcode(&self) -> OpCode {
        OpCode::FILL_ARRAY_DATA_PAYLOAD
    }

    // TODO: find a better fallback. This instruction
    // does not have "bytes" like the others but we
    // need to implement this function as part of the
    // trait implementation.
    fn bytes(&self) -> &[u16] {
        &[]
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
    pub fn parse_instructions(&mut self) -> Option<Vec<Box<dyn InstructionHandler>>> {
        let mut instructions = Vec::new();

        while self.offset < self.length {
            let ins = parse(self.bytes, self.offset, self.endianness);
            self.offset += ins.length();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_i32_big_endian_no_overflow() {
        let bytes = [0x1234, 0x5678];
        let result = read_i32(&bytes, 0, &DexEndianness::BigEndian);
        assert_eq!(result, 0x1234_5678);
    }

    #[test]
    fn test_read_i32_little_endian_no_overflow() {
        let bytes = [0x1234, 0x5678];
        let result = read_i32(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(result, 0x5678_1234);
    }

    #[test]
    fn test_read_i32_big_endian_overflow() {
        let bytes = [0xFFFF, 0xFFFF];
        let result = read_i32(&bytes, 0, &DexEndianness::BigEndian);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_read_i32_little_endian_overflow() {
        let bytes = [0xFFFF, 0xFFFF];
        let result = read_i32(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(result, -1);
    }

    #[test]
    fn test_instruction10x() {
        let inst = Instruction10x {
            opcode: OpCode::NOP,
            length: 1,
            bytes: vec![0x00]
        };

        assert_eq!(inst.length(), 1);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x00]);
        assert_eq!(inst.inst_format(), "Instruction10x");

        assert_eq!(inst.a(inst.bytes()), None);
        assert_eq!(inst.b(inst.bytes()), None);
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction11n() {
        let inst = Instruction11n {
            opcode: OpCode::NOP,
            length: 1,
            bytes: vec![0x2100]
        };

        assert_eq!(inst.length(), 1);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x2100]);
        assert_eq!(inst.inst_format(), "Instruction11n");

        assert_eq!(inst.a(inst.bytes()), Some(1));
        assert_eq!(inst.b(inst.bytes()), Some(2));
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction12x() {
        let inst = Instruction12x {
            opcode: OpCode::NOP,
            length: 1,
            bytes: vec![0x2100]
        };

        assert_eq!(inst.length(), 1);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x2100]);
        assert_eq!(inst.inst_format(), "Instruction12x");

        assert_eq!(inst.a(inst.bytes()), Some(1));
        assert_eq!(inst.b(inst.bytes()), Some(2));
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction11x() {
        let inst = Instruction11x {
            opcode: OpCode::NOP,
            length: 1,
            bytes: vec![0x1000]
        };

        assert_eq!(inst.length(), 1);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1000]);
        assert_eq!(inst.inst_format(), "Instruction11x");

        assert_eq!(inst.a(inst.bytes()), Some(0x10));
        assert_eq!(inst.b(inst.bytes()), None);
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction10t() {
        let inst = Instruction10t {
            opcode: OpCode::NOP,
            length: 1,
            bytes: vec![0x1000]
        };

        assert_eq!(inst.length(), 1);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1000]);
        assert_eq!(inst.inst_format(), "Instruction10t");

        assert_eq!(inst.a(inst.bytes()), Some(0x10));
        assert_eq!(inst.b(inst.bytes()), None);
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction20t() {
        let inst = Instruction20t {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x0000, 0x1234]
        };

        assert_eq!(inst.length(), 2);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x0000, 0x1234]);
        assert_eq!(inst.inst_format(), "Instruction20t");

        assert_eq!(inst.a(inst.bytes()), Some(0x1234));
        assert_eq!(inst.b(inst.bytes()), None);
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction21c() {
        let inst = Instruction21c {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.length(), 2);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x3456]);
        assert_eq!(inst.inst_format(), "Instruction21c");

        assert_eq!(inst.a(inst.bytes()), Some(0x12));
        assert_eq!(inst.b(inst.bytes()), Some(0x3456));
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction21h() {
        let inst = Instruction21h {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.length(), 2);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x3456]);
        assert_eq!(inst.inst_format(), "Instruction21h");

        assert_eq!(inst.a(inst.bytes()), Some(0x12));
        assert_eq!(inst.b(inst.bytes()), Some(0x3456));
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction21s() {
        let inst = Instruction21s {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.length(), 2);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x3456]);
        assert_eq!(inst.inst_format(), "Instruction21s");

        assert_eq!(inst.a(inst.bytes()), Some(0x12));
        assert_eq!(inst.b(inst.bytes()), Some(0x3456));
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction21t() {
        let inst = Instruction21t {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.length(), 2);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x3456]);
        assert_eq!(inst.inst_format(), "Instruction21t");

        assert_eq!(inst.a(inst.bytes()), Some(0x12));
        assert_eq!(inst.b(inst.bytes()), Some(0x3456));
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction22x() {
        let inst = Instruction22x {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.length(), 2);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x3456]);
        assert_eq!(inst.inst_format(), "Instruction22x");

        assert_eq!(inst.a(inst.bytes()), Some(0x12));
        assert_eq!(inst.b(inst.bytes()), Some(0x3456));
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction23x() {
        let inst = Instruction23x {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.length(), 2);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x3456]);
        assert_eq!(inst.inst_format(), "Instruction23x");

        assert_eq!(inst.a(inst.bytes()), Some(0x12));
        assert_eq!(inst.b(inst.bytes()), Some(0x34));
        assert_eq!(inst.c(inst.bytes()), Some(0x56));
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction22b() {
        let inst = Instruction22b {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.length(), 2);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x3456]);
        assert_eq!(inst.inst_format(), "Instruction22b");

        assert_eq!(inst.a(inst.bytes()), Some(0x12));
        assert_eq!(inst.b(inst.bytes()), Some(0x34));
        assert_eq!(inst.c(inst.bytes()), Some(0x56));
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction22c() {
        let inst = Instruction22c {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.length(), 2);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x3456]);
        assert_eq!(inst.inst_format(), "Instruction22c");

        assert_eq!(inst.a(inst.bytes()), Some(0x2));
        assert_eq!(inst.b(inst.bytes()), Some(0x1));
        assert_eq!(inst.c(inst.bytes()), Some(0x3456));
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction22s() {
        let inst = Instruction22s {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.length(), 2);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x3456]);
        assert_eq!(inst.inst_format(), "Instruction22s");

        assert_eq!(inst.a(inst.bytes()), Some(0x2));
        assert_eq!(inst.b(inst.bytes()), Some(0x1));
        assert_eq!(inst.c(inst.bytes()), Some(0x3456));
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction22t() {
        let inst = Instruction22t {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.length(), 2);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x3456]);
        assert_eq!(inst.inst_format(), "Instruction22t");

        assert_eq!(inst.a(inst.bytes()), Some(0x2));
        assert_eq!(inst.b(inst.bytes()), Some(0x1));
        assert_eq!(inst.c(inst.bytes()), Some(0x3456));
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction30t() {
        let inst = Instruction30t {
            opcode: OpCode::NOP,
            length: 3,
            bytes: vec![0x00, 0x5678, 0x1234]
        };

        assert_eq!(inst.length(), 3);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x00, 0x5678, 0x1234]);
        assert_eq!(inst.inst_format(), "Instruction30t");

        assert_eq!(inst.a(inst.bytes()), Some(0x12345678));
        assert_eq!(inst.b(inst.bytes()), None);
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction31c() {
        let inst = Instruction31c {
            opcode: OpCode::NOP,
            length: 3,
            bytes: vec![0x1200, 0x5678, 0x1234]
        };

        assert_eq!(inst.length(), 3);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x5678, 0x1234]);
        assert_eq!(inst.inst_format(), "Instruction31c");

        assert_eq!(inst.a(inst.bytes()), Some(0x12));
        assert_eq!(inst.b(inst.bytes()), Some(0x12345678));
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction31i() {
        let inst = Instruction31i {
            opcode: OpCode::NOP,
            length: 3,
            bytes: vec![0x1200, 0x5678, 0x1234]
        };

        assert_eq!(inst.length(), 3);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x5678, 0x1234]);
        assert_eq!(inst.inst_format(), "Instruction31i");

        assert_eq!(inst.a(inst.bytes()), Some(0x12));
        assert_eq!(inst.b(inst.bytes()), Some(0x12345678));
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction31t() {
        let inst = Instruction31t {
            opcode: OpCode::NOP,
            length: 3,
            bytes: vec![0x1200, 0x5678, 0x1234]
        };

        assert_eq!(inst.length(), 3);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x5678, 0x1234]);
        assert_eq!(inst.inst_format(), "Instruction31t");

        assert_eq!(inst.a(inst.bytes()), Some(0x12));
        assert_eq!(inst.b(inst.bytes()), Some(0x12345678));
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction32x() {
        let inst = Instruction32x {
            opcode: OpCode::NOP,
            length: 3,
            bytes: vec![0x00, 0x1234, 0x5678]
        };

        assert_eq!(inst.length(), 3);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x00, 0x1234, 0x5678]);
        assert_eq!(inst.inst_format(), "Instruction32x");

        assert_eq!(inst.a(inst.bytes()), Some(0x1234));
        assert_eq!(inst.b(inst.bytes()), Some(0x5678));
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction35c() {
        let inst = Instruction35c {
            opcode: OpCode::NOP,
            length: 3,
            bytes: vec![0x1200, 0x3456, 0x7890]
        };

        assert_eq!(inst.length(), 3);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x3456, 0x7890]);
        assert_eq!(inst.inst_format(), "Instruction35c");

        assert_eq!(inst.a(inst.bytes()), Some(0x01));
        assert_eq!(inst.b(inst.bytes()), Some(0x3456));
        assert_eq!(inst.c(inst.bytes()), Some(0x00));
        assert_eq!(inst.d(inst.bytes()), Some(0x09));
        assert_eq!(inst.e(inst.bytes()), Some(0x08));
        assert_eq!(inst.f(inst.bytes()), Some(0x07));
        assert_eq!(inst.g(inst.bytes()), Some(0x02));
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction3rc() {
        let inst = Instruction3rc {
            opcode: OpCode::NOP,
            length: 3,
            bytes: vec![0x1200, 0x3456, 0x7890]
        };

        assert_eq!(inst.length(), 3);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x3456, 0x7890]);
        assert_eq!(inst.inst_format(), "Instruction3rc");

        assert_eq!(inst.a(inst.bytes()), Some(0x12));
        assert_eq!(inst.b(inst.bytes()), Some(0x3456));
        assert_eq!(inst.c(inst.bytes()), Some(0x7890));
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_instruction45cc() {
        let inst = Instruction45cc {
            opcode: OpCode::NOP,
            length: 4,
            bytes: vec![0x1200, 0x3456, 0x7890, 0x1234]
        };

        assert_eq!(inst.length(), 4);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x3456, 0x7890, 0x1234]);
        assert_eq!(inst.inst_format(), "Instruction45cc");

        assert_eq!(inst.a(inst.bytes()), Some(0x01));
        assert_eq!(inst.b(inst.bytes()), Some(0x03456));
        assert_eq!(inst.c(inst.bytes()), Some(0x00));
        assert_eq!(inst.d(inst.bytes()), Some(0x09));
        assert_eq!(inst.e(inst.bytes()), Some(0x08));
        assert_eq!(inst.f(inst.bytes()), Some(0x07));
        assert_eq!(inst.g(inst.bytes()), Some(0x02));
        assert_eq!(inst.h(inst.bytes()), Some(0x1234));
    }

    #[test]
    fn test_instruction4rcc() {
        let inst = Instruction4rcc {
            opcode: OpCode::NOP,
            length: 4,
            bytes: vec![0x1200, 0x3456, 0x7890, 0x1234]
        };

        assert_eq!(inst.length(), 4);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x3456, 0x7890, 0x1234]);
        assert_eq!(inst.inst_format(), "Instruction4rcc");

        assert_eq!(inst.a(inst.bytes()), Some(0x12));
        assert_eq!(inst.b(inst.bytes()), Some(0x3456));
        assert_eq!(inst.c(inst.bytes()), Some(0x7890));
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), Some(0x1234));
    }

    #[test]
    fn test_instruction51l() {
        let inst = Instruction51l {
            opcode: OpCode::NOP,
            length: 5,
            bytes: vec![0x1200, 0x3456, 0x9012, 0x5678, 0x1234]
        };

        assert_eq!(inst.length(), 5);
        assert_eq!(inst.opcode(), OpCode::NOP);
        assert_eq!(inst.bytes(), vec![0x1200, 0x3456, 0x9012, 0x5678, 0x1234]);
        assert_eq!(inst.inst_format(), "Instruction51l");

        assert_eq!(inst.a(inst.bytes()), Some(0x12));
        assert_eq!(inst.b(inst.bytes()), Some(0x1234_5678_9012_3456));
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_packed_switch_payload() {
        let inst = PackedSwitchPayload {
            size: 4,
            first_key: 0,
            targets: vec![1, 2, 3, 4]
        };

        assert_eq!(inst.length(), 12);
        assert_eq!(inst.opcode(), OpCode::PACKED_SWITCH_PAYLOAD);
        assert_eq!(inst.bytes(), vec![]);
        assert_eq!(inst.inst_format(), "PackedSwitchPayload");

        assert_eq!(inst.a(inst.bytes()), None);
        assert_eq!(inst.b(inst.bytes()), None);
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_sparse_switch_payload() {
        let inst = SparseSwitchPayload {
            size: 4,
            keys: vec![1, 2, 3, 4],
            targets: vec![1, 2, 3, 4]
        };

        assert_eq!(inst.length(), 18);
        assert_eq!(inst.opcode(), OpCode::SPARSE_SWITCH_PAYLOAD);
        assert_eq!(inst.bytes(), vec![]);
        assert_eq!(inst.inst_format(), "SparseSwitchPayload");

        assert_eq!(inst.a(inst.bytes()), None);
        assert_eq!(inst.b(inst.bytes()), None);
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_fill_array_data_payload() {
        let inst = FillArrayDataPayload {
            element_width: 4,
            size: 4,
            data: vec![1, 2, 3, 4]
        };

        assert_eq!(inst.length(), 12);
        assert_eq!(inst.opcode(), OpCode::FILL_ARRAY_DATA_PAYLOAD);
        assert_eq!(inst.bytes(), vec![]);
        assert_eq!(inst.inst_format(), "FillArrayDataPayload");

        assert_eq!(inst.a(inst.bytes()), None);
        assert_eq!(inst.b(inst.bytes()), None);
        assert_eq!(inst.c(inst.bytes()), None);
        assert_eq!(inst.d(inst.bytes()), None);
        assert_eq!(inst.e(inst.bytes()), None);
        assert_eq!(inst.f(inst.bytes()), None);
        assert_eq!(inst.g(inst.bytes()), None);
        assert_eq!(inst.h(inst.bytes()), None);
    }

    #[test]
    fn test_parse_instruction() {
        let bytes = [0x00, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction10x");

        let bytes = [0x01, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x02, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22x");

        let bytes = [0x03, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction32x");

        let bytes = [0x04, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x05, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22x");

        let bytes = [0x06, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction32x");

        let bytes = [0x07, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x08, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22x");

        let bytes = [0x09, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction32x");

        let bytes = [0x0a, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction11x");

        let bytes = [0x0b, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction11x");

        let bytes = [0x0c, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction11x");

        let bytes = [0x0d, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction11x");

        let bytes = [0x0e, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction10x");

        let bytes = [0x0f, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction11x");

        let bytes = [0x10, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction11x");

        let bytes = [0x11, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction11x");

        let bytes = [0x12, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction11n");

        let bytes = [0x13, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21s");

        let bytes = [0x14, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction31i");

        let bytes = [0x15, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21h");

        let bytes = [0x16, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21s");

        let bytes = [0x17, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction31i");

        let bytes = [0x18, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction51l");

        let bytes = [0x19, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21h");

        let bytes = [0x1a, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x1b, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction31c");

        let bytes = [0x1c, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x1d, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction11x");

        let bytes = [0x1e, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction11x");

        let bytes = [0x1f, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x20, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22c");

        let bytes = [0x21, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x22, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x23, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22c");

        let bytes = [0x24, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction35c");

        let bytes = [0x25, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction3rc");

        let bytes = [0x26, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction31t");

        let bytes = [0x27, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction11x");

        let bytes = [0x28, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction10t");

        let bytes = [0x29, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction20t");

        let bytes = [0x2a, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction30t");

        let bytes = [0x2b, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction31t");

        let bytes = [0x2c, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction31t");

        let bytes = [0x2d, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x2e, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x2f, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x30, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x31, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x32, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22t");

        let bytes = [0x33, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22t");

        let bytes = [0x34, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22t");

        let bytes = [0x35, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22t");

        let bytes = [0x36, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22t");

        let bytes = [0x37, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22t");

        let bytes = [0x38, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21t");

        let bytes = [0x39, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21t");

        let bytes = [0x3a, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21t");

        let bytes = [0x3b, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21t");

        let bytes = [0x3c, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21t");

        let bytes = [0x3d, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21t");

        /* 0x3e..=0x43 : unused */

        let bytes = [0x44, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x45, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x46, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x47, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x48, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x49, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x4a, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x4b, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x4c, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x4d, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x4e, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x4f, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x50, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x51, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x52, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22c");

        let bytes = [0x53, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22c");

        let bytes = [0x54, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22c");

        let bytes = [0x55, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22c");

        let bytes = [0x56, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22c");

        let bytes = [0x57, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22c");

        let bytes = [0x58, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22c");

        let bytes = [0x59, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22c");

        let bytes = [0x5a, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22c");

        let bytes = [0x5b, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22c");

        let bytes = [0x5c, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22c");

        let bytes = [0x5d, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22c");

        let bytes = [0x5e, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22c");

        let bytes = [0x5f, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22c");

        let bytes = [0x60, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x61, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x62, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x63, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x64, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x65, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x66, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x67, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x68, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x69, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x6a, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x6b, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x6c, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x6d, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0x6e, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction35c");

        let bytes = [0x6f, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction35c");

        let bytes = [0x70, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction35c");

        let bytes = [0x71, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction35c");

        let bytes = [0x72, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction35c");
        
        /* 0x73 : unused */

        let bytes = [0x74, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction3rc");

        let bytes = [0x75, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction3rc");

        let bytes = [0x76, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction3rc");

        let bytes = [0x77, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction3rc");

        let bytes = [0x78, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction3rc");

        /* 0x79 | 0x7a : unused */

        let bytes = [0x7b, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x7c, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x7d, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x7e, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x7f, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x80, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x81, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x82, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x83, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x84, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x85, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x86, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x87, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x88, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x89, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x8a, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x8b, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x8c, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x8d, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x8e, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x8f, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0x90, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x91, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x92, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x93, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x94, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x95, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x96, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x97, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x98, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x99, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x9a, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x9b, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x9c, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x9d, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x9e, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0x9f, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xa0, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xa1, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xa2, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xa3, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xa4, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xa5, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xa6, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xa7, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xa8, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xa9, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xaa, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xab, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xac, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xad, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xae, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xaf, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction23x");

        let bytes = [0xb0, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xb1, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xb2, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xb3, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xb4, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xb5, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xb6, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xb7, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xb8, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xb9, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xba, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xbb, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xbc, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xbd, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xbe, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xbf, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xc0, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xc1, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xc2, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xc3, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xc4, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xc5, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xc6, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xc7, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xc8, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xc9, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xca, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xcb, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xcc, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xcd, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xce, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xcf, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction12x");

        let bytes = [0xd0, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22s");

        let bytes = [0xd1, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22s");

        let bytes = [0xd2, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22s");

        let bytes = [0xd3, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22s");

        let bytes = [0xd4, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22s");

        let bytes = [0xd5, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22s");

        let bytes = [0xd6, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22s");

        let bytes = [0xd7, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22s");

        let bytes = [0xd8, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22b");

        let bytes = [0xd9, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22b");

        let bytes = [0xda, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22b");

        let bytes = [0xdb, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22b");

        let bytes = [0xdc, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22b");

        let bytes = [0xdd, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22b");

        let bytes = [0xde, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22b");

        let bytes = [0xdf, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22b");

        let bytes = [0xe0, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22b");

        let bytes = [0xe1, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22b");

        let bytes = [0xe2, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction22b");

        /* 0xe3..=0xf9 : unused */

        let bytes = [0xfa, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction45cc");

        let bytes = [0xfb, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction4rcc");

        let bytes = [0xfc, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction35c");

        let bytes = [0xfd, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction3rc");

        let bytes = [0xfe, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");

        let bytes = [0xff, 0x00, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "Instruction21c");
    }

    #[test]
    fn test_parse_packed_switch_payload() {
        let bytes = [
            0x0100,     // opcode
            0x0002,     // size
            0x0000,     // first_key
            0x0000,     // ...
            0x0000,      // targets
            0x0000,     // ...
            0x0000,     // ...
            0x0001,     // ...
        ];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "PackedSwitchPayload");

        let any_inst = match inst.as_ref()
                                 .as_any()
                                 .downcast_ref::<PackedSwitchPayload>() {
            Some(ins) => ins,
            None      => {
                error!("cannot access PackedSwitchPayload from Box");
                panic!("error: cannot access PackedSwitchPayload from Box");
            }
        };
        assert_eq!(any_inst.get_size(), 2);
        assert_eq!(any_inst.get_first_key(), 0);
        assert_eq!(any_inst.get_targets(), &vec![0x00, 0x0001_0000]);
    }

    #[test]
    fn test_parse_sparse_switch_payload() {
        let bytes = [
            0x0200,     // opcode
            0x0002,     // size
            0x0000,     // keys
            0x0000,     // ...
            0x0000,     // ...
            0x0001,     // ...
            0x0000,     // targets
            0x0002,     // ...
            0x0000,     // ...
            0x0003,     // ...
        ];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "SparseSwitchPayload");

        let any_inst = match inst.as_ref()
                                 .as_any()
                                 .downcast_ref::<SparseSwitchPayload>() {
            Some(ins) => ins,
            None      => {
                error!("cannot access SparseSwitchPayload from Box");
                panic!("error: cannot access SparseSwitchPayload from Box");
            }
        };
        assert_eq!(any_inst.get_size(), 2);
        assert_eq!(any_inst.get_keys(), &vec![0x00, 0x0001_0000]);
        assert_eq!(any_inst.get_targets(), &vec![0x0002_0000, 0x0003_0000]);
    }

    #[test]
    fn test_parse_fill_array_data_payload() {
        let bytes = [0x0300, 0x01, 0x00, 0x00, 0x00];
        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
        assert_eq!(inst.inst_format(), "FillArrayDataPayload");
    }
}
