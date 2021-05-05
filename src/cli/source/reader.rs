use ::std::thread;

use ::async_std::channel::unbounded;
use ::async_std::channel::Sender as ChannelSender;
use ::async_std::task::block_on;
use ::async_std::task::spawn as spawn_async;
use ::lazy_static::lazy_static;
use ::log::debug;
use ::log::trace;

use crate::common::api::SourceIdentifier;
use crate::common::api::Upstream;
use crate::common::api::{SourceContent, SourceResponse};
use crate::common::util::ReqSender;

use crate::cli::source::io::read_file;
use crate::cli::source::lookup::identifier_to_file;

lazy_static! {
    static ref READER_SENDER: ChannelSender<ReadRequest> = start_reader();
}

#[derive(Debug)]
struct ReadRequest {
    identifier: SourceIdentifier,
    known_ts_ms: Option<u64>,
    sender: ReqSender,
}

pub fn load_file(identifier: SourceIdentifier, sender: ReqSender) -> Result<(), String> {
    block_on(READER_SENDER.send(ReadRequest {
        identifier,
        known_ts_ms: None,
        sender,
    }))
    .map_err(|_| "Failed to send file read request to reader thread".to_owned())
}

pub fn load_file_if_changed(identifier: SourceIdentifier, known_ts_ms: u64, sender: ReqSender) -> Result<(), String> {
    block_on(READER_SENDER.send(ReadRequest {
        identifier,
        known_ts_ms: Some(known_ts_ms),
        sender,
    }))
    .map_err(|_| "Failed to send file read request to reader thread".to_owned())
}

fn start_reader() -> ChannelSender<ReadRequest> {
    let (sender, recver) = unbounded::<ReadRequest>();
    thread::spawn(move || {
        debug!("starting source reader channel");
        //TODO @mark: there is a threaded one, https://docs.rs/tokio-async-std/1.5.3/async_std/runtime/index.html
        block_on(async {
            while let Ok(request) = recver.recv().await {
                handle_read_request(request);
            }
        });
        debug!("shutting down source reader channel");
    });
    sender
}

fn handle_read_request(request: ReadRequest) {
    let ReadRequest {
        identifier,
        known_ts_ms,
        sender,
    } = request;
    trace!("source reader thread received request for '{}'", identifier.as_str());
    let path = match identifier_to_file(identifier.as_str()) {
        Ok(path) => path,
        Err(err_msg) => {
            debug!("did not find source '{}'; {}", identifier.as_str(), err_msg);
            sender.send(Upstream::Source(SourceResponse::SourceNotFound(identifier)));
            return;
        }
    };
    spawn_async(async move {
        //TODO @mark: buffer these messages and send a veco f them so they can be compressed together
        trace!("source reader reading '{}'", path.to_string_lossy());
        match read_file(path.as_path(), known_ts_ms).await {
            Some((current_ts_ms, data)) => {
                trace!(
                    "sending source content for '{}' at '{}' (ts: {}, length: {})",
                    identifier.as_str(),
                    path.as_path().to_string_lossy(),
                    current_ts_ms,
                    data.len()
                );
                sender.send(Upstream::Source(SourceResponse::Source(vec![SourceContent::new(
                    identifier,
                    current_ts_ms,
                    data,
                )])));
            }
            None => {
                trace!(
                    "source has not changed for '{}' at '{}', because ts for both is {}",
                    identifier.as_str(),
                    path.as_path().to_string_lossy(),
                    known_ts_ms.unwrap()
                );
                sender.send(Upstream::Source(SourceResponse::Unchanged(vec![identifier])));
            }
        }
    });
}

//TODO @mark: unit test at least handle_read_request
