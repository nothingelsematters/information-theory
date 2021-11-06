use crate::{bwt, config::WINDOW_SIZE, huffman, mtf};
use std::{
    io::Read,
    time::{Duration, SystemTime},
};
use std::{io::Result as IoResult, time::UNIX_EPOCH};

pub fn encode(read: Box<dyn Read>) -> Box<impl Iterator<Item = u8>> {
    let iter = WindowedIterator::from_read(WINDOW_SIZE, read)
        .take_while(|x| x.is_ok())
        .map(|x| x.unwrap())
        .flat_map(encode_block);
    Box::new(iter)
}

fn get_millis() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}

fn encode_block(block: Vec<u8>) -> Box<(dyn Iterator<Item = u8> + 'static)> {
    let time = get_millis();
    let (bwted, initial) = bwt::apply(&block);
    let new_time = get_millis();
    println!("BWT: {:?}", new_time - time);
    let time = get_millis();
    let mtfed = mtf::apply(&bwted);
    let new_time = get_millis();
    println!("MTF: {:?}", new_time - time);
    huffman::encode(|| Box::new(mtfed.clone().into_iter()), initial)
}

pub struct WindowedIterator {
    window: usize,
    read: Box<dyn Read>,
}

impl WindowedIterator {
    pub fn from_read(window: usize, read: Box<dyn Read>) -> WindowedIterator {
        WindowedIterator { window, read }
    }
}

impl Iterator for WindowedIterator {
    type Item = IoResult<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = vec![0u8; self.window];

        return match self.read.read(buffer.as_mut_slice()) {
            Ok(size) if size == 0 => None,
            Ok(size) => Some(Ok(buffer[0..size].to_vec())),
            Err(err) => Some(Err(err)),
        };
    }
}
