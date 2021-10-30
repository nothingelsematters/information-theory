use crate::bwt;
use huffman::Result;
use std::io::Read;
use std::iter::once;

pub fn decode(read: Box<dyn Read>) -> Box<dyn Iterator<Item = Result<u8>>> {
    let input_iter = read.bytes().take_while(|x| x.is_ok()).map(|x| x.unwrap());
    let input_iter = Box::new(input_iter);
    Box::new(DecodeIterator::new(input_iter).flatten())
}

struct DecodeIterator {
    input_iter: Box<dyn Iterator<Item = u8>>,
}

impl DecodeIterator {
    fn new(input_iter: Box<dyn Iterator<Item = u8>>) -> DecodeIterator {
        DecodeIterator { input_iter }
    }
}

impl<'a> Iterator for DecodeIterator {
    type Item = Box<dyn Iterator<Item = Result<u8>>>;

    fn next(&mut self) -> Option<Self::Item> {
        match huffman::decode(&mut self.input_iter) {
            None => return None,
            Some(iter) => match iter {
                Ok(iter) => {
                    let vec: Result<Vec<u8>> = iter.collect();
                    let vec = match vec {
                        Ok(vec) => vec,
                        Err(err) => return Some(Box::new(once(Err(err)))),
                    };

                    let debwted = bwt::reverse(&vec, vec![] /* TODO ?? */);
                    Some(Box::new(debwted.into_iter().map(Ok)))
                }
                Err(err) => Some(Box::new(once(Err(err)))),
            },
        }
    }
}
