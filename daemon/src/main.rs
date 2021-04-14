use ::std::process;
use ::std::process::exit;

use ::ws::listen;

use ::mango_cli_common::util::lockfile::load_lock;
use ::mango_cli_common::util::lockfile::LockInfo;

use crate::status::options::{Command, MangodArgs, MangodStartArgs};

use crate::status::startstop::start;

mod status;

#[paw::main]
fn main(args: MangodArgs) {
    let lockfile = load_lock();
    match args.cmd {
        None => {
            eprintln!("no subcommand, assuming 'mangod start'");
            start(&MangodStartArgs::default(), &lockfile)
        },
        Some(Command::Start(start_args)) => start(&start_args, &lockfile),
        Some(Command::Stop(_)) => stop(&lockfile),
        Some(Command::Get(get_args)) => get_pid(&get_args, &lockfile),
    }
}
