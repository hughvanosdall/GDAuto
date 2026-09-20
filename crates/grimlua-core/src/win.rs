//! Thin, safe-ish wrappers over the few Win32 calls grimlua needs.

use std::ffi::{c_void, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;

use windows_sys::Win32::Foundation::HMODULE;
use windows_sys::Win32::System::LibraryLoader::{
    GetModuleFileNameW, GetModuleHandleW, GetProcAddress, LoadLibraryW,
};
use windows_sys::Win32::System::SystemInformation::GetSystemDirectoryW;

pub fn wide(s: &str) -> Vec<u16> {
    std::ffi::OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

/// Base address of an already-loaded module, or `None` if it is not loaded.
///
/// On Windows an `HMODULE` *is* the module's base address, which is what makes
/// `base + rva` work for cross-referencing against a disassembler.
pub fn module_base(name: &str) -> Option<usize> {
    let handle = unsafe { GetModuleHandleW(wide(name).as_ptr()) };
    (!handle.is_null()).then(|| handle as usize)
}

/// Call a `GetXxxW(buf, len) -> len` style API, growing the buffer as needed.
///
/// The buffer lives on the heap and starts small. An earlier version used a
/// 32,768-element stack array, which is a 64 KB stack frame -- taken inside
/// `DllMain`, on whichever of the game's threads happens to trigger the load,
/// whose stack may be far smaller than the 1 MB a main thread gets. A stack
/// overflow there kills the process with no error dialog and no crash dump.
fn query_path(mut fill: impl FnMut(&mut [u16]) -> u32) -> Option<PathBuf> {
    let mut buf = vec![0u16; 260];
    loop {
        let n = fill(&mut buf) as usize;
        if n == 0 {
            return None;
        }
        if n < buf.len() {
            return Some(PathBuf::from(OsString::from_wide(&buf[..n])));
        }
        if buf.len() >= 32_768 {
            return None; // longer than any real Windows path
        }
        buf.resize(buf.len() * 2, 0);
    }
}

pub fn module_path(handle: HMODULE) -> Option<PathBuf> {
    query_path(|buf| unsafe { GetModuleFileNameW(handle, buf.as_mut_ptr(), buf.len() as u32) })
}

pub fn system_directory() -> Option<PathBuf> {
    query_path(|buf| unsafe { GetSystemDirectoryW(buf.as_mut_ptr(), buf.len() as u32) })
}

/// # Safety
/// Loading a library runs its `DllMain`. Callers inside a `DllMain` of their
/// own are holding the loader lock; see the note in `grimlua-dinput8`.
pub unsafe fn load_library(path: &std::path::Path) -> Option<HMODULE> {
    let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    let handle = LoadLibraryW(wide.as_ptr());
    (!handle.is_null()).then_some(handle)
}

/// Resolve an export by its raw (still mangled, for C++) name.
///
/// This is the project's substitute for signature scanning: Grim Dawn ships
/// modding-enabled builds that export ~31,600 mangled C++ symbols, so the
/// mangled name is a stable handle that survives patches.
pub fn proc_address(module: HMODULE, name: &str) -> Option<*const c_void> {
    let mut bytes: Vec<u8> = name.bytes().collect();
    bytes.push(0);
    let addr = unsafe { GetProcAddress(module, bytes.as_ptr()) };
    addr.map(|f| f as *const c_void)
}
