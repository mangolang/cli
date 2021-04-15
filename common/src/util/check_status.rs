use ::std::sync::RwLock;

use ::lazy_static::lazy_static;
use std::time::{UNIX_EPOCH, SystemTime};

lazy_static! {
    static ref LAST_STATUS: RwLock<Option<(u128, MangodStatus)>> = RwLock::new(None);
}

#[derive(Debug, Clone)]
pub enum MangodStatus {
    /// There is no lockfile to suggest mangod is running.
    Inactive,
    /// There is a lockfile, but no mangod is responding to requests.
    Unresponsive { pid: u32, address: String },
    /// The mangod process is running and responding to requests.
    Ok { pid: u32, address: String },
}

impl MangodStatus {
    pub fn determine(pid: u32, address: impl Into<String>) -> Self {
        get_status(pid, address)
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

fn get_status(pid: u32, address: impl Into<String>) -> MangodStatus {

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
    }).unwrap_or_else(|| determine_status(pid, address))
}

fn determine_status(pid: u32, address: impl Into<String>) -> MangodStatus {
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}
