
#[derive(Debug)]
pub enum MangodStatus {
    /// There is no lockfile to suggest mangod is running.
    Inactive,
    /// There is a lockfile, but the pid does not belong to a running process.
    NotFound,
    /// The mangod process is running, but it is not responding to requests quickly.
    Unresponsive,
    /// The mangod process is running and responding to requests.
    Ok,
}

impl MangodStatus {
    pub fn is_ok(&self) -> bool {
        matches!(self, MangodStatus::Ok)
    }

    pub fn as_str(&self) -> &str {
        self.as_code()
    }

    /// Status code string. These codes should not change.
    pub fn as_code(&self) -> &str {
        match self {
            MangodStatus::Inactive => "not-started",
            MangodStatus::NotFound => "died-unexpectedly",
            MangodStatus::Unresponsive => "unresponsive",
            MangodStatus::Ok => "running",
        }
    }
}

pub fn determine_status(pid: u32) -> MangodStatus {
    unimplemented!()
}

