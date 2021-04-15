use mango_cli_common::util::{MangodArgs, load_lock, MangodStatus};

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
        MangodStatus::NotFound { pid: pid } => {},
        MangodStatus::Unresponsive { pid: pid, address: old_address } => {},
        MangodStatus::Ok { pid: pid, address: old_address } => {},
    }
}