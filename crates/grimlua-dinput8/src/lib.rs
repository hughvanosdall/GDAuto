//! `dinput8.dll` proxy: the shim that gets grimlua into Grim Dawn's process.
//!
//! Grim Dawn's own `DirectInput.dll` imports `DirectInput8Create` from
//! `DINPUT8.dll`, and no `dinput8.dll` ships in the game folder, so a
//! `dinput8.dll` dropped next to the executable wins the loader's
//! application-directory-first search. Install is one file copy and uninstall
//! is one delete, with nothing shipped renamed.
//!
//! (`version.dll`, the other name CLAUDE.md suggested, does **not** work here:
//! nothing in the process dependency closure imports it.)
//!
//! ## Why runtime thunks rather than linker forwarders
//!
//! The tidy approach would be `/EXPORT:Name=dinput8.Name`, but a forwarder
//! string names a *module*, which the loader resolves by the usual rules --
//! and the module named `dinput8` is us. It would forward to itself. So each
//! export is a naked `jmp` through a pointer filled in at attach time from the
//! real System32 copy. The thunks never touch arguments, so they work for any
//! signature and any of these functions' calling conventions.
//!
//! ## The `minimal` feature
//!
//! `--features minimal` builds a pure passthrough: no logging, no file I/O,
//! no Rust `std` on the `DllMain` path at all. It exists to answer one
//! question when the game misbehaves -- is the proxy *slot* viable, or is our
//! own initialisation at fault? Everything else is held constant between the
//! two builds.

use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};

use windows_sys::Win32::Foundation::{BOOL, HMODULE};
use windows_sys::Win32::System::LibraryLoader::{
    DisableThreadLibraryCalls, GetProcAddress, LoadLibraryExW,
};
use windows_sys::Win32::System::SystemInformation::GetSystemDirectoryW;
use windows_sys::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

/// Our own module handle, recorded at attach so we can prove we never
/// resolve the "real" dinput8 back to ourselves.
static SELF_MODULE: AtomicUsize = AtomicUsize::new(0);

fn module_self() -> HMODULE {
    SELF_MODULE.load(Ordering::Acquire) as HMODULE
}

/// Logging that compiles away entirely in the `minimal` build.
macro_rules! note {
    ($($arg:tt)*) => {
        #[cfg(feature = "full")]
        { grimlua_core::log::write(&format!($($arg)*)); }
    };
}

macro_rules! proxy_exports {
    ($($name:ident => $slot:ident),* $(,)?) => {
        $(
            static $slot: AtomicUsize = AtomicUsize::new(0);

            /// Naked tail-jump to the real export. Arguments stay untouched in
            /// their registers and on the stack, so the declared signature
            /// below is irrelevant to correctness.
            #[unsafe(naked)]
            #[no_mangle]
            pub unsafe extern "system" fn $name() {
                core::arch::naked_asm!(
                    "jmp qword ptr [rip + {slot}]",
                    slot = sym $slot,
                )
            }
        )*

        /// Point every thunk at the real implementation.
        ///
        /// Uses a NUL-terminated byte literal per name so the `minimal` build
        /// needs no allocation.
        unsafe fn bind_thunks(real: HMODULE) {
            $(
                let name = concat!(stringify!($name), "\0");
                match GetProcAddress(real, name.as_ptr()) {
                    Some(addr) => $slot.store(addr as usize, Ordering::Release),
                    None => { note!("  WARN      unresolved export {}", stringify!($name)); }
                }
            )*
        }
    };
}

proxy_exports! {
    DirectInput8Create  => REAL_DIRECT_INPUT8_CREATE,
    DllCanUnloadNow     => REAL_DLL_CAN_UNLOAD_NOW,
    DllGetClassObject   => REAL_DLL_GET_CLASS_OBJECT,
    DllRegisterServer   => REAL_DLL_REGISTER_SERVER,
    DllUnregisterServer => REAL_DLL_UNREGISTER_SERVER,
    GetdfDIJoystick     => REAL_GETDF_DI_JOYSTICK,
}

/// Load the genuine System32 `dinput8.dll` and point the thunks at it.
///
/// **The path must be fully qualified.** Given a bare `"dinput8.dll"` the
/// loader first checks whether a module of that name is already loaded and
/// returns it -- and that module is *us*. Every thunk then points at itself
/// and the first call spins until the stack dies. `LOAD_LIBRARY_SEARCH_SYSTEM32`
/// does not help, because the already-loaded check happens before any search.
/// A full path is matched against loaded modules by resolved path instead, so
/// the System32 copy loads alongside ours as a distinct module.
///
/// The handle is intentionally never freed: it must outlive every call the
/// game makes through us.
fn attach_real() -> bool {
    // "<system32>\dinput8.dll" built in a fixed buffer: no allocation, so the
    // minimal build stays free of the CRT's heap.
    const LEAF: &[u16] = &[
        b'\\' as u16, b'd' as u16, b'i' as u16, b'n' as u16, b'p' as u16, b'u' as u16,
        b't' as u16, b'8' as u16, b'.' as u16, b'd' as u16, b'l' as u16, b'l' as u16, 0,
    ];
    let mut path = [0u16; 320];
    let n = unsafe { GetSystemDirectoryW(path.as_mut_ptr(), (path.len() - LEAF.len()) as u32) }
        as usize;
    if n == 0 || n > path.len() - LEAF.len() {
        note!("FATAL: GetSystemDirectoryW failed");
        return false;
    }
    path[n..n + LEAF.len()].copy_from_slice(LEAF);

    // Loading a library from DllMain runs under the loader lock. It is
    // tolerable only because this specific target is a system DLL whose own
    // dependencies (kernel32, user32, ole32) are already mapped by the time
    // Grim Dawn asks for DirectInput. Nothing else may be loaded here.
    let real = unsafe { LoadLibraryExW(path.as_ptr(), std::ptr::null_mut(), 0) };
    if real.is_null() {
        note!("FATAL: could not load System32 dinput8.dll");
        return false;
    }

    // Guard against ever resolving back into ourselves again.
    if real == module_self() {
        note!("FATAL: resolved dinput8.dll to ourselves");
        return false;
    }

    unsafe { bind_thunks(real) };

    // Only DirectInput8Create is actually imported by Grim Dawn; the rest are
    // forwarded for correctness, and their absence is not fatal.
    if REAL_DIRECT_INPUT8_CREATE.load(Ordering::Acquire) == 0 {
        note!("FATAL: DirectInput8Create did not resolve");
        return false;
    }
    note!("  proxy     forwarding to System32\\dinput8.dll");
    true
}

#[no_mangle]
pub extern "system" fn DllMain(module: HMODULE, reason: u32, _reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            // We never act on thread attach/detach, and Grim Dawn makes many
            // threads. Opting out removes that per-thread loader work.
            SELF_MODULE.store(module as usize, Ordering::Release);
            unsafe { DisableThreadLibraryCalls(module) };

            #[cfg(feature = "full")]
            grimlua_core::init(module);

            if !attach_real() {
                // Fail the load outright rather than leaving the game holding
                // a dinput8 that cannot forward. It breaks either way; this
                // way the log names the cause.
                return 0;
            }
            note!("proxy ready");
        }
        DLL_PROCESS_DETACH => {
            #[cfg(feature = "full")]
            grimlua_core::shutdown();
        }
        _ => {}
    }
    1
}
