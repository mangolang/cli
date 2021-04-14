use ::structopt::StructOpt;

//TODO @mark: make multiple commands, of which 'start' is the implicit one

const DEFAULT_PORT: u16 = 47558;

#[derive(StructOpt, Debug)]
#[structopt(
    before_help = "Mango compiler daemon that does the actual compilation in the background.\nIt is often preferable to only use `mango` and not touch `mangod` yourself.",
    after_help = "Mango documentation: https://docs.mangocode.org/\nWarning: all Mango daemon options are subject to change!",
)]
pub struct MangodArgs {
    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
#[structopt(
    after_help = "Start the mango compiler daemon in the background."
)]
pub struct MangodStartArgs {

    #[structopt(
        short = "b",
        long = "hostname",
        default_value = "localhost",
        help = "Hostname to listen on. Do not expose mangod to untrusted networks.",
    )]
    pub host: String,

    #[structopt(
        short = "p",
        long = "port",
        default_value = "47558",  // DEFAULT_PORT
        help = "Port to listen on.",
    )]
    pub port: u16,
}
//TODO @mark: worker thread count? or just set though socket?

impl Default for MangodStartArgs {
    fn default() -> Self {
        MangodStartArgs {
            host: "localhost".to_owned(),
            port: DEFAULT_PORT,
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    after_help = "Stop the mango compiler daemon if it is running in the background."
)]
pub struct MangodStopArgs {}
//TODO @mark: force stop?

#[derive(StructOpt, Debug)]
#[structopt(
    after_help = "Stop the mango compiler daemon if it is running in the background."
)]
pub struct MangodGetArgs {
    #[structopt(subcommand)]
    pub cmd: GetCommand,
}

#[derive(StructOpt, Debug)]
pub enum GetCommand {
    #[structopt(about = "Get the status of mangod.")]
    Status,

    #[structopt(about = "Get the process id of the currently running mangod, if any.")]
    Pid,

    #[structopt(about = "Get the address that mangod is currently available at, if any.")]
    Address,
}


#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(about = "Compile the code in the current directory to one of various formats")]
    Start(MangodStartArgs),

    #[structopt(about = "Run the current Mango project")]
    Stop(MangodStopArgs),

    #[structopt(about = "Execute tests for the current Mango project")]
    Get(MangodGetArgs),
}
