use ::log::debug;
use ::log::error;
use ::ws::{CloseCode, Handshake};
use ::ws::connect;
use ::ws::Message;
use ::ws::Sender;

use crate::api::{Request, RequestEnvelope, ResponseEnvelope};
use crate::api::Response;

#[derive(Debug)]
pub struct ReqSender<'a> {
    id: u64,
    sender: &'a Sender,
}

impl <'a> ReqSender<'a> {
    pub fn new(sender: &'a Sender) -> Self {
        ReqSender {
            id: 0,
            sender,
        }
    }

    pub fn send(&self, data: Request) {
        let envelope = RequestEnvelope {
            id: self.id,
            data,
        };
        let req_data = bincode::serialize(&envelope)
            .expect("could not encode Request");
        self.sender.send(req_data)
            .expect("failed to send websocket request");
    }

    pub fn try_send(&self, data: Request) -> Result<(), ()> {
        let envelope = RequestEnvelope {
            id: self.id,
            data,
        };
        let req_data = match bincode::serialize(&envelope) {
            Ok(data) => data,
            Err(_) => return Err(()),
        };
        self.sender.send(req_data)
            .map_err(|_| ())
    }

    pub fn close(&self) {
        self.sender.close(CloseCode::Normal)
            .expect("failed to close daemon connection");
    }
}

struct ClientHandler<S: Fn(&ReqSender), H: Fn(Response, &ReqSender) -> Result<(), String>> {
    sender: Sender,
    on_start: S,
    handler: H,
}

impl <S: Fn(&ReqSender), H: Fn(Response, &ReqSender) -> Result<(), String>> ws::Handler for ClientHandler<S, H> {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        let sender = ReqSender::new(&self.sender);
        (self.on_start)(&sender);
        Ok(())
    }

    fn on_message(&mut self, req_msg: Message) -> ws::Result<()> {
        let mut sender = ReqSender::new(&self.sender);
        match req_msg {
            Message::Text(_) => error!("got text message, but all messages should be binary"),
            Message::Binary(resp_data) => {
                match bincode::deserialize::<ResponseEnvelope>(&resp_data) {
                    Ok(response_envelope) => {
                        let ResponseEnvelope { id, data } = response_envelope;
                        sender.id = id;
                        match (self.handler)(data, &sender) {
                            Ok(()) => {},
                            Err(err_msg) => error!("error occurred: {}", err_msg),
                        }
                    }
                    Err(err_msg) => {
                        error!("failed to deserialize response: {}", &err_msg);
                    },
                }
            }
        }
        Ok(())
    }
}

pub fn client(addr: &str, on_start: impl Fn(&ReqSender) + Copy, handler: impl Fn(Response, &ReqSender) -> Result<(), String> + Copy) -> Result<(), ()> {
    connect(format!("ws://{}", addr), move |sender| ClientHandler {
        sender,
        on_start,
        handler
    }).map_err(|err| {
        debug!("could not connect to daemon, reason: {}", err);
        ()
    })
}
