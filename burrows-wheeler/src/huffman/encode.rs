use super::header::Header;
use super::BoxedByteIterator;
use crate::config::Index;
use crate::huffman::iterator::ByteIterator;
use bit_vec::BitVec;
use priority_queue::PriorityQueue;
use std::collections::HashMap;

pub fn encode<F>(input_iter_supplier: F, initial: Index) -> BoxedByteIterator
where
    F: Fn() -> BoxedByteIterator,
{
    let frequencies = count_frequency(input_iter_supplier());
    let codes = build_codes(&frequencies);
    let iter = iter(frequencies, input_iter_supplier(), codes, initial);
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
    if queue.is_empty() {
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
    update_codes(&mut codes, *root, BitVec::new());
    codes
}

fn update_codes(codes: &mut HashMap<u8, BitVec>, node: HuffmanNode, code: BitVec) {
    match node {
        HuffmanNode::Leaf(letter) => {
            codes.insert(letter, code);
        }
        HuffmanNode::InnerVertex(left, right) => {
            update_codes(codes, *left, vec_with(&code, false));
            update_codes(codes, *right, vec_with(&code, true));
        }
    }
}

fn iter(
    letter_frequency: HashMap<u8, u64>,
    input_iter: impl Iterator<Item = u8> + 'static,
    codes: HashMap<u8, BitVec>,
    initial: Index,
) -> impl Iterator<Item = u8> {
    let header = Header::encode(letter_frequency, &codes, initial);
    let header_iter = std::iter::once(header);

    let coded_iter = EncodingIterator {
        input_iter: Box::new(input_iter),
        codes,
    };

    ByteIterator::new(Box::new(header_iter.chain(coded_iter)))
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

#[cfg(test)]
mod tests {
    use super::*;

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

    macro_rules! code {
        ( $( $x:expr ),* ) => {
            {
                let bytes = vec![$($x,)*];
                let result: BitVec = bytes.iter().map(|x| *x != 0).collect();
                result
            }
        };
    }

    #[test]
    fn count_frequency_test() {
        let input = "abbcccdddd";
        let frequencies = count_frequency(input.bytes());

        let expected = hashmap! {
            b'a' => 1,
            b'b' => 2,
            b'c' => 3,
            b'd' => 4,
        };

        assert_eq!(expected, frequencies)
    }

    #[test]
    fn build_codes_test() {
        let letter_frequency = hashmap! {
            b'a' => 1,
            b'b' => 2,
            b'c' => 3,
            b'd' => 4,
        };

        let codes = build_codes(&letter_frequency);

        let expected = hashmap! {
            b'a' => code![1, 1, 0],
            b'b' => code![1, 1, 1],
            b'c' => code![1, 0],
            b'd' => code![0],
        };

        assert_eq!(expected, codes)
    }

    #[test]
    fn encode_test() {
        let codes = hashmap! {
            b'a' => code![1, 1, 0],
            b'b' => code![1, 1, 1],
            b'c' => code![1, 0],
            b'd' => code![0],
        };

        let input = "abbcccddddddddd";
        let encoded: Vec<u8> = iter(HashMap::new(), input.bytes(), codes, 0)
            .skip(((Index::BITS + usize::BITS) / 8) as usize)
            .collect();

        let expected = vec![
            0b10011010, 0b01101000, 0b10101100, 0b01100001, 0b10100011, 0b11111101, 0b00010101,
            0b00000000,
        ];

        assert_eq!(encoded, expected)
    }
}
