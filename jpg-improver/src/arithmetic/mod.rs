mod decode;
mod encode;
mod frequencies;

pub use decode::decode;
pub use encode::encode;

const NUMBER_OF_CHARS: usize = 256;
const CODE_VALUE_BITS: i32 = 16;
const CODE_VALUE_MAX: usize = (1 << CODE_VALUE_BITS) - 1;
const CODE_VALUE_FIRST_QUARTER: usize = CODE_VALUE_MAX / 4 + 1;
const CODE_VALUE_HALF: usize = 2 * CODE_VALUE_FIRST_QUARTER;
const CODE_VALUE_THIRD_QUARTER: usize = 3 * CODE_VALUE_FIRST_QUARTER;
const MAX_FREQUENCY: usize = CODE_VALUE_FIRST_QUARTER - 1;
const EOF_SYMBOL: usize = NUMBER_OF_CHARS + 1;
const NUMBER_OF_SYMBOLS: usize = NUMBER_OF_CHARS + 1;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn application_test() {
        let input = "aabbb".bytes();
        println!("{:?}", encode(&input.collect::<Vec<_>>()));
    }

    #[test]
    fn reverse_simpliest_test() {
        let input = "aabbb";
        assert_eq!(
            input.bytes().collect::<Vec<_>>(),
            decode(&encode(&input.bytes().collect::<Vec<_>>()))
        );
    }

    #[test]
    fn reverse_simple_test() {
        let input = String::from("fqwefhqoiqwiwiiwfqwefhqoiqwiwiiw");
        assert_eq!(
            input.bytes().collect::<Vec<_>>(),
            decode(&encode(&input.bytes().collect::<Vec<_>>()))
        );
    }
}
