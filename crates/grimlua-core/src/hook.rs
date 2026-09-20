//! The frame hook: the only place grimlua is allowed to touch the game.
//!
//! Grim Dawn exports `GameEngine::Update(int)`, which is the per-frame tick.
//! Hooking an exported *non-static member* function is what makes this project
//! tractable: on x64 the `this` pointer arrives in RCX, so the detour receives
//! the `GameEngine*` directly. No pointer chain, no signature scan, no offsets.
//!
//! Everything the runtime does to the game must happen on this thread. Game
//! engines are not thread-safe, and the web server and any workers will queue
//! work for this hook rather than calling in themselves.

use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::OnceLock;

use retour::GenericDetour;

use crate::log;
use crate::win;

/// `void GameEngine::Update(int)`.
///
/// On `x86_64-pc-windows-msvc` there is a single calling convention, so
/// `extern "C"` here *is* the convention MSVC compiled the member function
/// with: RCX = `this`, EDX = the int argument.
type UpdateFn = unsafe extern "C" fn(*mut c_void, i32);

pub const UPDATE_SYMBOL: &str = "?Update@GameEngine@GAME@@QEAAXH@Z";

static DETOUR: OnceLock<GenericDetour<UpdateFn>> = OnceLock::new();

/// The live `GameEngine*`, captured from the first tick. Zero until then.
static GAME_ENGINE: AtomicUsize = AtomicUsize::new(0);
static FRAMES: AtomicU64 = AtomicU64::new(0);

/// How often the hook samples and logs vitals. ~5s at 120fps -- slow, so the
/// probe output below stays readable during a discovery session.
const SAMPLE_FRAMES: u64 = 600;

/// How often the state probe runs. ~8Hz, fast enough to catch a menu opening
/// and closing as distinct events.
const PROBE_FRAMES: u64 = 15;

/// Whether the previous sample found a player, so that entering and leaving
/// the world logs one line each instead of one per sample.
static HAD_PLAYER: AtomicBool = AtomicBool::new(false);

/// Last value of the UI gate, encoded so it fits an atomic: 0 unknown,
/// 1 clear, 2 open.
static LAST_GATE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0xff);

fn encode_gate(v: Option<bool>) -> u8 {
    match v {
        None => 0,
        Some(false) => 1,
        Some(true) => 2,
    }
}

/// The `GameEngine*` captured from the frame hook, if a tick has run.
pub fn game_engine() -> Option<*mut c_void> {
    let ptr = GAME_ENGINE.load(Ordering::Acquire);
    (ptr != 0).then_some(ptr as *mut c_void)
}

pub fn frame_count() -> u64 {
    FRAMES.load(Ordering::Relaxed)
}

/// Runs on the game's own thread, once per frame. Keep it cheap and keep it
/// incapable of panicking -- the crate aborts on panic, and aborting here
/// takes the player's character with it.
unsafe extern "C" fn update_hook(this: *mut c_void, arg: i32) {
    GAME_ENGINE.store(this as usize, Ordering::Release);

    let n = FRAMES.fetch_add(1, Ordering::Relaxed) + 1;

    // Step 4 discovery scaffolding: report any GameEngine const getter whose
    // value changes, so opening a UI panel names its own flag. Remove once the
    // UI-open flag is identified.
    if n % PROBE_FRAMES == 0 {
        // Report UI-open transitions so the flag can be re-verified against
        // panels it was not originally tested on.
        let open = crate::state::any_ui_open(this);
        if LAST_GATE.swap(encode_gate(open), Ordering::Relaxed) != encode_gate(open) {
            log!("gate: any_ui_open -> {open:?}");
        }
    }

    if n == 1 {
        log!("frame hook live: GameEngine* = {this:p}");
    } else if n % SAMPLE_FRAMES == 0 {
        match crate::state::vitals(this) {
            Some(v) => {
                if !HAD_PLAYER.swap(true, Ordering::Relaxed) {
                    log!("player found");
                }
                        let gate = match crate::state::any_ui_open(this) {
                    Some(true) => "UI-OPEN",
                    Some(false) => "clear",
                    None => "UNKNOWN (fail closed)",
                };
                let pct = v.life_fraction().unwrap_or(0.0) * 100.0;
                log!(
                    "frame {n}  life {:.1}/{:.0} ({pct:.0}%)  mana {:.1}/{:.0}  gate {gate}",
                    v.life, v.life_max, v.mana, v.mana_max
                );
            }
            None => {
                if HAD_PLAYER.swap(false, Ordering::Relaxed) {
                    log!("frame {n}  no player (menu, loading, or death)");
                }
            }
        }
    }

    // Always call through. If the detour is somehow missing we would be
    // silently stopping the game's tick, which is far worse than doing
    // nothing, so this is the one place that must never be skipped.
    if let Some(detour) = DETOUR.get() {
        detour.call(this, arg);
    }
}

/// Resolve and install the frame hook.
///
/// Must not be called from `DllMain`: it allocates, patches code pages and
/// takes locks the loader may already hold.
pub fn install() -> bool {
    let Some(module) = win::module_base("Game.dll").map(|b| b as *mut c_void) else {
        log!("hook: Game.dll not loaded");
        return false;
    };
    let Some(target) = win::proc_address(module as _, UPDATE_SYMBOL) else {
        log!("hook: {UPDATE_SYMBOL} not found -- has the game been patched?");
        return false;
    };
    log!("hook: {UPDATE_SYMBOL} at {target:p} (Game.dll+{:#x})",
         target as usize - module as usize);

    let detour = unsafe {
        match GenericDetour::<UpdateFn>::new(std::mem::transmute(target), update_hook) {
            Ok(d) => d,
            Err(e) => {
                log!("hook: could not build detour: {e}");
                return false;
            }
        }
    };

    // Publish before enabling: the moment the detour is live the game may
    // enter update_hook, which needs DETOUR populated to call through.
    if DETOUR.set(detour).is_err() {
        log!("hook: already installed");
        return false;
    }
    if let Err(e) = unsafe { DETOUR.get().unwrap().enable() } {
        log!("hook: could not enable detour: {e}");
        return false;
    }

    log!("hook: installed, waiting for first tick");
    true
}
