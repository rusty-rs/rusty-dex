use std::io::{ Cursor, Seek, SeekFrom };
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
pub struct DexReader {
    pub bytes: Cursor<Vec<u8>>,
    pub bytes_len: u64,
    pub endianness: DexEndianness,
}

impl DexReader {
    pub fn build(raw_dex: Vec<u8>) -> Self {
        let endianness = DexReader::check_endianness(&raw_dex).unwrap();

        let mut bytes = Cursor::new(raw_dex);

        let bytes_len = bytes.seek(SeekFrom::End(0)).unwrap();
        bytes.rewind().unwrap();

        DexReader {
            bytes,
            bytes_len,
            endianness
        }
    }

    pub fn check_endianness(bytes: &[u8]) -> Result<DexEndianness, DexError> {
        // Cannot use self here as we need to know the endianness before anything else

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
        if self.bytes.position() == self.bytes_len {
            return Err(DexError::new("Error: no data left to read"));
        }

        Ok(self.bytes.read_u8().unwrap())
    }

    pub fn read_u16(&mut self) -> Result<u16, DexError> {
        if self.bytes_len - self.bytes.position() < 2 {
            return Err(DexError::new("Error: no data left to read"));
        }

        match self.endianness {
            DexEndianness::BigEndian => Ok(self.bytes.read_u16::<BigEndian>().unwrap()),
            DexEndianness::LittleEndian => Ok(self.bytes.read_u16::<LittleEndian>().unwrap()),
        }
    }

    pub fn read_u32(&mut self) -> Result<u32, DexError> {
        if self.bytes_len - self.bytes.position() < 4 {
            return Err(DexError::new("Error: no data left to read"));
        }

        match self.endianness {
            DexEndianness::BigEndian => Ok(self.bytes.read_u32::<BigEndian>().unwrap()),
            DexEndianness::LittleEndian => Ok(self.bytes.read_u32::<LittleEndian>().unwrap()),
        }
    }

    pub fn read_uleb128(&mut self) -> Result<(u32, usize), DexError> {
        let mut bytes_read: usize = 0;
        let mut result: u32 = 0;
        let mut shift = 0;

        loop {
            let byte = self.bytes.read_u8().unwrap();
            bytes_read += 1;
            let payload = (byte & 0b0111_1111) as u32;
            result |= payload << shift;
            shift += 7;

            if (byte & 0b1000_0000) == 0 {
                break;
            }

            if bytes_read >= 5 {
                return Err(DexError::new("Error: too many bytes in unsigned LEB128 value"));
            }
        }

        Ok((result, bytes_read))
    }

    pub fn read_sleb128(&mut self) -> Result<(i32, usize), DexError> {
        let mut bytes_read: usize = 0;
        let mut result: u32 = 0;
        let mut shift = 0;
        let mut byte;

        loop {
            byte = self.bytes.read_u8().unwrap() as u32;
            bytes_read += 1;
            let payload = byte & 0b0111_1111;
            result |= payload << shift;

            shift += 7;

            if (byte & 0b1000_0000) == 0 {
                break;
            }

            if bytes_read >= 5 {
                return Err(DexError::new("Error: too many bytes in unsigned LEB128 value"));
            }
        }

        let mut result = result as i32;
        if (byte & 0b0100_0000) == 0b0100_0000 {
            /* sign extend */
            result |= -(1 << shift);
        }

        Ok((result, bytes_read))
    }

    pub fn read_uleb128p1(&mut self) -> Result<(i32, usize), DexError> {
        let (uleb128, bytes_read) = self.read_uleb128().unwrap();
        Ok(((uleb128 as i32) - 1, bytes_read))
    }
}
