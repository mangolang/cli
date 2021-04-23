use ::log::warn;
use ::log::error;
use ::ws::connect;
use ::ws::listen;
use ::ws::Message;
use ::ws::Sender;

use crate::api::{Request, RequestEnvelope, ResponseEnvelope};
use crate::api::Response;
use ws::CloseCode;

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

    pub fn stop(&self) {
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

pub fn client(addr: &str, handler: fn(Response, &ReqSender) -> Result<Request, String>) {
    match connect(addr, |out| {
        move |req_msg: Message| {
            let mut sender = ReqSender::new(&out);
            match req_msg {
                Message::Text(_) => error!("got text message, but all messages should be binary"),
                Message::Binary(resp_data) => {
                    match bincode::deserialize::<ResponseEnvelope>(&resp_data) {
                        Ok(response_envelope) => {
                            let ResponseEnvelope { id, data } = response_envelope;
                            sender.id = id;
                            match handler(data, &sender) {
                                Ok(resp) => sender.send(resp),
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
    }) {
        Ok(()) => {}
        Err(err) => eprintln!("could not connect to daemon, reason: {}", err),
    };

    //TODO @mark: TEMPORARY! REMOVE THIS!
    // if let Err(_) = connect(format!("ws://{}", address), |out| {
    //     //TODO @mark: change this to bincode with serde
    //     let sender = sender.clone();
    //     if let Err(_) = out.send("ping") {
    //         debug!("failed to send ping message to {}", address);
    //         sender.send(false).unwrap();
    //         out.close(CloseCode::Normal).unwrap();
    //     }
    //
    //     move |msg: Message| {
    //         let got_pong = msg.as_text().unwrap() == "pong";
    //         if !got_pong {
    //             debug!("got unexpected answer from {} in response to ping", address);
    //         }
    //         sender.send(got_pong).unwrap();
    //         out.close(CloseCode::Normal)
    //     }
    // }) {
    //     debug!("failed to not connect to {} for ping", address);
    //     return false
    // };
    //
    // // Check if we got a pong message back.
    // return match receiver.recv_timeout(timeout) {
    //     Ok(true) => true,
    //     Ok(false) => false,
    //     Err(RecvTimeoutError::Timeout) => {
    //         debug!("timed out while connecting to {}", address);
    //         false
    //     }
    //     Err(RecvTimeoutError::Disconnected) => {
    //         debug!("connection to {} was immediately broken", address);
    //         false
    //     }
    // }
}
