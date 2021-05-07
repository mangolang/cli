use ::std::collections::HashSet;
use ::std::env;
use ::std::path::PathBuf;
use ::std::process::Command;
use ::std::process::Output;
use ::std::str::from_utf8;
use ::std::str::FromStr;
use ::std::sync::RwLock;
use ::std::thread::sleep;
use ::std::time::Duration;

use ::lazy_static::lazy_static;
use ::serde_json::Map;
use ::serde_json::Value;
use ::tempfile::TempDir;

use serde::__private::from_utf8_lossy;
use std::path::Path;

lazy_static! {
    static ref MANGO_EXE: RwLock<Option<PathBuf>> = RwLock::new(None);
}

fn compile_exe() {
    MANGO_EXE.write().unwrap().get_or_insert_with(|| {
        println!("starting building executable");
        let result = Command::new("cargo").arg("build").arg("--message-format=json").output().unwrap();
        assert!(result.status.success(), "build failed: {}", from_utf8(&result.stderr).unwrap());
        println!("finished building executable");

        let build_infos = from_utf8(&result.stdout)
            .expect("could not parse json output of cargo build");
        let mut exes = HashSet::with_capacity(1);
        for line in build_infos.lines() {
            let json = serde_json::from_str::<Map<String, Value>>(line)
                .expect("output of cargo build was not valid json despite --message-format=json");
            match json.get("executable") {
                Some(Value::String(exe_path)) => {
                    exes.insert(PathBuf::from_str(exe_path).unwrap());
                },
                Some(Value::Null) => continue,
                None => continue,
                Some(_) => panic!("unexpected value for 'executable' in cargo output"),
            }
        }
        assert!(exes.len() <= 1, "found multiple executables! {:?}", exes);
        assert!(exes.len() > 0, "could not find executable in cargo output");
        exes.into_iter().next().unwrap()
    });
}

fn run_cli(exe_path: &Path, args: &[&str]) -> Output {
    //TODO @mark: does this cause double locking?
    let exe_read = MANGO_EXE.read().unwrap();
    let dir = TempDir::new().unwrap();
    env::set_var("MANGO_USER_CACHE_PATH", &dir.path().to_string_lossy().into_owned());
    let output = Command::new(exe_path)
        .args(args)
        .output()
        .unwrap();
    // Explicitly drop to make sure the lock is held until here.
    drop(exe_read);
    drop(dir);
    output
}

fn run_with_daemon(args: &[&str], test: impl FnOnce(Output)) {
    compile_exe();

    let dir = TempDir::new().unwrap();
    env::set_var("MANGO_USER_CACHE_PATH", &dir.path().to_string_lossy().into_owned());
    MANGO_EXE.read().unwrap().map(|exe_pth| {
        let mut child = Command::new(&exe_pth)
            .arg("run-as-daemon")
            .spawn()
            .unwrap();
        test(run_cli(&exe_pth, args));
        child.wait();
    });
    drop(dir);
}

#[test]
fn show_help() {
    run_with_daemon(
        &["-h"],
        |output| {
            let out = from_utf8_lossy(&output.stdout);
            assert!(out.contains("mango"));
        },
    );
}

#[test]
fn compile_ir() {
    run_with_daemon(
        &["compile"],
        |output| assert!(output.status.success()),
    );
}

#[test]
fn daemon_start_stop() {
    let dir = TempDir::new().unwrap();
    env::set_var("MANGO_USER_CACHE_PATH", &dir.path().to_string_lossy().into_owned());
    compile_exe();

    // Start
    let res = run_cli(&["daemon", "start", "-p", "47559"]);
    let start_txt = from_utf8(&res.stderr).unwrap();
    println!("starting:\n{}/starting", start_txt);
    assert!(res.status.success(), "{}", start_txt);

    let res = run_cli(&["daemon", "get", "status"]);
    let out = from_utf8(&res.stdout).unwrap().trim();
    assert_eq!(out, "running");

    // Stop
    let res = run_cli(&["daemon", "stop"]);
    let stop_txt = from_utf8(&res.stderr).unwrap();
    println!("stopping:\n{}/stopped", stop_txt);
    assert!(res.status.success(), "{}", from_utf8(&res.stderr).unwrap());

    // Sleep here because the server sleept 50ms before shutting down
    //TODO get rid of sleep when the server no longer sleeps
    sleep(Duration::from_millis(75));

    let res = run_cli(&["daemon", "get", "status"]);
    let out = from_utf8(&res.stdout).unwrap().trim();
    assert_eq!(out, "not-started");
}
