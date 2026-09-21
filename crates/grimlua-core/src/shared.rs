//! State shared between the frame hook and the web server.
//!
//! The division is strict, and it is the project's central safety rule: the
//! frame hook is the only thing that ever touches the game. The server never
//! calls into Grim Dawn. They meet only here, through two locks:
//!
//! * [`SNAPSHOT`] — written by the hook, read by the server.
//! * [`CONFIG`] — written by the server, read by the hook.
//!
//! Both are plain data. Nothing in here holds a game pointer, so a slow or
//! wedged HTTP client can never stall or corrupt the game thread.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;

use serde::{Deserialize, Serialize};

/// What the gate decided on the most recent evaluation.
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Gate {
    /// Free to act.
    Clear,
    /// Deliberately off.
    Disarmed,
    /// Grim Dawn is not the foreground window.
    Unfocused,
    /// A UI panel, stash or vendor is open.
    UiOpen,
    /// Something fired recently.
    Cooldown,
    /// No character in the world yet.
    NoPlayer,
    /// The gate could not be resolved, so nothing may happen. This is the
    /// fail-closed case, not an error case.
    Unknown,
}

impl Gate {
    pub fn allows_action(self) -> bool {
        matches!(self, Gate::Clear)
    }

    pub fn reason(self) -> &'static str {
        match self {
            Gate::Clear => "clear",
            Gate::Disarmed => "disarmed",
            Gate::Unfocused => "game is not focused",
            Gate::UiOpen => "a UI panel is open",
            Gate::Cooldown => "global cooldown",
            Gate::NoPlayer => "no character in the world",
            Gate::Unknown => "gate state unknown - refusing to act",
        }
    }
}

#[derive(Clone, Serialize, Default)]
pub struct Vitals {
    pub life: f64,
    pub life_max: f32,
    pub energy: f32,
    pub energy_max: f32,
}

/// Everything the browser is shown. Rebuilt on the frame hook a few times a
/// second and copied out wholesale, so a reader never sees a half-written mix.
#[derive(Clone, Serialize, Default)]
pub struct Snapshot {
    /// False at the main menu and during loading.
    pub in_world: bool,
    pub vitals: Vitals,
    pub gate: Option<Gate>,
    pub gate_reason: String,
    pub armed: bool,
    /// Actions actually performed this session.
    pub actions: u64,
    /// Times the gate refused one.
    pub blocked: u64,
    /// Last action taken, for the activity line.
    pub last_action: Option<String>,
    pub frame: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PotionRule {
    pub enabled: bool,
    /// Fire when the resource drops below this percentage of its maximum.
    pub threshold: f32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    /// The master switch. Starts **off** every run: a tool that arms itself on
    /// load is a tool that acts before its owner is watching, and hardcore
    /// characters do not get a second chance.
    pub armed: bool,
    pub health_potion: PotionRule,
    pub energy_potion: PotionRule,
    /// Minimum gap between any two actions, in milliseconds.
    pub global_cooldown_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            armed: false,
            health_potion: PotionRule { enabled: true, threshold: 55.0 },
            energy_potion: PotionRule { enabled: false, threshold: 30.0 },
            global_cooldown_ms: 750,
        }
    }
}

pub static SNAPSHOT: RwLock<Option<Snapshot>> = RwLock::new(None);
pub static CONFIG: RwLock<Option<Config>> = RwLock::new(None);

/// Bumped by the server whenever the config changes, so the hook can notice a
/// hot-apply without taking the lock every frame.
pub static CONFIG_REVISION: AtomicU64 = AtomicU64::new(0);

pub fn snapshot() -> Snapshot {
    SNAPSHOT
        .read()
        .ok()
        .and_then(|g| g.clone())
        .unwrap_or_default()
}

pub fn put_snapshot(s: Snapshot) {
    if let Ok(mut g) = SNAPSHOT.write() {
        *g = Some(s);
    }
}

pub fn config() -> Config {
    CONFIG
        .read()
        .ok()
        .and_then(|g| g.clone())
        .unwrap_or_default()
}

pub fn put_config(c: Config) {
    if let Ok(mut g) = CONFIG.write() {
        *g = Some(c);
    }
    CONFIG_REVISION.fetch_add(1, Ordering::Release);
}
