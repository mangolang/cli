use ::std::process::exit;

use ::log::error;
use ::log::info;

use ::mango_cli_common::api::{ControlRequest, ControlResponse, Response, StopMode};
use ::mango_cli_common::util::{clear_lock, RespSender};

pub fn handle_control(request: &ControlRequest, sender: &RespSender) -> Result<Response, String> {
    match request {
        ControlRequest::Ping => Ok(Response::Control(ControlResponse::Pong)),
        ControlRequest::Stop(mode) => {
            match mode {
                StopMode::Quick => {
                    info!("shutting down quickly because of quick-stop request; current work will be abandoned");
                    shutdown_quick()
                },
                StopMode::FinishCurrentWork => {
                    sender.send_err("shutdown mode 'finish current work' is not implemented yet, shutting down quickly");
                    shutdown_quick()
                },
                StopMode::WhenIdle => {
                    sender.send_err("shutdown mode 'when idle' is not implemented yet, shutting down quickly");
                    shutdown_quick()
                },
            }
        },
    }
}

pub fn shutdown_quick() -> ! {
    //TODO @mark: send notifications to all connected clients
    error!("shutdown mode 'quick' is not implemented yet");
    clear_lock();
    exit(0)
}