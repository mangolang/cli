use ::ws::listen;

use crate::options::{MangodArgs, Command, MangodStartArgs};

mod options;

#[paw::main]
fn main(args: MangodArgs) {
    match args.cmd {
        Command::Start(start_args) => start(&start_args),
        Command::Stop(_) => stop(),
        Command::GetPid(_) => get_pid(),
    }
}

fn start(args: &MangodStartArgs) {
    assert!(!args.host.contains(":"));
    assert!(!args.host.contains(" "));
    let addr = format!("{}:{}", &args.host, &args.port);
    println!("starting mangod, listening on {}", &addr);
    listen(&addr, |out| {
        move |msg| {
            out.send(msg)
        }
    }).unwrap()
}

fn stop() {
    unimplemented!()
}

fn get_pid() {
    unimplemented!()
}
