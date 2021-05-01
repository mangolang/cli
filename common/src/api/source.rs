use ::serde::{Deserialize, Serialize};

//TODO: possible optimizations:
//TODO: - single string with some delimiter to save allocations
//TODO: - tree-like structure for efficient storage of nested paths

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceIdentifier {
    // The Mango-style dot-separated path
    path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceState {
    identifier: SourceIdentifier,
    ts_changed_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceRequest {
    Need(Vec<SourceIdentifier>),
    IfChanged(Vec<SourceState>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceCompressionKind {
    None,
    Brotli,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceContent {
    identifier: SourceIdentifier,
    ts_changed_ms: u64,
    content: Vec<u8>,
    compression: SourceCompressionKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceResponse {
    Source(Vec<SourceContent>),
    Unchanged(Vec<SourceIdentifier>),
    SourceNotFound(SourceIdentifier),
}