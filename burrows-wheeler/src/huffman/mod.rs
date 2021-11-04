mod common;
mod decode;
mod encode;

pub use decode::decode;
pub use encode::encode;

pub type BoxedByteIterator = Box<dyn Iterator<Item = u8>>;
