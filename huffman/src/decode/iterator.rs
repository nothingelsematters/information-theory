use std::collections::HashMap;

use crate::byte_processor::{Error, Result};
use crate::common::Code;

struct BitIterator {
    input_iter: Box<dyn Iterator<Item = Result<u8>>>,
    current: u8,
    current_position: u8,
    next: Option<u8>,
    last_byte_size: u8,
}

impl Iterator for BitIterator {
    type Item = Result<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_position == 8 {
            self.current = if let Some(byte) = self.next {
                byte
            } else {
                match self.input_iter.next() {
                    None => return None,
                    Some(Err(err)) => return Some(Err(err)),
                    Some(Ok(byte)) => byte,
                }
            };

            self.next = match self.input_iter.next() {
                None => None,
                Some(Err(err)) => return Some(Err(err)),
                Some(Ok(byte)) => Some(byte),
            };
            self.current_position = 0;
        }

        if self.next == None && self.last_byte_size == self.current_position {
            return None;
        }
        let bit = ((self.current >> self.current_position) & 1) != 0;
        self.current_position += 1;
        Some(Ok(bit))
    }
}

pub struct DecoderIterator {
    input_iter: Box<dyn Iterator<Item = Result<bool>>>,
    codes: HashMap<Code, u8>,
}

impl DecoderIterator {
    pub fn new(
        codes: HashMap<Code, u8>,
        input_iter: Box<dyn Iterator<Item = Result<u8>>>,
        last_byte_size: u8,
    ) -> DecoderIterator {
        let bit_iter = BitIterator {
            input_iter,
            last_byte_size,
            current_position: 8,
            current: 0,
            next: None,
        };

        DecoderIterator {
            input_iter: Box::new(bit_iter),
            codes,
        }
    }
}

impl Iterator for DecoderIterator {
    type Item = Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = Vec::new();
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
