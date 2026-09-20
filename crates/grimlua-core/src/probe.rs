//! Discovering named game state by calling the game's own accessors.
//!
//! Build-order step 4 needs a "is any UI panel open" flag, and no export is
//! named that outright. Rather than reach for a disassembler, we exploit being
//! *inside* the process with a working frame hook: call every zero-argument
//! `const` getter on `GameEngine` each tick, and report only the ones that
//! change. Open the escape menu, and whatever flips is the answer -- by
//! exported name, which survives patches, rather than by address, which does
//! not.
//!
//! This is diagnostic scaffolding, not runtime machinery. Once a flag is
//! identified it moves into `state.rs` and the probe is switched off.

use std::ffi::c_void;
use std::sync::OnceLock;

use crate::log;
use crate::probe_table::{PROBES_CHARACTER, PROBES_ENGINE};
use crate::win;

#[derive(Clone, Copy, PartialEq)]
pub enum Kind {
    /// MSVC returns `bool` in AL, so only the low byte is meaningful.
    Bool,
    /// Enums are int-sized here.
    Enum,
}

pub struct Probe {
    pub label: &'static str,
    pub symbol: &'static str,
    pub kind: Kind,
}

type BoolFn = unsafe extern "C" fn(*mut c_void) -> u8;
type EnumFn = unsafe extern "C" fn(*mut c_void) -> i32;

struct Bound {
    probe: &'static Probe,
    func: usize,
    last: i32,
    seen: bool,
}

type Group = Vec<std::sync::Mutex<Bound>>;

static ENGINE: OnceLock<Option<Group>> = OnceLock::new();
static CHARACTER: OnceLock<Option<Group>> = OnceLock::new();

fn bind_group(table: &'static [Probe], what: &str) -> Option<Group> {
    let module = win::module_base("Game.dll")? as _;
    let mut out = Vec::with_capacity(table.len());
    let mut missing = 0;
    for probe in table {
        match win::proc_address(module, probe.symbol) {
            Some(addr) => out.push(std::sync::Mutex::new(Bound {
                probe,
                func: addr as usize,
                last: 0,
                seen: false,
            })),
            None => missing += 1,
        }
    }
    log!("probe: bound {} {what} getters ({missing} missing)", out.len());
    Some(out)
}

fn bind_engine() -> Option<Group> {
    bind_group(PROBES_ENGINE, "engine")
}

fn bind_character() -> Option<Group> {
    bind_group(PROBES_CHARACTER, "character")
}

/// Call one group against one `this` pointer, logging only changes.
unsafe fn poll_group(group: &Group, this: *mut c_void) {
    if this.is_null() {
        return;
    }
    for cell in group {
        let Ok(mut b) = cell.lock() else { continue };
        let value = match b.probe.kind {
            Kind::Bool => {
                let f: BoolFn = std::mem::transmute(b.func);
                f(this) as i32
            }
            Kind::Enum => {
                let f: EnumFn = std::mem::transmute(b.func);
                f(this)
            }
        };
        if !b.seen {
            b.seen = true;
            b.last = value;
            log!("probe: {:<42} = {value}  (initial)", b.probe.label);
        } else if value != b.last {
            log!("probe: {:<42} {} -> {value}", b.probe.label, b.last);
            b.last = value;
        }
    }
}

/// Probe both groups and log anything that changed.
///
/// # Safety
/// `engine` must be the live `GameEngine*` and this must run on the frame-hook
/// thread. Every probe is a `const` member function, so the worst outcome of a
/// wrong binding is a meaningless number in the log.
pub unsafe fn poll(engine: *mut c_void) {
    if let Some(group) = ENGINE.get_or_init(bind_engine) {
        poll_group(group, engine);
    }
    // Character probes need the player, which does not exist in menus.
    if let Some(group) = CHARACTER.get_or_init(bind_character) {
        if let Some(player) = crate::state::main_player(engine) {
            poll_group(group, player);
        }
    }
}
