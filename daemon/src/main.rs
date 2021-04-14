use ::mango_cli_common::util::lockfile::load_lock;

use crate::status::check_status::{determine_status, MangodStatus};
use crate::status::get::get_property;
use crate::status::options::{MangodCommand, MangodArgs, MangodStartArgs};
use crate::status::startstop::{start, stop};
use crate::status::handle_cmd;

mod status;
mod connection;

#[paw::main]
fn main(args: MangodArgs) {
    let lockfile = load_lock();
    let status = match &lockfile {
        Some(info) => determine_status(info.pid()),
        None => MangodStatus::Inactive,
    };
    handle_cmd(&args, &lockfile, &status)
}
