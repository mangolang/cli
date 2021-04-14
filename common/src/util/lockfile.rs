use ::std::fs::File;
use ::std::io::BufReader;
use ::std::io::BufWriter;
use ::std::time::SystemTime;
use ::std::time::UNIX_EPOCH;

use ::serde::{Deserialize, Serialize};
use crate::util::paths::get_lock_file;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockInfo {
    pid: Option<u32>,
    timestamp: u64,
}

impl LockInfo {
    pub fn new(pid: Option<u32>) -> Self {
        let since_the_epoch_s = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        LockInfo {
            pid,
            timestamp: since_the_epoch_s,
        }
    }
}

pub fn load_lock() -> LockInfo {
    let pth = get_lock_file();
    let reader = BufReader::new(File::open(&pth)
        .unwrap_or_else(|err| panic!("could not access the mangod lock file: '{}', reason: {}", pth.to_string_lossy(), err)));
    serde_json::from_reader(reader)
        .unwrap_or_else(|err| panic!("could not read the content of the mangod lock file (is it valid json?): '{}', reason: {}", pth.to_string_lossy(), err))
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
        let before = LockInfo::new(Some(1234));
        store_lock(&before);
        assert!(get_lock_file().is_file());
        let after = load_lock();
        assert_eq!(before, after);
    }

    #[serial]
    #[test]
    fn read_write_no_pid() {
        let before = LockInfo::new(None);
        store_lock(&before);
        assert!(get_lock_file().is_file());
        let after = load_lock();
        assert_eq!(before, after);
    }
}
