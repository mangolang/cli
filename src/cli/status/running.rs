use ::log::debug;

use ::mango_cli_common::util::MangodArgs;
use ::mango_cli_common::util::MangodStatus;

use crate::status::start::start_daemon;

pub fn ensure_running(status: &MangodStatus) -> Result<(), String> {
    match status {
        MangodStatus::Unresponsive { .. } => {
            debug!("mangod is not responding; giving up");
            eprintln!("mango daemon is not responding");
            Err("could not complete the task because the mango daemon is not responding".to_owned())
        }
        MangodStatus::Ok { .. } => {
            debug!("mangod is already running");
            Ok(())
        }
        MangodStatus::Inactive => {
            debug!("mangod is is not running; starting it");
            start_daemon(&MangodArgs::default())
                .map_err(|_| "could not complete the task because the mango daemon failed to start".to_owned())
        }
    }
}
