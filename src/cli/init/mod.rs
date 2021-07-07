use ::std::io::stdin;

use crate::cli::options::init::InitCmd;

pub fn handle_init_cmd(args: &InitCmd) -> Result<(), String> {
    let name = args.name.clone()
        .unwrap_or_else(|| ask_name("project name? ")?);


    unimplemented!()
}

fn ask_name(prompt: &str) -> Result<String, String> {
    print!("{}", prompt);
    let q: Name = Name::new();
    let mut name = String::with_capacity(16);
    loop {
        let input = stdin().read_line(&mut name);

    }
}
