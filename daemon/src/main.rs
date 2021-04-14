use ::ws::listen;

use crate::options::{MangodArgs, Command, MangodStartArgs};

mod options;

#[paw::main]
fn main(args: MangodArgs) {
    match args.cmd {
        None => {
            eprintln!("no subcommand, assuming 'mangod start'");
            start(&MangodStartArgs::default())
        },
        Some(Command::Start(start_args)) => start(&start_args),
        Some(Command::Stop(_)) => stop(),
        Some(Command::GetPid(_)) => get_pid(),
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
