use ::mango_cli_common::util::lockfile::load_lock;

use crate::status::check_status::{determine_status, MangodStatus};
use crate::status::get::get_property;
use crate::status::options::{Command, MangodArgs, MangodStartArgs};
use crate::status::startstop::{start, stop};

mod status;

#[paw::main]
fn main(args: MangodArgs) {
    let lockfile = load_lock();
    let status = match &lockfile {
        Some(info) => determine_status(info.pid()),
        None => MangodStatus::Inactive,
    };
    match args.cmd {
        None => {
            eprintln!("no subcommand, assuming 'mangod start'");
            start(&MangodStartArgs::default(), &lockfile, &status)
        },
        Some(Command::Start(start_args)) => start(&start_args, &lockfile, &status),
        Some(Command::Stop(_)) => stop(&lockfile, &status),
        Some(Command::Get(get_args)) => get_property(&get_args, &lockfile, &status),
    }
}
