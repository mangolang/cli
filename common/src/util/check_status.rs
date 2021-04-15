


#[derive(Debug)]
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
        determine_status(pid, address)
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

fn determine_status(pid: u32, address: impl Into<String>) -> MangodStatus {
    unimplemented!()
}

