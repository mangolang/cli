use ::structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    after_help = "Mango documentation: https://docs.mangocode.org/\nWarning: all Mango daemon options are subject to change!"
)]
#[rustfmt::skip]
pub struct MangodArgs {
    #[structopt(
        short = "h",
        long = "hostname",
        default = "localhost",
        help = "Hostname to listen on.",
    )]
    pub host: String,

    #[structopt(
        short = "p",
        long = "port",
        //TODO @mark:
        default = "47557",
        help = "Port to listen on.",
    )]
    pub port: u16,
}
