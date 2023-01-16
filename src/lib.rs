use vmmap::{ffi, get_offset, mach_vm_read_overwrite, task_for_pid};

pub mod vmmap;

pub type Result<T, E = ffi::kern_return_t> = core::result::Result<T, E>;

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
        get_offset(self.task)
    }

    pub fn read_at_into(&self, offset: usize, buf: &mut [u8]) -> Result<()> {
        // remove out?
        let mut out: u64 = 0;
        mach_vm_read_overwrite(self.task, offset as _, buf.len() as _, buf.as_mut_ptr() as _, &mut out)
    }
}
