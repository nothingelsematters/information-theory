use std::{
    fmt::Debug,
    fs::File,
    io::{BufReader, BufWriter, Read, Result as IoResult, Write},
};

pub fn launch<F>(function: F)
where
    F: FnOnce(&str, Box<dyn Read>) -> IoResult<()>,
{
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: <input file path> <output file path>");
        return;
    }

    let input_file_path = &args[1];
    let output_file_path = &args[2];

    let result = File::open(input_file_path)
        .and_then(|file| function(output_file_path, Box::new(BufReader::new(file))));

    if let Err(err) = result {
        println!("Failed: {:?}", err);
    }
}

pub fn write_iter<'a>(
    output_file_path: &str,
    iter: Box<dyn Iterator<Item = u8> + 'a>,
) -> IoResult<()> {
    let mut buf_writer = BufWriter::new(File::create(output_file_path)?);
    iter.map(|x| buf_writer.write(&[x]).map(|_| ())).collect()
}

pub fn write_iter_result<'a, E: Debug>(
    output_file_path: &str,
    iter: Box<dyn Iterator<Item = Result<u8, E>> + 'a>,
) -> IoResult<()> {
    let mut buf_writer = BufWriter::new(File::create(output_file_path)?);

    for byte in iter {
        buf_writer.write(&[byte.unwrap()])?;
    }

    Ok(())
}
