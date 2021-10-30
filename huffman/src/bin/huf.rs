fn main() {
    compress_utils::launch(|input_file_path, output_file_path| {
        let encoded = huffman::encode(|| compress_utils::file::file_iter(input_file_path));
        compress_utils::file::write_iter(output_file_path, encoded)
    });
}
