extern crate kernel32;
extern crate win32_error;
extern crate winapi;

use {OSVersion, Result};

use self::win32_error::Win32Error;
use self::winapi::sysinfoapi::MEMORYSTATUSEX;
use self::winapi::winnt::OSVERSIONINFOEXW;
use semver::{Identifier, Version};
use std::ffi::OsString;
use std::mem;
use std::os::windows::ffi::OsStringExt;

pub type PlatformError = Win32Error;

/// Get the operating system.
///
/// # Remarks
///
/// Returns Windows 8 for version of Windows above 8 (FIXME:
/// https://msdn.microsoft.com/en-us/library/windows/desktop/dn481241(v=vs.85).
/// aspx).
///
/// # Requirements
///
/// At least Windows 2000 Professional/Server.
pub fn os() -> Result<OSVersion> {
    let mut buf: OSVERSIONINFOEXW = unsafe { mem::uninitialized() };
    buf.dwOSVersionInfoSize = mem::size_of::<OSVERSIONINFOEXW>() as u32;
    let buf_ptr = &mut buf as *mut OSVERSIONINFOEXW;
    if unsafe { kernel32::GetVersionExW(mem::transmute(buf_ptr)) } == 0 {
        return Err(Win32Error::new().into());
    }

    let version = Version {
        major: u64::from(buf.dwMajorVersion),
        minor: u64::from(buf.dwMinorVersion),
        patch: u64::from(buf.dwBuildNumber),
        pre: Vec::new(),
        build: vec![
            Identifier::AlphaNumeric(
                OsString::from_wide(&buf.szCSDVersion)
                    .to_string_lossy()
                    .into_owned()
            ),
        ],
    };

    Ok(OSVersion {
        name: format!("Windows {}", version),
        version,

        service_pack_version: Version {
            major: u64::from(buf.wServicePackMajor),
            minor: u64::from(buf.wServicePackMinor),
            patch: 0,
            pre: Vec::new(),
            build: Vec::new(),
        },
    })
}

pub fn total_memory() -> u64 {
    let mut buf: MEMORYSTATUSEX = unsafe { mem::uninitialized() };
    buf.dwLength = mem::size_of::<MEMORYSTATUSEX>() as u32;
    unsafe {
        kernel32::GlobalMemoryStatusEx(&mut buf as *mut MEMORYSTATUSEX);
    }

    buf.ullTotalPhys
}
