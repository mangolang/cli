use ::std::process::exit;

use ::ws::listen;

use ::mango_cli_common::util::lockfile::load_lock;
use ::mango_cli_common::util::lockfile::LockInfo;

use crate::options::{Command, MangodArgs, MangodStartArgs, MangodGetArgs, GetCommand};
use std::process;

mod options;

#[paw::main]
fn main(args: MangodArgs) {
    let lockfile = load_lock();
    match args.cmd {
        None => {
            eprintln!("no subcommand, assuming 'mangod start'");
            start(&MangodStartArgs::default(), &lockfile)
        },
        Some(Command::Start(start_args)) => start(&start_args, &lockfile),
        Some(Command::Stop(_)) => stop(&lockfile),
        Some(Command::Get(get_args)) => get_pid(&get_args, &lockfile),
    }
}

fn start(args: &MangodStartArgs, lock_info: &Option<LockInfo>) {
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

fn stop(lock_info: &Option<LockInfo>) {
    unimplemented!()
}

fn get_pid(args: &MangodGetArgs, lock_info: &Option<LockInfo>) {
    match lock_info {
        None => {
            eprintln!("mangod is not running");
            exit(1);
        }
        Some(info) => {
            match args.cmd {
                GetCommand::Pid => println!("{}", info.pid()),
                GetCommand::Address => println!("{}", info.address()),
            }
            exit(0);
        }
    }
    unimplemented!()
}

