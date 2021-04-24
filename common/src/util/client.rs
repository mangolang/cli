use ::log::debug;
use ::log::error;
use ::ws::{CloseCode, Handshake};
use ::ws::connect;
use ::ws::Message;
use ::ws::Sender;

use crate::api::{Request, RequestEnvelope, ResponseEnvelope};
use crate::api::Response;
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::time::Duration;

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

struct ClientHandler<T, S: Fn(&T, &ReqSender), H: Fn(&T, Response, &ReqSender) -> Result<(), String>> {
    sender: Sender,
    scope: T,
    on_start: S,
    handler: H,
}

impl <T, S: Fn(&T, &ReqSender), H: Fn(&T, Response, &ReqSender) -> Result<(), String>> ws::Handler for ClientHandler<T, S, H> {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        let sender = ReqSender::new(&self.sender);
        (self.on_start)(&self.scope, &sender);
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
                        match (self.handler)(&self.scope, data, &sender) {
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

pub fn client<T: Clone>(addr: &str, scope: T, on_start: impl Fn(&T, &ReqSender) + Copy, handler: impl Fn(&T, Response, &ReqSender) -> Result<(), String> + Copy) -> Result<(), ()> {
    connect(format!("ws://{}", addr), move |sender| ClientHandler {
        sender,
        scope: scope.clone(),
        on_start,
        handler
    }).map_err(|err| {
        debug!("could not connect to daemon, reason: {}", err);
        ()
    })
}

pub fn single_msg_client(address: &str, request: Request, await_response: Option<fn(&Response) -> bool>, timeout: Duration) -> bool {
    let (channel_sender, channel_receiver) = channel();

    // Send a message to the server.
    if let Err(_) = client(
        address,
        (channel_sender, request, await_response),
        move |scope, req_sender| {
            if let Err(()) = req_sender.try_send(scope.1.clone()) {
                debug!("failed to send single-message request to {}", address);
                scope.0.send(false).unwrap();
                req_sender.close();
            }
            if await_response.is_none() {
                req_sender.close();
            }
        },
        move |scope, actual_response, req_sender| {
            if let Some(is_expected_response) = scope.2 {
                if is_expected_response(&actual_response) {
                    debug!("received the expected response from {} to single-message request", address);
                    scope.0.send(true).unwrap();
                    req_sender.close();
                } else if let Response::DaemonError(err_msg) = actual_response {
                    error!("daemon error: {}", err_msg);
                    req_sender.close();
                }
            }
            Ok(())
    }) {
        debug!("failed to connect to {} to send single-message request", address);
        return false
    };

    // Check if we got the expected response back.
    return match channel_receiver.recv_timeout(timeout) {
        Ok(true) => true,
        Ok(false) => false,
        Err(RecvTimeoutError::Timeout) => {
            debug!("timed out while connecting to {}", address);
            false
        }
        Err(RecvTimeoutError::Disconnected) => {
            debug!("connection to {} was immediately broken", address);
            false
        }
    }
}