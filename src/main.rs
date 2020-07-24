mod options;

use options::{compile, Command, MangoArgs};

#[paw::main]
fn main(args: MangoArgs) {
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
                }
                todo!()
            }
            _ => eprintln!("This operation is not supported yet"),
        },
    }
}
