use ::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum StopMode {
    Force,
    Quick,
    FinishCurrentWork,
    WhenIdle,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ControlRequest {
    Ping,
    Stop(StopMode),
    Stats
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ControlResponse {
    Pong,
    Stats
}
