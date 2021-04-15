use ::std::process;
use ::std::process::exit;

use ::mango_cli_common::util::lockfile::load_lock;
use ::mango_cli_common::util::lockfile::LockInfo;
use ::ws::listen;

use mango_cli_common::util::check_status::MangodStatus;
use mango_cli_common::util::mangod_options::{MangodGetArgs, MangodGetCommand};

pub fn get_daemon_property(args: &MangodGetArgs, status: &MangodStatus) {
    match args.cmd {
        MangodGetCommand::Status => get_status(status),
        MangodGetCommand::Pid => get_pid(status),
        MangodGetCommand::Address => get_address(status),
    }
    exit(match status {
        MangodStatus::Inactive => 1,
        MangodStatus::NotFound { .. } => 2,
        MangodStatus::Unresponsive { .. } => 2,
        MangodStatus::Ok { .. } => 0,
    })
}

pub fn get_status(status: &MangodStatus) {
    println!("{}", status.as_code());
}

pub fn get_pid(status: &MangodStatus) {
    match status {
        MangodStatus::Inactive => {},
        MangodStatus::NotFound { pid: pid, .. } => println!("{}", pid),
        MangodStatus::Unresponsive { pid: pid, .. } => println!("{}", pid),
        MangodStatus::Ok { pid: pid, .. } => println!("{}", pid),
    }
}

pub fn get_address(status: &MangodStatus) {
    match status {
        MangodStatus::Inactive => {},
        MangodStatus::NotFound { .. } => {},
        MangodStatus::Unresponsive { address: address, .. } => println!("{}", address),
        MangodStatus::Ok { address: address, .. } => println!("{}", address),
    }
}
