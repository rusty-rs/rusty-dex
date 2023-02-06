use std::io::Read;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use crate::error::DexError;

/* Endianness constants */
const ENDIAN_CONSTANT: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
const REVERSE_ENDIAN_CONSTANT: [u8; 4] = [0x78, 0x56, 0x34, 0x12];

#[derive(Debug)]
pub enum DexEndianness {
    LittleEndian,
    BigEndian,
}

#[derive(Debug)]
pub struct DexCursor<'a> {
    pub bytes: &'a [u8],
    pub endianness: DexEndianness,
}

impl <'a> DexCursor<'a> {
    pub fn check_endianness(bytes: &[u8]) -> Result<DexEndianness, DexError> {
        if bytes.len() < 44 {
            return Err(DexError::new("Error: DEX header too short"));
        }

        let endian_tag = &bytes[40..44];
        // try_into to convert the slice into an array
        match endian_tag.try_into().unwrap() {
            ENDIAN_CONSTANT => Ok(DexEndianness::BigEndian),
            REVERSE_ENDIAN_CONSTANT => Ok(DexEndianness::LittleEndian),
            _ => panic!("Error: invalid endian tag in DEX header")
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, DexError> {
        if self.bytes.len() < 1 {
            return Err(DexError::new("Error: no data left to read"));
        }

        Ok(self.bytes.read_u8().unwrap())
    }

    pub fn read_u16(&mut self) -> Result<u16, DexError> {
        if self.bytes.len() < 2 {
            return Err(DexError::new("Error: no data left to read"));
        }

        match self.endianness {
            DexEndianness::BigEndian => Ok(self.bytes.read_u16::<BigEndian>().unwrap()),
            DexEndianness::LittleEndian => Ok(self.bytes.read_u16::<LittleEndian>().unwrap()),
        }
    }

    pub fn read_u32(&mut self) -> Result<u32, DexError> {
        if self.bytes.len() < 4 {
            return Err(DexError::new("Error: no data left to read"));
        }

        match self.endianness {
            DexEndianness::BigEndian => Ok(self.bytes.read_u32::<BigEndian>().unwrap()),
            DexEndianness::LittleEndian => Ok(self.bytes.read_u32::<LittleEndian>().unwrap()),
        }
    }
}
