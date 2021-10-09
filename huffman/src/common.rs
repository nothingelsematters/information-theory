use serde::{Deserialize, Serialize};

pub type Code = Vec<bool>;

#[derive(Serialize, Deserialize)]
pub struct CodeDescriptor {
    pub code: Code,
    pub letter: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub code_descriptors: Vec<CodeDescriptor>,
    pub last_byte_size: u8,
}
