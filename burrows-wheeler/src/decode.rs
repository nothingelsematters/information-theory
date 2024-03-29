use crate::result::Result;
use crate::{bwt, huffman, mtf};
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

impl Iterator for DecodeIterator {
    type Item = Box<dyn Iterator<Item = Result<u8>>>;

    fn next(&mut self) -> Option<Self::Item> {
        match huffman::decode(&mut self.input_iter) {
            None => None,
            Some(iter) => match iter {
                Ok((iter, initial)) => {
                    let vec = match iter.collect::<Result<Vec<u8>>>() {
                        Ok(vec) => vec,
                        Err(err) => return Some(Box::new(once(Err(err)))),
                    };

                    let demtfed = mtf::reverse(&vec);
                    let debwted = bwt::reverse(&demtfed, initial);
                    Some(Box::new(debwted.into_iter().map(Ok)))
                }
                Err(err) => Some(Box::new(once(Err(err)))),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;
    use crate::encode;

    #[test]
    fn decode_encoded() {
        let string = "qwertyuiopasdfghjkfwjeyyyyyyyqowoooolzxcvbnm,wwert6y7u89".as_bytes();

        let encoded = encode(Box::new(string)).collect::<Vec<_>>();
        let encoded = Cursor::new(encoded);
        let decoded = decode(Box::new(encoded))
            .collect::<Result<Vec<_>>>()
            .unwrap();

        assert_eq!(&decoded, string)
    }
}
