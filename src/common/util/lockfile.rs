use ::std::fs::remove_file;
use ::std::fs::File;
use ::std::io::BufReader;
use ::std::io::BufWriter;
use ::std::time::SystemTime;
use ::std::time::UNIX_EPOCH;

use ::log::trace;
use ::serde::{Deserialize, Serialize};
use ::whoami;

use crate::common::util::paths::mangod_lock_file_path;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockInfo {
    pid: Option<u32>,
    address: String,
    update_ts: u64,
    username: String,
    hostname: String,
}

impl LockInfo {
    pub fn new(pid: u32, address: impl Into<String>) -> Self {
        let address = address.into();
        assert!(address.contains(':'), "address must contain port");
        let update_ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        LockInfo {
            pid: Some(pid),
            update_ts,
            address,
            username: whoami::username(),
            hostname: whoami::hostname(),
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }
}

pub fn load_lock() -> Option<LockInfo> {
    let pth = mangod_lock_file_path();
    if let Ok(file) = File::open(&pth) {
        trace!("reading mangod lockfile from '{}'", pth.to_string_lossy());
        let reader = BufReader::new(file);
        Some(serde_json::from_reader(reader).unwrap_or_else(|err| {
            panic!(
                "could not read the content of the mangod lock file (is it valid json?): '{}', reason: {}",
                pth.to_string_lossy(),
                err
            )
        }))
    } else {
        trace!("did not find mangod lockfile at '{}' for reading", pth.to_string_lossy());
        None
    }
}

pub fn store_lock(info: &LockInfo) {
    let pth = mangod_lock_file_path();
    trace!("writing mangod lockfile at '{}'", pth.to_string_lossy());
    let writer = BufWriter::new(File::create(&pth).unwrap_or_else(|err| {
        panic!(
            "could not access the mangod lock file: '{}', reason: {}",
            pth.to_string_lossy(),
            err
        )
    }));
    serde_json::to_writer_pretty(writer, info).unwrap_or_else(|err| {
        panic!(
            "could not write to the mangod lock file: '{}', reason: {}",
            pth.to_string_lossy(),
            err
        )
    })
}

pub fn clear_lock() {
    let pth = mangod_lock_file_path();
    trace!("removing mangod lockfile at '{}'", pth.to_string_lossy());
    remove_file(&pth).unwrap_or_else(|err| panic!("could not remove mangod lock file: '{}', reason: {}", pth.to_string_lossy(), err))
}

#[cfg(test)]
mod tests {
    use ::std::env;

    use ::serial_test::serial;
    use ::tempfile::TempDir;

    use super::*;

    #[serial]
    #[test]
    fn read_write_pid() {
        let dir = TempDir::new().unwrap();
        env::set_var("MANGO_USER_CACHE_PATH", &dir.path().to_string_lossy().into_owned());

        let before = LockInfo::new(1234, "localhost:47558");
        store_lock(&before);
        assert!(mangod_lock_file_path().is_file());
        let after = load_lock().unwrap();
        assert_eq!(before, after);
    }
}
