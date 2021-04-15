use ::structopt::StructOpt;
use ::mango_cli_common::util::MangodArgs;

#[derive(StructOpt, Debug)]
pub enum DaemonGetCmd {
    Status,
    Address,
}

#[derive(StructOpt, Debug)]
pub struct DaemonStopCmd {}
//TODO @mark:

#[derive(StructOpt, Debug)]
pub enum DaemonCmd {
    #[structopt(about = "Start the Mango cli compiler daemon (automatic on compile)")]
    Start(MangodArgs),

    #[structopt(about = "Stop the Mango cli compiler daemon")]
    Stop(DaemonStopCmd),

    #[structopt(about = "Get details about the Mango cli compiler daemon")]
    Get(DaemonGetCmd),
}
