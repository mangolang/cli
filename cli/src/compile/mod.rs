use ::log::debug;
use ::log::error;
use ::log::trace;

use ::mango_cli_common::api::{TaskRequest, Upstream};
use ::mango_cli_common::api::CompileTarget;
use ::mango_cli_common::api::ControlResponse;
use ::mango_cli_common::api::Downstream;
use ::mango_cli_common::api::StopMode;
use ::mango_cli_common::util::client;
use ::mango_cli_common::util::MangodArgs;
use ::mango_cli_common::util::MangodStatus;

use crate::options::compile::CompileCmd;
use crate::status::running::ensure_running;

pub fn handle_compile_cmd(_args: &CompileCmd, status: &MangodStatus) -> Result<(), String> {
    ensure_running(status)?;
    client(&MangodArgs::default().address(), (),
           |_, sender| {
               debug!("sending compile task to daemon");
               //TODO @mark: hard-coded 'IR'
               sender.send(Upstream::Task(TaskRequest::Compile(CompileTarget::IR)))
           },
           |_, message, sender| {
               trace!("received from server: {:?}", message);
               match message {
                   Downstream::Ok => {}
                   Downstream::DaemonError(err_msg) => {
                       error!("daemon error: {}", err_msg);
                   }
                   Downstream::Control(response) => match response {
                       // These responses should arrive through `single_msg_client`
                       ControlResponse::Pong => panic!("unexpected pong"),
                       ControlResponse::Stopping(stop_mode) => match stop_mode {
                           StopMode::Quick => eprintln!("mango daemon is abandoning work and shutting down"),
                           StopMode::FinishCurrentWork => eprintln!("mango daemon is no longer accepting work and will shut down when ready"),
                           StopMode::WhenIdle => eprintln!("mango daemon will shut down when everything is ready"),
                       },
                       ControlResponse::Stopped => {
                           eprintln!("mango daemon stopped, probably on request");
                           sender.close()
                       }
                   }
                   Downstream::Task(response) => unimplemented!(),
                   Downstream::Source(request) => handle_source_request(request),
               }
               Ok(())
           })
}
