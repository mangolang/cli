use ::std::process;

use ::ws::listen;

use mango_cli_common::util::{LockInfo, MangodArgs, MangodStatus};

pub fn start_daemon(args: &MangodArgs, status: &MangodStatus) {
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

pub fn stop_daemon(status: &MangodStatus) {
    let addr = match status {
        MangodStatus::Inactive => {
            eprintln!("mangod is not running");
            return
        },
        MangodStatus::Unresponsive { address: addr } => addr,
        MangodStatus::Ok { address: addr } => addr,
    };
    unimplemented!()
    //TODO @mark: send stop message
    //TODO @mark: if that doesn't work, kill
    //TODO @mark: if successful, remove lock file
}
