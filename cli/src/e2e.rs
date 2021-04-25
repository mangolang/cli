use ::std::process::Command;
use ::std::str::from_utf8;
use ::std::sync::Once;
use std::process::Output;

use ::assert_cmd::prelude::*;
use ::serial_test::serial;
use ::structopt::clap::ErrorKind;
use ::structopt::StructOpt;

use super::*;

static INIT: Once = Once::new();

fn init() {
    // Assumes executables without extensions. Might have to be adapted to non-linux.
    INIT.call_once(|| {
        println!("starting building");
        let result = Command::new("cargo")
            .arg("build")
            .arg("--workspace")
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

fn do_cli(args: &[&str]) -> Output {
    Command::cargo_bin("mango").unwrap()
        .args(args)
        .output()
        .unwrap()
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

#[serial]
#[test]
fn daemon_start_stop() {
    init();
    do_cli(&["mango", "daemon", "stop", "-c"]);

    // Start
    let res = do_cli(&["daemon", "start"]);
    assert!(res.status.success(), "{:?}", res);

    let res = do_cli(&["daemon", "get", "status"]);
    let out = from_utf8(&res.stdout).unwrap().trim();
    assert_eq!(out, "running");

    // Stop
    let res = do_cli(&["daemon", "stop"]);
    assert!(res.status.success());

    let res = do_cli(&["daemon", "get", "status"]);
    let out = from_utf8(&res.stdout).unwrap().trim();
    assert_eq!(out, "stopped");
}
