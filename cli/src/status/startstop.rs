use ::std::process::Command;
use ::std::process::exit;

use ::mango_cli_common::util::{MangodArgs, MangodStatus};
use std::time::{SystemTime, Duration};
use mango_cli_common::util::can_ping;
use std::thread::sleep;

#[cfg(debug_assertions)]
fn start_daemon_cmd(args: &[String]) -> Command {
    let mut all_args = vec!["run".to_owned(), "-p".to_owned(),
            "mango-cli-daemon".to_owned(), "--".to_owned()];
    all_args.extend_from_slice(args);
    let mut cmd = Command::new("cargo");
    cmd.args(&all_args);
    cmd
}

#[cfg(not(debug_assertions))]
fn start_daemon_base_cmd(args: &MangodArgs) -> Command {
    let mut cmd = Command::new("cargo run -p mango-cli-daemon");
    cmd.args(&args);
    cmd
}

pub fn start_daemon(args: &MangodArgs) {
    match start_daemon_cmd(&args.as_vec())
            .spawn() {
        Ok(mut child) => {
            let start = SystemTime::now();
            let delay = Duration::from_millis(100);
            while SystemTime::now().duration_since(start).unwrap().as_millis() < 5000 {
                if child.try_wait().is_ok() {
                    eprintln!("started mango daemon (mangod), but it terminated");
                    return;
                }
                if can_ping(&args.address()) {
                    println!("started mango daemon (mangod)");
                    return;
                }
                sleep(delay);
            }
            eprintln!("started mango daemon (mangod), but could not connect to it");
        },
        Err(err) => {
            eprintln!("could not start mango daemon (mangod), reason: {}", err);
            exit(1);  //TODO @mark: use return instead of exit
        },
    }
}

pub fn stop_daemon(status: &MangodStatus) {
    match status {
        MangodStatus::Inactive => {
            eprintln!("mangod is not running");
            return
        },
        MangodStatus::Unresponsive { address: addr } => {
            eprintln!("cannot stop mango daemon because it is not responding; if it is still running, stop it manually");
            return
        },
        MangodStatus::Ok { address: addr } => {
            unimplemented!("stop daemon")
        },
    };
    //TODO @mark: send stop message
    //TODO @mark: if successful, remove lock file
}
