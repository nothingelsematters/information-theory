use crate::config::Index;
use std::collections::HashMap;
use std::mem::swap;

pub fn apply(buffer: &[u8]) -> (Vec<u8>, Index) {
    let indices = sort_cyclic_shifts(buffer);

    let last_symbols = indices
        .iter()
        .map(|i| buffer[(buffer.len() + i - 1) % buffer.len()])
        .collect();

    let initial = indices.iter().position(|x| *x == 0).unwrap();

    (last_symbols, initial as Index)
}

fn sort_cyclic_shifts(buffer: &[u8]) -> Vec<usize> {
    let len = buffer.len();
    let alphabet = 256;
    let mut p = vec![0; len];
    let mut c = vec![0; len];
    let mut count = vec![0; std::cmp::max(alphabet, len)];

    for i in 0..len {
        count[buffer[i] as usize] += 1;
    }
    for i in 1..alphabet {
        count[i] += count[i - 1];
    }
    for i in 0..len {
        count[buffer[i] as usize] -= 1;
        p[count[buffer[i] as usize] as usize] = i;
    }

    c[p[0]] = 0;

    let mut classes = 1;

    for i in 1..len {
        if buffer[p[i]] != buffer[p[i - 1]] {
            classes += 1;
        }
        c[p[i]] = classes - 1;
    }

    let mut pn = vec![0; len];
    let mut cn = vec![0; len];

    let mut h = 0;

    while (1 << h) < len {
        for i in 0..len {
            pn[i] = if (1 << h) > p[i] {
                p[i] + len - (1 << h)
            } else {
                p[i] - (1 << h)
            };
        }

        (0..classes).for_each(|i| {
            count[i] = 0;
        });

        for i in 0..len {
            count[c[pn[i]]] += 1;
        }
        for i in 1..classes {
            count[i] += count[i - 1];
        }
        for i in (0..len).rev() {
            count[c[pn[i]]] -= 1;
            p[count[c[pn[i]]]] = pn[i];
        }
        cn[p[0]] = 0;
        classes = 1;

        for i in 1..len {
            let cur = (c[p[i]], c[(p[i] + (1 << h)) % len]);
            let prev = (c[p[i - 1]], c[(p[i - 1] + (1 << h)) % len]);
            if cur != prev {
                classes += 1;
            }
            cn[p[i]] = classes - 1;
        }
        swap(&mut c, &mut cn);

        h += 1;
    }

    p
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
    sorted.sort_unstable();
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
