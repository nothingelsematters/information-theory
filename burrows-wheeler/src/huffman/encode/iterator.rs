use super::BoxedByteIterator;
use bit_vec::BitVec;
use std::collections::HashMap;

pub fn encoded_iterator(
    input_iter: BoxedByteIterator,
    codes: HashMap<u8, BitVec>,
) -> impl Iterator<Item = u8> {
    ByteIterator::new(Box::new(EncodingIterator { input_iter, codes }))
}

struct ByteIterator {
    input_iterator: Box<dyn Iterator<Item = bool>>,
}

impl ByteIterator {
    fn new(input_iterator: Box<dyn Iterator<Item = BitVec>>) -> ByteIterator {
        ByteIterator {
            input_iterator: Box::new(input_iterator.flatten()),
        }
    }
}

impl Iterator for ByteIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut byte: u8 = 0;

        for i in 0..8 {
            match self.input_iterator.next() {
                None if i == 0 => return None,
                None => return Some(byte),
                Some(bit) => byte |= (bit as u8) << i,
            }
        }

        Some(byte)
    }
}

struct EncodingIterator {
    input_iter: BoxedByteIterator,
    codes: HashMap<u8, BitVec>,
}

impl Iterator for EncodingIterator {
    type Item = BitVec;

    fn next(&mut self) -> Option<Self::Item> {
        match self.input_iter.next() {
            None => None,
            Some(byte) => match self.codes.get(&byte) {
                None => panic!("Data changed invalidating header: unexpected letter"),
                Some(code) => Some(code.clone()),
            },
        }
    }
}
