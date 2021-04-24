use ::std::process::{Command, Stdio};
use ::std::thread::sleep;
use ::std::time::{Duration, SystemTime};

use ::log::info;
use ::log::debug;

use ::mango_cli_common::util::{MangodArgs, MangodStatus};
use ::mango_cli_common::util::can_ping;
use mango_cli_common::api::{Request, ControlRequest, StopMode};

#[cfg(debug_assertions)]
fn start_daemon_cmd(args: &[String]) -> Command {
    let mut cmd = Command::new("cargo");
    let mut all_args = vec!["run".to_owned(), "-q".to_owned(),
            "-p".to_owned(), "mango-cli-daemon".to_owned(), "--".to_owned()];
    all_args.extend_from_slice(args);
    info!("start daemon (debug) cmd: cargo {}", all_args.join(" "));
    cmd.args(&all_args);
    cmd
}

#[cfg(not(debug_assertions))]
fn start_daemon_cmd(args: &[String]) -> Command {
    let mut cmd = Command::new("cargo run -p mango-cli-daemon");
    let mut all_args = vec![];
    all_args.extend_from_slice(args);
    cmd.args(&all_args);
    cmd
}

pub fn start_daemon(args: &MangodArgs) -> Result<(), ()> {
    match start_daemon_cmd(&args.as_vec())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn() {
        //TODO @mark: fail if memory/cpu are different in lock: `load_lock()`
        Ok(mut child) => {
            let start = SystemTime::now();
            let delay = Duration::from_millis(100);
            while SystemTime::now().duration_since(start).unwrap().as_millis() < 5000 {
                let has_exit_code = child.try_wait()
                    .map(|exit_code| exit_code.is_some())
                    .unwrap_or(false);
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
        },
        Err(err) => {
            eprintln!("could not start mango daemon (mangod), reason: {}", err);
            Err(())
        },
    }
}

pub fn stop_daemon(status: &MangodStatus) -> Result<(), ()> {
    match status {
        MangodStatus::Inactive => {
            eprintln!("mangod is not running");
            Ok(())
        },
        MangodStatus::Unresponsive { .. } => {
            eprintln!("cannot stop mango daemon because it is not responding; if it is still running, stop it manually");
            Err(())
        },
        MangodStatus::Ok { address } => {
            //TODO @mark: single_msg_client
            // Send ping message to the server.
            if let Err(_) = client(
                address,
                |req_sender| {
                    if let Err(()) = req_sender.try_send(Request::Control(ControlRequest::Stop(StopMode::FinishCurrentWork))) {
                        debug!("failed to send shutdown request");
                        req_sender.close();
                    }
                },
                |resp, req_sender| { Ok(()) }) {
                debug!("failed to not connect to {} for ping", address);
                return Err(())
            };
            unimplemented!("stop daemon")
        },
    }
    //TODO @mark: send stop message
    //TODO @mark: if successful, remove lock file
}
