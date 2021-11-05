use crate::config::Index;
use std::collections::HashMap;

pub fn apply(buffer: &[u8]) -> (Vec<u8>, Index) {
    // TODO use suffix array instead
    let mut indices: Vec<usize> = (0..buffer.len()).collect();
    indices.sort_by(|&x, &y| {
        let mut i = x;
        let mut j = y;
        let mut count = 0;
        let len = buffer.len();

        while buffer[i] == buffer[j] && count < len {
            i = (i + 1) % len;
            j = (j + 1) % len;
            count += 1;
        }

        buffer[i].cmp(&buffer[j])
    });

    let last_symbols = indices
        .iter()
        .map(|i| buffer[(buffer.len() + i - 1) % buffer.len()])
        .collect();

    let initial = indices.iter().position(|x| *x == 0).unwrap();

    (last_symbols, initial as Index)
}

pub fn reverse(buffer: &[u8], initial: Index) -> Vec<u8> {
    let shift = shift_vector(buffer);
    let mut output = Vec::with_capacity(buffer.len());
    let mut current = initial;

    for _ in 0..buffer.len() {
        current = shift[current as usize];
        output.push(buffer[current as usize]);
    }

    output
}

fn shift_vector(buffer: &[u8]) -> Vec<Index> {
    let mut sorted = buffer.to_vec();
    sorted.sort();
    let mut used = HashMap::new();
    let mut reverse = Vec::with_capacity(buffer.len());

    for i in buffer {
        let used_byte = used.entry(i).or_insert(0);
        reverse.push(*used_byte + sorted.partition_point(|x| x < i));
        *used_byte += 1;
    }

    let mut shift = vec![0; buffer.len()];
    for (index, i) in reverse.iter().enumerate() {
        shift[*i] = index as Index;
    }

    shift
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn apply_test() {
        let string = "DRDOBBS";
        assert_eq!(apply(string.as_bytes()), ("OBRSDDB".bytes().collect(), 3))
    }

    #[test]
    fn reverse_test() {
        let string = "DRDOBBS".as_bytes();
        let (buffer, initial) = apply(string);
        assert_eq!(reverse(&buffer, initial), string)
    }
}
