use ::std::process;
use ::std::process::exit;

use ::ws::listen;

use ::mango_cli_common::util::lockfile::load_lock;
use ::mango_cli_common::util::lockfile::LockInfo;

use mango::::options::{MangodGetCommand, MangodGetArgs, MangodStartArgs};
use mango::::check_status::MangodStatus;
use std::process::{Command, Output};
use std::io::Error;

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
    let (pid, addr) = match status {
        MangodStatus::Inactive => {
            eprintln!("mangod is not running");
            return
        },
        MangodStatus::NotFound { pid: pid } => {
            eprintln!("mangod process is not found (pid: {})", pid);
            return
        }
        MangodStatus::Unresponsive { pid: pid, address: addr } => (pid, addr),
        MangodStatus::Ok { pid: pid, address: addr } => (pid, addr),
    };
    unimplemented!()
    //TODO @mark: send stop message
    //TODO @mark: if that doesn't work, kill
    //TODO @mark: if successful, remove lock file
}
