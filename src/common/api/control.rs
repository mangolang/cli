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

impl ControlRequest {
    pub fn type_name(&self) -> &str {
        match self {
            ControlRequest::Ping => "Ping",
            ControlRequest::Stop(_) => "Stop",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlResponse {
    Pong,
    Stopping(StopMode),
    Stopped,
}

impl ControlResponse {
    pub fn type_name(&self) -> &str {
        match self {
            ControlResponse::Pong => "Pong",
            ControlResponse::Stopping(_) => "Stopping",
            ControlResponse::Stopped => "Stopped",
        }
    }
}
