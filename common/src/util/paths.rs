use ::std::env::temp_dir;
use ::std::fs::create_dir_all;
use ::std::path::PathBuf;

use ::dirs::cache_dir;
use ::dirs::home_dir;


pub fn get_cache_dir() -> PathBuf {
    let mut pth = cache_dir()
        .or_else(|| home_dir())
        .unwrap_or_else(|| temp_dir());
    pth.push("mango");
    create_dir_all(&pth)
        .expect("could not create mango cache directory");
    pth
}

pub fn get_lock_file() -> PathBuf {
    let mut pth = get_cache_dir();
    pth.push("mangod.lock");
    pth
}
