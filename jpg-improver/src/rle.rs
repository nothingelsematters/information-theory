const MAX_SEQUENCE_SIZE: u8 = u8::MAX;

pub fn apply(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut previous_byte = None;
    let mut current_sequence = 0;

    for i in data {
        if Some(i) == previous_byte {
            current_sequence += 1;
        } else {
            if current_sequence >= 2 {
                result.push(*previous_byte.unwrap());
                result.push(current_sequence);
            }
            result.push(*i);
            current_sequence = 1;
        }

        if current_sequence == MAX_SEQUENCE_SIZE {
            result.push(*i);
            result.push(current_sequence);
            current_sequence = 1;
        }
        previous_byte = Some(i);
    }

    if current_sequence >= 2 {
        result.push(*previous_byte.unwrap());
        result.push(current_sequence);
    }

    result
}

pub fn reverse(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut previous_byte = None;

    let mut i = 0;
    while i < data.len() {
        result.push(data[i]);

        previous_byte = if Some(data[i]) == previous_byte {
            i += 1;
            let current_sequence = data[i];

            for _ in 0..current_sequence - 2 {
                result.push(data[i - 1])
            }
            None
        } else {
            Some(data[i])
        };

        i += 1;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn application_test() {
        let input = "aaaaaaaaaatttttqwojdkqwdoibbbbwjw".bytes();
        println!("{:?}", apply(&input.collect::<Vec<_>>()));
    }

    #[test]
    fn reversing_test() {
        let input = vec![97, 97, 97];
        println!("{:?}", reverse(&input));
    }

    #[test]
    fn reverse_simpliest_test() {
        let input = "abcdef";
        let bytes = input.bytes().collect::<Vec<_>>();
        assert_eq!(bytes, reverse(&apply(&bytes)));
    }

    #[test]
    fn reverse_simplier_test() {
        let input = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let bytes = input.bytes().collect::<Vec<_>>();
        assert_eq!(bytes, reverse(&apply(&bytes)));
    }

    #[test]
    fn reverse_simple_test() {
        let input = String::from("aaaaaaaaaatttttqwojdkqwdoibbbbwjw");
        let bytes = input.bytes().collect::<Vec<_>>();
        assert_eq!(bytes, reverse(&apply(&bytes)));
    }
}
