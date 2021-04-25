use ::std::collections::HashMap;
use ::std::sync::Arc;
use ::std::sync::atomic::{AtomicBool, Ordering};
use ::std::sync::atomic::AtomicU64;
use ::std::sync::RwLock;
use ::std::thread;

use ::bincode;
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

#[derive(Debug, Clone)]
pub struct RespSender {
    id: u64,
    content: Arc<RespSenderContent>,
}

#[derive(Debug)]
pub struct RespSenderContent {
    is_active: AtomicBool,
    use_json: AtomicBool,
    sender: Sender,
    control: Arc<ServerControl>,
}

impl RespSender {
    pub fn new(sender: Sender, control: Arc<ServerControl>) -> Self {
        RespSender {
            content: Arc::new(RespSenderContent {
                is_active: AtomicBool::new(true),
                id: AtomicU64::new(0),
                use_json: AtomicBool::new(false),
                sender,
                control
            }),
        }
    }

    pub fn token(&self) -> Token {
        self.content.sender.token()
    }

    pub fn send(&self, data: Response) {
        let envelope = ResponseEnvelope {
            id: self.content.id.load(Ordering::Acquire),
            data,
        };
        trace!("sending {:?}", envelope);
        assert!(!self.content.use_json.load(Ordering::Acquire), "to implement: json");  //TODO @mark:
        let resp_data = bincode::serialize(&envelope)
            .expect("could not encode Response");
        self.content.sender.send(resp_data)
            .expect("failed to send websocket response");
    }

    pub fn send_err(&self, msg: impl Into<String>) {
        let msg = msg.into();
        warn!("sending error response: {}", &msg);
        self.send(Response::DaemonError(msg))
    }
}

#[derive(Debug)]
pub struct ServerControl {
    clients: RwLock<HashMap<Token, Arc<RespSender>>>,
    handle: RwLock<Option<Sender>>,
}

struct ServerHandler<H: Fn(Request, &RespSender) -> Result<Response, String>> {
    sender: RespSender,
    handler: H,
}

impl <H: Fn(Request, &RespSender) -> Result<Response, String>> ws::Handler for ServerHandler<H> {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        //TODO @mark: too long path
        self.sender.content.control.clients.write().unwrap()
            .insert(self.sender.token(), Arc::new(self.sender.clone()));
        Ok(())
    }

    fn on_message(&mut self, req_msg: Message) -> ws::Result<()> {
        let request_envelope = match req_msg {
            Message::Text(req_data) => {
                //TODO @mark: test this path
                self.sender.content.use_json.store(true, Ordering::Release);
                serde_json::from_str::<RequestEnvelope>(&req_data)
                    .map_err(|err| format!("{}", err))
            },
            Message::Binary(req_data) => {
                self.sender.content.use_json.store(false, Ordering::Release);
                bincode::deserialize::<RequestEnvelope>(&req_data)
                    .map_err(|err| format!("{}", err))
            },
        };
        match request_envelope {
            Ok(request_envelope) => {
                let RequestEnvelope { id, data } = request_envelope;
                self.sender.content.id.store(id, Ordering::Release);
                match (self.handler)(data, &self.sender) {
                    Ok(resp) => self.sender.send(resp),
                    Err(err_msg) => self.sender.send_err(err_msg),
                }
            }
            Err(err_msg) => {
                warn!("failed to deserialize binary request: {}", &err_msg);
                self.sender.send_err("could not understand binary request");
            },
        }
        Ok(())

        // let mut sender = RespSender::new(&self.sender);
        // match req_msg {
        //     Message::Text(_) => error!("got text message, but all messages should be binary"),
        //     Message::Binary(resp_data) => {
        //         match bincode::deserialize::<ResponseEnvelope>(&resp_data) {
        //             Ok(response_envelope) => {
        //                 trace!("received: {:?}", response_envelope);
        //                 let ResponseEnvelope { id, data } = response_envelope;
        //                 sender.id = id;
        //                 match (self.handler)(&self.scope, data, &sender) {
        //                     Ok(()) => {},
        //                     Err(err_msg) => error!("error occurred: {}", err_msg),
        //                 }
        //             }
        //             Err(err_msg) => {
        //                 error!("failed to deserialize response: {}", &err_msg);
        //             },
        //         }
        //     }
        // }
        // Ok(())
    }

    //TODO @mark: is on_close always called? also when timeout/dropped/crashed?
    fn on_close(&mut self, _: CloseCode, _: &str) {
        self.sender.content.control.clients.write().unwrap()
            .remove(&self.sender.token());
    }
}

//TODO @mark: check all Arc and RwLock to make sure it's not excessive
pub fn server(addr: &str, handler: impl Fn(Request, &RespSender) -> Result<Response, String> + Clone + Send + 'static) {
    info!("starting server at {}", addr);
    let control = Arc::new(ServerControl {
        clients: RwLock::new(HashMap::new()),
        handle: RwLock::new(None),
    });
    let control_ref = control.clone();
    let socket = ws::Builder::new()
        .build(move |sender| ServerHandler {
            sender: RespSender::new(sender, control.clone()),
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


    // |out| {
    //     move |req_msg: Message| {
    //         let mut sender = RespSender::new(&out);
    //         match req_msg {
    //             Message::Text(_) => sender.send_err("got text message, but all messages should be binary"),
    //             Message::Binary(req_data) => {
    //                 match bincode::deserialize::<RequestEnvelope>(&req_data) {
    //                     Ok(request_envelope) => {
    //                         let RequestEnvelope { id, data } = request_envelope;
    //                         sender.id = id;
    //                         match handler(data, &sender) {
    //                             Ok(resp) => sender.send(resp),
    //                             Err(err_msg) => sender.send_err(err_msg),
    //                         }
    //                     }
    //                     Err(err_msg) => {
    //                         warn!("failed to deserialize request: {}", &err_msg);
    //                         sender.send_err("could not understand request");
    //                     },
    //                 }
    //             }
    //         }
    //         Ok(())
    //     }
    // }).map_err(|err| {
    //     error!("could not start daemon at {}, reason: {}", addr, err);
    //     ()
    // })
}
