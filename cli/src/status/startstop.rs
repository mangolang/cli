use ::std::process::{Command, Stdio};
use ::std::process::exit;
use ::std::thread::sleep;
use ::std::time::{Duration, SystemTime};

use ::log::info;

use ::mango_cli_common::util::{MangodArgs, MangodStatus};
use ::mango_cli_common::util::can_ping;

#[cfg(debug_assertions)]
fn start_daemon_cmd(args: &[String]) -> Command {
    let mut cmd = Command::new("cargo");
    let mut all_args = vec!["run".to_owned(), "-q".to_owned(),
            "-p".to_owned(), "mango-cli-daemon".to_owned(), "--".to_owned()];
    all_args.extend_from_slice(args);
    info!("start daemon (debug) cmd: cargo {:?}", all_args.join(" "));
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

pub fn start_daemon(args: &MangodArgs) {
    match start_daemon_cmd(&args.as_vec())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn() {
        Ok(mut child) => {
            let start = SystemTime::now();
            let delay = Duration::from_millis(100);
            while SystemTime::now().duration_since(start).unwrap().as_millis() < 5000 {
                let has_exit_code = child.try_wait()
                    .map(|exit_code| exit_code.is_some())
                    .unwrap_or(false);
                if has_exit_code {
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
        MangodStatus::Unresponsive { .. } => {
            eprintln!("cannot stop mango daemon because it is not responding; if it is still running, stop it manually");
            return
        },
        MangodStatus::Ok { .. } => {
            unimplemented!("stop daemon")
        },
    };
    //TODO @mark: send stop message
    //TODO @mark: if successful, remove lock file
}
