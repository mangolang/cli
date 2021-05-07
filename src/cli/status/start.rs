use ::std::env;
use ::std::process::{Command, Stdio};
use ::std::thread::sleep;
use ::std::time::{Duration, SystemTime};

use ::log::debug;

use crate::common::util::{can_ping, MangodArgs};

fn start_daemon_cmd(args: &[String]) -> Command {
    let exe_path = env::args().next().expect("could not find executable name, no arguments");
    debug!("start daemon (debug) cmd: {} run-as-daemon {}", &exe_path, args.join(" "));
    let mut cmd = Command::new(exe_path);
    cmd.arg("run-as-daemon");
    cmd.args(args);
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
