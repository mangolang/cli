use ::mango_cli_common::util::lockfile::LockInfo;
use mango_cli_common::util::check_status::MangodStatus;
use mango_cli_common::util::mangod_options::{MangodArgs, MangodArgs, MangodCommand};

use crate::status::get::get_daemon_property;
use crate::status::startstop::{start_daemon, stop_daemon};

pub mod startstop;
pub mod get;

pub fn handle_cmd(args: &MangodArgs, status: &MangodStatus) {
    match &args.cmd {
        None => {
            eprintln!("no subcommand, assuming 'mangod start'");
            start_daemon(&MangodArgs::default(), &status)
        },
        Some(MangodCommand::Start(start_args)) => start_daemon(&start_args, &status),
        Some(MangodCommand::Stop(_)) => stop_daemon(status),
        Some(MangodCommand::Get(get_args)) => get_daemon_property(&get_args, &status),
    }
}
