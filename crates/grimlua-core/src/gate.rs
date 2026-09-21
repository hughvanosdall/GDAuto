//! The safety gate: one veto point for every action.
//!
//! This is the whole justification for the project existing rather than a
//! keyboard macro. Rules cannot opt out of it per-call, because the rules are
//! not what is trusted -- the host is. Everything funnels through
//! [`evaluate`], and only [`Gate::Clear`] permits an action.
//!
//! It fails closed. Any state it cannot establish is a refusal, not a
//! shrug: an unresolvable UI flag blocks every action rather than letting the
//! runtime act blind while the player is in a menu.

use std::ffi::c_void;
use std::time::{Duration, Instant};

use windows_sys::Win32::System::Threading::GetCurrentProcessId;
use windows_sys::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

use crate::shared::{config, Gate};
use crate::state;

/// Whether the foreground window belongs to this process.
///
/// Comparing process ids rather than a stored HWND means it keeps working
/// across the game's own window recreation, and needs nothing captured at
/// startup.
fn game_has_focus() -> bool {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() {
            return false;
        }
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, &mut pid);
        pid != 0 && pid == GetCurrentProcessId()
    }
}

/// Decide whether an action may be taken this tick.
///
/// `last_action` is when something last fired, for the global cooldown.
///
/// # Safety
/// `engine` must be the live `GameEngine*`, on the frame-hook thread.
pub unsafe fn evaluate(engine: *mut c_void, last_action: Option<Instant>) -> Gate {
    let cfg = config();

    if !cfg.armed {
        return Gate::Disarmed;
    }
    if state::main_player(engine).is_none() {
        return Gate::NoPlayer;
    }

    // Fail closed: `None` means the UI flag could not be trusted on this
    // build, so nothing may happen at all.
    match state::any_ui_open(engine) {
        None => return Gate::Unknown,
        Some(true) => return Gate::UiOpen,
        Some(false) => {}
    }

    if !game_has_focus() {
        return Gate::Unfocused;
    }
    if let Some(t) = last_action {
        if t.elapsed() < Duration::from_millis(cfg.global_cooldown_ms) {
            return Gate::Cooldown;
        }
    }
    Gate::Clear
}
