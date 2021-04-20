use ::serde::{Deserialize, Serialize};

pub use self::control::{ControlRequest, ControlResponse, StopMode};

mod control;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestEnvelope {
    pub id: u64,
    pub data: Request,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Control(ControlRequest),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseEnvelope {
    pub id: u64,
    pub data: Response,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Ok,
    DaemonError(String),
    Control(ControlResponse),
    //CompileError(),
}
