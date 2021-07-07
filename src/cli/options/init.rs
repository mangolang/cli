use ::structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct InitCmd {
    #[structopt(
        short = "n",
        long = "name",
        help = "Name of the project to initialize. If omitted, you will be asked interactively."
    )]
    pub name: Option<String>,
}
