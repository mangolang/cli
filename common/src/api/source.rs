use ::serde::{Deserialize, Serialize};

use crate::api::compression::Compression;

//TODO: possible optimizations:
//TODO: - single string with some delimiter to save allocations
//TODO: - tree-like structure for efficient storage of nested paths

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceIdentifier {
    // The Mango-style dot-separated path, including project name.
    path: String,
}

impl SourceIdentifier {
    pub fn as_str(&self) -> &str {
        &self.path
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceState {
    pub identifier: SourceIdentifier,
    pub ts_changed_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceRequests {
    //TODO @mark: compression here?
    requests: Vec<SourceRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceRequest {
    Need(SourceIdentifier),
    IfChanged(SourceState),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceContent {
    identifier: SourceIdentifier,
    ts_changed_ms: u64,
    content: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceResponses {
    responses: Compression<Vec<SourceResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceResponse {
    Source(Vec<SourceContent>),
    Unchanged(Vec<SourceIdentifier>),
    SourceNotFound(SourceIdentifier),
}
