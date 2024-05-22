use crate::opcodes::OpCode;
use crate::dex_reader::DexEndianness;
use crate::DexReader;

/// Utility function to read i32 from [u16]
fn read_i32(bytes: &[u16],
            offset: usize,
            endianness: DexEndianness) -> i32
{
    match endianness {
        DexEndianness::LittleEndian =>
            ((bytes[offset + 1] as i32) << 16) + bytes[offset] as i32,
        DexEndianness::BigEndian =>
            ((bytes[offset] as i32) << 16) + bytes[offset + 1] as i32,
    }
}

// TODO replace old instructions by these ones (remove prefix too)
#[derive(Debug, Clone)]
pub struct Instruction10t  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 1] }
#[derive(Debug, Clone)]
pub struct Instruction10x  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 1] }
#[derive(Debug, Clone)]
pub struct Instruction11n  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 1] }
#[derive(Debug, Clone)]
pub struct Instruction11x  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 1] }
#[derive(Debug, Clone)]
pub struct Instruction12x  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 1] }
#[derive(Debug, Clone)]
pub struct Instruction20t  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 2] }
#[derive(Debug, Clone)]
pub struct Instruction21c  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 2] }
#[derive(Debug, Clone)]
pub struct Instruction21h  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 2] }
#[derive(Debug, Clone)]
pub struct Instruction21s  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 2] }
#[derive(Debug, Clone)]
pub struct Instruction21t  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 2] }
#[derive(Debug, Clone)]
pub struct Instruction22b  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 2] }
#[derive(Debug, Clone)]
pub struct Instruction22c  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 2] }
#[derive(Debug, Clone)]
pub struct Instruction22s  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 2] }
#[derive(Debug, Clone)]
pub struct Instruction22t  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 2] }
#[derive(Debug, Clone)]
pub struct Instruction22x  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 2] }
#[derive(Debug, Clone)]
pub struct Instruction23x  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 2] }
#[derive(Debug, Clone)]
pub struct Instruction30t  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 3] }
#[derive(Debug, Clone)]
pub struct Instruction31c  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 3] }
#[derive(Debug, Clone)]
pub struct Instruction31i  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 3] }
#[derive(Debug, Clone)]
pub struct Instruction31t  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 3] }
#[derive(Debug, Clone)]
pub struct Instruction32x  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 3] }
#[derive(Debug, Clone)]
pub struct Instruction35c  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 3] }
#[derive(Debug, Clone)]
pub struct Instruction3rc  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 3] }
#[derive(Debug, Clone)]
pub struct Instruction45cc { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 4] }
#[derive(Debug, Clone)]
pub struct Instruction4rcc { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 4] }
#[derive(Debug, Clone)]
pub struct Instruction51l  { pub(crate) opcode: OpCode, length: usize, bytes: [u16; 5] }
#[derive(Debug, Clone)]
pub struct PackedSwitchPayload {
    pub(crate) opcode: OpCode,
    size: u16,
    first_key: i32,
    targets: Vec<i32>
}
#[derive(Debug, Clone)]
pub struct SparseSwitchPayload {
    pub(crate) opcode: OpCode,
    size: u16,
    keys: Vec<i32>,
    targets: Vec<i32>
}
#[derive(Debug, Clone)]
pub struct FillArrayDataPayload {
    pub(crate) opcode: OpCode,
    element_width: u16,
    size: u32,
    data: Vec<u8>
}

impl Instructions {
    pub fn length(&self) -> usize {
        match self {
            Instructions::Instruction10t(inst) => inst.length,
            Instructions::Instruction10x(inst) => inst.length,
            Instructions::Instruction11n(inst) => inst.length,
            Instructions::Instruction11x(inst) => inst.length,
            Instructions::Instruction12x(inst) => inst.length,
            Instructions::Instruction20t(inst) => inst.length,
            Instructions::Instruction21c(inst) => inst.length,
            Instructions::Instruction21h(inst) => inst.length,
            Instructions::Instruction21s(inst) => inst.length,
            Instructions::Instruction21t(inst) => inst.length,
            Instructions::Instruction22b(inst) => inst.length,
            Instructions::Instruction22c(inst) => inst.length,
            Instructions::Instruction22s(inst) => inst.length,
            Instructions::Instruction22t(inst) => inst.length,
            Instructions::Instruction22x(inst) => inst.length,
            Instructions::Instruction23x(inst) => inst.length,
            Instructions::Instruction30t(inst) => inst.length,
            Instructions::Instruction31c(inst) => inst.length,
            Instructions::Instruction31i(inst) => inst.length,
            Instructions::Instruction31t(inst) => inst.length,
            Instructions::Instruction32x(inst) => inst.length,
            Instructions::Instruction35c(inst) => inst.length,
            Instructions::Instruction3rc(inst) => inst.length,
            Instructions::Instruction45cc(inst) => inst.length,
            Instructions::Instruction4rcc(inst) => inst.length,
            Instructions::Instruction51l(inst) => inst.length,
            Instructions::PackedSwitchPayload(inst) => inst.length(),
            Instructions::SparseSwitchPayload(inst) => inst.length(),
            Instructions::FillArrayDataPayload(inst) => inst.length(),
        }
    }

    pub fn opcode(&self) -> OpCode {
        match self {
            Instructions::Instruction10t(inst) => inst.opcode,
            Instructions::Instruction10x(inst) => inst.opcode,
            Instructions::Instruction11n(inst) => inst.opcode,
            Instructions::Instruction11x(inst) => inst.opcode,
            Instructions::Instruction12x(inst) => inst.opcode,
            Instructions::Instruction20t(inst) => inst.opcode,
            Instructions::Instruction21c(inst) => inst.opcode,
            Instructions::Instruction21h(inst) => inst.opcode,
            Instructions::Instruction21s(inst) => inst.opcode,
            Instructions::Instruction21t(inst) => inst.opcode,
            Instructions::Instruction22b(inst) => inst.opcode,
            Instructions::Instruction22c(inst) => inst.opcode,
            Instructions::Instruction22s(inst) => inst.opcode,
            Instructions::Instruction22t(inst) => inst.opcode,
            Instructions::Instruction22x(inst) => inst.opcode,
            Instructions::Instruction23x(inst) => inst.opcode,
            Instructions::Instruction30t(inst) => inst.opcode,
            Instructions::Instruction31c(inst) => inst.opcode,
            Instructions::Instruction31i(inst) => inst.opcode,
            Instructions::Instruction31t(inst) => inst.opcode,
            Instructions::Instruction32x(inst) => inst.opcode,
            Instructions::Instruction35c(inst) => inst.opcode,
            Instructions::Instruction3rc(inst) => inst.opcode,
            Instructions::Instruction45cc(inst) => inst.opcode,
            Instructions::Instruction4rcc(inst) => inst.opcode,
            Instructions::Instruction51l(inst) => inst.opcode,
            Instructions::PackedSwitchPayload(inst) => inst.opcode,
            Instructions::SparseSwitchPayload(inst) => inst.opcode,
            Instructions::FillArrayDataPayload(inst) => inst.opcode,
        }
    }

    pub fn bytes(&self) -> &[u16] {
        match self {
            Instructions::Instruction10t(inst) => &inst.bytes,
            Instructions::Instruction10x(inst) => &inst.bytes,
            Instructions::Instruction11n(inst) => &inst.bytes,
            Instructions::Instruction11x(inst) => &inst.bytes,
            Instructions::Instruction12x(inst) => &inst.bytes,
            Instructions::Instruction20t(inst) => &inst.bytes,
            Instructions::Instruction21c(inst) => &inst.bytes,
            Instructions::Instruction21h(inst) => &inst.bytes,
            Instructions::Instruction21s(inst) => &inst.bytes,
            Instructions::Instruction21t(inst) => &inst.bytes,
            Instructions::Instruction22b(inst) => &inst.bytes,
            Instructions::Instruction22c(inst) => &inst.bytes,
            Instructions::Instruction22s(inst) => &inst.bytes,
            Instructions::Instruction22t(inst) => &inst.bytes,
            Instructions::Instruction22x(inst) => &inst.bytes,
            Instructions::Instruction23x(inst) => &inst.bytes,
            Instructions::Instruction30t(inst) => &inst.bytes,
            Instructions::Instruction31c(inst) => &inst.bytes,
            Instructions::Instruction31i(inst) => &inst.bytes,
            Instructions::Instruction31t(inst) => &inst.bytes,
            Instructions::Instruction32x(inst) => &inst.bytes,
            Instructions::Instruction35c(inst) => &inst.bytes,
            Instructions::Instruction3rc(inst) => &inst.bytes,
            Instructions::Instruction45cc(inst) => &inst.bytes,
            Instructions::Instruction4rcc(inst) => &inst.bytes,
            Instructions::Instruction51l(inst) => &inst.bytes,
            Instructions::PackedSwitchPayload(inst) => &[],  // FIXME
            Instructions::SparseSwitchPayload(inst) => &[],  // FIXME
            Instructions::FillArrayDataPayload(inst) => &[],  // FIXME
        }
    }
}

impl PackedSwitchPayload {
    fn build(reader: &mut DexReader) -> Self {
        let size = reader.read_u16().unwrap();
        let first_key = reader.read_i32().unwrap();
        let mut targets = Vec::new();
        for _ in 0..size {
            targets.push(reader.read_i32().unwrap());
        }

        PackedSwitchPayload {
            opcode: OpCode::PACKED_SWITCH_PAYLOAD,
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

impl SparseSwitchPayload {
    fn build(reader: &mut DexReader) -> Self {
        let size = reader.read_u16().unwrap();

        let mut keys = Vec::new();
        for _ in 0..size {
            keys.push(reader.read_i32().unwrap());
        }

        let mut targets = Vec::new();
        for _ in 0..size {
            targets.push(reader.read_i32().unwrap());
        }

        SparseSwitchPayload {
            opcode: OpCode::SPARSE_SWITCH_PAYLOAD,
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

impl FillArrayDataPayload {
    fn build(reader: &mut DexReader) -> Self {
        // FIXME the bytes come up empty, check the bounds of the for loop
        let element_width = reader.read_u16().unwrap();
        let size = reader.read_u32().unwrap();

        let mut data = Vec::new();
        for _ in 0..size {
            for _ in 0..element_width {
                data.push(reader.read_u8().unwrap());
            }
        }

        FillArrayDataPayload {
            opcode: OpCode::FILL_ARRAY_DATA_PAYLOAD,
            element_width,
            size,
            data
        }
    }

    fn get_element_width(&self) -> u16 {
        self.element_width
    }

    fn get_size(&self) -> u32 {
        self.size
    }

    fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn length(&self) -> usize {
        ((self.size * self.element_width as u32) / 2 + 4) as usize
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


#[derive(Debug, Clone)]
pub enum Instructions {
    Instruction10t(Instruction10t),
    Instruction10x(Instruction10x),
    Instruction11n(Instruction11n),
    Instruction11x(Instruction11x),
    Instruction12x(Instruction12x),
    Instruction20t(Instruction20t),
    Instruction21c(Instruction21c),
    Instruction21h(Instruction21h),
    Instruction21s(Instruction21s),
    Instruction21t(Instruction21t),
    Instruction22b(Instruction22b),
    Instruction22c(Instruction22c),
    Instruction22s(Instruction22s),
    Instruction22t(Instruction22t),
    Instruction22x(Instruction22x),
    Instruction23x(Instruction23x),
    Instruction30t(Instruction30t),
    Instruction31c(Instruction31c),
    Instruction31i(Instruction31i),
    Instruction31t(Instruction31t),
    Instruction32x(Instruction32x),
    Instruction35c(Instruction35c),
    Instruction3rc(Instruction3rc),
    Instruction45cc(Instruction45cc),
    Instruction4rcc(Instruction4rcc),
    Instruction51l(Instruction51l),
    PackedSwitchPayload(PackedSwitchPayload),
    SparseSwitchPayload(SparseSwitchPayload),
    FillArrayDataPayload(FillArrayDataPayload),
}


/////////////////////////////////////////////////////////////////

pub fn parse_read(reader: &mut DexReader, container: &mut Vec<Instructions>) -> usize {
    println!("----------------- start parse_read()");
    // check opcode, infer length
    // read appropriate amount of bytes
    // convert to Instructions enum member
    // return

    let offset = reader.bytes.position();
    println!("initial position: {offset}");
    let raw_opcode = reader.read_u16().unwrap();
    println!("raw opcode: {raw_opcode:#06X?} | position {}", reader.bytes.position());
    println!("          : {:#04X?}", raw_opcode & 0xff);
    println!("          : {:#04X?}", raw_opcode >> 8);

    let opcode = match OpCode::parse((raw_opcode & 0xff).try_into().unwrap()) {
        // Deal with the special cases of fill-array-data-payload,
        // packed-switch-payload, and sparse-switch-payload
        Some(OpCode::NOP) => match raw_opcode >> 8 {
            0x01 => OpCode::PACKED_SWITCH_PAYLOAD,
            0x02 => OpCode::SPARSE_SWITCH_PAYLOAD,
            0x03 => OpCode::FILL_ARRAY_DATA_PAYLOAD,
            _    => OpCode::NOP
        },
        Some(code) => code,
        None => panic!("Cannot parse instruction from: 0x{:X?}", raw_opcode & 0xff)
    };

    println!("got opcode: {opcode:?} | position {}", reader.bytes.position());

    match opcode {
        OpCode::GOTO => {
            let mut bytes = [0u16; 1];
            bytes[0] = raw_opcode;
            container.push(Instructions::Instruction10t(Instruction10t {
                opcode,
                bytes,
                length: 1
            }));
            return 1;
        },

        OpCode::NOP | OpCode::RETURN_VOID => {
            let mut bytes = [0u16; 1];
            bytes[0] = raw_opcode;
            container.push(Instructions::Instruction10x(Instruction10x{
                opcode,
                bytes,
                length: 1
            }));
            return 1;
        },

        OpCode::CONST_4 => {
            let mut bytes = [0u16; 1];
            bytes[0] = raw_opcode;
            container.push(Instructions::Instruction11n(Instruction11n{
                opcode,
                bytes,
                length: 1
            }));
            return 1;
        },

        OpCode::MONITOR_ENTER            | OpCode::MONITOR_EXIT
            | OpCode::MOVE_EXCEPTION     | OpCode::MOVE_RESULT
            | OpCode::MOVE_RESULT_OBJECT | OpCode::MOVE_RESULT_WIDE
            | OpCode::RETURN             | OpCode::RETURN_OBJECT
            | OpCode::RETURN_WIDE        | OpCode::THROW
            => {
                let mut bytes = [0u16; 1];
                bytes[0] = raw_opcode;
                container.push(Instructions::Instruction11x(Instruction11x{
                    opcode,
                    bytes,
                    length: 1
                }));
                return 1;
            },

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
            => {
                let mut bytes = [0u16; 1];
                bytes[0] = raw_opcode;
                container.push(Instructions::Instruction12x(Instruction12x{
                    opcode,
                    bytes,
                    length: 1
                }));
                return 1;
            },

        OpCode::GOTO_16 => {
            let mut bytes = [0u16; 2];
            bytes[0] = raw_opcode;
            bytes[1] = reader.read_u16().unwrap();
            container.push(Instructions::Instruction20t(Instruction20t{
                opcode,
                bytes,
                length: 2
            }));
            return 2;
        },

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
            => {
                let mut bytes = [0u16; 2];
                bytes[0] = raw_opcode;
                bytes[1] = reader.read_u16().unwrap();
                container.push(Instructions::Instruction21c(Instruction21c {
                    opcode,
                    bytes,
                    length: 2
                }));
                return 2;
            },

        OpCode::CONST_HIGH16 | OpCode::CONST_WIDE_HIGH16
            => {
                let mut bytes = [0u16; 2];
                bytes[0] = raw_opcode;
                bytes[1] = reader.read_u16().unwrap();
                container.push(Instructions::Instruction21h(Instruction21h {
                    opcode,
                    bytes,
                    length: 2
                }));
                return 2;
            },

        OpCode::CONST_16 | OpCode::CONST_WIDE_16
            => {
                let mut bytes = [0u16; 2];
                bytes[0] = raw_opcode;
                bytes[1] = reader.read_u16().unwrap();
                container.push(Instructions::Instruction21s(Instruction21s {
                    opcode,
                    bytes,
                    length: 2
                }));
                return 2;
            },

        OpCode::IF_EQZ       | OpCode::IF_GEZ
            | OpCode::IF_GTZ | OpCode::IF_LEZ
            | OpCode::IF_LTZ | OpCode::IF_NEZ
            => {
                let mut bytes = [0u16; 2];
                bytes[0] = raw_opcode;
                bytes[1] = reader.read_u16().unwrap();
                container.push(Instructions::Instruction21t(Instruction21t {
                    opcode,
                    bytes,
                    length: 2
                }));
                return 2;
            },

        OpCode::ADD_INT_LIT8        | OpCode::AND_INT_LIT8
            | OpCode::DIV_INT_LIT8  | OpCode::MUL_INT_LIT8
            | OpCode::OR_INT_LIT8   | OpCode::REM_INT_LIT8
            | OpCode::RSUB_INT_LIT8 | OpCode::SHL_INT_LIT8
            | OpCode::SHR_INT_LIT8  | OpCode::USHR_INT_LIT8
            | OpCode::XOR_INT_LIT8
            => {
                let mut bytes = [0u16; 2];
                bytes[0] = raw_opcode;
                bytes[1] = reader.read_u16().unwrap();
                container.push(Instructions::Instruction22b(Instruction22b {
                    opcode,
                    bytes,
                    length: 2
                }));
                return 2;
            },

        OpCode::IGET_BOOLEAN       | OpCode::IGET_BYTE
            | OpCode::IGET_CHAR    | OpCode::IGET
            | OpCode::IGET_OBJECT  | OpCode::IGET_SHORT
            | OpCode::IGET_WIDE    | OpCode::INSTANCE_OF
            | OpCode::IPUT_BOOLEAN | OpCode::IPUT_BYTE
            | OpCode::IPUT_CHAR    | OpCode::IPUT
            | OpCode::IPUT_OBJECT  | OpCode::IPUT_SHORT
            | OpCode::IPUT_WIDE    | OpCode::NEW_ARRAY
            => {
                let mut bytes = [0u16; 2];
                bytes[0] = raw_opcode;
                bytes[1] = reader.read_u16().unwrap();
                container.push(Instructions::Instruction22c(Instruction22c {
                    opcode,
                    bytes,
                    length: 2
                }));
                return 2;
            },

        OpCode::ADD_INT_LIT16       | OpCode::AND_INT_LIT16
            | OpCode::DIV_INT_LIT16 | OpCode::MUL_INT_LIT16
            | OpCode::OR_INT_LIT16  | OpCode::REM_INT_LIT16
            | OpCode::RSUB_INT      | OpCode::XOR_INT_LIT16
            => {
                let mut bytes = [0u16; 2];
                bytes[0] = raw_opcode;
                bytes[1] = reader.read_u16().unwrap();
                container.push(Instructions::Instruction22s(Instruction22s {
                    opcode,
                    bytes,
                    length: 2
                }));
                return 2;
            },

        OpCode::IF_EQ       | OpCode::IF_GE
            | OpCode::IF_GT | OpCode::IF_LE
            | OpCode::IF_LT | OpCode::IF_NE
            => {
                let mut bytes = [0u16; 2];
                bytes[0] = raw_opcode;
                bytes[1] = reader.read_u16().unwrap();
                container.push(Instructions::Instruction22t(Instruction22t {
                    opcode,
                    bytes,
                    length: 2
                }));
                return 2;
            },

        OpCode::MOVE_FROM16 | OpCode::MOVE_OBJECT_FROM16
            | OpCode::MOVE_WIDE_FROM16
            => {
                let mut bytes = [0u16; 2];
                bytes[0] = raw_opcode;
                bytes[1] = reader.read_u16().unwrap();
                container.push(Instructions::Instruction22x(Instruction22x {
                    opcode,
                    bytes,
                    length: 2
                }));
                return 2;
            },

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
            => {
                let mut bytes = [0u16; 2];
                bytes[0] = raw_opcode;
                bytes[1] = reader.read_u16().unwrap();
                container.push(Instructions::Instruction23x(Instruction23x {
                    opcode,
                    bytes,
                    length: 2
                }));
                return 2;
            },

        OpCode::GOTO_32 => {
            let mut bytes = [0u16; 3];
            bytes[0] = raw_opcode;
            bytes[1] = reader.read_u16().unwrap();
            bytes[2] = reader.read_u16().unwrap();
            container.push(Instructions::Instruction30t(Instruction30t {
                opcode,
                bytes,
                length: 3
            }));
            return 3;
        },

        OpCode::CONST_STRING_JUMBO => {
            let mut bytes = [0u16; 3];
            bytes[0] = raw_opcode;
            bytes[1] = reader.read_u16().unwrap();
            bytes[2] = reader.read_u16().unwrap();
            container.push(Instructions::Instruction31c(Instruction31c {
                opcode,
                bytes,
                length: 3
            }));
            return 3;
        },

        OpCode::CONST | OpCode::CONST_WIDE_32
            => {
                let mut bytes = [0u16; 3];
                bytes[0] = raw_opcode;
                bytes[1] = reader.read_u16().unwrap();
                bytes[2] = reader.read_u16().unwrap();
                container.push(Instructions::Instruction31i(Instruction31i {
                    opcode,
                    bytes,
                    length: 3
                }));
                return 3;
            },

        OpCode::FILL_ARRAY_DATA | OpCode::PACKED_SWITCH
            | OpCode::SPARSE_SWITCH => {
                let mut bytes = [0u16; 3];
                bytes[0] = raw_opcode;
                bytes[1] = reader.read_u16().unwrap();
                bytes[2] = reader.read_u16().unwrap();
                container.push(Instructions::Instruction31t(Instruction31t {
                    opcode,
                    bytes,
                    length: 3
                }));
                return 3;
            },

        OpCode::MOVE_16 | OpCode::MOVE_OBJECT_16
            | OpCode::MOVE_WIDE_16 => {
                let mut bytes = [0u16; 3];
                bytes[0] = raw_opcode;
                bytes[1] = reader.read_u16().unwrap();
                bytes[2] = reader.read_u16().unwrap();
                container.push(Instructions::Instruction32x(Instruction32x {
                    opcode,
                    bytes,
                    length: 3
                }));
                return 3;
            },

        OpCode::FILLED_NEW_ARRAY    | OpCode::INVOKE_CUSTOM
            | OpCode::INVOKE_DIRECT | OpCode::INVOKE_INTERFACE
            | OpCode::INVOKE_STATIC | OpCode::INVOKE_SUPER
            | OpCode::INVOKE_VIRTUAL
            => {
                let mut bytes = [0u16; 3];
                bytes[0] = raw_opcode;
                bytes[1] = reader.read_u16().unwrap();
                bytes[2] = reader.read_u16().unwrap();
                container.push(Instructions::Instruction35c(Instruction35c {
                    opcode,
                    bytes,
                    length: 3
                }));
                return 3;
            },

        OpCode::FILLED_NEW_ARRAY_RANGE    | OpCode::INVOKE_CUSTOM_RANGE
            | OpCode::INVOKE_DIRECT_RANGE | OpCode::INVOKE_INTERFACE_RANGE
            | OpCode::INVOKE_STATIC_RANGE | OpCode::INVOKE_SUPER_RANGE
            | OpCode::INVOKE_VIRTUAL_RANGE
            => {
                let mut bytes = [0u16; 3];
                bytes[0] = raw_opcode;
                bytes[1] = reader.read_u16().unwrap();
                bytes[2] = reader.read_u16().unwrap();
                container.push(Instructions::Instruction3rc(Instruction3rc {
                    opcode,
                    bytes,
                    length: 3
                }));
                return 3;
            },

        OpCode::INVOKE_POLYMORPHIC => {
            let mut bytes = [0u16; 4];
            bytes[0] = raw_opcode;
            bytes[1] = reader.read_u16().unwrap();
            bytes[2] = reader.read_u16().unwrap();
            bytes[3] = reader.read_u16().unwrap();
            container.push(Instructions::Instruction45cc(Instruction45cc {
                opcode,
                bytes,
                length: 4
            }));
            return 4;
        },

        OpCode::INVOKE_POLYMORPHIC_RANGE => {
            let mut bytes = [0u16; 4];
            bytes[0] = raw_opcode;
            bytes[1] = reader.read_u16().unwrap();
            bytes[2] = reader.read_u16().unwrap();
            bytes[3] = reader.read_u16().unwrap();
            container.push(Instructions::Instruction4rcc(Instruction4rcc {
                opcode,
                bytes,
                length: 4
            }));
            return 4;
        },

        OpCode::CONST_WIDE => {
            let mut bytes = [0u16; 5];
            bytes[0] = raw_opcode;
            bytes[1] = reader.read_u16().unwrap();
            bytes[2] = reader.read_u16().unwrap();
            bytes[3] = reader.read_u16().unwrap();
            bytes[4] = reader.read_u16().unwrap();
            container.push(Instructions::Instruction51l(Instruction51l {
                opcode,
                bytes,
                length: 5
            }));
            return 5;
        },

        OpCode::PACKED_SWITCH_PAYLOAD => {
            let inst = PackedSwitchPayload::build(reader);
            let len = inst.length();
            container.push(Instructions::PackedSwitchPayload(inst));
            reader.align_cursor();
            return len;
        },

        OpCode::SPARSE_SWITCH_PAYLOAD => {
            let inst = SparseSwitchPayload::build(reader);
            let len = inst.length();
            container.push(Instructions::SparseSwitchPayload(inst));
            reader.align_cursor();
            return len;
        },

        OpCode::FILL_ARRAY_DATA_PAYLOAD => {
            let inst = FillArrayDataPayload::build(reader);
            let len = inst.length();
            container.push(Instructions::FillArrayDataPayload(inst));
            reader.align_cursor();
            return len;
        }
    }
}

/////////////////////////////////////////////////////////////////

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

        assert_eq!(inst.inst_format(), "Instruction10x");
    }

    #[test]
    fn test_instruction11n() {
        let inst = Instruction11n {
            opcode: OpCode::NOP,
            length: 1,
            bytes: vec![0x2100]
        };

        assert_eq!(inst.inst_format(), "Instruction11n");
        assert_eq!(inst.a(), 1);
        assert_eq!(inst.b(), 2);
    }

    #[test]
    fn test_instruction12x() {
        let inst = Instruction12x {
            opcode: OpCode::NOP,
            length: 1,
            bytes: vec![0x2100]
        };

        assert_eq!(inst.inst_format(), "Instruction12x");
        assert_eq!(inst.a(), 1);
        assert_eq!(inst.b(), 2);
    }

    #[test]
    fn test_instruction11x() {
        let inst = Instruction11x {
            opcode: OpCode::NOP,
            length: 1,
            bytes: vec![0x1000]
        };

        assert_eq!(inst.inst_format(), "Instruction11x");
        assert_eq!(inst.a(), 0x10);
    }

    #[test]
    fn test_instruction10t() {
        let inst = Instruction10t {
            opcode: OpCode::NOP,
            length: 1,
            bytes: vec![0x1000]
        };

        assert_eq!(inst.inst_format(), "Instruction10t");
        assert_eq!(inst.a(), 0x10);
    }

    #[test]
    fn test_instruction20t() {
        let inst = Instruction20t {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x0000, 0x1234]
        };

        assert_eq!(inst.inst_format(), "Instruction20t");
        assert_eq!(inst.a(), 0x1234);
    }

    #[test]
    fn test_instruction21c() {
        let inst = Instruction21c {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.inst_format(), "Instruction21c");
        assert_eq!(inst.a(), 0x12);
        assert_eq!(inst.b(), 0x3456);
    }

    #[test]
    fn test_instruction21h() {
        let inst = Instruction21h {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.inst_format(), "Instruction21h");
        assert_eq!(inst.a(), 0x12);
        assert_eq!(inst.b(), 0x3456);
    }

    #[test]
    fn test_instruction21s() {
        let inst = Instruction21s {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.inst_format(), "Instruction21s");
        assert_eq!(inst.a(), 0x12);
        assert_eq!(inst.b(), 0x3456);
    }

    #[test]
    fn test_instruction21t() {
        let inst = Instruction21t {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.inst_format(), "Instruction21t");
        assert_eq!(inst.a(), 0x12);
        assert_eq!(inst.b(), 0x3456);
    }

    #[test]
    fn test_instruction22x() {
        let inst = Instruction22x {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.inst_format(), "Instruction22x");
        assert_eq!(inst.a(), 0x12);
        assert_eq!(inst.b(), 0x3456);
    }

    #[test]
    fn test_instruction23x() {
        let inst = Instruction23x {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.inst_format(), "Instruction23x");
        assert_eq!(inst.a(), 0x12);
        assert_eq!(inst.b(), 0x34);
        assert_eq!(inst.c(), 0x56);
    }

    #[test]
    fn test_instruction22b() {
        let inst = Instruction22b {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.inst_format(), "Instruction22b");
        assert_eq!(inst.a(), 0x12);
        assert_eq!(inst.b(), 0x34);
        assert_eq!(inst.c(), 0x56);
    }

    #[test]
    fn test_instruction22c() {
        let inst = Instruction22c {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.inst_format(), "Instruction22c");
        assert_eq!(inst.a(), 0x2);
        assert_eq!(inst.b(), 0x1);
        assert_eq!(inst.c(), 0x3456);
    }

    #[test]
    fn test_instruction22s() {
        let inst = Instruction22s {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.inst_format(), "Instruction22s");
        assert_eq!(inst.a(), 0x2);
        assert_eq!(inst.b(), 0x1);
        assert_eq!(inst.c(), 0x3456);
    }

    #[test]
    fn test_instruction22t() {
        let inst = Instruction22t {
            opcode: OpCode::NOP,
            length: 2,
            bytes: vec![0x1200, 0x3456]
        };

        assert_eq!(inst.inst_format(), "Instruction22t");
        assert_eq!(inst.a(), 0x2);
        assert_eq!(inst.b(), 0x1);
        assert_eq!(inst.c(), 0x3456);
    }

    #[test]
    fn test_instruction30t() {
        let inst = Instruction30t {
            opcode: OpCode::NOP,
            length: 3,
            bytes: vec![0x00, 0x5678, 0x1234]
        };

        assert_eq!(inst.inst_format(), "Instruction30t");
        assert_eq!(inst.a(), 0x12345678);
    }

    #[test]
    fn test_instruction31c() {
        let inst = Instruction31c {
            opcode: OpCode::NOP,
            length: 3,
            bytes: vec![0x1200, 0x5678, 0x1234]
        };

        assert_eq!(inst.inst_format(), "Instruction31c");
        assert_eq!(inst.a(), 0x12);
        assert_eq!(inst.b(), 0x12345678);
    }

    #[test]
    fn test_instruction31i() {
        let inst = Instruction31i {
            opcode: OpCode::NOP,
            length: 3,
            bytes: vec![0x1200, 0x5678, 0x1234]
        };

        assert_eq!(inst.inst_format(), "Instruction31i");
        assert_eq!(inst.a(), 0x12);
        assert_eq!(inst.b(), 0x12345678);
    }

    #[test]
    fn test_instruction31t() {
        let inst = Instruction31t {
            opcode: OpCode::NOP,
            length: 3,
            bytes: vec![0x1200, 0x5678, 0x1234]
        };

        assert_eq!(inst.inst_format(), "Instruction31t");
        assert_eq!(inst.a(), 0x12);
        assert_eq!(inst.b(), 0x12345678);
    }

    #[test]
    fn test_instruction32x() {
        let inst = Instruction32x {
            opcode: OpCode::NOP,
            length: 3,
            bytes: vec![0x00, 0x1234, 0x5678]
        };

        assert_eq!(inst.inst_format(), "Instruction32x");
        assert_eq!(inst.a(), 0x1234);
        assert_eq!(inst.b(), 0x5678);
    }

    #[test]
    fn test_instruction35c() {
        let inst = Instruction35c {
            opcode: OpCode::NOP,
            length: 3,
            bytes: vec![0x1200, 0x3456, 0x7890]
        };

        assert_eq!(inst.inst_format(), "Instruction35c");
        assert_eq!(inst.a(), 0x01);
        assert_eq!(inst.b(), 0x3456);
        assert_eq!(inst.c(), 0x00);
        assert_eq!(inst.d(), 0x09);
        assert_eq!(inst.e(), 0x08);
        assert_eq!(inst.f(), 0x07);
        assert_eq!(inst.g(), 0x02);
    }

    #[test]
    fn test_instruction3rc() {
        let inst = Instruction3rc {
            opcode: OpCode::NOP,
            length: 3,
            bytes: vec![0x1200, 0x3456, 0x7890]
        };

        assert_eq!(inst.inst_format(), "Instruction3rc");
        assert_eq!(inst.a(), 0x12);
        assert_eq!(inst.b(), 0x3456);
        assert_eq!(inst.c(), 0x7890);
    }

    #[test]
    fn test_instruction45cc() {
        let inst = Instruction45cc {
            opcode: OpCode::NOP,
            length: 4,
            bytes: vec![0x1200, 0x3456, 0x7890, 0x1234]
        };

        assert_eq!(inst.inst_format(), "Instruction45cc");
        assert_eq!(inst.a(), 0x01);
        assert_eq!(inst.b(), 0x03456);
        assert_eq!(inst.c(), 0x00);
        assert_eq!(inst.d(), 0x09);
        assert_eq!(inst.e(), 0x08);
        assert_eq!(inst.f(), 0x07);
        assert_eq!(inst.g(), 0x02);
        assert_eq!(inst.h(), 0x1234);
    }

    #[test]
    fn test_instruction4rcc() {
        let inst = Instruction4rcc {
            opcode: OpCode::NOP,
            length: 4,
            bytes: vec![0x1200, 0x3456, 0x7890, 0x1234]
        };

        assert_eq!(inst.inst_format(), "Instruction4rcc");
        assert_eq!(inst.a(), 0x12);
        assert_eq!(inst.b(), 0x3456);
        assert_eq!(inst.c(), 0x7890);
        assert_eq!(inst.h(), 0x1234);
    }

    #[test]
    fn test_instruction51l() {
        let inst = Instruction51l {
            opcode: OpCode::NOP,
            length: 5,
            bytes: vec![0x1200, 0x3456, 0x9012, 0x5678, 0x1234]
        };

        assert_eq!(inst.inst_format(), "Instruction51l");
        assert_eq!(inst.a(), 0x12);
        assert_eq!(inst.b(), 0x1234_5678_9012_3456);
    }

    #[test]
    fn test_packed_switch_payload() {
        let inst = PackedSwitchPayload {
            opcode: OpCode::PACKED_SWITCH_PAYLOAD,
            size: 4,
            first_key: 0,
            targets: vec![1, 2, 3, 4]
        };

        assert_eq!(inst.length(), 12);
        assert_eq!(inst.opcode(), OpCode::PACKED_SWITCH_PAYLOAD);
        assert_eq!(inst.bytes(), vec![]);
        assert_eq!(inst.inst_format(), "PackedSwitchPayload");

        // TODO: test instruction-specific functions
    }

    #[test]
    fn test_sparse_switch_payload() {
        let inst = SparseSwitchPayload {
            opcode: OpCode::SPARSE_SWITCH_PAYLOAD,
            size: 4,
            keys: vec![1, 2, 3, 4],
            targets: vec![1, 2, 3, 4]
        };

        assert_eq!(inst.length(), 18);
        assert_eq!(inst.opcode(), OpCode::SPARSE_SWITCH_PAYLOAD);
        assert_eq!(inst.bytes(), vec![]);
        assert_eq!(inst.inst_format(), "SparseSwitchPayload");

        // TODO: test instruction-specific functions
    }

    #[test]
    fn test_fill_array_data_payload() {
        let inst = FillArrayDataPayload {
            opcode: OpCode::FILL_ARRAY_DATA_PAYLOAD,
            element_width: 4,
            size: 4,
            data: vec![1, 2, 3, 4]
        };

        assert_eq!(inst.length(), 12);
        assert_eq!(inst.opcode(), OpCode::FILL_ARRAY_DATA_PAYLOAD);
        assert_eq!(inst.bytes(), vec![]);
        assert_eq!(inst.inst_format(), "FillArrayDataPayload");

        // TODO: test instruction-specific functions
    }
}

//    #[test]
//    fn test_parse_instruction() {
//        let bytes = [0x00, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction10x");
//
//        let bytes = [0x01, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x02, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22x");
//
//        let bytes = [0x03, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction32x");
//
//        let bytes = [0x04, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x05, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22x");
//
//        let bytes = [0x06, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction32x");
//
//        let bytes = [0x07, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x08, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22x");
//
//        let bytes = [0x09, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction32x");
//
//        let bytes = [0x0a, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction11x");
//
//        let bytes = [0x0b, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction11x");
//
//        let bytes = [0x0c, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction11x");
//
//        let bytes = [0x0d, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction11x");
//
//        let bytes = [0x0e, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction10x");
//
//        let bytes = [0x0f, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction11x");
//
//        let bytes = [0x10, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction11x");
//
//        let bytes = [0x11, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction11x");
//
//        let bytes = [0x12, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction11n");
//
//        let bytes = [0x13, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21s");
//
//        let bytes = [0x14, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction31i");
//
//        let bytes = [0x15, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21h");
//
//        let bytes = [0x16, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21s");
//
//        let bytes = [0x17, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction31i");
//
//        let bytes = [0x18, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction51l");
//
//        let bytes = [0x19, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21h");
//
//        let bytes = [0x1a, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x1b, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction31c");
//
//        let bytes = [0x1c, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x1d, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction11x");
//
//        let bytes = [0x1e, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction11x");
//
//        let bytes = [0x1f, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x20, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22c");
//
//        let bytes = [0x21, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x22, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x23, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22c");
//
//        let bytes = [0x24, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction35c");
//
//        let bytes = [0x25, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction3rc");
//
//        let bytes = [0x26, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction31t");
//
//        let bytes = [0x27, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction11x");
//
//        let bytes = [0x28, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction10t");
//
//        let bytes = [0x29, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction20t");
//
//        let bytes = [0x2a, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction30t");
//
//        let bytes = [0x2b, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction31t");
//
//        let bytes = [0x2c, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction31t");
//
//        let bytes = [0x2d, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x2e, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x2f, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x30, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x31, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x32, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22t");
//
//        let bytes = [0x33, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22t");
//
//        let bytes = [0x34, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22t");
//
//        let bytes = [0x35, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22t");
//
//        let bytes = [0x36, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22t");
//
//        let bytes = [0x37, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22t");
//
//        let bytes = [0x38, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21t");
//
//        let bytes = [0x39, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21t");
//
//        let bytes = [0x3a, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21t");
//
//        let bytes = [0x3b, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21t");
//
//        let bytes = [0x3c, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21t");
//
//        let bytes = [0x3d, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21t");
//
//        /* 0x3e..=0x43 : unused */
//
//        let bytes = [0x44, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x45, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x46, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x47, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x48, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x49, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x4a, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x4b, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x4c, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x4d, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x4e, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x4f, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x50, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x51, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x52, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22c");
//
//        let bytes = [0x53, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22c");
//
//        let bytes = [0x54, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22c");
//
//        let bytes = [0x55, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22c");
//
//        let bytes = [0x56, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22c");
//
//        let bytes = [0x57, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22c");
//
//        let bytes = [0x58, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22c");
//
//        let bytes = [0x59, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22c");
//
//        let bytes = [0x5a, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22c");
//
//        let bytes = [0x5b, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22c");
//
//        let bytes = [0x5c, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22c");
//
//        let bytes = [0x5d, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22c");
//
//        let bytes = [0x5e, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22c");
//
//        let bytes = [0x5f, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22c");
//
//        let bytes = [0x60, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x61, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x62, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x63, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x64, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x65, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x66, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x67, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x68, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x69, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x6a, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x6b, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x6c, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x6d, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0x6e, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction35c");
//
//        let bytes = [0x6f, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction35c");
//
//        let bytes = [0x70, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction35c");
//
//        let bytes = [0x71, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction35c");
//
//        let bytes = [0x72, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction35c");
//        
//        /* 0x73 : unused */
//
//        let bytes = [0x74, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction3rc");
//
//        let bytes = [0x75, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction3rc");
//
//        let bytes = [0x76, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction3rc");
//
//        let bytes = [0x77, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction3rc");
//
//        let bytes = [0x78, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction3rc");
//
//        /* 0x79 | 0x7a : unused */
//
//        let bytes = [0x7b, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x7c, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x7d, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x7e, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x7f, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x80, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x81, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x82, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x83, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x84, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x85, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x86, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x87, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x88, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x89, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x8a, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x8b, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x8c, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x8d, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x8e, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x8f, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0x90, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x91, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x92, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x93, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x94, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x95, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x96, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x97, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x98, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x99, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x9a, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x9b, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x9c, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x9d, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x9e, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0x9f, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xa0, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xa1, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xa2, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xa3, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xa4, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xa5, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xa6, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xa7, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xa8, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xa9, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xaa, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xab, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xac, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xad, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xae, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xaf, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction23x");
//
//        let bytes = [0xb0, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xb1, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xb2, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xb3, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xb4, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xb5, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xb6, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xb7, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xb8, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xb9, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xba, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xbb, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xbc, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xbd, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xbe, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xbf, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xc0, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xc1, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xc2, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xc3, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xc4, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xc5, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xc6, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xc7, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xc8, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xc9, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xca, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xcb, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xcc, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xcd, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xce, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xcf, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction12x");
//
//        let bytes = [0xd0, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22s");
//
//        let bytes = [0xd1, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22s");
//
//        let bytes = [0xd2, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22s");
//
//        let bytes = [0xd3, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22s");
//
//        let bytes = [0xd4, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22s");
//
//        let bytes = [0xd5, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22s");
//
//        let bytes = [0xd6, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22s");
//
//        let bytes = [0xd7, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22s");
//
//        let bytes = [0xd8, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22b");
//
//        let bytes = [0xd9, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22b");
//
//        let bytes = [0xda, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22b");
//
//        let bytes = [0xdb, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22b");
//
//        let bytes = [0xdc, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22b");
//
//        let bytes = [0xdd, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22b");
//
//        let bytes = [0xde, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22b");
//
//        let bytes = [0xdf, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22b");
//
//        let bytes = [0xe0, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22b");
//
//        let bytes = [0xe1, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22b");
//
//        let bytes = [0xe2, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction22b");
//
//        /* 0xe3..=0xf9 : unused */
//
//        let bytes = [0xfa, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction45cc");
//
//        let bytes = [0xfb, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction4rcc");
//
//        let bytes = [0xfc, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction35c");
//
//        let bytes = [0xfd, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction3rc");
//
//        let bytes = [0xfe, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//
//        let bytes = [0xff, 0x00, 0x00, 0x00, 0x00];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "Instruction21c");
//    }
//
//    #[test]
//    fn test_parse_packed_switch_payload() {
//        let bytes = [
//            0x0100,     // opcode
//            0x0002,     // size
//            0x0000,     // first_key
//            0x0000,     // ...
//            0x0000,      // targets
//            0x0000,     // ...
//            0x0000,     // ...
//            0x0001,     // ...
//        ];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "PackedSwitchPayload");
//
//        let any_inst = match inst.as_ref()
//                                 .as_any()
//                                 .downcast_ref::<PackedSwitchPayload>() {
//            Some(ins) => ins,
//            None      => {
//                error!("cannot access PackedSwitchPayload from Box");
//                panic!("error: cannot access PackedSwitchPayload from Box");
//            }
//        };
//        assert_eq!(any_inst.get_size(), 2);
//        assert_eq!(any_inst.get_first_key(), 0);
//        assert_eq!(any_inst.get_targets(), &vec![0x00, 0x0001_0000]);
//    }
//
//    #[test]
//    fn test_parse_sparse_switch_payload() {
//        let bytes = [
//            0x0200,     // opcode
//            0x0002,     // size
//            0x0000,     // keys
//            0x0000,     // ...
//            0x0000,     // ...
//            0x0001,     // ...
//            0x0000,     // targets
//            0x0002,     // ...
//            0x0000,     // ...
//            0x0003,     // ...
//        ];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "SparseSwitchPayload");
//
//        let any_inst = match inst.as_ref()
//                                 .as_any()
//                                 .downcast_ref::<SparseSwitchPayload>() {
//            Some(ins) => ins,
//            None      => {
//                error!("cannot access SparseSwitchPayload from Box");
//                panic!("error: cannot access SparseSwitchPayload from Box");
//            }
//        };
//        assert_eq!(any_inst.get_size(), 2);
//        assert_eq!(any_inst.get_keys(), &vec![0x00, 0x0001_0000]);
//        assert_eq!(any_inst.get_targets(), &vec![0x0002_0000, 0x0003_0000]);
//    }
//
//    #[test]
//    fn test_parse_fill_array_data_payload() {
//        let bytes = [
//            0x0300,     // opcode
//            0x0001,     // element width
//            0x0004,     // size
//            0x0000,     // ...
//            0x0100,     // data
//            0x0302,     // ...
//        ];
//        let inst = parse(&bytes, 0, &DexEndianness::LittleEndian);
//        assert_eq!(inst.inst_format(), "FillArrayDataPayload");
//
//        let any_inst = match inst.as_ref()
//                                 .as_any()
//                                 .downcast_ref::<FillArrayDataPayload>() {
//            Some(ins) => ins,
//            None      => {
//                error!("cannot access FillArrayDataPayload from Box");
//                panic!("error: cannot access FillArrayDataPayload from Box");
//            }
//        };
//        assert_eq!(any_inst.get_element_width(), 1);
//        assert_eq!(any_inst.get_size(), 4);
//        assert_eq!(any_inst.get_data(), &vec![0, 1, 2, 3]);
//    }
//    */
//}
