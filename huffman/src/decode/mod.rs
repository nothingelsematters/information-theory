use std::collections::HashMap;

mod iterator;
use bit_vec::BitVec;

use crate::byte_processor::{BoxedByteIterator, ByteProcessor, Error, Result};
use crate::common::{CodeDescriptor, Header};

pub struct Decoder {}

impl ByteProcessor for Decoder {
    fn process<F>(input_iter_supplier: F) -> Result<BoxedByteIterator>
    where
        F: Fn() -> Result<BoxedByteIterator>,
    {
        let mut input_iter = input_iter_supplier()?;
        let header = Decoder::decode_header(&mut input_iter)?;
        let codes = Decoder::code_map(header.code_descriptors);
        let iter = Decoder::iter(codes, input_iter, header.last_byte_size);
        Ok(Box::new(iter))
    }
}

impl Decoder {
    // TODO adequate header decoding
    fn decode_header(input_iter: &mut BoxedByteIterator) -> Result<Header> {
        let mut header = Vec::new();
        let mut braces_count = 0;

        loop {
            match input_iter.next() {
                None => {
                    return Err(Error {
                        message: String::from("Encoded header is invalid"),
                    });
                }
                Some(Err(err)) => {
                    return Err(err);
                }
                Some(Ok(value)) => {
                    header.push(value);
                    match value as char {
                        '{' => braces_count += 1,
                        '}' => {
                            braces_count -= 1;
                            if braces_count == 0 {
                                return serde_json::from_slice(header.as_slice()).map_err(|err| {
                                    Error {
                                        message: err.to_string(),
                                    }
                                });
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn code_map(code_descriptors: Vec<CodeDescriptor>) -> HashMap<BitVec, u8> {
        code_descriptors
            .into_iter()
            .map(|code_descriptor| {
                (
                    code_descriptor.code.into_iter().collect(),
                    code_descriptor.letter,
                )
            })
            .collect()
    }

    fn iter(
        codes: HashMap<BitVec, u8>,
        input_iter: BoxedByteIterator,
        last_byte_size: u8,
    ) -> impl Iterator<Item = Result<u8>> {
        iterator::DecoderIterator::new(codes, input_iter, last_byte_size)
    }
}

#[cfg(test)]
mod tests {
    use crate::byte_processor::{ByteProcessor, Result};
    use crate::decode::Decoder;
    use crate::encode::Encoder;

    #[test]
    fn decode_encoded() -> Result<()> {
        let input = "abbcccddddd";
        let decoded_iter = Decoder::process(|| {
            Encoder::process(|| Ok(Box::new(input.bytes().into_iter().map(|x| Ok(x)))))
        })?;
        let decoded: Vec<u8> = decoded_iter.map(|x| x.unwrap()).collect();
        assert_eq!(Ok(String::from(input)), String::from_utf8(decoded));
        Ok(())
    }
}
