use ::std::sync::mpsc::channel;
use ::std::sync::RwLock;
use ::std::time::{SystemTime, UNIX_EPOCH};
use ::std::time::Duration;

use ::lazy_static::lazy_static;
use ::ws::CloseCode;
use ::ws::connect;
use ::ws::Message;

use crate::util::load_lock;

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

    let (sender, receiver) = channel();
    if let Some(info) = load_lock() {

        // Send ping message to the server.
        if let Err(_) = connect(format!("ws://{}", info.address()), |out| {
            //TODO @mark: change this to bincode with serde
            let sender = sender.clone();
            if let Err(_) = out.send("ping") {
                sender.send(false).unwrap();
                out.close(CloseCode::Normal).unwrap();
            }
            move |msg: Message| {
                let got_pong = msg.as_text().unwrap() == "pong";
                sender.send(got_pong).unwrap();
                out.close(CloseCode::Normal)
            }
        }) {
            return MangodStatus::Unresponsive { address: info.address().to_owned() }
        };

        // Check if we got a pong message back.
        return match receiver.recv_timeout(Duration::new(1, 0)) {
            Ok(true) => MangodStatus::Ok { address: info.address().to_owned() },
            Err(_) | Ok(false) => MangodStatus::Unresponsive { address: info.address().to_owned() },
        }
    }

    MangodStatus::Inactive
}
