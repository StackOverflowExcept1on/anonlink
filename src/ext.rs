use std::process::Output;
use std::{io, str};

pub trait OutputExt: Sized {
    fn exit_result(self) -> io::Result<Self>;
}

impl OutputExt for Output {
    #[inline]
    fn exit_result(self) -> io::Result<Self> {
        self.status
            .exit_ok()
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidInput))?;
        Ok(self)
    }
}

pub trait SliceExt {
    fn read_process_line(&self) -> io::Result<&str>;
}

impl<T: AsRef<[u8]>> SliceExt for T {
    #[inline]
    fn read_process_line(&self) -> io::Result<&str> {
        let input = str::from_utf8(self.as_ref())
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;
        let line = input
            .lines()
            .next()
            .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
        Ok(line.trim())
    }
}
