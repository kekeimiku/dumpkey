#![allow(non_camel_case_types, non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
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
    }

    #[allow(clippy::missing_safety_doc)] // FIXME
    pub unsafe fn mach_task_self() -> mach_port_t {
        mach_task_self_
    }
}

use core::mem;

use self::ffi::{
    dyld_all_image_infos, dyld_image_info, kern_return_t, mach_header_64, mach_vm_address_t, mach_vm_size_t,
    segment_command_64,
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

pub fn task_for_pid(pid: i32) -> Result<ffi::mach_port_name_t, ffi::kern_return_t> {
    let mut task: ffi::mach_port_name_t = ffi::MACH_PORT_NULL;
    let result = unsafe { ffi::task_for_pid(ffi::mach_task_self(), pid, &mut task) };
    if result != ffi::KERN_SUCCESS {
        return Err(result);
    }

    Ok(task)
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

pub fn get_offset(task: ffi::mach_port_name_t) -> Result<u64, kern_return_t> {
    const TASK_DYLD_INFO_COUNT: ffi::mach_msg_type_number_t = (mem::size_of::<ffi::task_dyld_info>()
        / mem::size_of::<ffi::natural_t>())
        as ffi::mach_msg_type_number_t;

    let mut count = TASK_DYLD_INFO_COUNT;
    let mut dyld_info = unsafe { mem::zeroed::<ffi::task_dyld_info>() };
    let result: ffi::kern_return_t = unsafe {
        ffi::task_info(task, ffi::TASK_DYLD_INFO, &mut dyld_info as *mut ffi::task_dyld_info as _, &mut count)
    };

    if result != ffi::KERN_SUCCESS {
        return Err(result);
    }

    let mut image_infos = unsafe { core::mem::zeroed::<ffi::dyld_all_image_infos>() };
    let mut read_len = std::mem::size_of_val(&image_infos) as mach_vm_size_t;

    mach_vm_read_overwrite(
        task,
        dyld_info.all_image_info_addr,
        read_len,
        &mut image_infos as *mut dyld_all_image_infos as _,
        &mut read_len,
    )?;

    // 只需要第一个
    let mut module = unsafe { core::mem::zeroed::<ffi::dyld_image_info>() };
    let mut read_len = (core::mem::size_of::<dyld_image_info>()) as mach_vm_size_t;
    mach_vm_read_overwrite(
        task,
        image_infos.infoArray as mach_vm_address_t,
        read_len,
        &mut module as *mut ffi::dyld_image_info as _,
        &mut read_len,
    )?;

    let mut header = unsafe { core::mem::zeroed::<ffi::mach_header_64>() };
    let mut read_len = std::mem::size_of_val(&header) as mach_vm_size_t;
    mach_vm_read_overwrite(
        task,
        module.imageLoadAddress as u64,
        read_len,
        &mut header as *mut mach_header_64 as mach_vm_address_t,
        &mut read_len,
    )?;

    let mut commands_buffer = vec![0_i8; header.sizeofcmds as usize];
    let mut read_len = mach_vm_size_t::from(header.sizeofcmds);
    mach_vm_read_overwrite(
        task,
        (module.imageLoadAddress as usize + core::mem::size_of_val(&header)) as _,
        read_len,
        commands_buffer.as_mut_ptr() as mach_vm_address_t,
        &mut read_len,
    )?;

    let command = unsafe { *(commands_buffer.as_ptr() as *const segment_command_64) };
    let offset = module.imageLoadAddress as u64
        - unsafe { *(commands_buffer.as_ptr().offset(command.cmdsize as _) as *const segment_command_64) }
            .vmaddr
        + command.vmaddr;

    Ok(offset)
}
