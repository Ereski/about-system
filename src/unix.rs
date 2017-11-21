extern crate rs_release;
extern crate sysinfo;
extern crate uname;

use {OSVersion, Result};

use self::rs_release::OsReleaseError;
use self::sysinfo::{System, SystemExt};

pub type PlatformError = OsReleaseError;

/// Get the operating system.
///
/// # Remarks
///
/// Retrieval of the distribution name is made through the `os-release` file.
/// If no `os-release` file is found, the `distribution` field will be empty.
///
/// # Requirements
///
/// `uname()` system call.
pub fn os() -> Result<OSVersion> {
    let name = uname::uname()?;

    Ok(OSVersion {
        name: format!("{} {}", name.sysname, name.release),
        version: name.release.parse()?,

        distribution: rs_release::get_os_release()?
            .get("NAME")
            .cloned()
            .unwrap_or_else(String::new),
    })
}

pub fn total_memory() -> u64 {
    let mut sys = System::new();
    sys.refresh_system();

    sys.get_total_memory()
}
