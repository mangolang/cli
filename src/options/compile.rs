use ::structopt::StructOpt;

#[derive(StructOpt)]
pub struct CompileCmd {
    #[structopt(subcommand)]  // Note that we mark a field as a subcommand
    target: Target
}

// subsubcommand!
#[derive(StructOpt)]
pub enum Target {
    IR {
        applications: u32
    },
    WASM {},
    Executable {},
    Docker {},
}
