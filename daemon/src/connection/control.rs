use ::std::process::exit;

use ::log::error;

use ::mango_cli_common::api::{ControlRequest, ControlResponse, Request, Response, StopMode};

pub fn handle_control(request: &ControlRequest) -> Result<Response, ()> {
    match request {
        ControlRequest::Ping => Ok(Response::Control(ControlResponse::Pong)),
        ControlRequest::Stop(mode) => {
            match mode {
                StopMode::Quick => {
                    info!("shutting down quickly because of quick-stop request; current work will be abandoned");
                    shutdown_quick()
                },
                StopMode::FinishCurrentWork => {
                    error!("shutdown mode 'finish current work' is not implemented yet, shutting down quickly");
                    shutdown_quick()
                },
                StopMode::WhenIdle => {
                    error!("shutdown mode 'when idle' is not implemented yet, shutting down quickly");
                    shutdown_quick()
                },
            }
        },
    }
}

pub fn shutdown_quick() {
    //TODO @mark:
    error!("shutdown mode 'quick' is not implemented yet");
    exit(0)
}
