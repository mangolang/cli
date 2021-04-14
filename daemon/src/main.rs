use ::std::process::exit;

use ::mango_cli_common::util::lockfile::load_lock;
use mango::::check_status::{determine_status, MangodStatus};
use mango::::get::get_property;
use mango::::handle_cmd;
use mango::::options::{MangodArgs, MangodCommand, MangodStartArgs};
use mango::::startstop::{start, stop};

mod connection;

#[paw::main]
fn main(args: MangodArgs) {
    let lockfile = load_lock();
    let status = match &lockfile {
        Some(info) => determine_status(info.pid()),
        None => MangodStatus::Inactive,
    };
    handle_cmd(&args, &status);
}
