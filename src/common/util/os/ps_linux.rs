
pub fn kill_process(pid: u32) {
    if let Err(err) = Command::new("sh")
            .arg("-c")
            .arg(format!("kill -9 {}", pid))
            .output() {
        eprintln!("mango could not kill process {} (reason: {})", pid, err);
    }
}

pub fn is_process_alive(pid: u32) -> bool {
    if let Err(_) = Command::new("sh")
            .arg("-c")
            .arg(format!("kill -0 {}", pid))
            .output() {
        return false;
    }
    return true;
}
