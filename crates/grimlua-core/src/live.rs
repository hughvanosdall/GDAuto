//! The live skill and combat layer: what the character actually has, what it
//! is doing, and casting a skill directly.
//!
//! ## Why this is not the hot bar
//!
//! An earlier version pressed skill-bar slots. That was wrong, and it threw
//! away the only real advantage of being inside the process: going through a
//! slot means the skill must be bound to the bar, the binding must be known,
//! and a priority list has to name a bar position instead of a skill. Here a
//! rule names a *skill*, the skill id is resolved by record path, and the game
//! is asked to cast it — independent of the bar and of the player's input
//! configuration entirely.
//!
//! ## One symbol must not take unrelated features with it
//!
//! A resolve block is all-or-nothing: if any symbol in it is missing, the whole
//! struct is `None`. The first version of this module put every symbol in one
//! block, and the consequence showed up immediately in a live game --
//! `Name::Name()` turned out to live in **Engine.dll** while the block only
//! searched Game.dll, so one unresolvable constructor silently blanked the
//! skill list, the combat flags *and* the DPS readout, none of which have
//! anything to do with it.
//!
//! So the symbols are grouped by the feature they serve, each resolving on its
//! own, and each carries the module it belongs to. Losing `ActivateSkill` now
//! costs casting and nothing else.

use std::ffi::c_void;
use std::sync::OnceLock;

use crate::abi;
use crate::log;
use crate::state;
use crate::win;

/// Resolve a set of exports into typed function pointers, independently of any
/// other block. Mirrors `state::game_api!` but yields `None` for the whole
/// group rather than for the process.
macro_rules! live_api {
    ($name:ident { $($field:ident : $ty:ty = ($module:literal, $sym:literal)),* $(,)? }) => {
        #[allow(non_snake_case)]
        pub struct $name { $(pub $field: $ty),* }

        impl $name {
            fn resolve() -> Option<Self> {
                $(
                    let base = match win::module_base($module) {
                        Some(b) => b as _,
                        None => {
                            log!("live: {} is not loaded (for {})", $module, $sym);
                            return None;
                        }
                    };
                    let $field: $ty = match win::proc_address(base, $sym) {
                        Some(addr) => unsafe { std::mem::transmute(addr) },
                        None => {
                            log!("live: {} has no export {}", $module, $sym);
                            return None;
                        }
                    };
                )*
                Some(Self { $($field),* })
            }
        }
    };
}

// Everything needed to read and cast the character's own skills.
live_api!(SkillApi {
    // Character -> SkillManager. The player's own skill state lives here.
    get_skill_manager: unsafe extern "C" fn(*mut c_void) -> *mut c_void
        = ("Game.dll", "?GetSkillManager@Character@GAME@@QEAAAEAVSkillManager@2@XZ"),
    // const mem::vector<Skill*>& -- every skill the character has.
    get_skill_list: unsafe extern "C" fn(*mut c_void) -> *const c_void
        = ("Game.dll", "?GetSkillList@SkillManager@GAME@@QEBAAEBV?$vector@PEAVSkill@GAME@@@mem@@XZ"),
    // Record path -> the runtime skill id a cast needs.
    find_skill_id: unsafe extern "C" fn(*mut c_void, *const u8) -> u32
        = ("Game.dll", "?FindSkillId@SkillManager@GAME@@QEBA?BIPEBD@Z"),
    // The game's own "can this be cast right now": level, cooldown, energy.
    is_skill_valid_for_use: unsafe extern "C" fn(*mut c_void, u32, u8) -> u8
        = ("Game.dll", "?IsSkillValidForUse@SkillManager@GAME@@QEBA_NI_N@Z"),

    // Per-skill, on a Skill* from the list above. All non-virtual and const.
    skill_display_name_tag: unsafe extern "C" fn(*const c_void) -> *const c_void
        = ("Game.dll", "?GetDisplayNameTag@Skill@GAME@@QEBAAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@XZ"),
    skill_cooldown_remaining: unsafe extern "C" fn(*const c_void) -> i32
        = ("Game.dll", "?GetCooldownRemaining@Skill@GAME@@QEBAHXZ"),
    skill_level: unsafe extern "C" fn(*const c_void) -> u32
        = ("Game.dll", "?GetSkillLevel@Skill@GAME@@QEBA?BIXZ"),
});

// Combat and targeting. Independent of the skill layer: knowing whether you
// are fighting should not depend on being able to enumerate your skills.
live_api!(CombatApi {
    get_current_attack_target: unsafe extern "C" fn(*mut c_void, *mut u32, *mut c_void, *mut u32)
        = ("Game.dll", "?GetCurrentAttackTarget@Character@GAME@@QEAAXAEAIAEAVWorldVec3@2@0@Z"),
    is_under_attack: unsafe extern "C" fn(*const c_void) -> u8
        = ("Game.dll", "?IsUnderAttack@Character@GAME@@QEBA?B_NXZ"),
    last_attack_time: unsafe extern "C" fn(*const c_void) -> i32
        = ("Game.dll", "?GetLastAttackTime@Character@GAME@@QEBA?BHXZ"),
    is_moving: unsafe extern "C" fn(*const c_void) -> u8
        = ("Game.dll", "?IsMoving@Character@GAME@@QEBA_NXZ"),
    is_attack_moving: unsafe extern "C" fn(*const c_void) -> u8
        = ("Game.dll", "?IsAttackMoving@Character@GAME@@QEBA_NXZ"),
});

// One number for the dashboard, on its own so it can never cost anything else.
live_api!(DpsApi {
    // Writes through a float reference. The trailing unsigned is undecoded --
    // probably a damage-type or target filter -- and is passed as zero.
    calculate_dps: unsafe extern "C" fn(*const c_void, *mut f32, u32) -> ()
        = ("Game.dll", "?CalculateDps@Player@GAME@@QEBAXAEAMI@Z"),
});

// The character sheet's numbers. `CharAttributeType` is an enum whose values
// are not recoverable from the export table, so the index is discovered by
// probing rather than assumed -- see `probe_attributes`.
live_api!(AttributeApi {
    get_base_char_attribute: unsafe extern "C" fn(*const c_void, u32) -> f32
        = ("Game.dll", "?GetBaseCharAttribute@Character@GAME@@QEBAMW4CharAttributeType@2@@Z"),
});

// Casting, kept apart from reading: if this one ever fails to resolve, the
// editor still shows a correct picture of a character it cannot act for.
live_api!(CastApi {
    // void ActivateSkill(unsigned, Name const&, unsigned, WorldVec3 const&)
    //
    // Returns nothing, which is exactly the problem: it reported success by
    // saying nothing while doing nothing.
    activate_skill: unsafe extern "C" fn(*mut c_void, u32, *const c_void, u32, *const c_void)
        = ("Game.dll", "?ActivateSkill@Character@GAME@@QEAAXIAEBVName@2@IAEBVWorldVec3@2@@Z"),

    // bool StartSkill(unsigned, unsigned, WorldVec3 const&, unsigned,
    //                 TargetLeadingData const&)
    //
    // The same operation one level up, and it returns a bool -- so unlike
    // `ActivateSkill` it can tell us whether the game accepted the call at all.
    // That makes it the better thing to try while the argument meanings are
    // still unknown.
    start_skill: unsafe extern "C" fn(*mut c_void, u32, u32, *const c_void, u32, *const c_void) -> u8
        = ("Game.dll", "?StartSkill@Character@GAME@@QEAA?B_NIIAEBVWorldVec3@2@IAEBUTargetLeadingData@2@@Z"),
});

static SKILL_API: OnceLock<Option<SkillApi>> = OnceLock::new();
static COMBAT_API: OnceLock<Option<CombatApi>> = OnceLock::new();
static DPS_API: OnceLock<Option<DpsApi>> = OnceLock::new();
static CAST_API: OnceLock<Option<CastApi>> = OnceLock::new();
static ATTR_API: OnceLock<Option<AttributeApi>> = OnceLock::new();

pub fn api() -> Option<&'static SkillApi> {
    SKILL_API.get_or_init(SkillApi::resolve).as_ref()
}
fn combat_api() -> Option<&'static CombatApi> {
    COMBAT_API.get_or_init(CombatApi::resolve).as_ref()
}
fn dps_api() -> Option<&'static DpsApi> {
    DPS_API.get_or_init(DpsApi::resolve).as_ref()
}
fn cast_api() -> Option<&'static CastApi> {
    CAST_API.get_or_init(CastApi::resolve).as_ref()
}
fn attr_api() -> Option<&'static AttributeApi> {
    ATTR_API.get_or_init(AttributeApi::resolve).as_ref()
}

/// Whether skills can be read *and* cast, for the UI to report honestly.
pub fn available() -> bool {
    api().is_some() && cast_api().is_some()
}

/// A skill the character actually has right now.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct LiveSkill {
    /// `tagClass03SkillName04A` — the join key against the parsed database.
    pub tag: String,
    pub level: u32,
    /// Milliseconds until it can be cast again; 0 when ready.
    pub cooldown_remaining: i32,
}

/// Generously sized, over-aligned, zeroed scratch for a game struct whose
/// exact size we do not know.
///
/// Over-allocating is the safe direction: the game writes `sizeof(T)` bytes and
/// anything beyond that stays zero. Guessing *too small* would be a stack
/// overwrite, so these are deliberately far larger than any plausible layout
/// (`WorldVec3` is a `Region*` plus a `Vec3`, so 24 bytes; `Name` wraps a
/// single `u32` digest).
///
/// This is also why `Name` needs no constructor. Its only exported members are
/// `GetDigest`/`SetDigest` on a `u32`, so a default-constructed `Name` is one
/// with a zero digest -- which is exactly what zeroed scratch already is. The
/// first version called `Name::Name()`; that constructor is exported from
/// **Engine.dll**, the lookup only searched Game.dll, and the failure took the
/// whole module's features down with it. Not depending on it is better than
/// depending on it correctly.
#[repr(C, align(16))]
struct Scratch([u8; 128]);

impl Scratch {
    fn new() -> Self {
        Self([0u8; 128])
    }
    fn ptr(&mut self) -> *mut c_void {
        self.0.as_mut_ptr() as *mut c_void
    }
}

/// The character's `SkillManager`.
///
/// # Safety
/// `engine` must be the live `GameEngine*`, on the frame-hook thread.
unsafe fn skill_manager(engine: *mut c_void) -> Option<*mut c_void> {
    let api = api()?;
    let player = state::main_player(engine)?;
    let mgr = (api.get_skill_manager)(player);
    (!mgr.is_null()).then_some(mgr)
}

/// Every skill the character currently has.
///
/// This is what backs "only show skills I actually have": each entry carries
/// the same display-name tag the parsed database keys on, so the two are joined
/// without either side guessing at the other's record paths.
///
/// # Safety
/// As [`skill_manager`].
pub unsafe fn skills(engine: *mut c_void) -> Vec<LiveSkill> {
    let Some(api) = api() else { return Vec::new() };
    let Some(mgr) = skill_manager(engine) else { return Vec::new() };

    let list = (api.get_skill_list)(mgr);
    let Some(entries) = abi::read_vector::<*const c_void>(list) else {
        return Vec::new();
    };

    let mut out = Vec::with_capacity(entries.len());
    for skill in entries.iter().copied() {
        if skill.is_null() {
            continue;
        }
        let tag_ptr = (api.skill_display_name_tag)(skill);
        let Some(tag) = abi::read_string(tag_ptr) else { continue };
        if tag.is_empty() {
            continue;
        }
        out.push(LiveSkill {
            tag,
            level: (api.skill_level)(skill),
            cooldown_remaining: (api.skill_cooldown_remaining)(skill),
        });
    }
    out
}

/// What the character is doing and to whom.
#[derive(Clone, Copy, Debug, Default, serde::Serialize)]
pub struct Combat {
    /// Entity id of the current attack target, 0 for none.
    pub target: u32,
    /// Whether there is a target at all — the plain form of the question
    /// "am I on something".
    pub has_target: bool,
    /// Something is attacking us.
    pub under_attack: bool,
    pub moving: bool,
    /// Moving *as part of* an attack, which is the engine's own notion of
    /// "engaged" rather than "walking".
    pub attack_moving: bool,
    /// Engine milliseconds since we last attacked, or `None` if never.
    pub since_attack_ms: Option<i32>,
}

impl Combat {
    /// A single "am I fighting" answer, as a rule would ask it.
    ///
    /// Deliberately generous: a target *or* recent outgoing damage *or* being
    /// hit all count, because a rotation should keep running through the gap
    /// between one pack dying and the next arriving.
    pub fn in_combat(&self, recent_ms: i32) -> bool {
        self.has_target
            || self.under_attack
            || self.since_attack_ms.is_some_and(|ms| ms >= 0 && ms <= recent_ms)
    }
}

/// Read combat and targeting state.
///
/// # Safety
/// As [`skill_manager`].
pub unsafe fn combat(engine: *mut c_void) -> Option<Combat> {
    let api = combat_api()?;
    let player = state::main_player(engine)?;

    // The game fills these; we never construct a WorldVec3 ourselves, which is
    // what keeps its layout none of our business.
    let mut target: u32 = 0;
    let mut extra: u32 = 0;
    let mut position = Scratch::new();
    (api.get_current_attack_target)(player, &mut target, position.ptr(), &mut extra);

    let last = (api.last_attack_time)(player);

    Some(Combat {
        target,
        has_target: target != 0,
        under_attack: (api.is_under_attack)(player) != 0,
        moving: (api.is_moving)(player) != 0,
        attack_moving: (api.is_attack_moving)(player) != 0,
        // The engine reports a timestamp; negative or zero means "never".
        since_attack_ms: (last > 0).then_some(last),
    })
}

/// The character's damage-per-second, as the game's own character sheet
/// computes it.
///
/// The trailing argument's meaning is not decoded, so this is one of the few
/// numbers here that is **not** fully understood; zero is what a plain "all
/// damage" query ought to be. Implausible results are discarded rather than
/// shown, on the principle that a blank is better than a wrong number on a
/// live dashboard.
///
/// # Safety
/// As [`skill_manager`].
pub unsafe fn dps(engine: *mut c_void) -> Option<f32> {
    let api = dps_api()?;
    let player = state::main_player(engine)?;
    let mut out: f32 = 0.0;
    (api.calculate_dps)(player, &mut out, 0);
    (out.is_finite() && (0.0..=1.0e9).contains(&out)).then_some(out)
}

/// Highest `CharAttributeType` value probed. The enum is small; this is
/// generous and costs one cheap call per index, once.
pub const MAX_ATTRIBUTE: u32 = 128;

/// Read one character attribute by its enum value.
///
/// # Safety
/// As [`skill_manager`].
pub unsafe fn char_attribute(engine: *mut c_void, index: u32) -> Option<f32> {
    let api = attr_api()?;
    let player = state::main_player(engine)?;
    let value = (api.get_base_char_attribute)(player, index);
    value.is_finite().then_some(value)
}

/// Every attribute value the character has, by enum index.
///
/// # Safety
/// As [`skill_manager`].
pub unsafe fn char_attributes(engine: *mut c_void) -> Vec<f32> {
    let Some(api) = attr_api() else { return Vec::new() };
    let Some(player) = state::main_player(engine) else { return Vec::new() };
    (0..MAX_ATTRIBUTE)
        .map(|i| {
            let v = (api.get_base_char_attribute)(player, i);
            if v.is_finite() { v } else { 0.0 }
        })
        .collect()
}

/// What a probe of the attribute enum found.
pub struct AttributeProbe {
    pub values: Vec<f32>,
    /// The index whose value matches the character's maximum life, if one does.
    pub life_index: Option<u32>,
    /// Likewise for maximum energy.
    pub energy_index: Option<u32>,
}

/// Probe the `CharAttributeType` enum and anchor it against values we already
/// know.
///
/// This is the cheap discovery technique from CLAUDE.md -- call an exported
/// `const` getter and see what comes back -- with one addition that makes it
/// self-checking: maximum life and maximum energy are already known exactly
/// from confirmed accessors, so finding indices that reproduce them proves the
/// enum is being indexed correctly before anything is read from it. If neither
/// anchor is found, the probe is not to be trusted and reports so.
///
/// # Safety
/// As [`skill_manager`].
pub unsafe fn probe_attributes(engine: *mut c_void, life_max: f32, energy_max: f32) -> AttributeProbe {
    let values = char_attributes(engine);
    let close = |a: f32, b: f32| b > 0.0 && (a - b).abs() <= (b * 0.001).max(0.5);

    let life_index = values.iter().position(|v| close(*v, life_max)).map(|i| i as u32);
    let energy_index = values.iter().position(|v| close(*v, energy_max)).map(|i| i as u32);

    AttributeProbe { values, life_index, energy_index }
}

/// Resolve a record path to the runtime skill id used for casting.
///
/// # Safety
/// As [`skill_manager`].
pub unsafe fn skill_id(engine: *mut c_void, record_path: &str) -> Option<u32> {
    let api = api()?;
    let mgr = skill_manager(engine)?;

    let mut bytes: Vec<u8> = record_path.bytes().collect();
    bytes.push(0);
    let id = (api.find_skill_id)(mgr, bytes.as_ptr());
    (id != 0).then_some(id)
}

/// The game's own judgement on whether a skill can be cast this instant.
///
/// # Safety
/// As [`skill_manager`].
pub unsafe fn ready(engine: *mut c_void, id: u32) -> Option<bool> {
    let api = api()?;
    let mgr = skill_manager(engine)?;
    Some((api.is_skill_valid_for_use)(mgr, id, 0) != 0)
}

/// Why a cast did not happen, so the UI can say something true.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CastError {
    /// The skill symbols did not resolve on this build.
    Unavailable,
    /// No character, or no skill manager.
    NoPlayer,
    /// The record path is not a skill this character has.
    UnknownSkill,
    /// The game says it cannot be used right now: cooldown, energy, level.
    NotReady,
    /// The call was made and the game returned false. The arguments are wrong,
    /// or this is not the entry point a player cast goes through.
    Refused,
}

impl CastError {
    pub fn reason(self) -> &'static str {
        match self {
            CastError::Unavailable => "skill casting is not available on this game build",
            CastError::NoPlayer => "no character in the world",
            CastError::UnknownSkill => "your character does not have that skill",
            CastError::NotReady => "the game says that skill is not ready",
            CastError::Refused => "the game refused the cast (see the log)",
        }
    }
}

/// Cast a skill by its record path.
///
/// ## What is known, and what is not
///
/// Known: `FindSkillId` resolves the record path to a runtime id, and
/// `IsSkillValidForUse` is the game's own answer on whether it can be cast this
/// instant. Both are confirmed working in a live game -- a skill that is not
/// learned, on cooldown, or unaffordable is refused here rather than by us.
///
/// **Not known: how to make the cast actually happen.** The first attempt used
/// `Character::ActivateSkill`, which returns `void`, and in a live game it did
/// nothing at all while reporting nothing. The likely reason is that it is the
/// lower half of the operation -- the part that runs *after* the player's
/// controller has accepted the input -- and the player-facing entry points are
/// `ControllerPlayer::InstantSkillAction` and `SendSkillAction`, neither of
/// which is reachable: nothing exported returns a `ControllerPlayer*`.
///
/// So this now calls `Character::StartSkill`, which is the same operation one
/// level up **and returns a bool**. That does not make it correct, but it makes
/// it *informative*: a `false` means the game rejected the call and the
/// arguments are wrong, while a `true` that still does nothing means the call
/// is landing and something downstream is missing. Either answer is worth more
/// than silence, and both are logged.
///
/// # Safety
/// Must run on the frame-hook thread, with the gate already satisfied.
pub unsafe fn cast(engine: *mut c_void, record_path: &str) -> Result<(), CastError> {
    let cast = cast_api().ok_or(CastError::Unavailable)?;
    let combat = combat_api().ok_or(CastError::Unavailable)?;
    let player = state::main_player(engine).ok_or(CastError::NoPlayer)?;

    let id = skill_id(engine, record_path).ok_or(CastError::UnknownSkill)?;
    if ready(engine, id) != Some(true) {
        return Err(CastError::NotReady);
    }

    // A target position the game itself produced, so its layout is never our
    // problem. With no target this is the character's own position, which is
    // the right place for a self-buff anyway.
    let mut target: u32 = 0;
    let mut extra: u32 = 0;
    let mut position = Scratch::new();
    (combat.get_current_attack_target)(player, &mut target, position.ptr(), &mut extra);

    // `TargetLeadingData` is the aim-ahead hint for projectiles. Zeroed means
    // "no lead", which is what a stationary or self-targeted cast wants.
    let mut leading = Scratch::new();

    let accepted = (cast.start_skill)(player, id, 0, position.ptr(), target, leading.ptr()) != 0;

    // Logged every time until this is understood: the id, the target and the
    // answer are the three things that tell us what to try next.
    log!(
        "cast: StartSkill(skill {id}, target {target}) -> {}  [{record_path}]",
        if accepted { "accepted" } else { "REFUSED" }
    );

    if accepted {
        Ok(())
    } else {
        Err(CastError::Refused)
    }
}
