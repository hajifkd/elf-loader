use std;
use std::fs::File;
use std::io::{Seek, SeekFrom, Read};
use std::mem;

/// Copied from https://github.com/quininer/memsec
/// Prot enum.
#[cfg(unix)]
#[allow(non_snake_case, non_upper_case_globals, unused)]
pub mod Prot {
    pub use libc::c_int as Ty;

    pub const NoAccess: Ty = ::libc::PROT_NONE;
    pub const ReadOnly: Ty = ::libc::PROT_READ;
    pub const WriteOnly: Ty = ::libc::PROT_WRITE;
    pub const ReadWrite: Ty = (::libc::PROT_READ | ::libc::PROT_WRITE);
    pub const Execute: Ty = ::libc::PROT_EXEC;
    pub const ReadExec: Ty = (::libc::PROT_READ | ::libc::PROT_EXEC);
    pub const WriteExec: Ty = (::libc::PROT_WRITE | ::libc::PROT_EXEC);
    pub const ReadWriteExec: Ty = (::libc::PROT_READ | ::libc::PROT_WRITE | ::libc::PROT_EXEC);
}

/// Copied from https://github.com/quininer/memsec
/// Prot enum.
#[cfg(windows)]
#[allow(non_snake_case, non_upper_case_globals, unused)]
pub mod Prot {
    pub use winapi::shared::minwindef::DWORD as Ty;

    pub const NoAccess: Ty = ::winapi::um::winnt::PAGE_NOACCESS;
    pub const ReadOnly: Ty = ::winapi::um::winnt::PAGE_READONLY;
    pub const ReadWrite: Ty = ::winapi::um::winnt::PAGE_READWRITE;
    pub const WriteCopy: Ty = ::winapi::um::winnt::PAGE_WRITECOPY;
    pub const Execute: Ty = ::winapi::um::winnt::PAGE_EXECUTE;
    pub const ReadExec: Ty = ::winapi::um::winnt::PAGE_EXECUTE_READ;
    pub const ReadWriteExec: Ty = ::winapi::um::winnt::PAGE_EXECUTE_READWRITE;
    pub const WriteCopyExec: Ty = ::winapi::um::winnt::PAGE_EXECUTE_WRITECOPY;
    pub const Guard: Ty = ::winapi::um::winnt::PAGE_GUARD;
    pub const NoCache: Ty = ::winapi::um::winnt::PAGE_NOCACHE;
    pub const WriteCombine: Ty = ::winapi::um::winnt::PAGE_WRITECOMBINE;
    pub const RevertToFileMap: Ty = ::winapi::um::winnt::PAGE_REVERT_TO_FILE_MAP;
    pub const TargetsInvalid: Ty = ::winapi::um::winnt::PAGE_TARGETS_INVALID;
    pub const TargetsNoUpdate: Ty = ::winapi::um::winnt::PAGE_TARGETS_NO_UPDATE;
}

/// Unix `mprotect`.
/// Copied from https://github.com/quininer/memsec
#[cfg(unix)]
#[inline]
pub unsafe fn mprotect(ptr: *mut u8, len: usize, prot: Prot::Ty) -> bool {
    ::libc::mprotect(ptr as *mut ::libc::c_void, len, prot as ::libc::c_int) == 0
}

/// Windows `VirtualProtect`.
/// Copied from https://github.com/quininer/memsec
#[cfg(windows)]
#[inline]
pub unsafe fn mprotect(ptr: *mut u8, len: usize, prot: Prot::Ty) -> bool {
    let mut old = mem::uninitialized();
    ::winapi::um::memoryapi::VirtualProtect(
        ptr as ::winapi::shared::minwindef::LPVOID,
        len as ::winapi::shared::basetsd::SIZE_T,
        prot as ::winapi::shared::minwindef::DWORD,
        &mut old as ::winapi::shared::minwindef::PDWORD,
    ) != 0
}

pub unsafe fn read_into_ptr(
    file: &mut File,
    ptr: *mut u8,
    size: usize,
    offset: u64,
) -> Result<(), std::io::Error> {
    let slice = std::slice::from_raw_parts_mut(ptr, size);
    file.seek(SeekFrom::Start(offset))?;
    file.read_exact(slice)?;

    Ok(())
}
