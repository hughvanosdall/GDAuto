//! grimlua core: everything that is independent of how the DLL got loaded.
//!
//! The proxy crate (`grimlua-dinput8`) is a shim that calls [`init`] and
//! forwards its exports. Keeping all behaviour here means an alternative
//! loader -- a `CrashReport.dll` proxy, or a launcher that injects -- is a new
//! sibling crate rather than a rewrite.
//!
//! Build-order status (see CLAUDE.md): step 1 (proxy loads) is confirmed in
//! game. Step 2 (frame hook) is implemented in [`hook`] and installed from a
//! deferred worker below.

pub mod hook;
pub mod log;
pub mod probe;
mod probe_table;
pub mod scan;
pub mod state;
pub mod win;

use std::path::PathBuf;
use windows_sys::Win32::Foundation::HMODULE;

/// Modules whose exports the runtime resolves symbols from.
pub const GAME_MODULES: [&str; 3] = ["Game.dll", "Engine.dll", "Widget.dll"];

/// PE `TimeDateStamp` of each module as harvested into `symbols/symbols.json`.
///
/// Logged against the running game so a stale symbol index is obvious in the
/// first few lines of the log rather than as a mystery crash later.
/// Regenerate both together with `python tools/dump_exports.py`.
pub const HARVESTED_TIMESTAMPS: [(&str, u32); 3] = [
    ("Game.dll", 0x6A85_FBB3),
    ("Engine.dll", 0x6A85_FB5B),
    ("Widget.dll", 0x6A85_FB61),
];

/// Read a loaded module's PE `TimeDateStamp` straight out of its mapped headers.
fn pe_timestamp(base: usize) -> Option<u32> {
    if base == 0 {
        return None;
    }
    unsafe {
        let dos = base as *const u8;
        if std::ptr::read_unaligned(dos as *const u16) != 0x5A4D {
            return None; // "MZ"
        }
        let lfanew = std::ptr::read_unaligned(dos.add(0x3C) as *const i32);
        if !(0..0x1000).contains(&lfanew) {
            return None;
        }
        let nt = dos.offset(lfanew as isize);
        if std::ptr::read_unaligned(nt as *const u32) != 0x0000_4550 {
            return None; // "PE\0\0"
        }
        Some(std::ptr::read_unaligned(nt.add(8) as *const u32))
    }
}

/// Called once from the loader shim's `DLL_PROCESS_ATTACH`.
///
/// Does the minimum that is legal under the loader lock: works out where to
/// log, writes a banner, and returns. No threads, no game calls, no hooks.
pub fn init(self_module: HMODULE) {
    let self_path = win::module_path(self_module);
    let dir = self_path
        .as_ref()
        .and_then(|p| p.parent())
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    log::init(&dir);

    log!("grimlua {} loaded", env!("CARGO_PKG_VERSION"));
    if let Some(path) = &self_path {
        log!("  self      {}", path.display());
    }
    if let Some(exe) = win::module_path(std::ptr::null_mut()) {
        log!("  host      {}", exe.display());
    }

    for name in GAME_MODULES {
        match win::module_base(name) {
            Some(base) => {
                let stamp = pe_timestamp(base).unwrap_or(0);
                let expected = HARVESTED_TIMESTAMPS
                    .iter()
                    .find(|(n, _)| *n == name)
                    .map(|(_, t)| *t);
                let note = match expected {
                    Some(want) if want == stamp => "matches symbol index",
                    Some(_) => "!! DIFFERS from symbol index -- rerun tools/dump_exports.py",
                    None => "",
                };
                log!("  {name:<11} base {base:#018x}  build {stamp:#010x}  {note}");
            }
            // Expected during a very early load: the proxy can attach before
            // the game's own modules are mapped.
            None => log!("  {name:<11} not yet loaded"),
        }
    }

    spawn_deferred_init();
}

/// Install the frame hook off the loader's back.
///
/// Hooking from `DllMain` would patch code pages and allocate while the loader
/// lock is held. A thread created inside `DllMain` does not begin executing
/// until that lock is released, so this is the standard safe deferral. The
/// extra delay lets the game finish its own start-up first.
fn spawn_deferred_init() {
    let spawned = std::thread::Builder::new()
        .name("grimlua-init".into())
        .spawn(|| {
            std::thread::sleep(std::time::Duration::from_secs(3));
            hook::install();
        });
    if spawned.is_err() {
        log!("could not spawn deferred init thread; no hook installed");
    }
}

/// The PE `TimeDateStamp` of a loaded game module, used to refuse
/// build-specific knowledge (memory offsets) on an unrecognised build.
pub fn game_module_build(name: &str) -> Option<u32> {
    pe_timestamp(win::module_base(name)?)
}

/// Called from `DLL_PROCESS_DETACH`.
pub fn shutdown() {
    log!("grimlua unloading");
}
