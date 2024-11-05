use libc::{ftruncate, memfd_create, off_t, MFD_ALLOW_SEALING, MFD_CLOEXEC};
use std::ffi::CString;
use std::fs::File;
use std::io::Write;
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::process::Command;


/// Creates an in-memory file with the contents of the `pinctrl` binary.
fn load_pinctrl_in_memory() -> std::io::Result<File> {
    // Create an anonymous memory file descriptor
    let fd = unsafe {
        let name = CString::new("pinctrl")?;
        memfd_create(name.as_ptr(), MFD_CLOEXEC | MFD_ALLOW_SEALING)
    };

    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }

    // Convert the raw fd into a Rust File
    let mut mem_file = unsafe { File::from_raw_fd(fd) };

    // Load the embedded `pin ctrl` binary
    let pinctrl_bytes = include_bytes!("../assets/pinctrl");
    // Write the binary data to the memory file
    mem_file.write_all(pinctrl_bytes)?;

    // Use ftruncate and check the return value directly
    if unsafe { ftruncate(fd, pinctrl_bytes.len() as off_t) } != 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(mem_file)
}

/// Executes the `pin ctrl` binary loaded in memory with the given arguments.
pub fn execute_pinctrl(args: &[&str]) -> std::io::Result<()> {
    // Load pin ctrl binary into memory
    let mem_file = load_pinctrl_in_memory()?;

    // Use `into_raw_fd` to take ownership of the file descriptor
    let fd = mem_file.into_raw_fd();

    // Execute the in-memory binary
    let status = Command::new(format!("/proc/self/fd/{}", fd))
        .args(args)
        .status();

    // Close the file descriptor manually since we used `into_raw_fd`
    unsafe { libc::close(fd) };

    // Check the status of the command execution
    match status {
        Ok(exit_status) if exit_status.success() => Ok(()),
        Ok(exit_status) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("pinctrl exited with status: {:?}", exit_status),
        )),
        Err(e) => Err(e),
    }
}
