use ::structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct TestCmd {
    #[structopt(long = "pattern", help = "Regular expression pattern for test names.")]
    pub pattern: Option<String>,
}
