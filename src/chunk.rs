use std::{
    fmt,
    io::{BufReader, Read},
    str::FromStr,
};

use crate::{chunk_type::ChunkType, err::ChunkTypeError, Error};

#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    crc: u32,
    data: Vec<u8>,
}

const MAX_LENGTH: u32 = (1 << 31) - 1;

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);
        let mut buff: [u8; 4] = [0; 4];
        reader.read_exact(&mut buff)?;
        let length = u32::from_be_bytes(buff);
        if length > MAX_LENGTH {
            return Err(Box::from(ChunkTypeError::TooLong));
        }
        reader.read_exact(&mut buff)?;
        let chunk_type = ChunkType::try_from(buff).unwrap();
        let mut data: Vec<u8> = vec![0; usize::try_from(length)?];
        reader.read_exact(&mut data)?;
        println!("read data");
        reader.read_exact(&mut buff)?;

        println!("read buff");
        let crc = u32::from_be_bytes(buff);

        let actual_crc =
            crc::crc32::checksum_ieee(&[&chunk_type.bytes(), data.as_slice()].concat());

        if crc != actual_crc {
            return Err(Box::new(ChunkTypeError::WrongCrc));
        }

        Ok(Self {
            chunk_type,
            length,
            crc,
            data,
        })
    }
}
impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {:?} bytes", self.data_as_string())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc = crc::crc32::checksum_ieee(&[&chunk_type.bytes(), data.as_slice()].concat());
        Self {
            chunk_type,
            length: data.len() as u32,
            crc,
            data,
        }
    }

    fn length(&self) -> u32 {
        self.length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    fn data(&self) -> &[u8] {
        self.data.iter().as_slice()
    }
    fn crc(&self) -> u32 {
        self.crc
    }
    /// Returns the data stored in this chunk as a `String`. This function will return an error
    /// if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> Result<String, ChunkTypeError> {
        Ok(String::from_utf8(self.data.clone()).unwrap())
    }
    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
