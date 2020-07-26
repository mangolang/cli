//! The command line interface (CLI) is how you can make the Mango compiler convert your code to executables.
//!
//! Note that this is technical API documentation; for more general language documentation, see [docs.mangocode.org](https://docs.mangocode.org/en/latest/).
#![doc(html_favicon_url = "https://mangocode.org/mango_logo.png")]
#![doc(html_logo_url = "https://mangocode.org/mango_logo.png")]

// #[allow(unused_imports)]
// use ::mangolib;

use crate::options::compile::Target;
use crate::options::Command;
use crate::options::MangoArgs;

mod options;

#[paw::main]
fn main(args: MangoArgs) {
    cli(args)
}

pub fn cli(args: MangoArgs) {
    match args.cmd {
        Command::Compile(compile) => match compile.target {
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
        _ => {
            eprintln!("Not implemented");
        }
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
