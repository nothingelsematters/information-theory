use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result as IoResult, Write};

pub fn file_iter(input_file_path: &str) -> Box<dyn Iterator<Item = u8>> {
    let file = match File::open(input_file_path) {
        Err(err) => panic!("Could not open file: {}", err.to_string()),
        Ok(file) => file,
    };

    let buf_reader = BufReader::new(file);
    let bytes = buf_reader
        .bytes()
        .take_while(|x| x.is_ok())
        .map(|x| x.unwrap());
    Box::new(bytes)
}

pub fn write_iter<'a>(
    output_file_path: &str,
    iter: Box<dyn Iterator<Item = u8> + 'a>,
) -> IoResult<()> {
    let mut buf_writer = BufWriter::new(File::open(output_file_path)?);
    iter.map(|x| buf_writer.write(&[x]).map(|_| ())).collect()
}

pub fn write_iter_result<'a, E: Debug>(
    output_file_path: &str,
    iter: Box<dyn Iterator<Item = Result<u8, E>> + 'a>,
) -> IoResult<()> {
    let mut buf_writer = BufWriter::new(File::open(output_file_path)?);

    for i in iter {
        match i {
            Ok(byte) => {
                buf_writer.write(&[byte])?;
            }
            Err(_) => {
                panic!();
            }
        }
    }

    Ok(())
}
