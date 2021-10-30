use crate::common::{CodeDescriptor, Header};
use crate::BoxedByteIterator;
use bit_vec::BitVec;
use priority_queue::PriorityQueue;
use std::collections::HashMap;

mod iterator;

pub fn encode<F>(input_iter_supplier: F) -> BoxedByteIterator
where
    F: Fn() -> BoxedByteIterator,
{
    let frequencies = count_frequency(input_iter_supplier());
    let codes = build_codes(&frequencies);
    let iter = iter(frequencies, input_iter_supplier(), codes);
    Box::new(iter)
}

#[derive(PartialEq, Eq, Hash)]
enum HuffmanNode {
    Leaf(u8),
    InnerVertex(Box<HuffmanNode>, Box<HuffmanNode>),
}

fn vec_with(vec: &BitVec, element: bool) -> BitVec {
    let mut new_vec = vec.clone();
    new_vec.push(element);
    new_vec
}

fn count_frequency(input_iter: impl Iterator<Item = u8>) -> HashMap<u8, u64> {
    let mut letter_frequency = HashMap::new();
    for byte in input_iter {
        *letter_frequency.entry(byte).or_insert(0) += 1;
    }
    letter_frequency
}

fn build_codes(letter_frequency: &HashMap<u8, u64>) -> HashMap<u8, BitVec> {
    let mut queue = PriorityQueue::<Box<HuffmanNode>, i64>::with_capacity(letter_frequency.len());

    for (letter, number) in letter_frequency {
        queue.push(Box::new(HuffmanNode::Leaf(*letter)), -(*number as i64));
    }

    let mut codes = HashMap::new();
    if queue.len() == 0 {
        return codes;
    }

    while queue.len() != 1 {
        let (left, left_priority) = queue.pop().unwrap();
        let (right, right_priority) = queue.pop().unwrap();
        queue.push(
            Box::new(HuffmanNode::InnerVertex(left, right)),
            left_priority + right_priority,
        );
    }

    let (root, _) = queue.pop().unwrap();
    update_codes(&mut codes, root, BitVec::new());
    codes
}

fn update_codes(codes: &mut HashMap<u8, BitVec>, node: Box<HuffmanNode>, code: BitVec) {
    match *node {
        HuffmanNode::Leaf(letter) => {
            codes.insert(letter, code);
        }
        HuffmanNode::InnerVertex(left, right) => {
            update_codes(codes, left, vec_with(&code, false));
            update_codes(codes, right, vec_with(&code, true));
        }
    }
}

// TODO adequate header dump
fn header_iter(
    letter_frequency: HashMap<u8, u64>,
    codes: &HashMap<u8, BitVec>,
) -> impl Iterator<Item = u8> + 'static {
    let mut code_descriptors = Vec::new();
    for (k, v) in codes.iter() {
        code_descriptors.push(CodeDescriptor {
            code: v.iter().collect(),
            letter: *k,
        });
    }
    let mut bit_size = 0;
    for (k, v) in letter_frequency {
        bit_size += codes[&k].len() * (v as usize);
    }
    let last_byte_size = (bit_size % 8) as u8;
    let (byte_size, last_byte_size) = if last_byte_size == 0 {
        (bit_size / 8, 8)
    } else {
        (bit_size / 8 + 1, last_byte_size)
    };

    let header = Header {
        code_descriptors,
        byte_size,
        last_byte_size,
    };

    serde_json::to_string(&header)
        .unwrap()
        .into_bytes()
        .into_iter()
}

fn iter(
    letter_frequency: HashMap<u8, u64>,
    input_iter: impl Iterator<Item = u8> + 'static,
    codes: HashMap<u8, BitVec>,
) -> impl Iterator<Item = u8> {
    let header_iter = header_iter(letter_frequency, &codes);
    let coded_iter = iterator::encoded_iterator(Box::new(input_iter), codes);
    header_iter.chain(coded_iter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    macro_rules! code {
        ( $( $x:expr ),* ) => {
            {
                let mut bytes = Vec::new();
                $(
                    bytes.push($x);
                )*
                let result: BitVec = bytes.iter().map(|x| *x != 0).collect();
                result
            }
        };
    }

    fn to_bits(bytes: Vec<u8>) -> Vec<bool> {
        bytes
            .into_iter()
            .flat_map(|x| {
                let mut vec = Vec::with_capacity(8);
                for i in 0..8 {
                    vec.push(((x >> i) & 1) != 0);
                }
                vec.into_iter()
            })
            .collect()
    }

    #[test]
    fn count_frequency_test() {
        let input = "abbcccdddd";
        let frequencies = count_frequency(input.bytes());

        let expected = hashmap! {
            'a' as u8 => 1,
            'b' as u8 => 2,
            'c' as u8 => 3,
            'd' as u8 => 4,
        };

        assert_eq!(expected, frequencies)
    }

    #[test]
    fn build_codes_test() {
        let letter_frequency = hashmap! {
            'a' as u8 => 1,
            'b' as u8 => 2,
            'c' as u8 => 3,
            'd' as u8 => 4,
        };

        let codes = build_codes(&letter_frequency);

        let expected = hashmap! {
            'a' as u8 => code![1, 1, 0],
            'b' as u8 => code![1, 1, 1],
            'c' as u8 => code![1, 0],
            'd' as u8 => code![0],
        };

        assert_eq!(expected, codes)
    }

    #[test]
    fn encode_test() {
        let codes = hashmap! {
            'a' as u8 => code![1, 1, 0],
            'b' as u8 => code![1, 1, 1],
            'c' as u8 => code![1, 0],
            'd' as u8 => code![0],
        };

        let input = "abbcccddddddddd";
        let encoded: Vec<u8> = iter(HashMap::new(), input.bytes(), codes).collect();

        let expected =
            code![1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        assert!(to_bits(encoded).ends_with(&expected.into_iter().collect::<Vec<bool>>().as_slice()))
    }
}
