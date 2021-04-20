use ws::listen;
use crate::api::{Request, Response};

#[derive(Debug)]
pub struct ReqSender {}

#[derive(Debug)]
pub struct RespSender {}

pub fn server(addr: &str, handler: fn(Request, RespSender) -> Response) {
    listen(addr, |out| {
        move |req_data| {
            let req: Request = bincode::deserialize(&req_data)
                .expect("could not understand Request");  //TODO: better error handling
            let resp  = handler(req, todo);
            let resp_data = bincode::serialize(&world)
                .expect("could not encode Response");
            out.send(resp_data)
        }
    }).unwrap();
}

pub fn client() {
    unimplemented!()  //TODO @mark
}
