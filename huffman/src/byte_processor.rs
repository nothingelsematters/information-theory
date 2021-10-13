use std::fs::File;
use std::io::{BufReader, BufWriter, Error as IoError, Read, Write};

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
pub type BoxedByteIterator = Box<dyn Iterator<Item = Result<u8>>>;

pub trait ByteProcessor {
    fn process<F>(input_iter_supplier: F) -> Result<BoxedByteIterator>
    where
        F: Fn() -> Result<BoxedByteIterator>;

    fn process_file(input_file_path: &str) -> Result<BoxedByteIterator> {
        Self::process(|| {
            let file = File::open(input_file_path)?;
            let buf = BufReader::new(file);
            let iter = buf.bytes().map(|result| result.map_err(Error::from));
            Ok(Box::new(iter))
        })
    }

    fn write_processed(input_file_path: &str, output_file_path: &str) -> Result<()> {
        let mut iter = Self::process_file(input_file_path)?;

        let output_file = File::create(output_file_path)?;
        let mut buf_writer = BufWriter::new(output_file);

        while let Some(value) = iter.next() {
            match value {
                Err(err) => return Err(err),
                Ok(byte) => {
                    buf_writer.write(&[byte])?;
                }
            }
        }

        buf_writer.flush()?;
        Ok(())
    }
}
