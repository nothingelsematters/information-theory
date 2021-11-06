use atlist_rs::LinkedList;

pub fn apply(buffer: &[u8]) -> Vec<u8> {
    let mut alphabet: LinkedList<u8> = (0..=255).collect();

    let mut output = Vec::with_capacity(buffer.len());
    for i in buffer {
        let mut iter = alphabet.iter_mut();
        let mut index: u8 = 0;

        while *iter.unwrap() != *i {
            iter.next();
            index += 1;
        }

        output.push(index);
        alphabet.remove_iter_mut(&mut iter).unwrap();
        alphabet.push_front(*i).unwrap();
    }

    output
}

pub fn reverse(buffer: &[u8]) -> Vec<u8> {
    let mut alphabet: LinkedList<u8> = (0..=255).collect();

    let mut output = Vec::with_capacity(buffer.len());
    for i in buffer {
        let mut iter = alphabet.iter_mut();

        for _ in 0..*i {
            iter.next();
        }

        let letter = *iter.unwrap();
        output.push(letter);
        alphabet.remove_iter_mut(&mut iter).unwrap();
        alphabet.push_front(letter).unwrap();
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn move_to_front_test() {
        let string = "aaaabbbbeeeeeddddda";
        assert_eq!(
            apply(string.as_bytes()),
            vec![97, 0, 0, 0, 98, 0, 0, 0, 101, 0, 0, 0, 0, 101, 0, 0, 0, 0, 3]
        )
    }

    #[test]
    fn reverse_move_to_front_test() {
        let string = "aaaabbbbeeeeeddddda".as_bytes();
        assert_eq!(reverse(&apply(string)), string)
    }
}
