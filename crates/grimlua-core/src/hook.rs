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
use std::panic::{catch_unwind, AssertUnwindSafe};
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

/// The `GameEngine*` captured from the frame hook, if a tick has run.
pub fn game_engine() -> Option<*mut c_void> {
    let ptr = GAME_ENGINE.load(Ordering::Acquire);
    (ptr != 0).then_some(ptr as *mut c_void)
}

pub fn frame_count() -> u64 {
    FRAMES.load(Ordering::Relaxed)
}

/// Set once a panic has escaped the runtime. grimlua then does nothing at all
/// for the rest of the session: a bug in our tick is exactly the situation in
/// which we should stop touching someone's hardcore character.
static WEDGED: AtomicBool = AtomicBool::new(false);

pub fn wedged() -> bool {
    WEDGED.load(Ordering::Relaxed)
}

/// Runs on the game's own thread, once per frame.
///
/// **This is the boundary.** Grim Dawn calls in through an `extern "C"`
/// function pointer, so a panic that reached this frame would be undefined
/// behaviour at best and an aborted process at worst. Everything grimlua does
/// per frame therefore happens inside a `catch_unwind`, and the call through
/// to the game happens outside it — so even a panic in our own code leaves the
/// game's tick intact.
unsafe extern "C" fn update_hook(this: *mut c_void, arg: i32) {
    GAME_ENGINE.store(this as usize, Ordering::Release);

    let n = FRAMES.fetch_add(1, Ordering::Relaxed) + 1;

    // The runtime reads state, consults the gate and performs at most one
    // action. Everything it does happens here, on the game's own thread.
    let ours = catch_unwind(AssertUnwindSafe(|| {
        if n == 1 {
            log!("frame hook live: GameEngine* = {this:p}");
        }
        if n % crate::runtime::TICK_FRAMES == 0 && !wedged() {
            crate::runtime::tick(this, n);
        }
    }));

    if ours.is_err() && !WEDGED.swap(true, Ordering::Relaxed) {
        // Logged once. Disarm as well, so that if the flag above were ever
        // cleared there is still nothing to do.
        let mut cfg = crate::shared::config();
        cfg.armed = false;
        crate::shared::put_config(cfg);
        log!("frame hook: panicked -- grimlua is now inert for this session");
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
