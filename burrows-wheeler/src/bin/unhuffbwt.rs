use std::fs::File;
use std::io::BufReader;

fn main() {
    compress_utils::launch(|input_file_path: &str, output_file_path: &str| {
        let read = BufReader::new(File::open(input_file_path)?);
        let decoded = burrows_wheeler::decode(Box::new(read));
        compress_utils::file::write_iter_result(output_file_path, Box::new(decoded))
    });
}
