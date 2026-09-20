//! In-process memory diffing, anchored to pointers we can reach by name.
//!
//! Build-order step 4 needs a "UI panel is open" flag, and the export table
//! has no such accessor: `GameUIInterface` is abstract with no exported
//! members, and none of the 128 probed `const` getters move when the
//! inventory or skill panels open.
//!
//! This does what Cheat Engine does -- find a byte that flips when the user
//! opens a panel -- but scoped to the interior of objects we already hold
//! pointers to. That matters: Cheat Engine yields an absolute address, which
//! dies at the next patch, whereas a hit here is inherently
//! `<named anchor> + offset`, which is as stable as the object layout and
//! fits the same resolve-by-name strategy as everything else.
//!
//! Diagnostic scaffolding. Once the flag is found this is switched off and
//! the offset moves into `state.rs`.

use std::ffi::c_void;
use std::sync::Mutex;
use std::sync::OnceLock;

use windows_sys::Win32::System::Memory::{
    VirtualQuery, MEMORY_BASIC_INFORMATION, MEM_COMMIT, PAGE_GUARD, PAGE_NOACCESS,
};

use crate::log;

/// How far into each object to look. Large enough to cover a sizeable object,
/// small enough to stay cheap at 8Hz.
const WINDOW: usize = 0x2000;

/// A byte that changes more often than this is a timer or a counter, not a
/// panel flag, and is dropped from the candidate set permanently.
const CHURN_LIMIT: u16 = 40;

/// Offsets under active investigation, reported on every change regardless of
/// churn, and summarised periodically. Candidates for the step-4 UI-open flag:
/// these four bytes inside the UI object moved in lockstep, to the same
/// millisecond, as panels were opened and closed.
const WATCH: &[(&str, usize)] = &[
    ("ui", 0x17a9),
    ("ui", 0x1ef1),
    ("ui", 0x1ef4),
    ("ui", 0x1fb8),
];

struct Anchor {
    name: &'static str,
    prev: Vec<u8>,
    churn: Vec<u16>,
    primed: bool,
}

impl Anchor {
    fn new(name: &'static str) -> Self {
        Self { name, prev: vec![0; WINDOW], churn: vec![0; WINDOW], primed: false }
    }
}

static ANCHORS: OnceLock<[Mutex<Anchor>; 3]> = OnceLock::new();

fn anchors() -> &'static [Mutex<Anchor>; 3] {
    ANCHORS.get_or_init(|| {
        [
            Mutex::new(Anchor::new("engine")),
            Mutex::new(Anchor::new("ui")),
            Mutex::new(Anchor::new("player")),
        ]
    })
}

/// How many bytes starting at `base` are safely readable, up to `want`.
///
/// Objects are smaller than our window, so the tail of a read would run off
/// into unmapped or guarded pages. `VirtualQuery` bounds it instead of
/// trusting luck.
fn readable_len(base: *const u8, want: usize) -> usize {
    if base.is_null() {
        return 0;
    }
    let mut mbi: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
    let n = unsafe {
        VirtualQuery(
            base as *const c_void,
            &mut mbi,
            std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
        )
    };
    if n == 0 || mbi.State != MEM_COMMIT {
        return 0;
    }
    if mbi.Protect & (PAGE_NOACCESS | PAGE_GUARD) != 0 {
        return 0;
    }
    let end = mbi.BaseAddress as usize + mbi.RegionSize;
    want.min(end.saturating_sub(base as usize))
}

/// Diff one object against its previous snapshot, reporting quiet bytes that
/// just moved.
///
/// # Safety
/// `base` must point at a live object and this must run on the frame hook.
unsafe fn scan_one(slot: &Mutex<Anchor>, base: *mut c_void) {
    let Ok(mut a) = slot.lock() else { return };
    let len = readable_len(base as *const u8, WINDOW);
    if len == 0 {
        return;
    }
    let cur = std::slice::from_raw_parts(base as *const u8, len);

    if !a.primed {
        a.prev[..len].copy_from_slice(cur);
        a.primed = true;
        log!("scan: {} anchored at {base:p}, watching {len:#x} bytes", a.name);
        return;
    }

    for i in 0..len {
        let (old, new) = (a.prev[i], cur[i]);
        if old == new {
            continue;
        }
        a.prev[i] = new;
        if WATCH.iter().any(|(n, o)| *n == a.name && *o == i) {
            log!("WATCH {}+{i:#06x}  {old} -> {new}", a.name);
            continue;
        }
        if a.churn[i] >= CHURN_LIMIT {
            continue; // known-noisy byte
        }
        a.churn[i] += 1;
        // A panel flag is a boolean, so restrict reporting to bytes that look
        // like one. This is what keeps the log readable.
        if old <= 1 && new <= 1 {
            log!("scan: {}+{i:#06x}  {old} -> {new}", a.name);
        }
    }
}

/// Log the current value of every watched byte, so a run starts from a known
/// baseline rather than only reporting edges.
///
/// # Safety
/// As [`poll`].
pub unsafe fn report_watch(ui: *mut c_void) {
    if readable_len(ui as *const u8, WINDOW) < WINDOW {
        return;
    }
    let bytes = std::slice::from_raw_parts(ui as *const u8, WINDOW);
    let mut line = String::from("WATCH state ");
    for (name, off) in WATCH {
        if *name == "ui" {
            line.push_str(&format!("{name}+{off:#06x}={} ", bytes[*off]));
        }
    }
    log!("{line}");
}

/// Snapshot and diff all three anchors.
///
/// # Safety
/// `engine` must be the live `GameEngine*`, on the frame-hook thread.
pub unsafe fn poll(engine: *mut c_void, ui: *mut c_void, player: *mut c_void) {
    let a = anchors();
    scan_one(&a[0], engine);
    scan_one(&a[1], ui);
    scan_one(&a[2], player);
}
