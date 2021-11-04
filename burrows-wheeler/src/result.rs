use std::io::Error as IoError;

#[derive(Debug, PartialEq, Eq)]
pub struct Error {
    pub message: String,
}

impl From<IoError> for Error {
    fn from(io_error: IoError) -> Self {
        Error {
            message: io_error.to_string(),
        }
    }
}

impl Error {
    pub fn new(str: &str) -> Error {
        Error {
            message: String::from(str),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
