use ::serde::{Deserialize, Serialize};

pub mod util;

#[derive(Debug, Serialize, Deserialize)]
struct Request {
}

#[derive(Debug, Serialize, Deserialize)]
enum Response {
    Success {

    },
    Errors {

    }
}
