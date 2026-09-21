//! A global hotkey that opens the web UI in the default browser.
//!
//! Ctrl+Shift+G. This is the stand-in for an in-game escape-menu button,
//! which needs the engine's UI record loader and command dispatcher, neither
//! of which is exported. A hotkey costs thirty lines and, as CLAUDE.md
//! observes, is what people actually use after the first week anyway.

use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, MOD_CONTROL, MOD_NOREPEAT, MOD_SHIFT,
};
use windows_sys::Win32::UI::Shell::ShellExecuteW;
use windows_sys::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG, SW_SHOWNORMAL, WM_HOTKEY};

use crate::log;
use crate::server::PORT;
use crate::win::wide;

const HOTKEY_ID: i32 = 0x6C75; // "lu"
const VK_G: u32 = 0x47;

/// Register the hotkey and pump its messages. Blocks, so it owns its thread.
pub fn run() {
    // A null HWND delivers WM_HOTKEY to this thread's queue, so no window
    // class has to be registered inside someone else's process.
    let ok = unsafe {
        RegisterHotKey(
            std::ptr::null_mut(),
            HOTKEY_ID,
            MOD_CONTROL | MOD_SHIFT | MOD_NOREPEAT,
            VK_G,
        )
    };
    if ok == 0 {
        log!("hotkey: Ctrl+Shift+G is already taken by another program");
        return;
    }
    log!("hotkey: Ctrl+Shift+G opens the UI");

    let mut msg: MSG = unsafe { std::mem::zeroed() };
    loop {
        let got = unsafe { GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) };
        if got <= 0 {
            break;
        }
        if msg.message == WM_HOTKEY && msg.wParam as i32 == HOTKEY_ID {
            open_ui();
        }
    }
}

pub fn open_ui() {
    let url = wide(&format!("http://127.0.0.1:{PORT}/"));
    let verb = wide("open");
    let result = unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            verb.as_ptr(),
            url.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            SW_SHOWNORMAL as i32,
        )
    };
    // ShellExecuteW returns <= 32 on failure, as an HINSTANCE-shaped int.
    if (result as isize) <= 32 {
        log!("hotkey: could not open a browser (code {})", result as isize);
    }
}
