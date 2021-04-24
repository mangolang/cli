use ::structopt::StructOpt;
use ::mango_cli_common::util::MangodArgs;

#[derive(StructOpt, Debug)]
pub enum DaemonGetCmd {
    Status,
    Address,
}

#[derive(StructOpt, Debug)]
#[structopt(
//author = "Mango programming language CLI",
after_help = "Stop the mango daemon. Unless otherwise requested, finish all currently scheduled work first, but stop accepting new requests."
)]
pub struct DaemonStopCmd {
    #[structopt(
        long = "quick",
        about = "Stop all current tasks, just communicate the shutdown to clients and then stop.",
    )]
    quick: bool,

    #[structopt(
        long = "when-idle",
        conflicts_with = "quick",
        about = "Keep accepting new tasks, but if there is ever no work left, stop.",
    )]
    when_idle: bool,

}
//TODO @mark:

#[derive(StructOpt, Debug)]
pub enum DaemonCmd {
    #[structopt(about = "Start the Mango cli compiler daemon (automatic on compile)")]
    Start(MangodArgs),

    #[structopt(about = "Stop the Mango cli compiler daemon.")]
    Stop(DaemonStopCmd),

    #[structopt(about = "Get details about the Mango cli compiler daemon")]
    Get(DaemonGetCmd),
}
