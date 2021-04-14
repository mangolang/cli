use ::structopt::StructOpt;

//TODO @mark: make multiple commands, of which 'start' is the implicit one

#[derive(StructOpt, Debug)]
#[structopt(
    after_help = "Mango documentation: https://docs.mangocode.org/\nWarning: all Mango daemon options are subject to change!"
)]
#[rustfmt::skip]
pub struct MangodArgs {
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
        default_value = "47558",
        help = "Port to listen on.",
    )]
    pub port: u16,
}
