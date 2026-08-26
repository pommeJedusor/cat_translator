use std::{error, fmt, io};

pub enum Error {
    InvalidInput(String),
    Stdin(io::Error),
}

impl error::Error for Error {}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidInput(reason) => write!(f, "Input is not valid: {}", reason),
            Self::Stdin(error) => write!(f, "failed to read standard input: {}", error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidInput(reason) => write!(f, "Input is not valid: {}", reason),
            Self::Stdin(error) => write!(f, "failed to read standard input: {}", error),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Stdin(error)
    }
}
