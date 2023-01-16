#[allow(non_camel_case_types, non_snake_case, unused)]
pub mod ffi {
    pub type natural_t = ::core::ffi::c_uint;
    pub type integer_t = ::core::ffi::c_int;
    pub type mach_port_t = ::core::ffi::c_uint;
    pub type mach_port_name_t = natural_t;
    pub type vm_task_entry_t = mach_port_t;
    pub type mach_vm_address_t = u64;
    pub type mach_vm_size_t = u64;
    pub type vm_region_flavor_t = ::core::ffi::c_int;
    pub type vm_region_info_t = *mut ::core::ffi::c_int;
    pub type mach_msg_type_number_t = natural_t;
    pub type kern_return_t = ::core::ffi::c_int;
    pub type vm_region_extended_info_data_t = vm_region_extended_info;
    pub type vm_prot_t = ::core::ffi::c_int;
    pub type task_name_t = mach_port_t;
    pub type task_flavor_t = natural_t;
    pub type task_info_t = *mut integer_t;
    pub type vm_map_t = mach_port_t;
    pub type vm_offset_t = usize;
    pub type dyld_image_mode = ::core::ffi::c_uint;
    pub type darwin_size_t = ::core::ffi::c_ulong;
    pub type darwin_uuid_t = [::core::ffi::c_uchar; 16usize];
    pub type libc_size_t = usize;
    pub type cpu_type_t = integer_t;
    pub type cpu_subtype_t = integer_t;

    pub const VM_PROT_READ: vm_prot_t = 1;
    pub const VM_PROT_WRITE: vm_prot_t = 1 << 1;
    pub const VM_PROT_EXECUTE: vm_prot_t = 1 << 2;

    // https://github.com/apple-oss-distributions/xnu/blob/5c2921b07a2480ab43ec66f5b9e41cb872bc554f/bsd/sys/proc_info.h#L826
    pub const PROC_PIDPATHINFO_MAXSIZE: u32 = 4096;

    pub const VM_REGION_EXTENDED_INFO: vm_region_flavor_t = 13;
    pub const MACH_PORT_NULL: mach_port_t = 0;
    pub const TASK_DYLD_INFO: ::core::ffi::c_uint = 17;

    // TODO enum all error code
    // https://github.com/apple-oss-distributions/xnu/blob/5c2921b07a2480ab43ec66f5b9e41cb872bc554f/osfmk/mach/kern_return.h#L72
    pub const KERN_SUCCESS: kern_return_t = 0;

    #[repr(C)]
    pub struct vm_region_extended_info {
        pub protection: vm_prot_t,
        pub user_tag: ::core::ffi::c_uint,
        pub pages_resident: ::core::ffi::c_uint,
        pub pages_shared_now_private: ::core::ffi::c_uint,
        pub pages_swapped_out: ::core::ffi::c_uint,
        pub pages_dirtied: ::core::ffi::c_uint,
        pub ref_count: ::core::ffi::c_uint,
        pub shadow_depth: ::core::ffi::c_ushort,
        pub external_pager: ::core::ffi::c_uchar,
        pub share_mode: ::core::ffi::c_uchar,
        pub pages_reusable: ::core::ffi::c_uint,
    }

    #[repr(C, packed(4))]
    pub struct task_dyld_info {
        pub all_image_info_addr: mach_vm_address_t,
        pub all_image_info_size: mach_vm_size_t,
        pub all_image_info_format: integer_t,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct segment_command_64 {
        pub cmd: u32,
        pub cmdsize: u32,
        pub segname: [::core::ffi::c_char; 16_usize],
        pub vmaddr: u64,
        pub vmsize: u64,
        pub fileoff: u64,
        pub filesize: u64,
        pub maxprot: vm_prot_t,
        pub initprot: vm_prot_t,
        pub nsects: u32,
        pub flags: u32,
    }

    #[repr(C)]
    pub struct mach_header {
        pub _address: u8,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct dyld_image_info {
        pub imageLoadAddress: *mut mach_header,
        pub imageFilePath: *const ::core::ffi::c_char,
        pub imageFileModDate: usize,
    }

    #[repr(C)]
    pub struct dyld_uuid_info {
        pub imageLoadAddress: *const mach_header,
        pub imageUUID: darwin_uuid_t,
    }

    #[repr(C)]
    pub struct dyld_aot_image_info {
        pub x86LoadAddress: *const mach_header,
        pub aotLoadAddress: *const mach_header,
        pub aotImageSize: u64,
        pub aotImageKey: [u8; 32usize],
    }

    pub type dyld_image_notifier = ::core::option::Option<
        unsafe extern "C" fn(mode: dyld_image_mode, infoCount: u32, info: *const dyld_image_info),
    >;

    #[repr(C)]
    pub struct dyld_all_image_infos {
        pub version: u32,
        pub infoArrayCount: u32,
        pub infoArray: *const dyld_image_info,
        pub notification: dyld_image_notifier,
        pub processDetachedFromSharedRegion: bool,
        pub libSystemInitialized: bool,
        pub dyldImageLoadAddress: *const mach_header,
        pub jitInfo: *mut ::core::ffi::c_void,
        pub dyldVersion: *const ::core::ffi::c_char,
        pub errorMessage: *const ::core::ffi::c_char,
        pub terminationFlags: usize,
        pub coreSymbolicationShmPage: *mut ::core::ffi::c_void,
        pub systemOrderFlag: usize,
        pub uuidArrayCount: usize,
        pub uuidArray: *const dyld_uuid_info,
        pub dyldAllImageInfosAddress: *mut dyld_all_image_infos,
        pub initialImageCount: usize,
        pub errorKind: usize,
        pub errorClientOfDylibPath: *const ::core::ffi::c_char,
        pub errorTargetDylibPath: *const ::core::ffi::c_char,
        pub errorSymbol: *const ::core::ffi::c_char,
        pub sharedCacheSlide: usize,
        pub sharedCacheUUID: [u8; 16usize],
        pub sharedCacheBaseAddress: usize,
        pub infoArrayChangeTimestamp: u64,
        pub dyldPath: *const ::core::ffi::c_char,
        pub notifyPorts: [mach_port_t; 8usize],
        pub reserved: [usize; 7usize],
        pub sharedCacheFSID: u64,
        pub sharedCacheFSObjID: u64,
        pub compact_dyld_image_info_addr: usize,
        pub compact_dyld_image_info_size: darwin_size_t,
        pub platform: u32,
        pub aotInfoCount: u32,
        pub aotInfoArray: *const dyld_aot_image_info,
        pub aotInfoArrayChangeTimestamp: u64,
        pub aotSharedCacheBaseAddress: usize,
        pub aotSharedCacheUUID: [u8; 16usize],
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct mach_header_64 {
        pub magic: u32,
        pub cputype: cpu_type_t,
        pub cpusubtype: cpu_subtype_t,
        pub filetype: u32,
        pub ncmds: u32,
        pub sizeofcmds: u32,
        pub flags: u32,
        pub reserved: u32,
    }

    extern "C" {
        static mach_task_self_: mach_port_t;
        pub fn task_for_pid(
            target_tport: mach_port_name_t,
            pid: ::core::ffi::c_int,
            tn: *mut mach_port_name_t,
        ) -> kern_return_t;
        pub fn proc_regionfilename(
            pid: ::core::ffi::c_int,
            address: u64,
            buffer: *mut ::core::ffi::c_void,
            buffersize: u32,
        ) -> ::core::ffi::c_int;
        pub fn mach_vm_region(
            target_task: vm_task_entry_t,
            address: *mut mach_vm_address_t,
            size: *mut mach_vm_size_t,
            flavor: vm_region_flavor_t,
            info: vm_region_info_t,
            info_cnt: *mut mach_msg_type_number_t,
            object_name: *mut mach_port_t,
        ) -> kern_return_t;
        pub fn task_info(
            target_task: task_name_t,
            flavor: task_flavor_t,
            task_info_out: task_info_t,
            task_info_outCnt: *mut mach_msg_type_number_t,
        ) -> kern_return_t;
        pub fn mach_vm_read_overwrite(
            target_task: vm_task_entry_t,
            address: mach_vm_address_t,
            size: mach_vm_size_t,
            data: mach_vm_address_t,
            outsize: *mut mach_vm_size_t,
        ) -> kern_return_t;
        pub fn mach_vm_write(
            target_task: vm_map_t,
            address: mach_vm_address_t,
            data: vm_offset_t,
            dataCnt: mach_msg_type_number_t,
        ) -> kern_return_t;
        pub fn strlen(cs: *const core::ffi::c_char) -> libc_size_t;
    }

    #[allow(clippy::missing_safety_doc)] // FIXME
    pub unsafe fn mach_task_self() -> mach_port_t {
        mach_task_self_
    }
}

use core::mem;

use self::ffi::{
    dyld_all_image_infos, dyld_image_info, kern_return_t, mach_header_64, mach_vm_address_t, mach_vm_size_t,
    segment_command_64, task_dyld_info,
};

// macos 64 bit only
pub fn mach_vm_read_overwrite(
    target_task: ffi::vm_task_entry_t,
    address: ffi::mach_vm_address_t,
    size: ffi::mach_vm_size_t,
    data: ffi::mach_vm_address_t,
    outsize: *mut ffi::mach_vm_size_t,
) -> Result<(), ffi::kern_return_t> {
    let result: ffi::kern_return_t =
        unsafe { ffi::mach_vm_read_overwrite(target_task, address, size, data, outsize) };
    if result != ffi::KERN_SUCCESS {
        return Err(result);
    }
    Ok(())
}

pub fn mach_vm_write(task: ffi::vm_task_entry_t, addr: u64, buf: &[u8]) -> Result<(), ffi::kern_return_t> {
    let result: ffi::kern_return_t =
        unsafe { ffi::mach_vm_write(task, addr, buf.as_ptr() as _, buf.len() as _) };
    if result != ffi::KERN_SUCCESS {
        return Err(result);
    }
    Ok(())
}

pub fn task_for_pid(pid: i32) -> Result<ffi::mach_port_name_t, ffi::kern_return_t> {
    let mut task: ffi::mach_port_name_t = ffi::MACH_PORT_NULL;
    let result = unsafe { ffi::task_for_pid(ffi::mach_task_self(), pid, &mut task) };
    if result != ffi::KERN_SUCCESS {
        return Err(result);
    }

    Ok(task)
}

pub fn proc_regionfilename(pid: i32, address: u64) -> Option<Vec<u8>> {
    let mut buf: Vec<u8> = Vec::with_capacity((ffi::PROC_PIDPATHINFO_MAXSIZE - 1) as _);
    let ptr = buf.as_mut_ptr() as *mut ::core::ffi::c_void;
    let size = buf.capacity() as u32;

    let result: ffi::kern_return_t = unsafe { ffi::proc_regionfilename(pid, address, ptr, size) };

    if result <= 0 {
        None
    } else {
        unsafe {
            buf.set_len(result as _);
        }
        Some(buf)
    }
}

pub fn task_dyld_info(task: ffi::mach_port_name_t) -> Result<ffi::task_dyld_info, ffi::kern_return_t> {
    const TASK_DYLD_INFO_COUNT: ffi::mach_msg_type_number_t = (mem::size_of::<ffi::task_dyld_info>()
        / mem::size_of::<ffi::natural_t>())
        as ffi::mach_msg_type_number_t;

    let mut count = TASK_DYLD_INFO_COUNT;
    let mut dyld_info = unsafe { mem::zeroed::<ffi::task_dyld_info>() };
    let result: ffi::kern_return_t = unsafe {
        ffi::task_info(
            task,
            ffi::TASK_DYLD_INFO,
            &mut dyld_info as *mut ffi::task_dyld_info as ffi::task_info_t,
            &mut count,
        )
    };

    if result != ffi::KERN_SUCCESS {
        Err(result)
    } else {
        Ok(dyld_info)
    }
}

pub fn mach_vm_region(
    target_task: ffi::vm_task_entry_t,
    address: &mut ffi::mach_vm_address_t,
    size: &mut ffi::mach_vm_size_t,
    flavor: ffi::vm_region_flavor_t,
    info: ffi::vm_region_info_t,
    info_cnt: &mut ffi::mach_msg_type_number_t,
    object_name: &mut ffi::mach_port_t,
) -> ffi::kern_return_t {
    unsafe { ffi::mach_vm_region(target_task, address, size, flavor, info, info_cnt, object_name) }
}

// TODO Return similar linux-style maps? `start-end perm pathname/dyld`
#[allow(unused)]
pub struct Map {
    pub(crate) addr: ffi::mach_vm_address_t,
    pub(crate) size: ffi::mach_vm_size_t,
    pub(crate) count: ffi::mach_msg_type_number_t,
    pub(crate) info: ffi::vm_region_extended_info,
}

pub struct MapIter {
    task: ffi::vm_task_entry_t,
    addr: ffi::mach_vm_address_t,
}

impl MapIter {
    pub const fn new(task: ffi::mach_port_name_t) -> Self {
        Self { task, addr: 1 }
    }
}

impl Default for MapIter {
    fn default() -> Self {
        Self { task: unsafe { ffi::mach_task_self() }, addr: 1 }
    }
}

// TODO Maybe shouldnot use iterator?
impl Iterator for MapIter {
    type Item = Map;

    fn next(&mut self) -> Option<Self::Item> {
        let mut count = mem::size_of::<ffi::vm_region_extended_info_data_t>() as ffi::mach_msg_type_number_t;
        let mut object_name: ffi::mach_port_t = 0;
        let mut size = unsafe { mem::zeroed::<ffi::mach_vm_size_t>() };
        let mut info = unsafe { mem::zeroed::<ffi::vm_region_extended_info_data_t>() };
        let result: ffi::kern_return_t = mach_vm_region(
            self.task,
            &mut self.addr,
            &mut size,
            ffi::VM_REGION_EXTENDED_INFO,
            &mut info as *mut ffi::vm_region_extended_info_data_t as ffi::vm_region_info_t,
            &mut count,
            &mut object_name,
        );

        if result != ffi::KERN_SUCCESS {
            return None;
        }
        let region = Map { addr: self.addr, size, count, info };
        self.addr += region.size;
        Some(region)
    }
}

impl Map {
    pub const fn start(&self) -> usize {
        self.addr as _
    }

    pub const fn end(&self) -> usize {
        (self.addr + self.size) as _
    }

    pub const fn size(&self) -> usize {
        self.size as _
    }

    pub const fn is_read(&self) -> bool {
        self.info.protection & ffi::VM_PROT_READ != 0
    }

    pub const fn is_write(&self) -> bool {
        self.info.protection & ffi::VM_PROT_WRITE != 0
    }

    pub const fn is_exec(&self) -> bool {
        self.info.protection & ffi::VM_PROT_EXECUTE != 0
    }

    pub const fn tag(&self) -> u32 {
        self.info.user_tag
    }
}

pub struct DyldInfo {
    pub filename: String,
    pub address: usize,
    pub file_mod_date: usize,
    pub segment: segment_command_64,
}

pub fn get_dyld_all_image_infos(
    task: ffi::mach_port_name_t,
    dyld_info: task_dyld_info,
) -> Result<dyld_all_image_infos, kern_return_t> {
    let mut image_infos = unsafe { core::mem::zeroed::<ffi::dyld_all_image_infos>() };
    let mut read_len = std::mem::size_of_val(&image_infos) as mach_vm_size_t;

    mach_vm_read_overwrite(
        task,
        dyld_info.all_image_info_addr,
        read_len,
        (&mut image_infos) as *mut dyld_all_image_infos as mach_vm_address_t,
        &mut read_len,
    )?;
    Ok(image_infos)
}

pub fn get_dyld_image_info(
    task: ffi::mach_port_name_t,
    image_infos: dyld_all_image_infos,
) -> Result<Vec<dyld_image_info>, kern_return_t> {
    let mut modules =
        vec![unsafe { core::mem::zeroed::<ffi::dyld_image_info>() }; image_infos.infoArrayCount as usize];
    let mut read_len =
        (core::mem::size_of::<dyld_image_info>() * image_infos.infoArrayCount as usize) as mach_vm_size_t;
    mach_vm_read_overwrite(
        task,
        image_infos.infoArray as mach_vm_address_t,
        read_len,
        modules.as_mut_ptr() as mach_vm_address_t,
        &mut read_len,
    )?;
    Ok(modules)
}

pub fn get_image_filename(
    task: ffi::mach_port_name_t,
    module: dyld_image_info,
) -> Result<[i8; 512], kern_return_t> {
    let mut read_len = 512_u64;
    let mut image_filename = [0_i8; 512];
    mach_vm_read_overwrite(
        task,
        module.imageFilePath as mach_vm_address_t,
        read_len,
        image_filename.as_mut_ptr() as mach_vm_address_t,
        &mut read_len,
    )?;
    Ok(image_filename)
}

pub fn get_mach_header_64(
    task: ffi::mach_port_name_t,
    module: dyld_image_info,
) -> Result<mach_header_64, kern_return_t> {
    let mut header = unsafe { core::mem::zeroed::<ffi::mach_header_64>() };
    let mut read_len = std::mem::size_of_val(&header) as mach_vm_size_t;

    mach_vm_read_overwrite(
        task,
        module.imageLoadAddress as u64,
        read_len,
        (&mut header) as *mut mach_header_64 as mach_vm_address_t,
        &mut read_len,
    )?;
    Ok(header)
}

pub fn get_commands_buffer(
    task: ffi::mach_port_name_t,
    module: dyld_image_info,
    header: mach_header_64,
) -> Result<Vec<i8>, kern_return_t> {
    let mut commands_buffer = vec![0_i8; header.sizeofcmds as usize];
    let mut read_len = mach_vm_size_t::from(header.sizeofcmds);
    mach_vm_read_overwrite(
        task,
        (module.imageLoadAddress as usize + core::mem::size_of_val(&header)) as mach_vm_size_t,
        read_len,
        commands_buffer.as_mut_ptr() as mach_vm_address_t,
        &mut read_len,
    )?;
    Ok(commands_buffer)
}

pub fn get_offset(task: ffi::mach_port_name_t) -> Result<u64, kern_return_t> {
    let dyld_info = task_dyld_info(task)?;
    let image_infos = get_dyld_all_image_infos(task, dyld_info)?;
    let modules = get_dyld_image_info(task, image_infos)?;

    for module in modules {
        let header = get_mach_header_64(task, module)?;
        let commands_buffer = get_commands_buffer(task, module, header)?;
        let mut offset: u32 = 0;
        let mut slide: u64 = 0;
        for _ in 0..header.ncmds {
            unsafe {
                let command =
                    *(commands_buffer.as_ptr().offset(offset as isize) as *const segment_command_64);
                if command.cmd == 0x19 && command.segname[0..7] == [95, 95, 84, 69, 88, 84, 0] {
                    slide = module.imageLoadAddress as u64 - command.vmaddr;
                    break;
                }
                offset += command.cmdsize;
            }
        }

        let mut offset: u32 = 0;
        for _ in 0..header.ncmds {
            unsafe {
                let mut command =
                    *(commands_buffer.as_ptr().offset(offset as isize) as *const segment_command_64);
                if command.cmd == 0x19 {
                    command.vmaddr += slide;
                    return Ok(command.vmaddr);
                }
                offset += command.cmdsize;
            }
        }
    }
    Err(0)
}
