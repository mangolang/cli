use ::std::process;

use ::mango_cli_common::api::Upstream;
use ::mango_cli_common::util::server;
use ::mango_cli_common::util::{store_lock, LockInfo, MangodArgs};

use crate::connection::control::handle_control;

pub fn launch(args: &MangodArgs) {
    let addr = args.address();
    println!("starting mangod, listening on {}", &addr);
    let lock = LockInfo::new(process::id(), &addr);
    store_lock(&lock);
    server(&addr, |upstream, sender| match upstream {
        Upstream::Control(request) => handle_control(&request, sender),
        Upstream::Source(response) => {
            eprintln!("got source {:?}", response);
            unimplemented!("make server not always expect responses")
            //TODO @mark:
        }
    });
    eprintln!("bye from mangod"); //TODO @mark: TEMPORARY! REMOVE THIS!
}
