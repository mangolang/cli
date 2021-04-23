use ::log::warn;
use ::log::error;
use ::ws::connect;
use ::ws::listen;
use ::ws::Message;
use ::ws::Sender;

use crate::api::{Request, RequestEnvelope, ResponseEnvelope};
use crate::api::Response;
use ws::{CloseCode, Handshake};

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

    pub fn close(&self) {
        self.sender.close(CloseCode::Normal)
            .expect("failed to close daemon connection");
    }
}

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
        self.sender.send(resp_data)
            .expect("failed to send websocket response");
    }

    pub fn send_err(&self, msg: impl Into<String>) {
        let msg = msg.into();
        warn!("sending error response: {}", &msg);
        self.send(Response::DaemonError(msg))
    }
}

pub fn server(addr: &str, handler: fn(Request, &RespSender) -> Result<Response, String>) {
    match listen(addr, |out| {
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
    }) {
        Ok(()) => {}
        Err(err) => eprintln!("could not start daemon, reason: {}", err),
    };
}

struct ClientHandler {
    sender: Sender,
    on_start: fn(&ReqSender),
    handler: fn(Response, &ReqSender) -> Result<(), String>,
}

impl ws::Handler for ClientHandler {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
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

pub fn client(addr: &str, on_start: fn(&ReqSender), handler: fn(Response, &ReqSender) -> Result<(), String>) {
    match connect(addr, |sender| ClientHandler {
        sender,
        on_start,
        handler
    }) {
        Ok(()) => {}
        Err(err) => eprintln!("could not connect to daemon, reason: {}", err),
    };
}
