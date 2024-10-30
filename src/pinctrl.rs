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

fn load_pinctrl_in_memory() -> std::io::Result<File> {
    // Use memfd_create to create an anonymous memory file descriptor
    let fd = unsafe {
        let name = CString::new("pinctrl_memory")?;
        let name_ptr = name.as_ptr(); // This pointer is now safe to use

        memfd_create(
            name_ptr,
            MFD_CLOEXEC | MFD_ALLOW_SEALING,
        )
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
    unsafe { ftruncate(fd, pinctrl_data.data.len() as i64) };

    Ok(memfile)
}

pub(crate) fn execute_pinctrl_in_memory(args: &[&str]) -> std::io::Result<()> {
    // Load pinctrl binary into memory
    let memfile = load_pinctrl_in_memory()?;

    // Use the file descriptor to execute the binary with Command
    Command::new(format!("/proc/self/fd/{}", memfile.into_raw_fd()))
        .args(args)
        .status()
        .expect("Failed to execute pinctrl from memory");

    Ok(())
}