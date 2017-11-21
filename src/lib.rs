// TODO: no_std

extern crate cpuid;
#[macro_use]
extern crate quick_error;
extern crate semver;

#[cfg(unix)]
mod unix;

#[cfg(unix)]
use unix::{PlatformError, total_memory};
#[cfg(unix)]
pub use unix::os;

#[cfg(windows)]
mod windows;

#[cfg(windows)]
use windows::{PlatformError, total_memory};
#[cfg(windows)]
pub use windows::os;

use semver::{SemVerError, Version};
use std::{io, result};
use std::error::Error as ErrorTrait;

/// Information about the current operating system
pub struct OSVersion {
    /// Full name of the operating system
    pub name: String,
    // TODO: os_type
    /// OS version info
    pub version: Version,

    #[cfg(unix)]
    /// Distribution name
    pub distribution: String,

    #[cfg(windows)]
    /// Service pack version
    pub service_pack_version: Version,
}

/// Basic hardware information
pub struct Platform {
    // TODO: NUMA?
    /// CPU name
    pub cpu: String,
    /// CPU vendor
    pub cpu_vendor: String,

    /// Total installed memory
    pub memory: u64,
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        PlatformError(err: PlatformError) {
            from()
            description(err.description())
            display("{}", err)
        }

        IO(err: io::Error) {
            from()
            description(err.description())
            display(s) -> ("{}: {}", s.description(), err)
        }

        SemVer(err: SemVerError) {
            from()
            description(err.description())
            display(s) -> ("{}: {}", s.description(), err)
        }

        String(err: String) {
            from()
            description(err)
            display("{}", err)
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

/// Get basic information about the hardware we're running on.
///
/// # Requirements
///
/// For *nix: a functional `/proc` filesystem with system information. For
/// Windows: XP or Server 2003.
// TODO: don't show requirements for anothe system in the doc
pub fn platform() -> Result<Platform> {
    let cpu = cpuid::identify()?;

    Ok(Platform {
        cpu: cpu.brand,
        cpu_vendor: cpu.vendor,

        memory: total_memory(),
    })
}
