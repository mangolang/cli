use ::structopt::StructOpt;

mod compile;

#[derive(StructOpt)]
pub struct Mango {

    #[structopt(
        short = "v",
        long,
        help = "Show verbose information for debugging."
    )]
    verbose: bool,

    #[structopt(
        conflicts_with = "verbose",
        short = "q",
        long = "quiet",
        help = "Only show the most important output."
    )]
    quiet: bool,

    #[structopt(subcommand)]
    cmd: Command
}

#[derive(StructOpt)]
enum Command {
    Compile(compile::CompileCmd),
}
