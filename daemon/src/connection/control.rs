use ::log::info;

use ::mango_cli_common::api::{ControlRequest, ControlResponse, Response, StopMode};
use ::mango_cli_common::util::RespSender;

pub fn handle_control(request: &ControlRequest, sender: &RespSender) -> Result<Response, String> {
    match request {
        ControlRequest::Ping => Ok(Response::Control(ControlResponse::Pong)),
        ControlRequest::Stop(mode) => match mode {
            StopMode::Quick => {
                info!("shutting down quickly because of quick-stop request; current work will be abandoned");
                shutdown_quick(sender)
            }
            StopMode::FinishCurrentWork => {
                sender.send_err("shutdown mode 'finish current work' is not implemented yet, shutting down quickly");
                shutdown_quick(sender)
            }
            StopMode::WhenIdle => {
                sender.send_err("shutdown mode 'when idle' is not implemented yet, shutting down quickly");
                shutdown_quick(sender)
            }
        },
    }
}

pub fn shutdown_quick(sender: &RespSender) -> Result<Response, String> {
    sender
        .connection
        .broadcast(Response::Control(ControlResponse::Stopping(StopMode::Quick)));
    sender.connection.no_new_connections();
    sender.connection.broadcast(Response::Control(ControlResponse::Stopped));
    sender.connection.shutdown();
    info!("quick server shutdown complete");
    Ok(Response::Ok)
}
