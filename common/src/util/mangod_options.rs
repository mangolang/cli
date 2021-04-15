use ::structopt::StructOpt;

//TODO @mark: make multiple commands, of which 'start' is the implicit one

const DEFAULT_PORT: u16 = 47558;

#[derive(StructOpt, Debug)]
#[structopt(
    before_help = "Mango compiler daemon that does the actual compilation in the background.\nIt is often preferable to only use `mango` and not touch `mangod` yourself.",
    after_help = "Mango documentation: https://docs.mangocode.org/\nWarning: all Mango daemon options are subject to change!",
)]
pub struct MangodArgs {

    #[structopt(
        short = "b",
        long = "hostname",
        default_value = "localhost",
        help = "Hostname to listen on. Do not expose mangod to untrusted networks.",
    )]
    pub host: String,

    #[structopt(
        short = "p",
        long = "port",
        default_value = "47558",  // DEFAULT_PORT
        help = "Port to listen on.",
    )]
    pub port: u16,

    #[structopt(
        long = "cpus",
        help = "The number of compile worker threads.",
    )]
    pub worker_count: Option<u16>,

    #[structopt(
        long = "memory",
        help = "The amount of memory used for the compile cache (in MB).",
    )]
    pub cache_mem_mb: Option<u32>,

    #[structopt(
        long = "ignore-running",
        help = "Start the daemon even if there is already one running. Not recommended.",
    )]
    pub ignore_running: bool,
}

impl MangodArgs {
    pub fn address(&self) -> String {
        assert!(!self.host.contains(":"));
        assert!(!self.host.contains(" "));
        format!("{}:{}", &self.host, &self.port)
    }
}

impl MangodArgs {
    pub fn as_vec(&self) -> Vec<String> {
        let mut args = vec![];
        args.push("--hostname".to_owned());
        args.push(self.host.clone());
        args.push("--port".to_owned());
        args.push(self.port.to_string());
        if let Some(workers) = self.worker_count {
            args.push("--cpus".to_owned());
            args.push(workers.to_string());
        }
        if let Some(mem) = self.cache_mem_mb {
            args.push("--memory".to_owned());
            args.push(mem.to_string());
        }
        if self.ignore_running {
            args.push("--ignore-running".to_owned());
        }
        args
    }
}

impl Default for MangodArgs {
    fn default() -> Self {
        MangodArgs {
            host: "localhost".to_owned(),
            port: DEFAULT_PORT,
            worker_count: None,
            cache_mem_mb: None,
            ignore_running: false,
        }
    }
}
