use ::std::env;
use ::std::process::Command;
use ::std::process::Output;
use ::std::str::from_utf8;
use ::std::thread::sleep;
use ::std::time::Duration;

use ::assert_cmd::prelude::*;
use ::serial_test::serial;
use ::structopt::clap::ErrorKind;
use ::structopt::StructOpt;
use ::tempfile::TempDir;

use crate::cli;
use crate::cli::options::MangoArgs;

fn do_cli(args: &[&str]) -> Output {
    Command::cargo_bin("mango").unwrap().args(args).output().unwrap()
}

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

#[serial]
#[test]
fn compile_ir() {
    let dir = TempDir::new().unwrap();
    env::set_var("MANGO_USER_CACHE_PATH", &dir.path().to_string_lossy().into_owned());
    let args = MangoArgs::from_iter_safe(&["mango", "compile"]).unwrap();
    cli(args).unwrap()
}

#[serial]
#[test]
fn daemon_start_stop() {
    let dir = TempDir::new().unwrap();
    use crate::cli;
    env::set_var("MANGO_USER_CACHE_PATH", &dir.path().to_string_lossy().into_owned());

    // Start
    let res = do_cli(&["daemon", "start", "-p", "47559"]);
    let start_txt = from_utf8(&res.stderr).unwrap();
    println!("starting:\n{}/starting", start_txt);
    assert!(res.status.success(), "{}", start_txt);

    let res = do_cli(&["daemon", "get", "status"]);
    let out = from_utf8(&res.stdout).unwrap().trim();
    assert_eq!(out, "running");

    // Stop
    let res = do_cli(&["daemon", "stop"]);
    let stop_txt = from_utf8(&res.stderr).unwrap();
    println!("stopping:\n{}/stopped", stop_txt);
    assert!(res.status.success(), "{}", from_utf8(&res.stderr).unwrap());

    // Sleep here because the server sleept 50ms before shutting down
    //TODO get rid of sleep when the server no longer sleeps
    sleep(Duration::from_millis(75));

    let res = do_cli(&["daemon", "get", "status"]);
    let out = from_utf8(&res.stdout).unwrap().trim();
    assert_eq!(out, "not-started");
}
