use crate::{bwt, huffman, mtf};
use std::io::Read;
use std::io::Result as IoResult;

pub fn encode(read: Box<dyn Read>) -> Box<impl Iterator<Item = u8>> {
    let iter = WindowedIterator::from_read(100 * 1024, read)
        .take_while(|x| x.is_ok())
        .map(|x| x.unwrap())
        .flat_map(encode_block);
    Box::new(iter)
}

fn encode_block(block: Vec<u8>) -> Box<(dyn Iterator<Item = u8> + 'static)> {
    let (bwted, _initial) = bwt::apply(&block);
    let mtfed = mtf::apply(&bwted);
    huffman::encode(|| Box::new(mtfed.clone().into_iter()))
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
