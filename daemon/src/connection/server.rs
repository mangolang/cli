use ::std::process;

use ::mango_cli_common::util::{LockInfo, MangodArgs, store_lock};
use ::mango_cli_common::util::server;
use crate::connection::control::handle_control;
use mango_cli_common::api::Request;

pub fn launch(args: &MangodArgs) {
    let addr = args.address();
    println!("starting mangod, listening on {}", &addr);
    let lock = LockInfo::new(process::id(), &addr);
    store_lock(&lock);
    server(&addr, |request, sender| {
        match request {
            Request::Control(request) => handle_control(&request, sender)
        }
    });
    eprintln!("bye from mangod");  //TODO @mark: TEMPORARY! REMOVE THIS!
}
