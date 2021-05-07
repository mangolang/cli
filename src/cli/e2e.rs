use ::std::env;
use ::std::process::Command;
use ::std::process::Output;
use ::std::str::from_utf8;
use ::std::thread::sleep;
use ::std::time::Duration;

use ::assert_cmd::prelude::*;
use ::serde_json::Map;
use ::serde_json::Value;
use ::serial_test::serial;
use ::structopt::clap::ErrorKind;
use ::structopt::StructOpt;
use ::tempfile::TempDir;

use crate::cli;
use crate::cli::options::MangoArgs;

<<<<<<< HEAD
=======
static INIT: Once = Once::new();

fn init() {
    // Assumes executables without extensions. Might have to be adapted to non-linux.
    INIT.call_once(|| {
        println!("starting building");
        let result = Command::new("cargo").arg("build").arg("--message-format=json").output().unwrap();
        assert!(result.status.success(), "build failed: {}", from_utf8(&result.stderr).unwrap());
        let build_infos = from_utf8(&result.stdout)
            .expect("could not parse json output of cargo build");
        for line in build_infos.lines() {
            let json = serde_json::from_str::<Map<String, Value>>(line)
                .expect("output of cargo build was not valid json despite --message-format=json");
            match json.get("executable") {
                Some(Value::String(exe_path)) => todo!("here! {}", exe_path),
                Some(Value::Null) => continue,
                None => continue,
                Some(_) => panic!("unexpected value for 'executable' in cargo output"),
            }
        }

        unimplemented!(); //TODO @mark: TEMPORARY! REMOVE THIS!
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

>>>>>>> 7663480... Parse executable path in cargo output
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
