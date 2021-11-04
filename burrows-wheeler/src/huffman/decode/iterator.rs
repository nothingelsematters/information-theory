use super::{BoxedByteIterator, Error, Result};
use bit_vec::BitVec;
use std::collections::HashMap;

struct BitIterator<'a> {
    input_iter: &'a mut BoxedByteIterator,
    current: u8,
    current_position: u8,
    next: Option<u8>,
    byte_size: usize,
    last_byte_size: u8,
}

impl<'a> BitIterator<'a> {
    fn new(
        input_iter: &'a mut BoxedByteIterator,
        byte_size: usize,
        last_byte_size: u8,
    ) -> BitIterator {
        BitIterator {
            input_iter,
            byte_size,
            last_byte_size,
            current_position: 8,
            current: 0,
            next: None,
        }
    }
}

impl<'a> Iterator for BitIterator<'a> {
    type Item = Result<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_position == 8 {
            self.byte_size -= 1;

            self.current = if let Some(byte) = self.next {
                byte
            } else {
                match self.input_iter.next() {
                    None => return None,
                    Some(byte) => byte,
                }
            };

            if self.byte_size > 0 {
                self.next = match self.input_iter.next() {
                    None => None,
                    Some(byte) => Some(byte),
                };
                self.current_position = 0;
            }
        }

        if self.next == None && self.last_byte_size == self.current_position {
            return None;
        }
        let bit = ((self.current >> self.current_position) & 1) != 0;
        self.current_position += 1;
        Some(Ok(bit))
    }
}

pub struct DecoderIterator<'a> {
    input_iter: Box<BitIterator<'a>>,
    codes: HashMap<BitVec, u8>,
}

impl<'a> DecoderIterator<'a> {
    pub fn new(
        codes: HashMap<BitVec, u8>,
        input_iter: &'a mut Box<dyn Iterator<Item = u8>>,
        byte_size: usize,
        last_byte_size: u8,
    ) -> DecoderIterator {
        DecoderIterator {
            input_iter: Box::new(BitIterator::new(input_iter, byte_size, last_byte_size)),
            codes,
        }
    }
}

impl<'a> Iterator for DecoderIterator<'a> {
    type Item = Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = BitVec::new();
        let mut ended_flag = true;

        loop {
            let bit = match self.input_iter.next() {
                None if ended_flag => return None,
                None => return Some(Err(Error::new("Unexpected code word"))),
                Some(Err(err)) => return Some(Err(err)),
                Some(Ok(bit)) => bit,
            };
            ended_flag = false;
            current.push(bit);

            if let Some(letter) = self.codes.get(&current) {
                return Some(Ok(*letter));
            }
        }
    }
}
