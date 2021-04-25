use ::std::process::exit;
use ::std::thread::{sleep, spawn};
use ::std::time::Duration;

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
                    shutdown_quick(sender)
                },
                StopMode::FinishCurrentWork => {
                    sender.send_err("shutdown mode 'finish current work' is not implemented yet, shutting down quickly");
                    shutdown_quick(sender)
                },
                StopMode::WhenIdle => {
                    sender.send_err("shutdown mode 'when idle' is not implemented yet, shutting down quickly");
                    shutdown_quick(sender)
                },
            }
        },
    }
}

pub fn shutdown_quick(sender: &RespSender) -> Result<Response, String> {
    eprintln!("unimplemented: using send because broadcast does not work");  //TODO @mark: TEMPORARY! REMOVE THIS!
    sender.send(Response::Control(ControlResponse::Stopping(StopMode::Quick)));
    sender.send(Response::Control(ControlResponse::Stopped));
    //TODO @mark: without spawn the above messages don't arrive (tried sleep and nothing)
    spawn(|| {
        // Sleep so that the broadcast has time to reach all clients.
        // Sleep is not great, but the server is shutting down anyway.
        sleep(Duration::from_millis(500));
        info!("quick server shutdown complete");
        clear_lock();
        exit(0)
    });
    Err("going to shut down, bye".to_owned())
}
