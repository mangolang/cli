use ::std::process;
use ::std::process::exit;

use ::ws::listen;

use ::mango_cli_common::util::lockfile::load_lock;
use ::mango_cli_common::util::lockfile::LockInfo;

use crate::status::options::{MangodGetCommand, MangodGetArgs, MangodStartArgs};
use crate::status::check_status::MangodStatus;

pub fn get_property(args: &MangodGetArgs, lock_info: &Option<LockInfo>, status: &MangodStatus) {
    match lock_info {
        None => {
            debug_assert!(matches!(status, MangodStatus::Inactive));
            eprintln!("mangod is not running");
            exit(1);
        }
        Some(info) => {
            if status.is_ok() {
                if matches!(args.cmd, MangodGetCommand::Status) {
                    println!("{}", status.as_code())
                }
                exit(2);
            }
            match args.cmd {
                MangodGetCommand::Status => println!("{}", status.as_code()),
                MangodGetCommand::Pid => println!("{}", info.pid()),
                MangodGetCommand::Address => println!("{}", info.address()),
            }
            exit(0);
        }
    }
}

