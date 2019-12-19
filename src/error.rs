use crate::Range;
use core::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    InvalidDV { must_be: char, instead: char },
    InvalidFormat,
    OutOfRange,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidFormat => write!(f, "The input format is invalid"),
            Error::InvalidDV { must_be, instead } => {
                write!(f, "Invalid DV, must be {}, instead {}.", must_be, instead)
            }
            Error::OutOfRange => write!(
                f,
                "The input number must be between {} to {}",
                Range::MIN.to_string(),
                Range::MAX.to_string()
            ),
        }
    }
}
