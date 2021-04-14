use ::std::process::exit;

use ::mango_cli_common::util::lockfile::load_lock;

use crate::status::check_status::{determine_status, MangodStatus};
use crate::status::get::get_property;
use crate::status::handle_cmd;
use crate::status::options::{MangodArgs, MangodCommand, MangodStartArgs};
use crate::status::startstop::{start, stop};

mod status;
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
