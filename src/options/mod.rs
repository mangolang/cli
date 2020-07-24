use ::structopt::StructOpt;

pub mod compile;

#[derive(StructOpt)]
#[structopt(about = "Warning: all Mango CLI options are subject to change!")]
pub struct MangoArgs {
    #[structopt(short = "v", long, help = "Show verbose information for debugging.")]
    pub verbose: bool,

    #[structopt(
        conflicts_with = "verbose",
        short = "q",
        long = "quiet",
        help = "Only show the most important output."
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
