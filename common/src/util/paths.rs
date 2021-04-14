use ::std::path::PathBuf;
use ::dirs::cache_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct LockInfo {
    pid: Option<u32>,
}

pub fn load_lock() -> PathBuf {
    let pth = cache_dir();
}