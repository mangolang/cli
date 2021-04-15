use ::serde::{Deserialize, Serialize};

mod control;
pub use self::control::{ControlRequest, ControlResponse, StopMode};

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
    Ok,
    DaemonError(String),
    Control(ControlResponse),
    //CompileError(),
}
