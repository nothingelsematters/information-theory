use huffman::{byte_processor::ByteProcessor, encode::Encoder};
use std::fs::File;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn get_millis() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: <file input path>");
        return;
    }

    let input_file_path = &args[1];

    let start = get_millis();

    let coded_len = match Encoder::process_file(input_file_path) {
        Err(err) => {
            println!("{:#?}", err);
            return;
        }
        Ok(iter) => iter.count(),
    };

    let time = get_millis() - start;

    let source_len = File::open(input_file_path)
        .unwrap()
        .metadata()
        .unwrap()
        .len();

    println!(
        " source len: {} bytes\n  coded len: {} bytes\ncompression: {}\n       time: {}s {}ms",
        source_len,
        coded_len,
        (source_len as f64) / (coded_len as f64),
        time.as_secs(),
        time.subsec_millis(),
    );
}
