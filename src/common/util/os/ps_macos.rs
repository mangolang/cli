
pub fn kill_process(pid: u32) {
    //TODO @mark:
    eprintln!("mango cannot kill processes on this OS; please kill process {} yourself", pid);
}

pub fn is_process_alive(pid: u32) -> bool {
    //TODO @mark:
    eprintln!("mango cannot determine which processes are running on this OS; it will be assumed that process {} is running", pid);
    true
}
