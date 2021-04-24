use mango_cli_common::util::{MangodArgs, LockInfo, store_lock};
use std::process;
use mango_cli_common::api::{Request, ControlRequest, Response, ControlResponse};

fn launch(args: &MangodArgs) {
    let addr = args.address();
    println!("starting mangod, listening on {}", &addr);
    let lock = LockInfo::new(process::id(), &addr);
    store_lock(&lock);
    server(&addr, |request, _sender| {
        match request {
            Request::Control(req) => match req {
                ControlRequest::Ping => Ok(Response::Control(ControlResponse::Pong)),
                ControlRequest::Stop(_) => unimplemented!("shutdown"),
            }
        }
    }).expect("failed to start server");
    eprintln!("bye from mangod");  //TODO @mark: TEMPORARY! REMOVE THIS!
}
