use ::std::process::exit;

use ::mango_cli_common::util::MangodStatus;

use crate::options::daemon::DaemonGetCmd;

pub fn get_daemon_property(args: &DaemonGetCmd, status: &MangodStatus) -> Result<(), ()> {
    match args {
        DaemonGetCmd::Status => get_status(status),
        DaemonGetCmd::Address => get_address(status),
    }
    exit(match status {
        MangodStatus::Inactive => 1,
        MangodStatus::Unresponsive { .. } => 2,
        MangodStatus::Ok { .. } => 0,
    })
}

pub fn get_status(status: &MangodStatus) {
    println!("{}", status.as_code());
}

pub fn get_address(status: &MangodStatus) {
    match status {
        MangodStatus::Inactive => {}
        MangodStatus::Unresponsive { address, .. } => println!("{}", address),
        MangodStatus::Ok { address, .. } => println!("{}", address),
    }
}
