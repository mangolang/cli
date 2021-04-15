use ::mango_cli_common::util::MangodStatus;

use crate::options::daemon::DaemonCmd;
use crate::status::get::get_daemon_property;
use crate::status::startstop::{start_daemon, stop_daemon};

pub mod startstop;
pub mod get;

//TODO @mark: TEMPORARY! REMOVE THIS!
pub fn handle_daemon_cmd(args: &DaemonCmd, status: &MangodStatus) {
    match args {
        DaemonCmd::Start(start_args) => start_daemon(&start_args, &status),
        DaemonCmd::Stop(_) => stop_daemon(status),
        DaemonCmd::Get(get_args) => get_daemon_property(&get_args, &status),
    }
}
