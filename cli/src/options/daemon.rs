use ::structopt::StructOpt;
use ::mango_cli_common::util::MangodArgs;

#[derive(StructOpt, Debug)]
pub enum DaemonGetCmd {
    #[structopt(about = "Get the current status of the Mango daemon process")]
    Status,
    #[structopt(about = "Get the address that the Mango daemon process, if it is running")]
    Address,
}

#[derive(StructOpt, Debug)]
#[structopt(
//author = "Mango programming language CLI",
after_help = "Stop the mango daemon. Unless otherwise requested, finish all currently scheduled work first, but stop accepting new requests."
)]
pub struct DaemonStopCmd {
    #[structopt(
        short = "c",
        long = "clear-lock",
        help = "If a lockfile for a daemon is found, but it is not responding, clear the lockfile and proceed. Useful if the previous daemon did not stop gracefully.",
    )]
    pub clear: bool,

    #[structopt(
        long = "quick",
        help = "Stop all current tasks, just communicate the shutdown to clients and then stop.",
    )]
    pub quick: bool,

    #[structopt(
        long = "when-idle",
        conflicts_with = "quick",
        help = "Keep accepting new tasks, but if there is ever no work left, stop.",
    )]
    pub when_idle: bool,

}

#[derive(StructOpt, Debug)]
pub enum DaemonCmd {
    #[structopt(about = "Start the Mango cli compiler daemon (automatic on compile)")]
    Start(MangodArgs),

    #[structopt(about = "Stop the Mango cli compiler daemon.")]
    Stop(DaemonStopCmd),

    #[structopt(about = "Get details about the Mango cli compiler daemon")]
    Get(DaemonGetCmd),
}
