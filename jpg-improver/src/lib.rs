use std::convert::TryInto;

pub mod arithmetic;
pub mod rle;
pub mod utils;

pub fn encode(data: &[u8]) -> Vec<u8> {
    let (mut data, num) = burrows_wheeler::bwt::apply(data);
    let mut new_data = Vec::from(num.to_be_bytes());
    new_data.append(&mut data);
    let data = new_data;
    let data = burrows_wheeler::mtf::apply(&data);
    let data = rle::apply(&data);
    arithmetic::encode(&data)
}

pub fn decode(data: &[u8]) -> Vec<u8> {
    let data = arithmetic::decode(data);
    let data = rle::reverse(&data);
    let data = burrows_wheeler::mtf::reverse(&data);
    burrows_wheeler::bwt::reverse(
        &data[4..],
        u32::from_be_bytes(data[..4].try_into().expect("Invalid data")),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a0_rle_test() {
        let input = "abcdef";
        let bytes = input.bytes().collect::<Vec<_>>();

        let encoded = arithmetic::encode(&rle::apply(&bytes));
        let decoded = rle::reverse(&arithmetic::decode(&encoded));
        assert_eq!(bytes, decoded);
    }

    #[test]
    fn reverse_simplies_test() {
        let input = "abcdef";
        let bytes = input.bytes().collect::<Vec<_>>();
        assert_eq!(bytes, decode(&encode(&bytes)));
    }

    #[test]
    fn reverse_simplier_test() {
        let input = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let bytes = input.bytes().collect::<Vec<_>>();
        assert_eq!(bytes, decode(&encode(&bytes)));
    }

    #[test]
    fn reverse_simple_test() {
        let input = String::from("aaaaaaaaaatttttqwojdkqwdoibbbbwjw");
        let bytes = input.bytes().collect::<Vec<_>>();
        assert_eq!(bytes, decode(&encode(&bytes)));
    }
}
