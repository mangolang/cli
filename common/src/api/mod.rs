use ::serde::{Deserialize, Serialize};

use crate::api::source::{SourceRequest, SourceResponse};

pub use self::control::{ControlRequest, ControlResponse, StopMode};

mod control;
mod source;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamEnvelope {
    pub trace: u64,
    pub data: Upstream,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Upstream {
    Control(ControlRequest),
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
    Source(SourceRequest),
    //CompileError(),
}
