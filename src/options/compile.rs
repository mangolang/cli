use ::structopt::StructOpt;

#[derive(StructOpt)]
pub struct CompileCmd {
    #[structopt(subcommand)]  // Note that we mark a field as a subcommand
    target: Target
}

// subsubcommand!
#[derive(StructOpt)]
pub enum Target {
    #[structopt(about = "Mango intermediary representation (for debugging)")]
    IR {
        #[structopt(
            long = "json",
            help = "Dump as pretty-printed json for easy human inspection"
        )]
        json: bool,
        #[structopt(
            long = "packed",
            help = "Dump as efficiently-packed binary format, for small size and fast parsing"
        )]
        packed: bool,
    },
    #[structopt(about = "WebAssembly files and bindings")]
    WASM {},
    #[structopt(about = "Self-contained executable")]
    Executable {},
    #[structopt(about = "Docker image that runs your application")]
    Docker {},
}
