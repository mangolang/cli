use ::std::fs::File;
use ::std::io::BufReader;
use ::std::io::BufWriter;
use ::std::time::SystemTime;
use ::std::time::UNIX_EPOCH;

use ::serde::{Deserialize, Serialize};
use ::whoami;

use crate::util::paths::get_lock_file;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockInfo {
    pid: u32,
    address: String,
    update_ts: u64,
    username: String,
    hostname: String,
}

impl LockInfo {
    pub fn new(pid: u32, address: impl Into<String>) -> Self {
        let address = address.into();
        assert!(address.contains(":"), "address must contain port");
        let update_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        LockInfo {
            pid,
            update_ts,
            address,
            username: whoami::username(),
            hostname: whoami::hostname()
        }
    }

    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn address(&self) -> &str {
        &self.address
    }
}

pub fn load_lock() -> Option<LockInfo> {
    let pth = get_lock_file();
    if let Ok(file) = File::open(&pth) {
        let reader = BufReader::new(file);
        Some(serde_json::from_reader(reader)
            .unwrap_or_else(|err| panic!("could not read the content of the mangod lock file (is it valid json?): '{}', reason: {}", pth.to_string_lossy(), err)))
    } else {
        None
    }
}

pub fn store_lock(info: &LockInfo) {
    let pth = get_lock_file();
    let writer = BufWriter::new(File::create(&pth)
        .unwrap_or_else(|err| panic!("could not access the mangod lock file: '{}', reason: {}", pth.to_string_lossy(), err)));
    serde_json::to_writer_pretty(writer, info)
        .unwrap_or_else(|err| panic!("could not write to the mangod lock file: '{}', reason: {}", pth.to_string_lossy(), err))
}

#[cfg(test)]
mod tests {
    use ::serial_test::serial;

    use super::*;

    #[serial]
    #[test]
    fn read_write_pid() {
        let before = LockInfo::new(1234, "localhost:47558");
        store_lock(&before);
        assert!(get_lock_file().is_file());
        let after = load_lock();
        assert_eq!(before, after);
    }
}
