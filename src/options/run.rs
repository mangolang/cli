use ::structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct RunCmd {
    //TODO @mark: change this name if the entrypoint keyword is changed
    #[structopt(
        long = "main",
        help = "Which executables to compile, as comma-separated list. Required if multiple exist."
    )]
    pub mains: Option<String>,
}
