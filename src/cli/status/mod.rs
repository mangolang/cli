use ::std::process::exit;

use ::log::debug;
use ::log::trace;

use crate::cli::options::daemon::DaemonCmd;
use crate::cli::status::get::get_daemon_property;
use crate::cli::status::start::start_daemon;
use crate::cli::status::stop::stop_daemon;
use crate::common::util::MangodStatus;

pub mod get;
pub mod running;
pub mod start;
pub mod stop;

pub fn handle_daemon_cmd(args: &DaemonCmd, status: &MangodStatus) -> Result<(), String> {
    match match args {
        DaemonCmd::Start(start_args) => start_daemon(start_args),
        DaemonCmd::Stop(stop_args) => stop_daemon(stop_args, status),
        DaemonCmd::Get(get_args) => get_daemon_property(get_args, &status),
    } {
        Ok(()) => {
            trace!("status of daemon command was ok");
            exit(0)
        }
        Err(()) => {
            debug!("status of daemon command was err");
            exit(1)
        }
    }
}
