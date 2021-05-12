use crate::common::util::MangodArgs;
use crate::daemon::connection::server::launch;

pub mod connection;

pub fn run_mango_daemon(args: MangodArgs) -> Result<(), String> {
    launch(&args);
    Ok(())
}
