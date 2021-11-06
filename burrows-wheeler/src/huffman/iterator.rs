use super::BoxedByteIterator;
use bit_vec::BitVec;

pub struct ByteIterator {
    input_iterator: Box<dyn Iterator<Item = bool>>,
}

impl ByteIterator {
    pub fn new(input_iterator: Box<dyn Iterator<Item = BitVec>>) -> ByteIterator {
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

pub struct BitIterator<'a> {
    input_iter: &'a mut BoxedByteIterator,
    current: u8,
    current_position: u8,
    next: Option<u8>,
    bit_size: usize,
}

impl<'a> BitIterator<'a> {
    pub fn new(input_iter: &'a mut BoxedByteIterator, bit_size: usize) -> BitIterator {
        BitIterator {
            input_iter,
            bit_size,
            current_position: 8,
            current: 0,
            next: None,
        }
    }

    pub fn bit_size(&mut self, bit_size: usize) {
        self.bit_size = bit_size;
    }
}

impl<'a> Iterator for BitIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bit_size == 0 {
            return None;
        }

        if self.current_position == 8 {
            self.current = if let Some(byte) = self.next {
                byte
            } else {
                match self.input_iter.next() {
                    None => return None,
                    Some(byte) => byte,
                }
            };

            if self.bit_size > 8 {
                self.next = match self.input_iter.next() {
                    None => None,
                    Some(byte) => Some(byte),
                };
            }
            self.current_position = 0;
        }

        if self.bit_size == 0 {
            return None;
        }
        let bit = ((self.current >> self.current_position) & 1) != 0;
        self.current_position += 1;
        self.bit_size -= 1;
        Some(bit)
    }
}
