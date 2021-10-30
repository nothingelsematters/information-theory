use crate::bwt;
use crate::utils::WindowedIterator;
use std::io::Read;

pub fn encode(read: Box<dyn Read>) -> Box<impl Iterator<Item = u8>> {
    let iter = WindowedIterator::from_read(100 * 1024, read)
        .take_while(|x| x.is_ok())
        .map(|x| x.unwrap())
        .flat_map(|window| huffman::encode(|| Box::new(bwt::apply(&window).0.into_iter())));
    Box::new(iter)
}
