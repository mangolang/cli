use ::std::collections::HashSet;
use ::std::env;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::process::Command;
use ::std::str::from_utf8;
use ::std::str::FromStr;
use ::std::sync::RwLock;
use ::std::thread::sleep;
use ::std::time::Duration;

use ::lazy_static::lazy_static;
use ::log::debug;
use ::serde::__private::from_utf8_lossy;
use ::serde_json::Map;
use ::serde_json::Value;
use ::tempfile::TempDir;

lazy_static! {
    static ref MANGO_EXE: RwLock<Option<PathBuf>> = RwLock::new(None);
}

fn compile_exe() {
    MANGO_EXE.write().unwrap().get_or_insert_with(|| {
        debug!("starting building executable");
        let result = Command::new("cargo").arg("build").arg("--message-format=json").output().unwrap();
        assert!(result.status.success(), "build failed: {}", from_utf8(&result.stderr).unwrap());
        debug!("finished building executable");

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
        let exe_pth = exes.into_iter().next().unwrap();
        debug!("executable at: {}", &exe_pth.to_string_lossy());
        exe_pth
    });
}

fn run_cli(args: &[&str]) -> (i32, String, String) {
    MANGO_EXE.read().unwrap().as_ref().map(|exe_pth| {
        run_cli_with(&exe_pth, args)
    }).unwrap()
}

fn run_cli_with(exe_path: &Path, args: &[&str]) -> (i32, String, String) {
    let output = Command::new(exe_path)
        .args(args)
        .output()
        .unwrap();
    (
        output.status.code().unwrap(),
        from_utf8_lossy(&output.stdout).into_owned(),
        from_utf8_lossy(&output.stderr).into_owned(),
    )
}

fn run_with_daemon(args: &[&str], test: impl FnOnce(i32, &str, &str)) {
    compile_exe();

    let dir = TempDir::new().unwrap();
    env::set_var("MANGO_USER_CACHE_PATH", &dir.path().to_string_lossy().into_owned());
    MANGO_EXE.read().unwrap().as_ref().map(|exe_pth| {
        eprintln!("HELLO A");  //TODO @mark: TEMPORARY! REMOVE THIS!
        let mut child = Command::new(&exe_pth)
            .arg("run-as-daemon")
            .arg("-p")
            .arg("47559")
            .env("RUST_LOG", "debug,ws=warn,mio=warn")
            .spawn()
            .unwrap();
        eprintln!("HELLO B");  //TODO @mark: TEMPORARY! REMOVE THIS!
        let (_, out, _) = run_cli(&["daemon", "get", "status"]);
        eprintln!("HELLO C");  //TODO @mark: TEMPORARY! REMOVE THIS!
        debug!("started mango daemon, status: {}", out);
        let (status, out, err) = run_cli_with(&exe_pth, args);
        test(status, &out, &err);
        eprintln!("HELLO D");  //TODO @mark: TEMPORARY! REMOVE THIS!
        panic!();  //TODO @mark: TEMPORARY! REMOVE THIS!
        debug!("going to stop mango daemon");
        sleep(Duration::from_millis(200));  //TODO @mark: TEMPORARY! REMOVE THIS!
        let (code, _, err) = run_cli(&["daemon", "stop"]);
        assert_eq!(code, 0, "failed to stop {}", err);
        child.wait().unwrap();
    }).unwrap();
    drop(dir);
}

#[test]
fn show_help() {
    run_with_daemon(
        &["-h"],
        |_, out, _| {
            assert!(out.contains("mango"));
        },
    );
}

#[test]
fn compile_ir() {
    run_with_daemon(
        &["compile"],
        |status, out, err| {
            assert!(status == 0);
            println!("out: {}\n/out", out);  //TODO @mark: TEMPORARY! REMOVE THIS!
            println!("err: {}\n/err", err);  //TODO @mark: TEMPORARY! REMOVE THIS!
        },
    );
}

#[test]
fn daemon_start_stop() {
    let dir = TempDir::new().unwrap();
    env::set_var("MANGO_USER_CACHE_PATH", &dir.path().to_string_lossy().into_owned());
    compile_exe();

    // Start
    let (code, _, err) = run_cli(&["daemon", "start", "-p", "47559"]);
    debug!("starting:\n{}/starting", err);
    assert_eq!(code, 0);

    let (code, out, _) = run_cli(&["daemon", "get", "status"]);
    assert_eq!(out.trim(), "running");
    assert_eq!(code, 0);

    // Stop
    let (code, _, err) = run_cli(&["daemon", "stop"]);
    debug!("stopping:\n{}/stopping", err);
    assert_eq!(code, 0);

    //TODO get rid of sleep when the server no longer sleeps
    sleep(Duration::from_millis(75));

    let (code, out, _) = run_cli(&["daemon", "get", "status"]);
    assert_eq!(out.trim(), "not-started");
    assert_eq!(code, 0);
}
