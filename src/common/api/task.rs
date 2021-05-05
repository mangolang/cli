use ::serde::{Deserialize, Serialize};

use crate::api::SourceContent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompileTarget {
    IR,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntrypointIdentifier {
    name: String,
    source: SourceContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskRequest {
    Compile(CompileTarget),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskResponse {}
