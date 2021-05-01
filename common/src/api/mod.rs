use ::serde::{Deserialize, Serialize};

use crate::api::source::{SourceRequest, SourceResponse};

pub use self::control::{ControlRequest, ControlResponse, StopMode};

mod control;
mod source;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestEnvelope {
    pub trace: u64,
    pub data: Request,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    Control(ControlRequest),
    Source(SourceRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseEnvelope {
    pub trace: u64,
    pub data: Response,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Response {
    Ok,
    DaemonError(String),
    Control(ControlResponse),
    Source(SourceResponse),
    //CompileError(),
}
