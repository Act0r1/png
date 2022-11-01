use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidLength(usize),
    InvalidSymbol,
    TooLong,
    WrongCrc,
    InvalidHeader,
    FilledAllBuffer,
}
impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkTypeError::InvalidLength(actual) => {
                write!(
                    f,
                    "Your str have length:{}, but in can't be more than 4",
                    actual
                )
            }
            ChunkTypeError::InvalidSymbol => {
                write!(f, "Invalid symbol in str! You str should contain only UTF-8 symbols")
            }
            ChunkTypeError::TooLong => {
                write!(f, "Too long")
            }
            ChunkTypeError::WrongCrc => {
                write!(f, "Wrong Crc")
            }
            ChunkTypeError::InvalidHeader => {
                write!(f," Wrong Png Header, header should be equal -> [137, 80, 78, 71, 13, 10, 26, 10]")
            }
            ChunkTypeError::FilledAllBuffer => {
                write!(f, "Filled all buffer")
            }
        }
    }
}
impl Error for ChunkTypeError {}

