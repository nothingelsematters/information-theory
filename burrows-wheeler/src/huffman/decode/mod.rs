use self::iterator::DecoderIterator;
use super::common::Header;
use super::BoxedByteIterator;
use crate::config::Index;
use crate::result::{Error, Result};
use bit_vec::BitVec;
use std::collections::HashMap;

pub mod iterator;

pub fn decode<'a>(
    input_iter: &'a mut Box<dyn Iterator<Item = u8>>,
) -> Option<Result<(Box<DecoderIterator<'a>>, Index)>> {
    let header = decode_header(input_iter)?;
    let header = match header {
        Ok(header) => header,
        Err(err) => return Some(Err(err)),
    };

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

    let iter = iterator::DecoderIterator::new(codes, input_iter, header.bit_size);

    Some(Ok((Box::new(iter), header.initial)))
}

// TODO adequate header decoding
fn decode_header(input_iter: &mut BoxedByteIterator) -> Option<Result<Header>> {
    let mut header = Vec::new();
    let mut braces_count = 0;

    loop {
        match input_iter.next() {
            None => return None,
            Some(value) => {
                header.push(value);
                match value as char {
                    '{' => braces_count += 1,
                    '}' => {
                        braces_count -= 1;
                        if braces_count == 0 {
                            return Some(serde_json::from_slice(header.as_slice()).map_err(
                                |err| Error {
                                    message: err.to_string(),
                                },
                            ));
                        }
                    }
                    _ => {}
                }
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
