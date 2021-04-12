use ::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Request {
}

#[derive(Serialize, Deserialize)]
enum Response {
    Success {

    },
    Errors {

    }
}
