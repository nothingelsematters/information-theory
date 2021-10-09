use huffman::byte_processor::{ByteProcessor, Error, Result};
use huffman::decode::Decoder;
use huffman::encode::Encoder;
use std::fs::File;
use std::io::Read;

fn file_iter(input_file_name: &str) -> Result<Box<dyn Iterator<Item = Result<u8>>>> {
    let file = File::open(format!("files/{}", input_file_name))?;
    let iter = file.bytes().into_iter().map(|result| {
        result.map_err(|err| Error {
            message: err.to_string(),
        })
    });
    Ok(Box::new(iter))
}

fn test_file(input_file_path: &str) -> Result<()> {
    let decoded_iter = Decoder::process(|| Encoder::process(|| file_iter(input_file_path)))?;
    assert!(file_iter(input_file_path)?.eq(decoded_iter));
    Ok(())
}

#[test]
fn poe_test() -> Result<()> {
    test_file("poe.txt")
}

#[test]
fn war_and_peace_test() -> Result<()> {
    test_file("war&peace.txt")
}
