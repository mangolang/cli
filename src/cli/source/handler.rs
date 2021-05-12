use crate::common::api::SourceRequest;
use crate::common::util::ReqSender;

use crate::cli::source::reader::{load_file, load_file_if_changed};

pub fn handle_source_request(request: SourceRequest, sender: ReqSender) {
    match request {
        SourceRequest::Need(identifier) => {
            load_file(identifier, sender).unwrap();
        }
        SourceRequest::IfChanged(state) => load_file_if_changed(state.identifier, state.ts_changed_ms, sender).unwrap(),
    }
}
