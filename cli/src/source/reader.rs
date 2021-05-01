use ::std::path::PathBuf;
use ::std::thread;

use ::async_std::channel::Sender;
use ::async_std::channel::unbounded;
use ::async_std::task::block_on;
use ::lazy_static::lazy_static;
use ::log::debug;
use ::log::trace;

use crate::source::io::read_file;

lazy_static! {
    static ref READER_SENDER: Sender<ReadRequest> = start_reader();
}

#[derive(Debug)]
struct ReadRequest {
    path: PathBuf,
    known_ts_ms: Option<u64>,
}

pub fn load_file(path: PathBuf) -> Result<(), String> {
    block_on(READER_SENDER.send(ReadRequest {
        path,
        known_ts_ms: None,
    })).map_err(|_| "Failed to send file read request to reader thread".to_owned())
}

pub fn load_file_if_changed(path: PathBuf, known_ts_ms: u64) -> Result<(), String> {
    block_on(READER_SENDER.send(ReadRequest {
        path,
        known_ts_ms: Some(known_ts_ms),
    })).map_err(|_| "Failed to send file read request to reader thread".to_owned())
}

fn start_reader() -> Sender<ReadRequest> {
    let (sender, recver) = unbounded::<ReadRequest>();
    thread::spawn(move || {
        debug!("starting source reader channel");
        loop {
            match recver.recv().await {
                Ok(request) => {
                    //read_file()
                    trace!("source reader thread received request for '{}'", request.path.to_string_lossy());
                    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
                }
                Err(_) => {
                    break;
                }
            }
        }
        debug!("shutting down source reader channel");
    });
    sender
}
