use ::std::rc::Rc;
use ::std::sync::RwLock;
use ::std::thread;
use std::collections::HashMap;

use ::log::info;
use ::log::trace;
use ::log::warn;
use ::ws::CloseCode;
use ::ws::Handshake;
use ::ws::Message;
use ::ws::Sender;

use crate::api::{Request, Response, ResponseEnvelope};
use ws::util::Token;
use std::sync::Arc;
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub struct RespSender {
    content: Rc<RespSenderContent>,
}

#[derive(Debug)]
pub struct RespSenderContent {
    is_active: bool,
    id: u64,
    use_json: bool,
    sender: Sender,
}

impl RespSender {
    pub fn new(sender: Sender) -> Self {
        RespSender {
            content: Rc::new(RespSenderContent {
                is_active: true,
                id: 0,
                use_json: false,
                sender,
            }),
        }
    }

    pub fn token(&self) -> Token {
        self.content.sender.token()
    }

    pub fn send(&self, data: Response) {
        let envelope = ResponseEnvelope {
            id: self.content.id,
            data,
        };
        trace!("sending {:?}", envelope);
        assert!(!self.content.use_json, "to implement: json");  //TODO @mark:
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
struct ServerControl {
    clients: RwLock<HashMap<Token, Arc<RespSender>>>,
    handle: RwLock<Option<Sender>>,
}

struct ServerHandler<H: Fn(Request, &RespSender) -> Result<(), String>> {
    control: Arc<ServerControl>,
    sender: RespSender,
    handler: H,
}

impl <H: Fn(Request, &RespSender) -> Result<(), String>> ws::Handler for ServerHandler<H> {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        self.control.clients.write().unwrap()
            .insert(self.sender.token(), Arc::new(self.sender.clone()));
        Ok(())
    }

    fn on_message(&mut self, req_msg: Message) -> ws::Result<()> {
        unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!


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
        self.control.clients.write().unwrap()
            .remove(&self.sender.token());
    }
}

pub fn server(addr: &str, handler: impl Fn(Request, &RespSender) -> Result<(), String> + Clone) {
    info!("starting server at {}", addr);
    let control = Arc::new(ServerControl {
        clients: RwLock::new(HashMap::new()),
        handle: RwLock::new(None),
    });
    let socket = ws::Builder::new()
        .build(move |sender| ServerHandler {
            control: control.clone(),
            sender: RespSender::new(sender),
            handler: handler.clone(),
        })
        .expect("failed to build websocket server");
    // *control.handle.borrow_mut().get_mut().unwrap() = Some(socket.broadcaster());
    // let thrd = thread::spawn(move || {
    //     if let Err(err) = socket.listen(addr) {
    //         error!("could not start daemon at {}, reason: {}", addr, err)
    //     }
    // });




    // thrd.join().expect("something went wrong with the server thread");


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
