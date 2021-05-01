use ::mango_cli_common::util::MangodStatus;

use crate::options::compile::CompileCmd;
use crate::status::running::ensure_running;

pub fn handle_compile_cmd(args: &CompileCmd, status: &MangodStatus) -> Result<(), String> {
    ensure_running(status)?;
    Err("Compile is not supported yet".to_owned()) //TODO @mark:
    //TODO @mark: TEMPORARY! REMOVE THIS!
}
