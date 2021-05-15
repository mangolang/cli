use crate::cli::options::init::InitCmd;

pub fn handle_init_cmd(args: &InitCmd) -> Result<(), String> {
    let name = args.name.unwrap_or_else(|| "dirname".to_owned());


    unimplemented!()
}
