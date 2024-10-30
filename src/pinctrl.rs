use rust_embed::RustEmbed;
use std::io::Write;
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::process::Command;
use libc::{memfd_create, ftruncate, MFD_CLOEXEC, MFD_ALLOW_SEALING};
use std::ffi::CString;
use std::fs::File;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

/// Creates an in-memory file with the contents of the `pinctrl` binary.
fn load_pinctrl_in_memory() -> std::io::Result<File> {
    // Create an anonymous memory file descriptor
    let fd = unsafe {
        let name = CString::new("pinctrl_memory")?;
        memfd_create(name.as_ptr(), MFD_CLOEXEC | MFD_ALLOW_SEALING)
    };

    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }

    // Convert the raw fd into a Rust File
    let mut memfile = unsafe { File::from_raw_fd(fd) };

    // Load the embedded `pinctrl` binary
    let pinctrl_data = Asset::get("pinctrl").expect("pinctrl binary missing in assets");

    // Write the binary data to the memory file
    memfile.write_all(&pinctrl_data.data)?;

    // Use ftruncate and check the return value directly
    if unsafe { ftruncate(fd, pinctrl_data.data.len() as i64) } != 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(memfile)
}

/// Executes the `pinctrl` binary loaded in memory with the given arguments.
pub fn execute_pinctrl_in_memory(args: &[&str]) -> std::io::Result<()> {
    // Load pinctrl binary into memory
    let memfile = load_pinctrl_in_memory()?;

    // Use `into_raw_fd` to take ownership of the file descriptor
    let fd = memfile.into_raw_fd();

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
