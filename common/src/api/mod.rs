use ::serde::{Deserialize, Serialize};

pub use self::control::{ControlRequest, ControlResponse, StopMode};

mod control;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestEnvelope {
    pub trace: u64,
    pub data: Request,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    Control(ControlRequest),
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
    //CompileError(),
}
