use huffman::{Error, Result};

fn main() {
    compress_utils::launch(decode)
}

fn decode(input_file_path: &str, output_file_path: &str) -> Result<()> {
    let mut input_iter = compress_utils::file::file_iter(input_file_path);

    match huffman::decode(&mut input_iter) {
        None => Err(Error::new("Failed to read the input")),
        Some(Err(err)) => Err(err),
        Some(Ok(decoded)) => {
            compress_utils::file::write_iter_result(output_file_path, Box::new(decoded))?;
            Ok(())
        }
    }
}
