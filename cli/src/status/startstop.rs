use ::std::process::{Command, Stdio};
use ::std::thread::sleep;
use ::std::time::{Duration, SystemTime};

use ::log::debug;

use ::mango_cli_common::api::{ControlRequest, Request, StopMode};
use ::mango_cli_common::api::{ControlResponse, Response};
use ::mango_cli_common::util::can_ping;
use ::mango_cli_common::util::{clear_lock, single_msg_client};
use ::mango_cli_common::util::{MangodArgs, MangodStatus};

use crate::options::daemon::DaemonStopCmd;

#[cfg(debug_assertions)]
fn start_daemon_cmd(args: &[String]) -> Command {
    let mut cmd = Command::new("cargo");
    let mut all_args = vec![
        "run".to_owned(),
        "-q".to_owned(),
        "-p".to_owned(),
        "mango-cli-daemon".to_owned(),
        "--".to_owned(),
    ];
    all_args.extend_from_slice(args);
    debug!("start daemon (debug) cmd: cargo {}", all_args.join(" "));
    cmd.args(&all_args);
    cmd
}

#[cfg(not(debug_assertions))]
fn start_daemon_cmd(args: &[String]) -> Command {
    let mut cmd = Command::new("mango");
    debug!("start daemon (release) cmd: mango {}", args.join(" "));
    cmd.args(&*args);
    cmd
}

pub fn start_daemon(args: &MangodArgs) -> Result<(), ()> {
    match start_daemon_cmd(&args.as_vec())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        //TODO @mark: fail if memory/cpu are different in lock: `load_lock()`
        Ok(mut child) => {
            let start = SystemTime::now();
            let delay = Duration::from_millis(50);
            while SystemTime::now().duration_since(start).unwrap().as_millis() < 5000 {
                let has_exit_code = child.try_wait().map(|exit_code| exit_code.is_some()).unwrap_or(false);
                if has_exit_code {
                    eprintln!("started mango daemon (mangod), but it terminated");
                    return Err(());
                }
                if can_ping(&args.address()) {
                    println!("started mango daemon (mangod)");
                    return Ok(());
                }
                sleep(delay);
            }
            eprintln!("started mango daemon (mangod), but could not connect to it");
            Err(())
        }
        Err(err) => {
            eprintln!("could not start mango daemon (mangod), reason: {}", err);
            Err(())
        }
    }
}

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
                Request::Control(ControlRequest::Stop(mode)),
                Some(|resp| matches!(resp, Response::Control(ControlResponse::Stopped))),
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
