//! `fs-set-times` provides functions to set timestamps on files, directories,
//! and other filesystem objects.
//!
//! On Windows, modifying a file's timestamp requires write access to the file.

#![deny(missing_docs)]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

mod set_times;
mod system_time_spec;

pub use set_times::{set_atime, set_mtime, set_symlink_times, set_times, SetTimes};
pub use system_time_spec::SystemTimeSpec;

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, FromRawHandle};

/// Utility for returning an `AsRawFd` type as a `std::fs::File`.
///
/// # Safety
///
/// Callers must ensure that the resulting `std::fs::File` doesn't outlive the
/// underlying file descriptor.
#[cfg(not(windows))]
unsafe fn as_file<Fd: AsRawFd>(fd: &Fd) -> std::mem::ManuallyDrop<std::fs::File> {
    std::mem::ManuallyDrop::new(std::fs::File::from_raw_fd(fd.as_raw_fd()))
}

#[cfg(windows)]
unsafe fn as_file<Handle: AsRawHandle>(handle: &Handle) -> std::mem::ManuallyDrop<std::fs::File> {
    std::mem::ManuallyDrop::new(std::fs::File::from_raw_handle(handle.as_raw_handle()))
}
