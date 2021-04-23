use ::std::sync::mpsc::channel;
use ::std::sync::mpsc::RecvTimeoutError;
use ::std::sync::RwLock;
use ::std::time::{SystemTime, UNIX_EPOCH};
use ::std::time::Duration;

use ::lazy_static::lazy_static;
use ::log::debug;

use crate::api::{ControlResponse, Response};
use crate::util::{client, load_lock};

lazy_static! {
    static ref LAST_STATUS: RwLock<Option<(u128, MangodStatus)>> = RwLock::new(None);
}

#[derive(Debug, Clone)]
pub enum MangodStatus {
    /// There is no lockfile to suggest mangod is running.
    Inactive,
    /// There is a lockfile, but no mangod is responding to requests.
    Unresponsive { address: String },
    /// The mangod process is running and responding to requests.
    Ok { address: String },
}

impl MangodStatus {
    pub fn determine() -> Self {
        get_status()
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, MangodStatus::Ok { .. })
    }

    pub fn as_str(&self) -> &str {
        self.as_code()
    }

    /// Status code string. These codes should not change.
    pub fn as_code(&self) -> &str {
        match self {
            MangodStatus::Inactive => "not-started",
            MangodStatus::Unresponsive { .. } => "unresponsive",
            MangodStatus::Ok { .. } => "running",
        }
    }
}

fn get_status() -> MangodStatus {

    // Check if there is a recent status in the cache.
    LAST_STATUS.read().unwrap().as_ref().and_then(|(previous_ms, status)| {
        let current_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        if current_ms - previous_ms < 2_000 {
            Some(status.clone())
        } else {
            None
        }
    }).unwrap_or_else(|| determine_status())
}

fn determine_status() -> MangodStatus {

    if let Some(info) = load_lock() {
        return match can_ping(info.address()) {
            true => MangodStatus::Ok { address: info.address().to_owned() },
            false => MangodStatus::Unresponsive { address: info.address().to_owned() },
        }
    }

    MangodStatus::Inactive
}

pub fn can_ping(address: &str) -> bool {
    let (channel_sender, channel_receiver) = channel();
    let timeout = Duration::from_millis(700);

    // Send ping message to the server.
    if let Err(_) = client(address,
        |req_sender| if let Err(_) = req_sender.send(Request::Ping) {
            debug!("failed to send ping message to {}", address);
            channel_sender.send(false).unwrap();
            req_sender.close();
        },
       |resp, req_sender| {
           if matches!(resp, Response::Control(ControlResponse::Pong)) {
               channel_sender.send(true).unwrap();
           } else {
               debug!("got unexpected answer from {} in response to ping", address);
               channel_sender.send(false).unwrap();
           }
           req_sender.close();
           Ok(())
    }) {
        debug!("failed to not connect to {} for ping", address);
        return false
    };

    // Check if we got a pong message back.
    return match channel_receiver.recv_timeout(timeout) {
        Ok(true) => true,
        Ok(false) => false,
        Err(RecvTimeoutError::Timeout) => {
            debug!("timed out while connecting to {}", address);
            false
        }
        Err(RecvTimeoutError::Disconnected) => {
            debug!("connection to {} was immediately broken", address);
            false
        }
    }
}
