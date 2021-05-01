use ::structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct CompileCmd {
    //#[structopt(subcommand)]
    //pub target: Target,

    //TODO @mark: change this name if the entrypoint keyword is changed
    #[structopt(
        long = "main",
        required = false,
        help = "Which executables to compile, as comma-separated list. Required if multiple exist."
    )]
    pub mains: Vec<String>,
}

#[derive(StructOpt, Debug)]
pub enum Target {
    #[structopt(about = "Do all the compile checks, then exit")]
    Check {},
    #[structopt(about = "Mango intermediary representation (for debugging)")]
    IR {
        #[structopt(long = "json", help = "Dump as pretty-printed json for easy human inspection")]
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
