use ::mango_cli_common::api::SourceRequest;

use crate::source::reader::{load_file, load_file_if_changed};
use crate::source::lookup::identifier_to_file;

pub fn handle_source_request(request: SourceRequest) {
    match request {
        SourceRequest::Need(identifier) => {
            load_file(
                identifier
            ).unwrap();
        }
        SourceRequest::IfChanged(state) => {
            load_file_if_changed(
                state.identifier,
                state.ts_changed_ms,
            ).unwrap()
        }
    }
}
