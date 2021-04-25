use ::std::path::PathBuf;
use ::std::process::{Command, Stdio};
use ::std::sync::Once;

use ::structopt::clap::ErrorKind;
use ::structopt::StructOpt;

use super::*;
use std::str::from_utf8;

static INIT: Once = Once::new();

fn init() {
    // Assumes executables without extensions. Might have to be adapted to non-linux.
    INIT.call_once(|| {
        println!("starting building");
        let result = Command::new("cargo")
            .arg("build")
            .arg("--workspace")
            .arg("--release")
            .stdin(Stdio::null())
            .output()
            .unwrap();
        assert!(result.status.success(), "build failed: {}", from_utf8(&result.stderr).unwrap());
        // let mut path = PathBuf::from(env!("CARGO_TARGET_DIR"));
        // path.push("release");
        // let mut mango = path.clone();
        // mango.push("mango");
        // assert!(mango.is_file(), "mango cli executable not found at {}", &mango.to_string_lossy());
        // let mut mangod = path;
        // mangod.push("mangod");
        // assert!(mangod.is_file(), "mango daemon executable not found at {}", &mangod.to_string_lossy());
        println!("finished building");
    });
}

fn do_cli(args: &[&str]) {
    Command::new("target/debug/mango")
        .args(args)
        .stdin(Stdio::null())
        .output()
        .unwrap();
}

#[test]
fn show_help() {
    init();
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
    init();
    let args = MangoArgs::from_iter_safe(&["mango", "compile", "ir"]).unwrap();
    cli(args)
}

#[test]
fn daemon_start_stop() {
    init();
    //cli(MangoArgs::from_iter(&["mango", "daemon", "start"]));
    let args = do_cli(&["daemon", "get", "status"]);
    // dbg!(&args);  //TODO @mark: TEMPORARY! REMOVE THIS!
    // cli(args.unwrap());
    //cli(MangoArgs::from_iter(&["mango", "daemon", "stop", "-c", "--quick"]));
    //cli(MangoArgs::from_iter(&["mango", "daemon", "get", "status"]));
}
