//! The command line interface (CLI) is how you can make the Mango compiler convert your code to executables.
//!
//! Note that this is technical API documentation; for more general language documentation, see [docs.mangocode.org](https://docs.mangocode.org/en/latest/).
#![doc(html_favicon_url = "https://mangocode.org/mango_logo.png")]
#![doc(html_logo_url = "https://mangocode.org/mango_logo.png")]

// #[allow(unused_imports)]
// use ::mangolib;

use ::std::process::exit;

use ::env_logger;

use crate::cli::compile::handle_compile_cmd;
use crate::cli::options::MangoArgs;
use crate::cli::options::MangoCommand;
use crate::cli::status::handle_daemon_cmd;
use crate::common::util::MangodStatus;
use crate::daemon::run_mango_daemon;

mod cli;
mod common;
mod daemon;

#[paw::main]
fn main(args: MangoArgs) {
    env_logger::init();
    match cli(args) {
        Ok(_) => {}
        Err(err_msg) => {
            eprintln!("{}", err_msg);
            exit(1)
        }
    }
}

pub fn cli(args: MangoArgs) -> Result<(), String> {
    // let lockfile = load_lock();
    // let status = match &lockfile {
    //     Some(info) => determine_status(info.pid()),
    //     None => MangodStatus::Inactive,
    // };
    let status = MangodStatus::determine();
    match args.cmd {
        MangoCommand::Compile(compile) => handle_compile_cmd(&compile, &status),
        //TODO @mark:
        // match compile.target {
        //     Target::Check {} => {
        //         println!("Checking code...");
        //         todo!()
        //     }
        //     Target::IR { json, packed } => {
        //         match (json, packed) {
        //             (true, true) => println!("Creating json & packed IR..."),
        //             (true, false) => println!("Creating json IR..."),
        //             (false, true) => println!("Creating packed IR..."),
        //             (false, false) => println!("Creating json IR..."),
        //         };
        //         eprintln!("This operation is not supported yet");
        //     }
        //     _ => eprintln!("This operation is not supported yet"),
        // },
        MangoCommand::Run(_) => Err("Run is not supported yet".to_owned()),
        MangoCommand::Test(_) => Err("Test is not supported yet".to_owned()),
        MangoCommand::Clean(_) => Err("Cleaning output is not supported yet".to_owned()),
        MangoCommand::Daemon(cmd) => handle_daemon_cmd(&cmd, &status),
        MangoCommand::RunAsDaemon(args) => run_mango_daemon(args),
    }
}
