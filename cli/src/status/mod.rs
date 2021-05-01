use ::std::process::exit;

use ::mango_cli_common::util::MangodStatus;

use crate::options::daemon::DaemonCmd;
use crate::status::get::get_daemon_property;
use crate::status::stop::stop_daemon;
use crate::status::start::start_daemon;

pub mod get;
pub mod start;
pub mod stop;
pub mod running;

//TODO @mark: TEMPORARY! REMOVE THIS!
pub fn handle_daemon_cmd(args: &DaemonCmd, status: &MangodStatus) {
    match match args {
        DaemonCmd::Start(start_args) => start_daemon(start_args),
        DaemonCmd::Stop(stop_args) => stop_daemon(stop_args, status),
        DaemonCmd::Get(get_args) => get_daemon_property(get_args, &status),
    } {
        Ok(()) => exit(0),
        Err(()) => exit(1),
    }
}
