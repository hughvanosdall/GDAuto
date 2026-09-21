//! A tracer for the game's own skill calls.
//!
//! We are inside the process, so the cheapest way to learn how Grim Dawn casts
//! a skill is to watch it do it: detour the candidate entry points, log what
//! the *game* passes when the player presses a key, and call through. No
//! disassembler, and no guessing at argument meanings — the arguments arrive.
//!
//! This exists because casting is the first thing the export table could not
//! answer. `Character::ActivateSkill` does nothing when we call it,
//! `Character::StartSkill` refuses, and the player-facing entry points live on
//! `ControllerPlayer`, which nothing exported returns.
//!
//! ## Why every signature here is the same
//!
//! Each hook is declared as four integer arguments returning one, which is not
//! what any of these functions really looks like. It does not need to be. On
//! x64 the first four arguments arrive in RCX, RDX, R8 and R9 whatever their
//! declared types, so this reads the registers and hands them straight back.
//! The point of a trace is to find out *which* function runs and what the first
//! few values are; reconstructing an exact prototype for every candidate before
//! knowing whether it is even on the path would be the wrong way round.
//!
//! The cost: arguments beyond the fourth live on the stack and are not seen,
//! and a function returning a float would have its result misread. None of
//! these return a float, and the values that matter — `this` and the skill id —
//! are all in registers.
//!
//! ## The prize
//!
//! Hooking a non-static member function hands us `this` in RCX. So the moment
//! the game calls anything on `ControllerPlayer`, **we have the
//! `ControllerPlayer*`** that cannot be reached by name — the same trick that
//! made the project tractable when `GameEngine::Update` gave us the
//! `GameEngine*`.
//!
//! Every detour is log-and-pass-through. Nothing is altered, nothing is
//! swallowed, and each falls silent after [`MAX_LOGGED`] calls so a held key
//! cannot fill the disk.

use std::ffi::c_void;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::OnceLock;

use retour::GenericDetour;

use crate::log;
use crate::win;

/// Calls of each function written to the log before it goes quiet.
const MAX_LOGGED: u32 = 16;

/// The uniform shape every trace hook wears. See the module note.
type TraceFn = unsafe extern "C" fn(u64, u64, u64, u64) -> u64;

/// A `ControllerPlayer*` seen by a trace hook, if one has been.
static CONTROLLER_PLAYER: AtomicUsize = AtomicUsize::new(0);

pub fn controller_player() -> Option<*mut c_void> {
    let ptr = CONTROLLER_PLAYER.load(Ordering::Acquire);
    (ptr != 0).then_some(ptr as *mut c_void)
}

struct Traced {
    detour: GenericDetour<TraceFn>,
    logged: AtomicU32,
    label: &'static str,
    /// True when `this` is a `ControllerPlayer`, so RCX is worth keeping.
    is_controller: bool,
}

impl Traced {
    fn should_log(&self) -> bool {
        let n = self.logged.fetch_add(1, Ordering::Relaxed);
        if n == MAX_LOGGED {
            log!("trace: {} is quietening down after {MAX_LOGGED} calls", self.label);
        }
        n < MAX_LOGGED
    }
}

/// Render a register as a number, or as a pointer to floats when it looks like
/// one — which is how a `WorldVec3` argument gives itself away.
///
/// # Safety
/// Dereferences anything that looks like a user-space pointer, so it must only
/// run on values the game just passed as arguments.
unsafe fn describe(value: u64) -> String {
    if value < 0x1_0000 {
        return format!("{value}");
    }
    if value < 0x7fff_ffff_ffff {
        let floats: Vec<String> = (0..3)
            .map(|i| {
                let v = std::ptr::read_unaligned((value as *const f32).add(i));
                if v.is_finite() && v != 0.0 && v.abs() > 1.0e-4 && v.abs() < 1.0e7 {
                    format!("{v:.1}")
                } else {
                    "-".into()
                }
            })
            .collect();
        if floats.iter().any(|f| f != "-") {
            return format!("{value:#x}(f32 {})", floats.join(","));
        }
    }
    format!("{value:#x}")
}

/// Generate a hook and its slot. One pair per traced function, because each
/// needs its own static to find its own trampoline.
macro_rules! tracer {
    ($slot:ident, $hook:ident) => {
        static $slot: OnceLock<Traced> = OnceLock::new();

        unsafe extern "C" fn $hook(a: u64, b: u64, c: u64, d: u64) -> u64 {
            let traced = match $slot.get() {
                Some(t) => t,
                None => return 0,
            };
            if traced.is_controller && a != 0 {
                CONTROLLER_PLAYER.store(a as usize, Ordering::Release);
            }

            let result = traced.detour.call(a, b, c, d);

            // Logging allocates and formats, so it is fenced off completely.
            let _ = catch_unwind(AssertUnwindSafe(|| {
                if traced.should_log() {
                    log!(
                        "trace {}: this={a:#x} | {} | {} | {} -> {}",
                        traced.label,
                        describe(b),
                        describe(c),
                        describe(d),
                        result as u32
                    );
                }
            }));
            result
        }
    };
}

tracer!(T0, hook0);
tracer!(T1, hook1);
tracer!(T2, hook2);
tracer!(T3, hook3);
tracer!(T4, hook4);
tracer!(T5, hook5);
tracer!(T6, hook6);
tracer!(T7, hook7);

fn install_one(
    slot: &'static OnceLock<Traced>,
    hook: TraceFn,
    label: &'static str,
    symbol: &str,
    is_controller: bool,
) -> bool {
    let Some(module) = win::module_base("Game.dll") else {
        log!("trace: Game.dll is not loaded, so {label} cannot be hooked");
        return false;
    };
    let Some(target) = win::proc_address(module as _, symbol) else {
        log!("trace: Game.dll has no export for {label}");
        return false;
    };

    unsafe {
        let detour = match GenericDetour::<TraceFn>::new(std::mem::transmute(target), hook) {
            Ok(d) => d,
            Err(e) => {
                log!("trace: could not hook {label}: {e}");
                return false;
            }
        };
        if slot
            .set(Traced { detour, logged: AtomicU32::new(0), label, is_controller })
            .is_err()
        {
            log!("trace: {label} already installed");
            return false;
        }
        match slot.get().unwrap().detour.enable() {
            Ok(()) => {
                // Logged per hook, deliberately: a trace that produces nothing
                // only means something if it is known which hooks were live.
                log!("trace: hooked {label} at {target:p}");
                true
            }
            Err(e) => {
                log!("trace: could not enable {label}: {e}");
                false
            }
        }
    }
}

/// Watch every plausible way a skill cast could reach the engine.
///
/// The first pass hooked only the `Character` and `ControllerPlayer` skill
/// entry points, and pressing a skill key in a live game fired **none** of
/// them. So this net is deliberately wider and starts from the keypress itself
/// — `PlayerHotSlotCtrl::ActivateHotSlot`, which is where the bar's keybind
/// arrives — and follows every candidate down from there.
pub fn install() -> bool {
    log!("trace: watching the game's own skill calls");

    // (slot, hook, label, symbol, `this` is a ControllerPlayer)
    let targets: [(&'static OnceLock<Traced>, TraceFn, &'static str, &str, bool); 8] = [
        // Where a keypress on the skill bar arrives. If nothing else fires,
        // this should, and it anchors the rest of the path.
        (&T0, hook0, "ActivateHotSlot",
         "?ActivateHotSlot@PlayerHotSlotCtrl@GAME@@QEAAXI_N0@Z", false),
        // The controller's own skill entry points.
        (&T1, hook1, "InstantSkillAction",
         "?InstantSkillAction@ControllerPlayer@GAME@@QEAA_NAEAVCharacter@2@IAEBVWorldVec3@2@AEAI@Z", true),
        (&T2, hook2, "SendSkillAction",
         "?SendSkillAction@ControllerPlayer@GAME@@QEAA_NAEAVCharacter@2@_N1IAEBVWorldVec3@2@AEAI@Z", true),
        (&T3, hook3, "HandleActionFromMouse",
         "?HandleActionFromMouse@ControllerPlayer@GAME@@QEAA_N_N000AEBVWorldVec3@2@AEAIPEA_N@Z", true),
        // The controller state machine, which is what InstantSkillAction
        // dispatches into.
        (&T4, hook4, "DefaultRequestInstantSkillAction",
         "?DefaultRequestInstantSkillAction@ControllerPlayerState@GAME@@IEAA_NIAEBVWorldVec3@2@AEAI@Z", false),
        (&T5, hook5, "RequestInstantSkillAction(Idle)",
         "?RequestInstantSkillAction@ControllerPlayerStateIdle@GAME@@MEAA_NIAEBVWorldVec3@2@AEAI@Z", false),
        // And the two on Character already suspected.
        (&T6, hook6, "Character::StartSkill",
         "?StartSkill@Character@GAME@@QEAA?B_NIIAEBVWorldVec3@2@IAEBUTargetLeadingData@2@@Z", false),
        (&T7, hook7, "Character::ActivateSkill",
         "?ActivateSkill@Character@GAME@@QEAAXIAEBVName@2@IAEBVWorldVec3@2@@Z", false),
    ];

    let mut live = 0;
    for (slot, hook, label, symbol, is_controller) in targets {
        if install_one(slot, hook, label, symbol, is_controller) {
            live += 1;
        }
    }

    log!("trace: {live} of 8 hooks live");
    live > 0
}
