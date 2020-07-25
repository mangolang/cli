use options::{Command, compile, MangoArgs};

mod options;

#[paw::main]
fn main(args: MangoArgs) {
    cli(args)
}

pub fn cli(args: MangoArgs) {
    match args.cmd {
        Command::Compile(compile) => match compile.target {
            compile::Target::Check {} => {
                println!("Checking code...");
                todo!()
            }
            compile::Target::IR { json, packed } => {
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
    };
}

#[cfg(test)]
mod tests {
    use ::structopt::StructOpt;
    use ::structopt::clap::App;

    use super::*;

    #[test]
    fn show_help() {
        let args = MangoArgs::from_iter_safe(&["mango", "-h"]);
        let args = MangoArgs::from_iter_safe(&["mango", "--help"]);
    }

    #[test]
    fn compile_ir() {
        let args = MangoArgs::from_iter(&["mango", "compile", "ir"]);
        cli(args)
    }
}
