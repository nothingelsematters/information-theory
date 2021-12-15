use burrows_wheeler::utils;
use std::io::Read;

pub fn launch<F>(f: F)
where
    F: Fn(&[u8]) -> Vec<u8>,
{
    utils::launch(|output_file_path, read| {
        let bytes = read
            .bytes()
            .take_while(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        let transformed = f(&bytes);
        utils::write_iter(output_file_path, Box::new(transformed.into_iter()))
    });
}
