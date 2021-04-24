use ::log::error;
use ::log::info;
use ::log::trace;
use ::log::warn;
use ::ws::listen;
use ::ws::Message;
use ::ws::Sender;

use crate::api::{Request, RequestEnvelope, ResponseEnvelope};
use crate::api::Response;
use ws::CloseCode;

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
        trace!("sending {:?}", envelope);
        let resp_data = bincode::serialize(&envelope)
            .expect("could not encode Response");
        self.sender.send(resp_data)
            .expect("failed to send websocket response");
    }

    pub fn send_err(&self, msg: impl Into<String>) {
        let msg = msg.into();
        warn!("sending error response: {}", &msg);
        self.send(Response::DaemonError(msg))
    }

    pub fn broadcast(&self, data: Response) {
        let envelope = ResponseEnvelope {
            id: 0,
            data,
        };
        trace!("broadcasting {:?}", envelope);
        let resp_data = bincode::serialize(&envelope)
            .expect("could not encode Response");
        self.sender.broadcast(resp_data)
            .expect("failed to send websocket response");
    }

    // This close is important because exiting without returning control to the event loop
    // will cause pending messages to be dropped (including when sleeping).
    pub fn close(&self) {
        self.sender.close(CloseCode::Away).unwrap();
    }
}

pub fn server(addr: &str, handler: fn(Request, &RespSender) -> Result<Response, String>) -> Result<(), ()> {
    info!("starting server at {}", addr);
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
    }).map_err(|err| {
        error!("could not start daemon at {}, reason: {}", addr, err);
        ()
    })
}
