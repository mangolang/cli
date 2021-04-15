use ::mango_cli_common::util::lockfile::load_lock;

mod connection;

#[paw::main]
fn main(args: MangodArgs) {
    let lockfile = load_lock();
    let status = match &lockfile {
        Some(info) => determine_status(info.pid()),
        None => MangodStatus::Inactive,
    };
    handle_cmd(&args, &status);
}
