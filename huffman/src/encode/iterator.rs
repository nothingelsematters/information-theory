use std::collections::HashMap;

use crate::byte_processor::{BoxedByteIterator, Error, Result};
use crate::common::Code;

pub fn encoded_iterator(
    input_iter: BoxedByteIterator,
    codes: HashMap<u8, Code>,
) -> impl Iterator<Item = Result<u8>> {
    ByteIterator::new(Box::new(EncodingIterator { input_iter, codes }))
}

struct ByteIterator {
    input_iterator: Box<dyn Iterator<Item = Result<bool>>>,
}

impl ByteIterator {
    fn new(input_iterator: Box<dyn Iterator<Item = Result<Vec<bool>>>>) -> ByteIterator {
        ByteIterator {
            input_iterator: Box::new(input_iterator.flat_map(|x| {
                let result: Box<dyn Iterator<Item = Result<bool>>> = match x {
                    Err(err) => Box::new(std::iter::once(Err(err))),
                    Ok(vec) => Box::new(vec.into_iter().map(|x| Ok(x))),
                };
                result
            })),
        }
    }
}

impl Iterator for ByteIterator {
    type Item = Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut byte: u8 = 0;
        let mut none = true;

        for i in 0..8 {
            match self.input_iterator.next() {
                None if none => return None,
                None => return Some(Ok(byte)),
                Some(Err(err)) => return Some(Err(err)),
                Some(Ok(bit)) => byte |= (bit as u8) << i,
            }
            none = false;
        }

        Some(Ok(byte))
    }
}

struct EncodingIterator {
    input_iter: Box<dyn Iterator<Item = Result<u8>>>,
    codes: HashMap<u8, Code>,
}

impl Iterator for EncodingIterator {
    type Item = Result<Vec<bool>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.input_iter.next() {
            None => None,
            Some(Err(err)) => Some(Err(err)),
            Some(Ok(byte)) => match self.codes.get(&byte) {
                None => Some(Err(Error::new("Unexpected letter met"))),
                Some(code) => Some(Ok(code.clone())),
            },
        }
    }
}
