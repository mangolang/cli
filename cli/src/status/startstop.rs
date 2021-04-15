use ::std::process::Command;

use ::mango_cli_common::util::{MangodArgs, MangodStatus};
use std::io::Error;
use std::process::{Child, exit};

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

pub fn start_daemon(args: &MangodArgs, status: &MangodStatus) {
    match start_daemon_cmd(&args.as_vec())
            .spawn() {
        //TODO @mark:
        Ok(_) => {},
        Err(err) => {
            eprintln!("could not start mango daemon (mangod), reason: {}", err);
            exit(1);
        },
    }
}

pub fn stop_daemon(status: &MangodStatus) {
    let addr = match status {
        MangodStatus::Inactive => {
            eprintln!("mangod is not running");
            return
        },
        MangodStatus::Unresponsive { address: addr } => addr,
        MangodStatus::Ok { address: addr } => addr,
    };
    unimplemented!()
    //TODO @mark: send stop message
    //TODO @mark: if that doesn't work, kill
    //TODO @mark: if successful, remove lock file
}
