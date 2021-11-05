use crate::config::Index;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CodeDescriptor {
    pub code: Vec<bool>,
    pub letter: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub code_descriptors: Vec<CodeDescriptor>,
    pub bit_size: usize,
    pub initial: Index,
}
