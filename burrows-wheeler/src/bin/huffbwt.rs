use burrows_wheeler::utils;

fn main() {
    utils::launch(|output_file_path, read| {
        let encoded = burrows_wheeler::encode(Box::new(read));
        utils::write_iter(output_file_path, encoded)
    });
}
