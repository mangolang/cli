use ::log::debug;
use ::log::error;
use ::log::trace;

use crate::common::api::CompileTarget;
use crate::common::api::ControlResponse;
use crate::common::api::Downstream;
use crate::common::api::StopMode;
use crate::common::api::{TaskRequest, Upstream};
use crate::common::util::client;
use crate::common::util::MangodArgs;
use crate::common::util::MangodStatus;

use crate::cli::options::compile::CompileCmd;
use crate::cli::source::handler::handle_source_request;
use crate::cli::status::running::ensure_running;

pub fn handle_compile_cmd(_args: &CompileCmd, status: &MangodStatus) -> Result<(), String> {
    ensure_running(status)?;
    client(
        &MangodArgs::default().address(),
        (),
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
                },
                Downstream::Task(_response) => unimplemented!(),
                Downstream::Source(request) => handle_source_request(request, sender.clone()),
            }
            Ok(())
        },
    )
}
