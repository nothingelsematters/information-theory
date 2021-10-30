use huffman::Result;
use std::fs::File;
use std::io::Read;

fn file_iter(input_file_name: &str) -> Box<dyn Iterator<Item = u8>> {
    let file = match File::open(format!("files/{}", input_file_name)) {
        Err(err) => panic!("File not found: {}", err),
        Ok(file) => file,
    };

    let iter = file
        .bytes()
        .into_iter()
        .take_while(|x| x.is_ok())
        .map(|x| x.unwrap());
    Box::new(iter)
}

fn test_file(input_file_path: &str) -> Result<()> {
    let mut encoded = huffman::encode(|| file_iter(input_file_path));
    let decoded_iter = huffman::decode(&mut encoded).unwrap()?;
    assert!(file_iter(input_file_path).map(Ok).eq(decoded_iter));
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
