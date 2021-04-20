use ::ws::listen;
use ::ws::Message;
use ::ws::Sender;

use ::log::warn;

use crate::api::{Request, ResponseEnvelope, RequestEnvelope};
use crate::api::Response;

#[derive(Debug)]
pub struct ReqSender {}

#[derive(Debug)]
pub struct RespSender<'a> {
    id: u64,
    sender: &'a Sender,
}

impl <'a> RespSender<'a> {
    pub fn new(sender: &'a Sender) -> Self {
        RespSender {
            id: 0,
            sender,
        }
    }

    pub fn send(&self, data: Response) {
        let envelope = ResponseEnvelope {
            id: self.id,
            data,
        };
        let resp_data = bincode::serialize(&envelope)
            .expect("could not encode Response");
        //TODO @mark: expect msg
        self.sender.send(resp_data).expect("TODO");
        unimplemented!() //TODO @mark:
    }

    pub fn send_err(&self, msg: impl Into<String>) {
        let msg = msg.into();
        warn!("sending error response: {}", &msg);
        self.send(Response::DaemonError(msg))
    }
}

pub fn server(addr: &str, handler: fn(Request, &RespSender) -> Result<Response, String>) {
    listen(addr, |out| {
        move |req_msg: Message| {
            let mut sender = RespSender::new(&out);
            match req_msg {
                Message::Text(_) => sender.send_err("got text message, but all messages should be binary"),
                Message::Binary(req_data) => {
                    match bincode::deserialize::<RequestEnvelope>(&req_data) {
                        Ok(request_envelope) => {
                            let RequestEnvelope { id, data } = request_envelope;
                            sender.id = id;
                            match handler(data, &sender) {
                                Ok(resp) => sender.send(resp),
                                Err(err_msg) => sender.send_err(err_msg),
                            }
                        }
                        Err(err_msg) => {
                            warn!("failed to deserialize request: {}", &err_msg);
                            sender.send_err("could not understand request");
                        },
                    }
                }
            }
            Ok(())
        }
    }).unwrap();
}

pub fn client() {
    unimplemented!()  //TODO @mark
}
