//! Reading live game state by calling the game's own exported accessors.
//!
//! Every function here is a `const` member function in Grim Dawn's own code,
//! resolved by mangled export name. That means no struct layouts are reversed
//! and nothing is written: the worst a bug here can do is read a wrong number.
//!
//! **All of these must be called from the frame hook.** They run on game
//! objects that other threads are free to mutate.

use std::ffi::c_void;
use std::sync::OnceLock;

use crate::log;
use crate::win;

/// Resolve a set of exports into a struct of typed function pointers.
macro_rules! game_api {
    ($name:ident { $($field:ident : $ty:ty = $sym:literal),* $(,)? }) => {
        #[allow(non_snake_case)]
        pub struct $name { $(pub $field: $ty),* }

        impl $name {
            fn resolve() -> Option<Self> {
                let module = win::module_base("Game.dll")? as _;
                $(
                    let $field: $ty = match win::proc_address(module, $sym) {
                        Some(addr) => unsafe { std::mem::transmute(addr) },
                        None => {
                            log!("state: missing export {}", $sym);
                            return None;
                        }
                    };
                )*
                Some(Self { $($field),* })
            }
        }
    };
}

// Signatures come from demangling the export table; see symbols/ANCHORS.md.
// Note the widths: current life is a double, every other vital is a float.
// On x86_64-pc-windows-msvc `extern "C"` is the one calling convention MSVC
// used, with `this` in RCX.
game_api!(GameApi {
    get_main_player: unsafe extern "C" fn(*mut c_void) -> *mut c_void
        = "?GetMainPlayer@GameEngine@GAME@@QEBAPEAVPlayer@2@XZ",
    get_ui: unsafe extern "C" fn(*mut c_void) -> *mut c_void
        = "?GetUI@GameEngine@GAME@@QEBAPEAVGameUIInterface@2@XZ",
    is_transfer_open: unsafe extern "C" fn(*mut c_void) -> u8
        = "?IsTransferOpen@GameEngine@GAME@@QEBA_NXZ",
    get_current_life: unsafe extern "C" fn(*mut c_void) -> f64
        = "?GetCurrentLife@Character@GAME@@QEBA?BNXZ",
    get_life_limit: unsafe extern "C" fn(*mut c_void) -> f32
        = "?GetLifeLimit@Character@GAME@@QEBA?BMXZ",
    get_current_mana: unsafe extern "C" fn(*mut c_void) -> f32
        = "?GetCurrentMana@Character@GAME@@QEBA?BMXZ",
    get_mana_limit: unsafe extern "C" fn(*mut c_void) -> f32
        = "?GetManaLimit@Character@GAME@@QEBA?BMXZ",
});

static API: OnceLock<Option<GameApi>> = OnceLock::new();

pub fn api() -> Option<&'static GameApi> {
    API.get_or_init(GameApi::resolve).as_ref()
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vitals {
    pub life: f64,
    pub life_max: f32,
    pub mana: f32,
    pub mana_max: f32,
}

impl Vitals {
    /// Health as a 0..1 fraction, or `None` if the maximum is not yet sane.
    pub fn life_fraction(&self) -> Option<f32> {
        (self.life_max > 0.0).then(|| (self.life as f32 / self.life_max).clamp(0.0, 1.0))
    }

    /// Reject values that cannot be real, which is how a wrong signature or a
    /// bad `this` pointer shows up. Grim Dawn characters do not have negative
    /// or astronomically large health.
    fn plausible(&self) -> bool {
        let finite = self.life.is_finite()
            && self.life_max.is_finite()
            && self.mana.is_finite()
            && self.mana_max.is_finite();
        finite
            && (0.0..=1.0e9).contains(&self.life)
            && (0.0..=1.0e9).contains(&self.life_max)
            && self.mana >= 0.0
            && self.mana_max >= 0.0
    }
}

/// The main player's `Character*`, or `None` at the main menu and during load.
///
/// # Safety
/// `engine` must be the live `GameEngine*` captured by the frame hook, and
/// this must be called on the frame-hook thread.
pub unsafe fn main_player(engine: *mut c_void) -> Option<*mut c_void> {
    let api = api()?;
    if engine.is_null() {
        return None;
    }
    let player = (api.get_main_player)(engine);
    (!player.is_null()).then_some(player)
}

/// The game's UI object. Abstract with no exported members, so it is useful
/// only as a memory-scan anchor for now.
///
/// # Safety
/// As [`main_player`].
pub unsafe fn game_ui(engine: *mut c_void) -> Option<*mut c_void> {
    let api = api()?;
    if engine.is_null() {
        return None;
    }
    let ui = (api.get_ui)(engine);
    (!ui.is_null()).then_some(ui)
}

/// Byte inside the `GameUIInterface` object that is 1 while a UI panel is
/// open and 0 otherwise.
///
/// **This is the one thing in the project not resolved by name.** Nothing in
/// the export table reports panel visibility: `GameUIInterface` is abstract
/// with no exported members, and none of 128 probed `const` getters move when
/// the inventory or skill panels open. It was found instead by diffing the
/// interior of the UI object across panel toggles -- 1 for every panel
/// interaction observed, 0 across every resting sample, and the only bytes in
/// 24 KB of scanned memory to behave that way.
///
/// Being an offset rather than a name, it is the project's single
/// patch-fragile dependency, so it is pinned to the build it was found on and
/// refuses to answer on any other. See `symbols/ANCHORS.md`.
const UI_OPEN_OFFSET: usize = 0x1ef1;

/// `Game.dll` PE timestamp this offset was verified against (2026-08-19).
const UI_OPEN_VERIFIED_BUILD: u32 = 0x6A85_FBB3;

/// Whether any UI panel is open, or `None` if that cannot be determined.
///
/// `None` means the caller must **fail closed** and perform no action, per the
/// safety gate: a wrong answer here lets the runtime act while the player is
/// in a menu. It is returned when the game build differs from the one this
/// offset was verified against, when the UI object is unreachable, or when the
/// byte holds anything other than 0 or 1 -- all signs the layout has moved.
///
/// # Safety
/// As [`main_player`].
pub unsafe fn ui_panel_open(engine: *mut c_void) -> Option<bool> {
    if crate::game_module_build("Game.dll")? != UI_OPEN_VERIFIED_BUILD {
        return None;
    }
    let ui = game_ui(engine)?;
    match *(ui as *const u8).add(UI_OPEN_OFFSET) {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

/// Whether a stash, vendor or other transfer window is open.
///
/// Unlike [`ui_panel_open`] this one *is* an exported accessor, so it survives
/// patches. It covers only transfer-type panels, not the inventory or skills.
///
/// # Safety
/// As [`main_player`].
pub unsafe fn transfer_open(engine: *mut c_void) -> Option<bool> {
    let api = api()?;
    if engine.is_null() {
        return None;
    }
    Some((api.is_transfer_open)(engine) != 0)
}

/// The safety gate's view of the UI: true if anything is open, `None` if
/// unknown. Callers must treat `None` as "blocked".
///
/// # Safety
/// As [`main_player`].
pub unsafe fn any_ui_open(engine: *mut c_void) -> Option<bool> {
    let panel = ui_panel_open(engine)?;
    let transfer = transfer_open(engine)?;
    Some(panel || transfer)
}

/// Read the main player's vitals.
///
/// Returns `None` whenever there is no player or the numbers do not look
/// real -- failing closed rather than reporting a value we do not trust.
///
/// # Safety
/// As [`main_player`].
pub unsafe fn vitals(engine: *mut c_void) -> Option<Vitals> {
    let api = api()?;
    let player = main_player(engine)?;

    // Player derives from Character. With single inheritance and the base at
    // offset zero this pointer is directly usable as a Character*, which the
    // plausibility check below is what actually verifies.
    let v = Vitals {
        life: (api.get_current_life)(player),
        life_max: (api.get_life_limit)(player),
        mana: (api.get_current_mana)(player),
        mana_max: (api.get_mana_limit)(player),
    };
    v.plausible().then_some(v)
}
