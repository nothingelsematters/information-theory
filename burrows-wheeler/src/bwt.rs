pub fn apply(buffer: &[u8]) -> (Vec<u8>, Vec<usize>) {
    let (last_symbols, transformation) = sort_cyclic_shifts(buffer);
    let moved = move_to_front(&last_symbols);
    (moved, transformation)
}

fn sort_cyclic_shifts(buffer: &[u8]) -> (Vec<u8>, Vec<usize>) {
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

    let mut origins = vec![0; buffer.len()];
    for (index, i) in indices.iter().enumerate() {
        origins[*i] = index;
    }

    let transformation = (0..buffer.len())
        .map(|i| origins[(indices[i] + 1) % buffer.len()])
        .collect();

    let last_symbols = indices
        .iter()
        .map(|i| buffer[(buffer.len() + i - 1) % buffer.len()])
        .collect();

    (last_symbols, transformation)
}

fn move_to_front(buffer: &[u8]) -> Vec<u8> {
    // TODO optimize
    let mut alphabet: Vec<u8> = (0..255).collect();

    let mut output = Vec::with_capacity(buffer.len());
    for i in buffer {
        let position = alphabet.iter().position(|x| x == i).unwrap();
        output.push(position as u8);
        alphabet.remove(position);
        alphabet.insert(0, *i);
    }

    output
}

pub fn reverse(buffer: &[u8], transformation: Vec<usize>) -> Vec<u8> {
    let moved = reverse_move_to_front(buffer);
    reverse_sort_cyclic_shifts(&moved, transformation)
}

fn reverse_move_to_front(buffer: &[u8]) -> Vec<u8> {
    // TODO optimize
    let mut alphabet: Vec<u8> = (0..255).collect();

    let mut output = Vec::with_capacity(buffer.len());
    for i in buffer {
        let index = *i as usize;
        let letter = alphabet[index];

        output.push(letter);
        alphabet.remove(index);
        alphabet.insert(0, letter);
    }

    output
}

fn reverse_sort_cyclic_shifts(buffer: &[u8], transformation: Vec<usize>) -> Vec<u8> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sort_cyclic_shifts_test() {
        let string = "DRDOBBS";
        assert_eq!(
            sort_cyclic_shifts(string.as_bytes()),
            ("OBRSDDB".bytes().collect(), vec![1, 6, 4, 5, 0, 2, 3])
        )
    }

    #[test]
    fn move_to_front_test() {
        let string = "aaaabbbbeeeeeddddda";
        assert_eq!(
            move_to_front(string.as_bytes()),
            vec![97, 0, 0, 0, 98, 0, 0, 0, 101, 0, 0, 0, 0, 101, 0, 0, 0, 0, 3]
        )
    }
}
