use std::num::ParseIntError;

use vmmap::{ffi, get_offset, mach_vm_read_overwrite, task_for_pid};

pub mod vmmap;

#[derive(Debug)]
pub struct Error(String);

pub type Result<T, E = Error> = core::result::Result<T, E>;

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error(value.to_string())
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error(String::from("args error"))
    }
}

impl From<ffi::kern_return_t> for Error {
    fn from(value: ffi::kern_return_t) -> Self {
        Error(value.to_string())
    }
}

pub fn bytes_to_usize(buf: &[u8]) -> Result<usize> {
    Ok(usize::from_le_bytes(
        buf.try_into()
            .map_err(|_| Error("bytes_to_usize error".to_string()))?,
    ))
}

pub struct Process {
    pub pid: i32,
    pub task: ffi::mach_port_t,
}

impl Process {
    pub fn open(pid: i32) -> Result<Self> {
        let task = task_for_pid(pid)?;
        Ok(Self { pid, task })
    }

    pub fn get_offset(&self) -> Result<u64> {
        Ok(get_offset(self.task)?)
    }

    pub fn read_at_into(&self, offset: usize, buf: &mut [u8]) -> Result<()> {
        // remove out?
        let mut out: u64 = 0;
        Ok(mach_vm_read_overwrite(self.task, offset as _, buf.len() as _, buf.as_mut_ptr() as _, &mut out)?)
    }
}
