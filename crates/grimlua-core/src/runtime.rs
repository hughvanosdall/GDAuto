//! What actually runs on the frame hook: read state, consult the gate, and
//! perform at most one action.
//!
//! The rule list is evaluated top to bottom and stops at the first match.
//! That is deliberate and it is the model the whole design rests on: the gate
//! serialises everything, so "do A then B this tick" is not expressible, and
//! a flowchart that implied it would silently drop actions.

use std::ffi::c_void;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use crate::log;
use crate::shared::{self, Gate, Snapshot, Vitals};
use crate::{gate, state};

/// Evaluate every N frames. ~20Hz at 120fps: quick enough that a potion lands
/// before a spike kills you, slow enough to cost the game nothing.
pub const TICK_FRAMES: u64 = 6;

static LAST_ACTION_AT: Mutex<Option<Instant>> = Mutex::new(None);
static LAST_ACTION_NAME: Mutex<Option<String>> = Mutex::new(None);
static ACTIONS: AtomicU64 = AtomicU64::new(0);
static BLOCKED: AtomicU64 = AtomicU64::new(0);

/// One evaluation. Runs on the game thread.
///
/// # Safety
/// `engine` must be the live `GameEngine*`, on the frame-hook thread.
pub unsafe fn tick(engine: *mut c_void, frame: u64) {
    let cfg = shared::config();

    let vitals = state::vitals(engine).map(|v| Vitals {
        life: v.life,
        life_max: v.life_max,
        energy: v.mana,
        energy_max: v.mana_max,
    });

    let last_at = LAST_ACTION_AT.lock().ok().and_then(|g| *g);
    let verdict = gate::evaluate(engine, last_at);

    if verdict.allows_action() {
        if let Some(v) = &vitals {
            if let Some(action) = choose_action(&cfg, v) {
                perform(engine, action);
            }
        }
    } else if !matches!(verdict, Gate::Disarmed | Gate::NoPlayer) {
        // Only count refusals that stopped something we might have done;
        // being disarmed or at the menu is not the gate "holding".
        if cfg.armed && wants_something(&cfg, vitals.as_ref()) {
            BLOCKED.fetch_add(1, Ordering::Relaxed);
        }
    }

    shared::put_snapshot(Snapshot {
        in_world: vitals.is_some(),
        vitals: vitals.unwrap_or_default(),
        gate: Some(verdict),
        gate_reason: verdict.reason().to_string(),
        armed: cfg.armed,
        actions: ACTIONS.load(Ordering::Relaxed),
        blocked: BLOCKED.load(Ordering::Relaxed),
        last_action: LAST_ACTION_NAME.lock().ok().and_then(|g| g.clone()),
        frame,
    });
}

#[derive(Clone, Copy)]
enum Action {
    HealthPotion,
    EnergyPotion,
}

impl Action {
    fn label(self) -> &'static str {
        match self {
            Action::HealthPotion => "health potion",
            Action::EnergyPotion => "energy potion",
        }
    }
}

fn pct(cur: f64, max: f32) -> Option<f32> {
    (max > 0.0).then(|| (cur as f32 / max) * 100.0)
}

/// The priority list. First match wins.
fn choose_action(cfg: &shared::Config, v: &Vitals) -> Option<Action> {
    if cfg.health_potion.enabled {
        if let Some(p) = pct(v.life, v.life_max) {
            if p < cfg.health_potion.threshold {
                return Some(Action::HealthPotion);
            }
        }
    }
    if cfg.energy_potion.enabled {
        if let Some(p) = pct(v.energy as f64, v.energy_max) {
            if p < cfg.energy_potion.threshold {
                return Some(Action::EnergyPotion);
            }
        }
    }
    None
}

/// Whether any rule would have fired, used only to decide if a gate refusal
/// is worth counting.
fn wants_something(cfg: &shared::Config, v: Option<&Vitals>) -> bool {
    v.map(|v| choose_action(cfg, v).is_some()).unwrap_or(false)
}

unsafe fn perform(engine: *mut c_void, action: Action) {
    let ok = match action {
        Action::HealthPotion => state::drink_health_potion(engine),
        Action::EnergyPotion => state::drink_energy_potion(engine),
    };
    if !ok {
        log!("action: {} could not be reached", action.label());
        return;
    }

    ACTIONS.fetch_add(1, Ordering::Relaxed);
    if let Ok(mut g) = LAST_ACTION_AT.lock() {
        *g = Some(Instant::now());
    }
    if let Ok(mut g) = LAST_ACTION_NAME.lock() {
        *g = Some(action.label().to_string());
    }

    // The raw HotSlotOptionStatus is logged rather than acted on: its enum is
    // not yet decoded, and pressing a slot the game considers unavailable is
    // a no-op, exactly as it is for the keybind.
    let status = state::health_potion_status(engine).unwrap_or(-1);
    log!("action: {} (slot status {status})", action.label());
}
