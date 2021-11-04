use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CodeDescriptor {
    pub code: Vec<bool>,
    pub letter: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub code_descriptors: Vec<CodeDescriptor>,
    pub byte_size: usize,
    pub last_byte_size: u8,
}
