use ::std::thread;

use ::async_std::channel::Sender;
use ::async_std::channel::unbounded;
use ::async_std::task::block_on;
use ::async_std::task::spawn as spawn_async;
use ::lazy_static::lazy_static;
use ::log::debug;
use ::log::trace;

use crate::source::io::read_file;
use mango_cli_common::api::SourceIdentifier;
use crate::source::lookup::identifier_to_file;
use std::path::PathBuf;

lazy_static! {
    static ref READER_SENDER: Sender<ReadRequest> = start_reader();
}

#[derive(Debug)]
struct ReadRequest {
    identifier: SourceIdentifier,
    known_ts_ms: Option<u64>,
}

pub fn load_file(identifier: SourceIdentifier) -> Result<(), String> {
    block_on(READER_SENDER.send(ReadRequest {
        identifier,
        known_ts_ms: None,
    })).map_err(|_| "Failed to send file read request to reader thread".to_owned())
}

pub fn load_file_if_changed(identifier: SourceIdentifier, known_ts_ms: u64) -> Result<(), String> {
    block_on(READER_SENDER.send(ReadRequest {
        identifier,
        known_ts_ms: Some(known_ts_ms),
    })).map_err(|_| "Failed to send file read request to reader thread".to_owned())
}

fn start_reader() -> Sender<ReadRequest> {
    let (sender, recver) = unbounded::<ReadRequest>();
    thread::spawn(move || {
        debug!("starting source reader channel");
        //TODO @mark: there is a threaded one, https://docs.rs/tokio-async-std/1.5.3/async_std/runtime/index.html
        block_on(async {
            loop {
                match recver.recv().await {
                    //TODO @mark: separate function at this point:
                    Ok(request) => {
                        let ReadRequest { identifier, known_ts_ms } = request;
                        trace!("source reader thread received request for '{}'", identifier.as_str());
                        let path = match identifier_to_file(identifier.as_str()) {
                            Ok(path) => path,
                            Err(_) => todo!("tell the server that the file was not found, so it can stop"),
                        };
                        spawn_async(async {
                            trace!("source reader starting to read '{}'", path.to_string_lossy());
                            match read_file(path.as_path(), known_ts_ms).await {
                                Some((current_ts_ms, data)) => todo!("send the file data to server"),
                                None => todo!("tell the server that the file was up-to-date"),
                            }
                        });
                        unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        });
        unimplemented!("is this reachable? remove if so");  //TODO @mark
        debug!("shutting down source reader channel");
    });
    sender
}
