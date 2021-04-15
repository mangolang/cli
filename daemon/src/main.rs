use ::std::process::exit;

use ::mango_cli_common::util::{MangodArgs, MangodStatus};

use ::env_logger;

mod connection;

#[paw::main]
fn main(args: MangodArgs) {
    env_logger::init();
    let addr = format!("{}:{}", &args.host, args.port);
    if !args.ignore_running {
        abort_if_running(&addr);
    }
    // host
    // port
    // worker_count
    // cache_mem_mb

    // let lockfile = load_lock();
    // let status = match &lockfile {
    //     Some(info) => determine_status(info.pid()),
    //     None => MangodStatus::Inactive,
    // };
    // handle_cmd(&args, &status);
}

fn abort_if_running(new_addr: &str) {
    let status = MangodStatus::determine();
    let status = MangodStatus::determine();
    dbg!(&status);
    match status {
        MangodStatus::Inactive => {},
        MangodStatus::Unresponsive { address: old_addr } => {
            if old_addr == new_addr {
                eprintln!("mangod is already running at {} but is not responding", &old_addr);
                exit(1);
            } else {
                eprintln!("mangod is already running, but with address '{}' instead of '{}'; stop it and restart with the new address", &old_addr, &new_addr);
                exit(1);
            }
        },
        MangodStatus::Ok { address: old_addr, .. } => {
            if old_addr == new_addr {
                eprintln!("mangod is already running at {}", &old_addr);
                exit(0);
            } else {
                eprintln!("mangod is already running, but with address '{}' instead of '{}'; stop it and restart with the new address", &old_addr, &new_addr);
                exit(1);
            }
        },
    }
}
