use ::std::process;
use ::std::process::exit;

use ::env_logger;

use ::mango_cli_common::api::{RequestEnvelope, ResponseEnvelope};
use ::mango_cli_common::util::{MangodArgs, MangodStatus};
use ::mango_cli_common::util::{LockInfo, store_lock};

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

fn launch(args: &MangodArgs) {
    let addr = args.address();
    println!("starting mangod, listening on {}", &addr);
    let lock = LockInfo::new(process::id(), &addr);
    store_lock(&lock);
    // listen(&addr, |out| {
    //     move |req_data| {
    //         let req: Request = bincode::deserialize(&req_data)
    //             .expect("could not understand Request");  //TODO: better error handling
    //         let resp = Response;
    //         let resp_data = bincode::serialize(&world)
    //             .expect("could not encode Response");
    //         out.send(resp_data)
    //     }
    // }).unwrap();
    unimplemented!();  //TODO @mark: use common wrapper
    eprintln!("bye from mangod");  //TODO @mark: TEMPORARY! REMOVE THIS!
}
