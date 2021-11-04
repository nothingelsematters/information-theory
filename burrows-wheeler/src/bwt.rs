pub fn apply(buffer: &[u8]) -> (Vec<u8>, usize) {
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

    let initial = indices.binary_search(&0).unwrap();

    (last_symbols, initial)
}

pub fn reverse(buffer: &[u8], initial: usize) -> Vec<u8> {
    todo!()
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
