use ::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StopMode {
    Quick,
    FinishCurrentWork,
    WhenIdle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlRequest {
    Ping,
    Stop(StopMode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlResponse {
    Pong,
    Stopping,
    Stopped,
}
