use ::std::env::temp_dir;
use ::std::fs;
use ::std::fs::File;
use ::std::io::BufReader;
use ::std::path::PathBuf;

use ::dirs::cache_dir;
use ::dirs::home_dir;
use ::serde::{Deserialize, Serialize};
use std::io::BufWriter;

#[derive(Debug, Serialize, Deserialize)]
pub struct LockInfo {
    pid: Option<u32>,
}

fn get_cache_dir() -> PathBuf {
    let mut pth = cache_dir()
        .or_else(|| home_dir())
        .unwrap_or_else(|| temp_dir());
    pth.push("mango");
    fs::create_dir_all(&pth)
        .expect("could not create mango cache directory");
    pth
}

fn get_lock_file() -> PathBuf {
    let mut pth = get_cache_dir();
    pth.push("mangod.lock");
    pth
}

pub fn load_lock() -> LockInfo {
    let pth = get_lock_file();
    let reader = BufReader::new(File::open(&pth)
        .unwrap_or_else(|_| panic!("could not access the mangod lock file: '{}'", pth.to_string_lossy())));
    serde_json::from_reader(reader)
        .unwrap_or_else(|_| panic!("could not read the content of the mangod lock file (is it valid json?): '{}'", pth.to_string_lossy()))
}

pub fn store_lock(info: &LockInfo) {
    let pth = get_lock_file();
    let writer = BufWriter::new(File::open(&pth)
        .unwrap_or_else(|_| panic!("could not access the mangod lock file: '{}'", pth.to_string_lossy())));
    serde_json::to_writer_pretty(writer, info)
        .unwrap_or_else(|_| panic!("could not write to the mangod lock file: '{}'", pth.to_string_lossy()))
}
