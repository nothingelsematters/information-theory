use super::header::Header;
use super::iterator::BitIterator;
use crate::config::Index;
use crate::result::{Error, Result};
use bit_vec::BitVec;
use std::collections::HashMap;

pub fn decode<'a>(
    input_iter: &'a mut Box<dyn Iterator<Item = u8>>,
) -> Option<Result<(Box<DecoderIterator<'a>>, Index)>> {
    let mut bit_iter = Box::new(BitIterator::new(input_iter, usize::MAX));
    let header = Header::decode(&mut bit_iter)?;
    bit_iter.bit_size(header.bit_size);

    let codes: HashMap<BitVec, u8> = header
        .code_descriptors
        .into_iter()
        .map(|code_descriptor| {
            (
                code_descriptor.code.into_iter().collect(),
                code_descriptor.letter,
            )
        })
        .collect();

    let iter = DecoderIterator::new(codes, bit_iter);
    Some(Ok((Box::new(iter), header.initial)))
}

pub struct DecoderIterator<'a> {
    input_iter: Box<BitIterator<'a>>,
    codes: HashMap<BitVec, u8>,
}

impl<'a> DecoderIterator<'a> {
    pub fn new(codes: HashMap<BitVec, u8>, input_iter: Box<BitIterator<'a>>) -> DecoderIterator {
        DecoderIterator { input_iter, codes }
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
                Some(bit) => bit,
            };
            ended_flag = false;
            current.push(bit);

            if let Some(letter) = self.codes.get(&current) {
                return Some(Ok(*letter));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::huffman::encode;

    #[test]
    fn decode_encoded() -> Result<()> {
        let input = "abbcccdddddeeoifhweag128138y2o".as_bytes();
        let mut encoded_iter =
            &mut Box::new(encode(|| Box::new(input.iter().map(|x| x.clone())), 0));
        let (decoded_iter, _) = decode(&mut encoded_iter).unwrap()?;
        let decoded: Vec<u8> = decoded_iter.map(|x| x.unwrap()).collect();
        assert_eq!(input, &decoded);
        Ok(())
    }
}
