//! The command line interface (CLI) is how you can make the Mango compiler convert your code to executables.
//!
//! Note that this is technical API documentation; for more general language documentation, see [docs.mangocode.org](https://docs.mangocode.org/en/latest/).
#![doc(html_favicon_url = "https://mangocode.org/mango_logo.png")]
#![doc(html_logo_url = "https://mangocode.org/mango_logo.png")]

// #[allow(unused_imports)]
// use ::mangolib;


use ::env_logger;

use ::mango_cli_common::util::MangodStatus;

use crate::options::MangoCommand;
use crate::options::compile::Target;
use crate::options::MangoArgs;
use crate::status::handle_daemon_cmd;

mod options;
mod status;

#[paw::main]
fn main(args: MangoArgs) {
    env_logger::init();
    cli(args)
}

pub fn cli(args: MangoArgs) {
    // let lockfile = load_lock();
    // let status = match &lockfile {
    //     Some(info) => determine_status(info.pid()),
    //     None => MangodStatus::Inactive,
    // };
    let status = MangodStatus::determine();
    match args.cmd {
        MangoCommand::Compile(compile) => match compile.target {
            Target::Check {} => {
                println!("Checking code...");
                todo!()
            }
            Target::IR { json, packed } => {
                match (json, packed) {
                    (true, true) => println!("Creating json & packed IR..."),
                    (true, false) => println!("Creating json IR..."),
                    (false, true) => println!("Creating packed IR..."),
                    (false, false) => println!("Creating packed IR..."),
                };
                eprintln!("This operation is not supported yet");
            }
            _ => eprintln!("This operation is not supported yet"),
        },
        MangoCommand::Run(_) => eprintln!("Run is not supported yet"),
        MangoCommand::Test(_) => eprintln!("Test is not supported yet"),
        MangoCommand::Clean(_) => eprintln!("Cleaning output is not supported yet"),
        MangoCommand::Daemon(cmd) => handle_daemon_cmd(&cmd, &status),
    };
}

#[cfg(test)]
mod tests {
    use ::structopt::clap::ErrorKind;
    use ::structopt::StructOpt;

    use super::*;

    #[test]
    fn show_help() {
        assert_eq!(
            ErrorKind::HelpDisplayed,
            MangoArgs::from_iter_safe(&["mango", "-h"]).unwrap_err().kind
        );
        assert_eq!(
            ErrorKind::HelpDisplayed,
            MangoArgs::from_iter_safe(&["mango", "--help"]).unwrap_err().kind
        );
    }

    #[test]
    fn compile_ir() {
        let args = MangoArgs::from_iter(&["mango", "compile", "ir"]);
        cli(args)
    }
}
