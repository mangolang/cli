use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Compression<T> {
    Uncompressed(T),
    Brotli(Vec<u8>),
}
