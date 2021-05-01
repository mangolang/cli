use ::std::path::PathBuf;
use ::std::sync::Arc;
use ::std::sync::mpsc;
use ::std::sync::mpsc::Sender;
use ::std::thread;
use ::std::thread_local;

use ::log::info;
use std::sync::atomic::{AtomicBool, Ordering};

const IS_READER_INITIALIZED: AtomicBool = AtomicBool::new(false);
thread_local! {
    static READER_SENDER: Arc<Sender<ReadRequest>> = Arc::new(start_reader());
}

#[derive(Debug)]
pub struct ReadRequest {
    path: PathBuf,
    known_ts_ms: Option<u64>,
}

pub fn load_file() {
    start_reader();
    //let r = Runtime::new().unwrap();
}

pub fn start_reader() {
    assert!(!IS_READER_INITIALIZED.store(true, Ordering::Release));
    let (sender, recver) = mpsc::channel();
    thread::spawn(|| {
        info!("starting source reader channel");
        loop {
            match recver.recv() {
                Ok(msg) => {}
                Err(_) => {
                    break;
                }
            }
        }
        info!("shutting down source reader channel");
    });
    sender
}