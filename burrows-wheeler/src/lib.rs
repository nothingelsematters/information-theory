pub mod bwt;
mod config;
mod decode;
mod encode;
mod huffman;
pub mod mtf;
pub mod result;
pub mod utils;

pub use decode::decode;
pub use encode::encode;
