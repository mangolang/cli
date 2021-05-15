use ::structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct InitCmd {
    #[structopt(
        short = "n",
        long = "name",
        help = "Name of the project to initialize. Defaults to directory name."
    )]
    pub name: Option<String>,

}
