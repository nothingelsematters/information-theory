use crate::byte_processor::Result;

pub fn main<F>(processor: F)
where
    F: Fn(&str, &str) -> Result<()>,
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: <file input path> <file output path>");
        return;
    }

    let input_file_path = &args[1];
    let output_file_path = &args[2];

    if let Err(err) = processor(input_file_path, output_file_path) {
        println!("Failed: {:#?}", err);
    }
}
