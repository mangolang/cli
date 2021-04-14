use ::std::process;
use ::std::process::exit;

use ::ws::listen;

use ::mango_cli_common::util::lockfile::load_lock;
use ::mango_cli_common::util::lockfile::LockInfo;

use crate::status::options::{GetCommand, MangodGetArgs, MangodStartArgs};
use crate::status::check_status::MangodStatus;

pub fn get_property(args: &MangodGetArgs, lock_info: &Option<LockInfo>, status: &MangodStatus) {
    match lock_info {
        None => {
            eprintln!("mangod is not running");
            exit(1);
        }
        Some(info) => {
            if status.is_ok() {
                if matches!(args.cmd, GetCommand::Status) {
                    println!("{}", status.as_code())
                }
                exit(2);
            }
            match args.cmd {
                GetCommand::Status => println!("{}", status.as_code()),
                GetCommand::Pid => println!("{}", info.pid()),
                GetCommand::Address => println!("{}", info.address()),
            }
            exit(0);
        }
    }
}

