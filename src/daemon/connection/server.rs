use ::std::process;

use crate::common::api::CompileTarget;
use crate::common::api::{TaskRequest, Upstream};
use crate::common::util::server;
use crate::common::util::{store_lock, LockInfo, MangodArgs};

use crate::daemon::connection::control::handle_control;

pub fn launch(args: &MangodArgs) {
    let addr = args.address();
    println!("starting mangod, listening on {}", &addr);
    let lock = LockInfo::new(process::id(), &addr);
    store_lock(&lock);
    server(&addr, |upstream, sender| match upstream {
        Upstream::Control(request) => handle_control(&request, sender),
        Upstream::Source(response) => {
            eprintln!("got source {:?}", response); //TODO @mark
            unimplemented!("make server not always expect responses")
            //TODO @mark:
        }
        Upstream::Task(request) => match request {
            TaskRequest::Compile(compile) => {
                assert!(matches!(compile, CompileTarget::IR));
                eprintln!("got compile task {:?}", compile); //TODO @mark
                unimplemented!("compile IR"); //TODO @mark: TEMPORARY! REMOVE THIS!
            }
        },
    });
    eprintln!("bye from mangod"); //TODO @mark: TEMPORARY! REMOVE THIS!
}
