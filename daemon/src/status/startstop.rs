use ::std::process;
use ::std::process::exit;

use ::ws::listen;

use ::mango_cli_common::util::lockfile::load_lock;
use ::mango_cli_common::util::lockfile::LockInfo;
use crate::status::options::{MangodStartArgs, MangodGetArgs, GetCommand};


#[derive(Debug)]
pub enum MangodStatus {
    /// There is no lockfile to suggest mangod is running.
    Inactive,
    /// There is a lockfile, but the pid does not belong to a running process.
    NotFound,
    /// The mangod process is running, but it is not responding to requests quickly.
    Unresponsive,
    /// The mangod process is running and responding to requests.
    Ok,
}

impl MangodStatus {
    pub fn is_ok(&self) -> bool {
        matches!(self, MangodStatus::Ok)
    }

    pub fn as_str(&self) -> &str {
        match self {
            MangodStatus::Inactive => "not-started",
            MangodStatus::NotFound => "died-unexpectedly",
            MangodStatus::Unresponsive => "unresponsive",
            MangodStatus::Ok => "running",
        }
    }
}

pub fn determine_status(pid: u32) -> MangodStatus {
    unimplemented!()
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
            let status = determine_status();
            if status.is_ok() {
                if matches!(args.cmd, GetCommand::Status) {
                    println!("{}", status.as_str())
                }
                exit(2);
            }
            match args.cmd {
                GetCommand::Status => println!("{}", status.as_str()),
                GetCommand::Pid => println!("{}", info.pid()),
                GetCommand::Address => println!("{}", info.address()),
            }
            exit(0);
        }
    }
}

