use ::serde::{Deserialize, Serialize};

use crate::common::api::SourceContent;

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

impl TaskResponse {
    pub fn type_name(&self) -> &str {
        return "Task"
    }
}
