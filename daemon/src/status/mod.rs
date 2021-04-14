use crate::status::options::{MangodArgs, MangodCommand, MangodStartArgs};
use crate::status::startstop::{start, stop};
use crate::status::get::get_property;
use crate::status::check_status::MangodStatus;
use mango_cli_common::util::lockfile::LockInfo;

pub mod options;
pub mod startstop;
pub mod get;
pub mod check_status;

pub fn handle_cmd(args: &MangodArgs, lock_info: &Option<LockInfo>, status: &MangodStatus) {
    match &args.cmd {
        None => {
            eprintln!("no subcommand, assuming 'mangod start'");
            start(&MangodStartArgs::default(), &status)
        },
        Some(MangodCommand::Start(start_args)) => start(&start_args, &status),
        Some(MangodCommand::Stop(_)) => stop(lock_info, &status),
        Some(MangodCommand::Get(get_args)) => get_property(&get_args, lock_info, &status),
    }
}