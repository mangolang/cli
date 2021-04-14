use ::std::process;
use ::std::process::exit;

use ::ws::listen;

use ::mango_cli_common::util::lockfile::load_lock;
use ::mango_cli_common::util::lockfile::LockInfo;

use crate::status::options::{MangodGetCommand, MangodGetArgs, MangodStartArgs};
use crate::status::check_status::MangodStatus;

pub fn start(args: &MangodStartArgs, status: &MangodStatus) {
    assert!(!args.host.contains(":"));
    assert!(!args.host.contains(" "));
    let addr = format!("{}:{}", &args.host, &args.port);
    println!("starting mangod, listening on {}", &addr);
    LockInfo::new(process::id(), &addr);
    listen(&addr, |out| {
        move |msg| {
            out.send(msg)
        }
    }).unwrap()
}

pub fn stop(status: &MangodStatus) {
    unimplemented!()
}
