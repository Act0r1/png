use std::error::Error;
use std::fmt::Display;

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
                write!(f, "Invalid symbol in str!")
            }
        }
    }
}
impl Error for ChunkTypeError {}

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidLength(usize),
    InvalidSymbol,
}
