use ::std::process::exit;

use ::mango_cli_common::util::{load_lock, MangodArgs, MangodStatus};

mod connection;

#[paw::main]
fn main(args: MangodArgs) {
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
    let lockinfo = load_lock();
    let status = match &lockinfo {
        Some(info) => MangodStatus::determine(info.pid(), info.address()),
        None => MangodStatus::Inactive,
    };
    match status {
        MangodStatus::Inactive => {},
        MangodStatus::Unresponsive { pid, address: old_addr } => {
            if old_addr == new_addr {
                eprintln!("mangod is already running at {} but is not responding (pid: {})", &old_addr, pid);
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
