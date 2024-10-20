use input_linux::EvdevHandle;
use libc::{fcntl, F_GETFL, F_SETFL, O_NONBLOCK};
use std::io;
use std::os::unix::io::RawFd;

use std::{
    fs::File,
    os::fd::{AsRawFd, FromRawFd},
    path::PathBuf,
};

/// open file and set blocking behavior, returns evdev handler or error
pub fn open_evdev(path: &PathBuf, blocking: bool) -> std::io::Result<EvdevHandle<File>> {
    let file = File::open(path)?;
    if blocking {
        Ok(EvdevHandle::new(file))
    } else {
        let raw_fd = file.as_raw_fd();
        std::mem::forget(file); // we conceptually move ownership to the raw_fd
        set_blocking(raw_fd, false)?;
        let nb_file = unsafe { File::from_raw_fd(raw_fd) };
        Ok(EvdevHandle::new(nb_file))
    }
}

/// Set blocking mode of raw file handler, see [https://github.com/anowell/nonblock-rs]
pub(crate) fn set_blocking(fd: RawFd, blocking: bool) -> io::Result<()> {
    let flags = unsafe { fcntl(fd, F_GETFL, 0) };
    if flags < 0 {
        return Err(io::Error::last_os_error());
    }

    let flags = if blocking {
        flags & !O_NONBLOCK
    } else {
        flags | O_NONBLOCK
    };
    let res = unsafe { fcntl(fd, F_SETFL, flags) };
    if res != 0 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}
