use ::std::collections::HashMap;
use ::std::sync::Arc;
use ::std::sync::atomic::{AtomicBool, Ordering};
use ::std::sync::RwLock;
use ::std::thread;

use ::bincode;
use ::log::debug;
use ::log::error;
use ::log::info;
use ::log::trace;
use ::log::warn;
use ::serde_json;
use ::ws::CloseCode;
use ::ws::Handshake;
use ::ws::Message;
use ::ws::Sender;
use ::ws::util::Token;

use crate::api::{Request, RequestEnvelope, Response, ResponseEnvelope};
use crate::util::clear_lock;
use std::thread::{spawn, sleep};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RespSender<'a> {
    trace: u64,
    pub connection: &'a ConnectionData,
}

impl <'a> RespSender<'a> {
    pub fn new(trace: u64, connection: &'a ConnectionData) -> Self {
        RespSender {
            trace,
            connection,
        }
    }

    pub fn untraced(connection: &'a ConnectionData) -> Self {
        RespSender {
            trace: 0,
            connection,
        }
    }
}

#[derive(Debug)]
pub struct ConnectionData {
    use_json: AtomicBool,
    sender: Sender,
    control: Arc<ServerControl>,
}

impl ConnectionData {
    pub fn new(sender: Sender, control: Arc<ServerControl>) -> Arc<Self> {
        Arc::new(ConnectionData {
            use_json: AtomicBool::new(false),
            sender,
            control
        })
    }

    pub fn token(&self) -> Token {
        self.sender.token()
    }

    fn send_with_trace(&self, trace: u64, data: Response) {
        let envelope = ResponseEnvelope {
            trace,
            data,
        };
        trace!("sending {:?}", envelope);
        assert!(!self.use_json.load(Ordering::Acquire), "to implement: json");  //TODO @mark:
        let resp_data = bincode::serialize(&envelope)
            .expect("could not encode Response");
        self.sender.send(resp_data)
            .expect("failed to send websocket response");
    }

    pub fn send_untraced(&self, data: Response) {
        self.send_with_trace(0, data)
    }

    pub fn send_err_untraced(&self, msg: impl Into<String>) {
        let msg = msg.into();
        warn!("sending error response: {}", &msg);
        self.send_untraced(Response::DaemonError(msg))
    }

    pub fn broadcast(&self, response: Response) {
        self.control.clients.read().unwrap().values()
            .for_each(|client| client.send_untraced(response.clone()))
    }

    pub fn no_new_connections(&self) {
        self.control.is_accepting_connections.store(false, Ordering::Release);
    }

    pub fn shutdown(&self) {
        self.no_new_connections();
        let control_copy = self.control.clone();
        spawn(move || {
            //TODO get rid of spawn/sleep: https://github.com/housleyjk/ws-rs/issues/332
            sleep(Duration::from_millis(50));
            control_copy.handle.read().unwrap().as_ref()
                .expect("could not shut down server, the handle was not initialized at startup")
                .shutdown().unwrap();
        });
    }
}

impl <'a> RespSender<'a> {
    pub fn send(&self, data: Response) {
        self.connection.send_with_trace(self.trace, data);
    }

    pub fn send_err(&self, msg: impl Into<String>) {
        let msg = msg.into();
        warn!("sending error response: {}", &msg);
        self.send(Response::DaemonError(msg))
    }
}

#[derive(Debug)]
pub struct ServerControl {
    clients: RwLock<HashMap<Token, Arc<ConnectionData>>>,
    handle: RwLock<Option<Sender>>,
    is_accepting_connections: AtomicBool,
}

impl ServerControl {
    pub fn new() -> Arc<Self> {
        Arc::new(ServerControl {
            clients: RwLock::new(HashMap::new()),
            handle: RwLock::new(None),
            is_accepting_connections: AtomicBool::new(true),
        })
    }
}

struct ServerHandler<H: Fn(Request, &RespSender) -> Result<Response, String>> {
    connection: Arc<ConnectionData>,
    handler: H,
}

impl <H: Fn(Request, &RespSender) -> Result<Response, String>> ws::Handler for ServerHandler<H> {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        //TODO @mark: too long path
        let connection = &self.connection;
        if connection.control.is_accepting_connections.load(Ordering::Acquire) {
            connection.control.clients.write().unwrap()
                .insert(connection.token(), connection.clone());
        } else {
            debug!("rejecting connection because mangod is shutting down");
            connection.send_err_untraced("the mango daemon is currently shutting down, no new connections accepted");
        }
        Ok(())
    }

    fn on_message(&mut self, req_msg: Message) -> ws::Result<()> {
        let request_envelope = match req_msg {
            Message::Text(req_data) => {
                //TODO @mark: test this path
                self.connection.use_json.store(true, Ordering::Release);
                serde_json::from_str::<RequestEnvelope>(&req_data)
                    .map_err(|err| format!("{}", err))
            },
            Message::Binary(req_data) => {
                self.connection.use_json.store(false, Ordering::Release);
                bincode::deserialize::<RequestEnvelope>(&req_data)
                    .map_err(|err| format!("{}", err))
            },
        };
        match request_envelope {
            Ok(request_envelope) => {
                let RequestEnvelope { trace: id, data } = request_envelope;
                let sender = RespSender::new(id, &self.connection);
                match (self.handler)(data, &sender) {
                    Ok(resp) => sender.send(resp),
                    Err(err_msg) => sender.send_err(err_msg),
                }
            }
            Err(err_msg) => {
                warn!("failed to deserialize binary request: {}", &err_msg);
                self.connection.send_err_untraced("could not understand binary request");
            },
        }
        Ok(())
    }

    //TODO @mark: is on_close always called? also when timeout/dropped/crashed?
    fn on_close(&mut self, _: CloseCode, _: &str) {
        self.connection.control.clients.write().unwrap()
            .remove(&self.connection.token());
    }
}

//TODO @mark: check all Arc and RwLock to make sure it's not excessive
pub fn server(addr: &str, handler: impl Fn(Request, &RespSender) -> Result<Response, String> + Clone + Send + 'static) {
    info!("starting server at {}", addr);
    let control = ServerControl::new();
    let control_ref = control.clone();
    let socket = ws::Builder::new()
        .build(move |sender| ServerHandler {
            connection: ConnectionData::new(sender, control.clone()),
            handler: handler.clone(),
        })
        .expect("failed to build websocket server");
    *control_ref.handle.write().unwrap() = Some(socket.broadcaster());
    let addr_copy = addr.to_owned();
    let thrd = thread::spawn(move || {
        if let Err(err) = socket.listen(&addr_copy) {
            error!("could not start daemon at {}, reason: {}", &addr_copy, err)
        }
    });
    thrd.join().expect("something went wrong with the server thread");
    clear_lock();
}
