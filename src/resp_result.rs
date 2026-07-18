use std::fmt::{self};

#[derive(Debug)]
pub enum RESPError {
    OutOfBounds(usize),
}

impl fmt::Display for RESPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RESPError::OutOfBounds(index) => write!(f, "Out of bounds of index {}", index),
        }
    }
}

pub type RESPResult<T> = Result<T, RESPError>;
