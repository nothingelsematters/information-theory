use burrows_wheeler::utils;

fn main() {
    utils::launch(|output_file_path, read| {
        let decoded = burrows_wheeler::decode(Box::new(read));
        utils::write_iter_result(output_file_path, decoded)
    });
}
