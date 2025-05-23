//! DEX reader
//!
//! This module defines all the methods to read bytes from the DEX file while respecint ght
//! endianess which can change from one DEX file to another.

use std::fs::File;
use std::io::{ Read, Cursor, Seek, SeekFrom };
use zip::ZipArchive;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use crate::error::DexError;

/// Little-endian DEX file
const ENDIAN_CONSTANT: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
/// Big-endian DEX file
const REVERSE_ENDIAN_CONSTANT: [u8; 4] = [0x78, 0x56, 0x34, 0x12];

/// Possible endianness
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DexEndianness {
    LittleEndian,
    BigEndian,
}

/// A reader for a DEX file
#[derive(Debug)]
pub struct DexReader {
    /// A cursor over the bytes of the DEX file
    pub bytes: Cursor<Vec<u8>>,
    /// Number of bytes in the DEX file
    pub bytes_len: u64,
    /// Endianness of the DEX file
    pub endianness: DexEndianness,
}

impl DexReader {
    /// Open the file at the given path and create reader(s)
    ///
    /// Each APK can contain multiple DEX files. This function extracts them all, create a reader
    /// from each, and returns a vector of readers.
    pub fn build_from_file(filepath: &str) -> Result<Vec<DexReader>, DexError> {
        let raw_file = File::open(filepath)
            .unwrap_or_else(|err| panic!("could not open input file: {err}"));
        let mut zip_file = ZipArchive::new(raw_file)
            .unwrap_or_else(|err| panic!("could not create ZipArchive object: {err}"));

        let dex_entries_names = zip_file.file_names()
                                        .filter(|name| name.ends_with(".dex"))
                                        .map(|name| name.to_string())
                                        .collect::<Vec<String>>();

        let mut readers = Vec::new();
        for entry in dex_entries_names.iter() {
            let mut dex_entry = zip_file.by_name(entry)
                                        .unwrap_or_else(|_| panic!("cannot find classes.dex in the APK"));
            let mut raw_dex = Vec::new();
            dex_entry.read_to_end(&mut raw_dex)
                     .unwrap_or_else(|err| panic!("Could not read input file: {err}"));
            let reader = DexReader::build(raw_dex)?;
            readers.push(reader);
        }

        Ok(readers)
    }

    /// Read a DEX file and create a reader from it
    pub fn build(raw_dex: Vec<u8>) -> Result<Self, DexError> {
        let endianness = DexReader::check_endianness(&raw_dex)?;

        let mut bytes = Cursor::new(raw_dex);
        let bytes_len = bytes.seek(SeekFrom::End(0))?;
        bytes.rewind()?;

        Ok(DexReader {
            bytes,
            bytes_len,
            endianness
        })
    }

    /// Check the endianness of a DEX file
    pub fn check_endianness(bytes: &[u8]) -> Result<DexEndianness, DexError> {
        // Cannot use self here as we need to know the endianness before anything else

        if bytes.len() < 44 {
            return Err(DexError::DexHeaderTooShortError);
        }

        let endian_tag = &bytes[40..44];

        // try_into to convert the slice into an array
        match endian_tag.try_into().unwrap() {
            ENDIAN_CONSTANT => Ok(DexEndianness::BigEndian),
            REVERSE_ENDIAN_CONSTANT => Ok(DexEndianness::LittleEndian),
            _ => Err(DexError::InvalidEndianessTag)
        }
    }

    /// Check if the cursor is on an even-numbered bytecode offsets
    /// and, if not, consume data until it is
    pub fn align_cursor(&mut self) -> Result<(), DexError> {
        while self.bytes.position() % 2 != 0 {
            let _ = self.read_u8()?;
        }

        Ok(())
    }

    /// Read an unsigned 8 bits integer from the reader
    pub fn read_u8(&mut self) -> Result<u8, DexError> {
        if self.bytes.position() >= self.bytes_len {
            return Err(DexError::NoDataLeftError);
        }

        Ok(self.bytes.read_u8()?)
    }

    /// Read an unsigned 16 bits integer from the reader
    pub fn read_u16(&mut self) -> Result<u16, DexError> {
        if self.bytes.position() > self.bytes_len - 2 {
            return Err(DexError::NoDataLeftError);
        }

        match self.endianness {
            DexEndianness::BigEndian => Ok(self.bytes.read_u16::<BigEndian>()?),
            DexEndianness::LittleEndian => Ok(self.bytes.read_u16::<LittleEndian>()?),
        }
    }

    /// Read an unsigned 32 bits integer from the reader
    pub fn read_u32(&mut self) -> Result<u32, DexError> {
        if self.bytes.position() > self.bytes_len - 4 {
            return Err(DexError::NoDataLeftError);
        }

        match self.endianness {
            DexEndianness::BigEndian => Ok(self.bytes.read_u32::<BigEndian>()?),
            DexEndianness::LittleEndian => Ok(self.bytes.read_u32::<LittleEndian>()?),
        }
    }

    /// Read a signed 32 bits integer from the reader
    pub fn read_i32(&mut self) -> Result<i32, DexError> {
        if self.bytes.position() > self.bytes_len - 4 {
            return Err(DexError::NoDataLeftError);
        }

        match self.endianness {
            DexEndianness::BigEndian => Ok(self.bytes.read_i32::<BigEndian>()?),
            DexEndianness::LittleEndian => Ok(self.bytes.read_i32::<LittleEndian>()?),
        }
    }

    /// Read an unsigned LEB128 value from the reader
    pub fn read_uleb128(&mut self) -> Result<(u32, usize), DexError> {
        let mut bytes_read: usize = 0;
        let mut result: u32 = 0;
        let mut shift = 0;

        loop {
            let byte = self.bytes.read_u8()?;
            bytes_read += 1;
            let payload = (byte & 0b0111_1111) as u32;
            result |= payload << shift;
            shift += 7;

            if (byte & 0b1000_0000) == 0 {
                break;
            }

            if bytes_read >= 5 {
                return Err(DexError::InvalidUleb128Value);
            }
        }

        Ok((result, bytes_read))
    }

    /// Read a signed LEB128 value from the reader
    pub fn read_sleb128(&mut self) -> Result<(i32, usize), DexError> {
        let mut bytes_read: usize = 0;
        let mut result: u32 = 0;
        let mut shift = 0;
        let mut byte;

        loop {
            byte = self.bytes.read_u8()? as u32;
            bytes_read += 1;
            let payload = byte & 0b0111_1111;
            result |= payload << shift;

            shift += 7;

            if (byte & 0b1000_0000) == 0 {
                break;
            }

            if bytes_read >= 5 {
                return Err(DexError::InvalidSleb128Value);
            }
        }

        let mut result = result as i32;
        if (byte & 0b0100_0000) == 0b0100_0000 {
            /* sign extend */
            result |= -(1 << shift);
        }

        Ok((result, bytes_read))
    }

    /// Read a signed LEB128p1 value from the reader
    pub fn read_uleb128p1(&mut self) -> Result<(i32, usize), DexError> {
        match self.read_uleb128() {
            Ok((uleb128, bytes_read)) => Ok(((uleb128 as i32) - 1, bytes_read)),
            Err(_) => Err(DexError::InvalidUleb128p1Value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEX_DATA: [u8; 50] = [
        0x64, 0x65, 0x78, 0x0a, 0x30, 0x33, 0x35, 0x00, 0x00, 0x00,  // DEX magic
        0x7f, 0xdf, 0x80, 0x01, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00,  // uleb128 data
        0x7f, 0x80, 0x7f, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00,  // sleb128 data
        0x7f, 0xdf, 0x00, 0x00, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00,  // uleb128p1 data
        0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // endianness tag
    ];

    #[test]
    fn test_build() {
        let dex_reader = DexReader::build(DEX_DATA.to_vec()).unwrap();
        assert_eq!(dex_reader.bytes_len, DEX_DATA.len() as u64);
        assert_eq!(dex_reader.endianness, DexEndianness::LittleEndian);
    }

    #[test]
    fn test_check_endianness() {
        let dex_reader = DexReader::build(DEX_DATA.to_vec()).unwrap();
        let endianness = DexReader::check_endianness(&DEX_DATA).unwrap();
        assert_eq!(endianness, DexEndianness::LittleEndian);
        assert_eq!(dex_reader.endianness, endianness);
    }

    #[test]
    fn test_check_endianness_invalid() {
        let invalid_data = vec![0x00; 10];
        let result = DexReader::check_endianness(&invalid_data);
        assert_eq!(
            result.unwrap_err().to_string(),
            "DEX header too short"
        );
    }

    #[test]
    fn test_check_endianness_invalid_long() {
        let invalid_long_data = vec![0x00; 100];
        let result = DexReader::check_endianness(&invalid_long_data);
        assert_eq!(
            result.unwrap_err().to_string(),
            "invalid endianness tag"
        );
    }

    #[test]
    fn test_read_u8() {
        let mut dex_reader = DexReader::build(DEX_DATA.to_vec()).unwrap();
        let byte = dex_reader.read_u8().unwrap();
        assert_eq!(byte, 0x64);

        // Test reading at and after end of file
        dex_reader.bytes.seek(SeekFrom::End(0)).unwrap();
        let result = dex_reader.read_u8();
        assert_eq!(
            result.unwrap_err().to_string(),
            "no data left to read"
        );

        let bound = DEX_DATA.len() + 10;
        dex_reader.bytes.seek(SeekFrom::Start(bound as u64)).unwrap();
        let result = dex_reader.read_u8();
        assert_eq!(
            result.unwrap_err().to_string(),
            "no data left to read"
        );
    }

    #[test]
    fn test_read_u16() {
        let mut dex_reader = DexReader::build(DEX_DATA.to_vec()).unwrap();
        let u16_val = dex_reader.read_u16().unwrap();
        assert_eq!(u16_val, 0x6564);

        // Test reading at and after end of file
        dex_reader.bytes.seek(SeekFrom::End(0)).unwrap();
        let result = dex_reader.read_u16();
        assert_eq!(
            result.unwrap_err().to_string(),
            "no data left to read"
        );

        let bound = DEX_DATA.len() + 10;
        dex_reader.bytes.seek(SeekFrom::Start(bound as u64)).unwrap();
        let result = dex_reader.read_u16();
        assert_eq!(
            result.unwrap_err().to_string(),
            "no data left to read"
        );
    }

    #[test]
    fn test_read_u32() {
        let mut dex_reader = DexReader::build(DEX_DATA.to_vec()).unwrap();
        let u32_val = dex_reader.read_u32().unwrap();
        assert_eq!(u32_val, 0x0a786564);

        // Test reading at and after end of file
        dex_reader.bytes.seek(SeekFrom::End(0)).unwrap();
        let result = dex_reader.read_u32();
        assert_eq!(
            result.unwrap_err().to_string(),
            "no data left to read"
        );

        let bound = DEX_DATA.len() + 10;
        dex_reader.bytes.seek(SeekFrom::Start(bound as u64)).unwrap();
        let result = dex_reader.read_u32();
        assert_eq!(
            result.unwrap_err().to_string(),
            "no data left to read"
        );
    }

    #[test]
    fn test_read_uleb128() {
        let mut reader = DexReader::build(DEX_DATA.to_vec()).unwrap();
        reader.bytes.seek(SeekFrom::Start(10)).unwrap();

        let result = reader.read_uleb128().unwrap();
        assert_eq!(result, (0x7f, 1));

        let result = reader.read_uleb128().unwrap();
        assert_eq!(result, (0x405f, 3));

        let result = reader.read_uleb128();
        assert_eq!(
            result.unwrap_err().to_string(),
            "too many bytes in unsigned LEB128 value"
        );

        let dex_data = [
            0x64, 0x65, 0x78, 0x0a, 0x30, 0x33, 0x35, 0x00, 0x00, 0x00,  // DEX magic
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // padding
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // padding
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // padding
            0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // endianness tag
            0x00,                                                        // 0
            0x01,                                                        // 1
            0x7f,                                                        // 127
            0x80, 0x7f,                                                  // 16256
            0xb4, 0x07,                                                  // 0x3b4
            0x8c, 0x08,                                                  // 0x40c
            0xff, 0xff, 0xff, 0xff, 0xf                                  // 0xffffffff
        ];

        let mut reader = DexReader::build(dex_data.to_vec()).unwrap();
        reader.bytes.seek(SeekFrom::Start(50)).unwrap();

        let result = reader.read_uleb128().unwrap();
        assert_eq!(result, (0, 1));

        let result = reader.read_uleb128().unwrap();
        assert_eq!(result, (1, 1));

        let result = reader.read_uleb128().unwrap();
        assert_eq!(result, (127, 1));

        let result = reader.read_uleb128().unwrap();
        assert_eq!(result, (16256, 2));

        let result = reader.read_uleb128().unwrap();
        assert_eq!(result, (0x3b4, 2));

        let result = reader.read_uleb128().unwrap();
        assert_eq!(result, (0x40c, 2));

        let result = reader.read_uleb128().unwrap();
        assert_eq!(result, (0xffffffff, 5));
    }

    #[test]
    fn test_read_sleb128() {
        let mut reader = DexReader::build(DEX_DATA.to_vec()).unwrap();
        reader.bytes.seek(SeekFrom::Start(20)).unwrap();

        let result = reader.read_sleb128().unwrap();
        assert_eq!(result, (-1, 1));

        let result = reader.read_sleb128().unwrap();
        assert_eq!(result, (-128, 2));

        let result = reader.read_sleb128();
        assert_eq!(
            result.unwrap_err().to_string(),
            "too many bytes in signed LEB128 value"
        );

        let dex_data = [
            0x64, 0x65, 0x78, 0x0a, 0x30, 0x33, 0x35, 0x00, 0x00, 0x00,  // DEX magic
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // padding
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // padding
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // padding
            0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,  // endianness tag
            0x00,                                                        // 0
            0x01,                                                        // 1
            0x7f,                                                        // -1
            0x80, 0x7f,                                                  // -128
            0x3c,                                                        // 0x3c
        ];

        let mut reader = DexReader::build(dex_data.to_vec()).unwrap();
        reader.bytes.seek(SeekFrom::Start(50)).unwrap();

        let result = reader.read_sleb128().unwrap();
        assert_eq!(result, (0, 1));

        let result = reader.read_sleb128().unwrap();
        assert_eq!(result, (1, 1));

        let result = reader.read_sleb128().unwrap();
        assert_eq!(result, (-1, 1));

        let result = reader.read_sleb128().unwrap();
        assert_eq!(result, (-128, 2));

        let result = reader.read_sleb128().unwrap();
        assert_eq!(result, (0x3c, 1));
    }

    #[test]
    fn test_read_uleb128p1() {
        let mut reader = DexReader::build(DEX_DATA.to_vec()).unwrap();
        reader.bytes.seek(SeekFrom::Start(30)).unwrap();

        let result = reader.read_uleb128p1().unwrap();
        assert_eq!(result, (0x7e, 1));

        let result = reader.read_uleb128p1().unwrap();
        assert_eq!(result, (0x5e, 2));

        let result = reader.read_uleb128p1().unwrap();
        assert_eq!(result, (-1, 1));

        let result = reader.read_uleb128p1();
        assert_eq!(
            result.unwrap_err().to_string(),
            "too many bytes in unsigned LEB128p1 value"
        );
    }
}
