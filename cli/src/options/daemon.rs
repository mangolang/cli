use ::structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct DaemonStatusCmd {
    #[structopt(
        long,
        help = "Show verbose information in json format.",
    )]
    pub json: bool,
    //TODO @mark:

    #[structopt(
        conflicts_with = "json",
        short = "v",
        long,
        help = "Show extra details in the output.",
    )]
    pub verbose: bool,
    //TODO @mark:

}

#[derive(StructOpt, Debug)]
pub struct DaemonStartCmd {}
//TODO @mark:

#[derive(StructOpt, Debug)]
pub struct DaemonStopCmd {}
//TODO @mark:

#[derive(StructOpt, Debug)]
pub enum DaemonCmd {
    #[structopt(about = "Show the status the Mango cli compiler daemon")]
    Status(DaemonStatusCmd),

    #[structopt(about = "Start the Mango cli compiler daemon (automatic on compile)")]
    Start(DaemonStartCmd),

    #[structopt(about = "Stop the Mango cli compiler daemon")]
    Stop(DaemonStartCmd),
}
