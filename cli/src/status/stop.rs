use ::std::process::{Command, Stdio};
use ::std::thread::sleep;
use ::std::time::{Duration, SystemTime};

use ::log::debug;

use ::mango_cli_common::api::{ControlRequest, StopMode, Upstream};
use ::mango_cli_common::api::{ControlResponse, Downstream};
use ::mango_cli_common::util::{clear_lock, single_msg_client};
use ::mango_cli_common::util::{MangodArgs, MangodStatus};
use ::mango_cli_common::util::can_ping;

use crate::options::daemon::DaemonStopCmd;

pub fn stop_daemon(args: &DaemonStopCmd, status: &MangodStatus) -> Result<(), ()> {
    let mode = match (args.quick, args.when_idle) {
        (true, true) => panic!("conflicting arguments"),
        (true, false) => StopMode::Quick,
        (false, true) => StopMode::WhenIdle,
        (false, false) => StopMode::FinishCurrentWork,
    };
    match status {
        MangodStatus::Inactive => {
            eprintln!("mangod is not running");
            Ok(())
        }
        MangodStatus::Unresponsive { .. } => {
            if args.clear {
                eprintln!("could not get response from mango daemon; lockfile will be cleared");
                clear_lock();
                Ok(())
            } else {
                eprintln!("cannot stop mango daemon because it is not responding; if it is still running, stop it manually");
                Err(())
            }
        }
        MangodStatus::Ok { address } => {
            if single_msg_client(
                address,
                Upstream::Control(ControlRequest::Stop(mode)),
                Some(|resp| matches!(resp, Downstream::Control(ControlResponse::Stopped))),
                Duration::from_secs(30),
            ) {
                println!("shutdown complete");
                Ok(())
            } else {
                println!("shutdown failed");
                Err(())
            }
        }
    }
}
