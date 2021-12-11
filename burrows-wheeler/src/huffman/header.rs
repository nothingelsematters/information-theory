use super::iterator::BitIterator;
use crate::config::Index;
use bit_vec::BitVec;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct CodeDescriptor {
    pub code: BitVec,
    pub letter: u8,
}

#[derive(Debug)]
pub struct Header {
    pub code_descriptors: Vec<CodeDescriptor>,
    pub bit_size: usize,
    pub initial: Index,
}

impl Header {
    pub fn encode(
        letter_frequency: HashMap<u8, u64>,
        codes: &HashMap<u8, BitVec>,
        initial: Index,
    ) -> BitVec {
        let mut result = BitVec::new();
        Header::add_bytes(&mut result, &initial.to_be_bytes());

        let mut bit_size = 0;
        for (k, v) in letter_frequency {
            bit_size += codes[&k].len() * (v as usize);
        }
        Header::add_bytes(&mut result, &bit_size.to_be_bytes());

        let mut tree = codes.iter().collect::<Vec<_>>();
        tree.sort_by(|(_, first_code), (_, second_code)| {
            let min_len = std::cmp::min(first_code.len(), second_code.len());
            let bit_compare = first_code
                .iter()
                .take(min_len)
                .cmp(second_code.iter().take(min_len));

            match bit_compare {
                Ordering::Equal if first_code.len() == second_code.len() => Ordering::Equal,
                Ordering::Equal
                    if (first_code.len() > second_code.len() && !first_code[min_len])
                        || (first_code.len() < second_code.len() && second_code[min_len]) =>
                {
                    Ordering::Less
                }
                Ordering::Equal => Ordering::Greater,
                otherwise => otherwise,
            }
        });

        let mut current_len = 0;

        for (byte, code) in tree.into_iter() {
            for _ in current_len..code.len() {
                result.push(false);
            }
            result.push(true);
            Header::add_byte(&mut result, *byte);

            current_len = code
                .iter()
                .rev()
                .position(|x| !x)
                .map(|x| code.len() - x)
                .unwrap_or(code.len() - 1);
        }

        result
    }

    fn add_byte(bitvec: &mut BitVec, byte: u8) {
        for i in (0..u8::BITS).rev() {
            bitvec.push((byte >> i) & 1 == 1);
        }
    }

    fn add_bytes(bitvec: &mut BitVec, bytes: &[u8]) {
        for i in bytes {
            Header::add_byte(bitvec, *i);
        }
    }

    pub fn decode(input_iter: &mut Box<BitIterator<'_>>) -> Option<Header> {
        let mut initial: Index = 0;
        for i in (0..Index::BITS).rev() {
            initial |= (input_iter.next()? as Index) << i;
        }

        let mut bit_size: usize = 0;
        for i in (0..usize::BITS).rev() {
            bit_size |= (input_iter.next()? as usize) << i;
        }

        let mut code_descriptors = Vec::new();
        let mut current = BitVec::new();

        while {
            while !input_iter.next()? {
                current.push(false);
            }

            let mut byte: u8 = 0;
            for i in (0..u8::BITS).rev() {
                byte |= (input_iter.next()? as u8) << i;
            }

            code_descriptors.push(CodeDescriptor {
                code: current.clone(),
                letter: byte,
            });

            let condition = current.all();
            current = current
                .iter()
                .rev()
                .skip_while(|x| *x)
                .skip(1)
                .collect::<BitVec>()
                .into_iter()
                .rev()
                .collect::<BitVec>();
            current.push(true);

            !condition
        } {}

        let header = Header {
            initial,
            bit_size,
            code_descriptors,
        };

        Some(header)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::huffman::iterator::ByteIterator;

    macro_rules! code {
        ( $( $x:expr ),* ) => {
            {
                let bytes = vec![$($x,)*];
                let result: BitVec = bytes.iter().map(|x| *x != 0).collect();
                result
            }
        };
    }

    macro_rules! hashmap {
        (@single $($x:tt)*) => (());
        (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

        ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
        ($($key:expr => $value:expr),*) => {
            {
                let cap = hashmap!(@count $($key),*);
                let mut map = HashMap::with_capacity(cap);
                $(
                    map.insert($key, $value);
                )*
                map
            }
        };
    }

    #[test]
    fn encode_test() {
        let encoded = Header::encode(
            hashmap! {
                b'a' => 1,
                b'b' => 1,
                b'c' => 4,
                b'd' => 4,
                b'e' => 4,
            },
            &hashmap! {
                b'a' => code![0, 0, 0],
                b'b' => code![0, 0, 1],
                b'c' => code![0, 1],
                b'd' => code![1, 0],
                b'e' => code![1, 1],
            },
            2,
        );
        let encoded = encoded
            .into_iter()
            .skip((Index::BITS + usize::BITS) as usize)
            .collect::<BitVec>();

        let expected = code![
            0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1,
            1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1
        ];

        assert_eq!(encoded, expected);
    }

    #[test]
    fn decode_test() {
        let mut encoded = vec![0u8; ((Index::BITS + usize::BITS) / 8) as usize];
        let encoded_descriptors: Vec<u8> = vec![
            0b01101000, 0b11011000, 0b10101000, 0b10110001, 0b00100110, 0b01001101, 0b00000001,
        ];
        encoded_descriptors
            .into_iter()
            .for_each(|x| encoded.push(x));

        let mut byte_iter: Box<dyn Iterator<Item = u8>> = Box::new(encoded.into_iter());

        let bit_iter = BitIterator::new(&mut byte_iter, usize::MAX);
        let decoded = Header::decode(&mut Box::new(bit_iter));

        let expected: Vec<CodeDescriptor> = vec![
            CodeDescriptor {
                letter: b'a',
                code: code![0, 0, 0],
            },
            CodeDescriptor {
                letter: b'b',
                code: code![0, 0, 1],
            },
            CodeDescriptor {
                letter: b'c',
                code: code![0, 1],
            },
            CodeDescriptor {
                letter: b'd',
                code: code![1, 0],
            },
            CodeDescriptor {
                letter: b'e',
                code: code![1, 1],
            },
        ];

        assert_eq!(expected, decoded.unwrap().code_descriptors)
    }

    #[test]
    fn decode_encoded_test() {
        let encoded = Header::encode(
            hashmap! {
                b'a' => 1,
                b'b' => 1,
                b'c' => 4,
                b'd' => 4,
                b'e' => 4,
            },
            &hashmap! {
                b'a' => code![0, 0, 0],
                b'b' => code![0, 0, 1],
                b'c' => code![0, 1],
                b'd' => code![1, 0],
                b'e' => code![1, 1],
            },
            2,
        );

        let mut byte_iter: Box<dyn Iterator<Item = u8>> =
            Box::new(ByteIterator::new(Box::new(std::iter::once(encoded))));

        let bit_iter = BitIterator::new(&mut byte_iter, usize::MAX);
        let decoded = Header::decode(&mut Box::new(bit_iter));

        let expected: Vec<CodeDescriptor> = vec![
            CodeDescriptor {
                letter: b'a',
                code: code![0, 0, 0],
            },
            CodeDescriptor {
                letter: b'b',
                code: code![0, 0, 1],
            },
            CodeDescriptor {
                letter: b'c',
                code: code![0, 1],
            },
            CodeDescriptor {
                letter: b'd',
                code: code![1, 0],
            },
            CodeDescriptor {
                letter: b'e',
                code: code![1, 1],
            },
        ];

        assert_eq!(expected, decoded.unwrap().code_descriptors)
    }
}
