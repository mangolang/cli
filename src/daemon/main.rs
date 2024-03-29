use ::std::process::exit;

use ::env_logger;

use crate::common::util::{MangodArgs, MangodStatus};

use crate::connection::server::launch;

//TODO @mark: get rid of this file

mod connection;

//TODO @mark: stop gracefully and remove lockfile on shutdown hook

#[paw::main]
fn main(args: MangodArgs) {
    env_logger::init();
    let addr = &args.address();
    if !args.ignore_running {
        abort_if_running(&addr);
    }
    launch(&args);
}

fn abort_if_running(new_addr: &str) {
    let status = MangodStatus::determine();
    match status {
        MangodStatus::Inactive => {}
        MangodStatus::Unresponsive { address: old_addr } => exit({
            if old_addr == new_addr {
                eprintln!("mangod is already running at {} but is not responding", &old_addr);
                1
            } else {
                eprintln!(
                    "mangod is already running, but with address '{}' instead of '{}'; stop it and restart with the new address",
                    &old_addr, &new_addr
                );
                1
            }
        }),
        MangodStatus::Ok { address: old_addr, .. } => exit({
            if old_addr == new_addr {
                eprintln!("mangod is already running at {}", &old_addr);
                0
            } else {
                eprintln!(
                    "mangod is already running, but with address '{}' instead of '{}'; stop it and restart with the new address",
                    &old_addr, &new_addr
                );
                1
            }
        }),
    }
}
