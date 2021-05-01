use ::mango_cli_common::util::MangodArgs;
use ::mango_cli_common::util::MangodStatus;

use crate::options::compile::CompileCmd;
use crate::status::start::start_daemon;
use crate::status::running::ensure_running;

pub fn handle_compile_cmd(args: &CompileCmd, status: &MangodStatus) {
    ensure_running(status);
    eprintln!("Compile is not supported yet");
    //TODO @mark: TEMPORARY! REMOVE THIS!
}
