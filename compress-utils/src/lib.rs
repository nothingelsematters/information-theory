use std::fmt::Debug;

pub mod file;

pub fn launch<E: Debug, F>(function: F)
where
    F: Fn(&str, &str) -> Result<(), E>,
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: <input file path> <output file path>");
        return;
    }

    let input_file_path = &args[1];
    let output_file_path = &args[2];

    if let Err(err) = function(input_file_path, output_file_path) {
        println!("Failed: {:?}", err);
    }
}
