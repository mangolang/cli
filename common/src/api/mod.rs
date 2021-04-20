use ::serde::{Deserialize, Serialize};

pub use self::control::{ControlRequest, ControlResponse, StopMode};

mod control;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    id: u64,
    data: RequestData,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestData {
    Control(ControlRequest),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    id: u64,
    data: ResponseData,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseData {
    DaemonError(String),
    Control(ControlResponse),
    //CompileError(),
}
