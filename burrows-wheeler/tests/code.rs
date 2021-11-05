use burrows_wheeler::{decode, encode};
use std::{
    fs::File,
    io::{Cursor, Read},
};

#[test]
fn war_and_peace_test() {
    let file_name = "files/war&peace.txt";
    let war_and_peace = File::open(file_name).unwrap();
    let encoded = encode(Box::new(war_and_peace)).collect::<Vec<_>>();
    let encoded = Cursor::new(encoded);

    let file = File::open(file_name).unwrap().bytes().map(|x| x.unwrap());
    let decoded = decode(Box::new(encoded)).map(|x| x.unwrap());

    assert_eq!(std::cmp::Ordering::Equal, file.cmp(decoded));
}
