//TODO @mark: remove this whole module

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux", target_os = "freebsd", target_os = "openbsd")))]
pub use self::fallback::*;
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
pub use self::ps_linux::*;
#[cfg(target_os = "macos")]
pub use self::ps_macos::*;
#[cfg(target_os = "windows")]
pub use self::ps_windows::*;

#[cfg(target_os = "macos")]
mod ps_macos;
#[cfg(target_os = "windows")]
mod ps_windows;
#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
mod ps_linux;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux", target_os = "freebsd", target_os = "openbsd")))]
mod fallback {
    pub fn kill_process(pid: u32) {
        eprintln!("mango cannot kill processes on this OS; please kill process {} yourself", pid);
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux", target_os = "freebsd", target_os = "openbsd")))]
    pub fn is_process_alive(pid: u32) -> bool {
        eprintln!("mango cannot determine which processes are running on this OS; it will be assumed that process {} is running", pid);
        true
    }
}
