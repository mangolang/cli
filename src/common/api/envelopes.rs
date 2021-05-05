use ::serde::{Deserialize, Serialize};

use crate::common::api::source::{SourceRequest, SourceResponse};
use crate::common::api::task::{TaskRequest, TaskResponse};
use crate::common::api::{ControlRequest, ControlResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamEnvelope {
    pub trace: u64,
    pub data: Upstream,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Upstream {
    Control(ControlRequest),
    Task(TaskRequest),
    Source(SourceResponse),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownstreamEnvelope {
    pub trace: u64,
    pub data: Downstream,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Downstream {
    Ok,
    DaemonError(String),
    Control(ControlResponse),
    Task(TaskResponse),
    Source(SourceRequest),
    //CompileError(),
}
