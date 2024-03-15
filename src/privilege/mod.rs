#[cfg(target_os = "linux")]
pub fn check_privileged() -> bool {
    // On Linux, check if the user ID is 0 (root)

    use nix::libc;
    unsafe { libc::geteuid() == 0 }
}

#[cfg(target_os = "windows")]
pub fn check_privileged() -> bool {
    // On Windows, use the `is_elevated` method from the `winapi` crate
    // Note: You need to add `winapi = { version = "0.3", features = ["winnt"] }` to your Cargo.toml
    use winapi::um::securitybaseapi::IsUserAnAdmin;
    use winapi::um::winbase::GetLastError;

    unsafe {
        match IsUserAnAdmin() {
            -1 => {
                // If the function fails, call GetLastError to get more information
                println!("Error: {}", GetLastError());
                false
            },
            _ => true,
        }
    }
}