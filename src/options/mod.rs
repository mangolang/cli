use ::structopt::StructOpt;

pub mod compile;

#[derive(StructOpt)]
#[structopt(
    //author = "Mango programming language CLI",
    after_help = "Mango documentation: https://docs.mangocode.org/\nWarning: all Mango CLI options are subject to change!"
)]
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

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    // Note: this particular about text is part of a Github Action to check the CLI
    #[structopt(about = "Compile the code in the current directory to one of various formats")]
    Compile(compile::CompileCmd),
}
