use ::std::path::PathBuf;
use ::std::sync::mpsc;
use ::std::sync::mpsc::SyncSender;
use ::std::thread;

use ::lazy_static::lazy_static;
use ::log::info;
use ::log::trace;

lazy_static! {
    static ref READER_SENDER: SyncSender<ReadRequest> = start_reader();
}

#[derive(Debug)]
struct ReadRequest {
    path: PathBuf,
    known_ts_ms: Option<u64>,
}

pub fn load_file(path: PathBuf) -> Result<(), String> {
    READER_SENDER.send(ReadRequest {
        path,
        known_ts_ms: None,
    }).map_err(|_| "Failed to send file read request to reader thread".to_owned())
}

pub fn load_file_if_changed(path: PathBuf, known_ts_ms: u64) -> Result<(), String> {
    READER_SENDER.send(ReadRequest {
        path,
        known_ts_ms: Some(known_ts_ms),
    }).map_err(|_| "Failed to send file read request to reader thread".to_owned())
}

fn start_reader() -> SyncSender<ReadRequest> {
    let (sender, recver) = mpsc::sync_channel::<ReadRequest>(1024);
    thread::spawn(move || {
        info!("starting source reader channel");
        loop {
            match recver.recv() {
                Ok(request) => {
                    trace!("source reader thread received request for '{}'", request.path.to_string_lossy());
                    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
                }
                Err(_) => {
                    break;
                }
            }
        }
        info!("shutting down source reader channel");
    });
    sender
}
