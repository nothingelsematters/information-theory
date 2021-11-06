mod decode;
mod encode;
mod header;
mod iterator;

pub use decode::decode;
pub use encode::encode;

pub type BoxedByteIterator = Box<dyn Iterator<Item = u8>>;
