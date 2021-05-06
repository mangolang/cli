use ::structopt::StructOpt;

use crate::common::util::MangodArgs;

pub mod clean;
pub mod compile;
pub mod daemon;
pub mod exec_test;
pub mod run;

#[derive(StructOpt, Debug)]
#[structopt(
    //author = "Mango programming language CLI",
    after_help = "Mango documentation: https://docs.mangocode.org/\nWarning: all Mango CLI options are subject to change!"
)]
#[rustfmt::skip]
pub struct MangoArgs {
    #[structopt(
        short = "v",
        long,
        help = "Show verbose information for debugging.",
        hidden_short_help=true
    )]
    pub verbose: bool,

    #[structopt(
        conflicts_with = "verbose",
        short = "q",
        long = "quiet",
        help = "Only show the most important output.",
        hidden_short_help=true
    )]
    pub quiet: bool,

    #[structopt(
        long = "daemon",
        help = "Only show the most important output.",
        hidden_short_help=true
    )]
    pub daemon: bool,

    #[structopt(subcommand)]
    pub cmd: MangoCommand,
}

#[derive(StructOpt, Debug)]
pub enum MangoCommand {
    // Note: this particular about text is part of a Github Action to check the CLI
    #[structopt(about = "Compile the code in the current directory to one of various formats")]
    Compile(compile::CompileCmd),

    #[structopt(about = "Run the current Mango project")]
    Run(run::RunCmd),

    #[structopt(about = "Execute tests for the current Mango project")]
    Test(exec_test::TestCmd),

    #[structopt(about = "Clean any build results or cache for the current Mango project")]
    Clean(clean::CleanCmd),

    #[structopt(about = "Control the Mango daemon (for all projects)")]
    Daemon(daemon::DaemonCmd),

    #[structopt(about = "Run as a blocking mango daemon. Can be controlled using the other cli commands, like 'daemon'.")]
    RunAsDaemon(MangodArgs),
}
