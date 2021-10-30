use std::fs::File;
use std::io::BufReader;

fn main() {
    compress_utils::launch(|input_file_path, output_file_path| {
        let read = BufReader::new(File::open(input_file_path)?);
        let encoded = burrows_wheeler::encode(Box::new(read));
        compress_utils::file::write_iter(output_file_path, encoded)
    });
}
