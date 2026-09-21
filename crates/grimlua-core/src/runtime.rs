//! What actually runs on the frame hook: read state, consult the gate, ask the
//! script, and perform at most one action.
//!
//! The priority list is evaluated top to bottom and stops at the first match.
//! That is deliberate and it is the model the whole design rests on: the gate
//! serialises everything, so "do A then B this tick" is not expressible, and a
//! flowchart that implied it would silently drop actions.
//!
//! ## Where the script sits
//!
//! `choose_action` used to be a hardcoded stand-in. It is now a Lua call, and
//! the contract it had to satisfy is unchanged: given the config and the
//! vitals, return **at most one** [`Action`]. Four things hold:
//!
//! * the script receives a table of plain numbers and returns a name, so it
//!   cannot act, only ask;
//! * the host parses that name against a closed enum and can refuse it;
//! * [`perform`] is reachable from exactly one place, inside the
//!   `Gate::Clear` branch;
//! * the VM runs under an instruction budget, because this is the game's
//!   render thread.
//!
//! ## One refinement to the original design
//!
//! CLAUDE.md said a script is not consulted at all when the gate is closed.
//! That is relaxed *only* for the disarmed-and-being-watched case: when a
//! browser is connected the script is evaluated so the editor can show which
//! rule is currently winning while you are still writing it. Nothing changes
//! about who may act -- [`perform`] is still only called from `Gate::Clear`,
//! and evaluation cannot touch the game because the script is a pure function
//! of a snapshot taken beforehand. Without this, the editor's live feedback
//! would only work while armed, which is precisely when you do not want to be
//! experimenting.

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::c_void;
use std::time::{Duration, Instant};

use crate::log;
use crate::script::{Evaluator, ScriptState};
use crate::shared::{self, Gate, PotionStatus, ScriptStatus, Snapshot, Vitals};
use crate::{gate, state};

/// Evaluate every N frames. ~20Hz at 120fps: quick enough that a potion lands
/// before a spike kills you, slow enough to cost the game nothing.
pub const TICK_FRAMES: u64 = 6;

/// Consecutive runtime errors before evaluation stops until the config
/// changes. A script that errors every tick is a script burning the render
/// thread to no purpose, and the error it reports will be the same one.
const ERROR_LIMIT: u32 = 10;

/// How often the character's skill list is re-read from the game.
///
/// Two reasons this is not every tick. It is the only code in the project that
/// walks a game container by assumed layout, so every call is exposure that
/// buys nothing; and the set it reads changes when you level or swap gear, not
/// sixty times a second. Two seconds is far faster than the thing it watches.
const SKILL_REFRESH: Duration = Duration::from_secs(2);

/// After this many consecutive fruitless reads while in the world, back off to
/// [`SKILL_BACKOFF`]. An empty list from a live character means the container
/// layout did not validate, and retrying at full rate will not change that.
const SKILL_FAILURES_BEFORE_BACKOFF: u32 = 5;
const SKILL_BACKOFF: Duration = Duration::from_secs(30);

/// How often damage-per-second is recomputed.
///
/// `CalculateDps` is the character sheet's own calculation, not a field read,
/// so it is not free. Four times a second is faster than anyone can read it.
const DPS_REFRESH: Duration = Duration::from_millis(250);

/// How recently we must have attacked to still count as "in combat".
///
/// A rotation should keep running across the gap between one pack dying and
/// the next arriving, so this is deliberately generous.
const IN_COMBAT_WINDOW_MS: i32 = 4_000;

/// One action grimlua can take. A closed set the host owns: a script names a
/// member of this enum, it never carries one.
#[derive(Clone, PartialEq, Debug)]
pub enum Action {
    HealthPotion,
    EnergyPotion,
    /// Cast a skill, named by its database record path.
    ///
    /// Not a bar position: the skill is invoked directly, so nothing here
    /// depends on the player's hot bar or their key bindings.
    Skill(String),
}

impl Action {
    /// Parse what a script returned. The spelling matches
    /// [`crate::rules::ActionSpec::token`], so a generated script and a
    /// hand-written one speak the same language.
    pub fn parse(token: &str) -> Option<Action> {
        match token {
            "health_potion" => Some(Action::HealthPotion),
            "energy_potion" => Some(Action::EnergyPotion),
            _ => token
                .strip_prefix("skill:")
                .filter(|p| !p.is_empty())
                .map(|p| Action::Skill(p.to_string())),
        }
    }

    pub fn token(&self) -> String {
        match self {
            Action::HealthPotion => "health_potion".into(),
            Action::EnergyPotion => "energy_potion".into(),
            Action::Skill(path) => format!("skill:{path}"),
        }
    }

    /// Human text for the activity line. A record path is unreadable, so the
    /// parsed database is asked for the display name; falling back to the last
    /// path segment keeps it sane before the index has loaded.
    pub fn label(&self) -> String {
        match self {
            Action::HealthPotion => "health potion".into(),
            Action::EnergyPotion => "energy potion".into(),
            Action::Skill(path) => shared::skills()
                .and_then(|index| index.by_path(path).map(|s| s.name.clone()))
                .unwrap_or_else(|| {
                    path.rsplit('/').next().unwrap_or(path).trim_end_matches(".dbr").to_string()
                }),
        }
    }
}

/// Everything the frame hook keeps between ticks.
///
/// Thread-local rather than a set of statics, because all of it belongs to the
/// game thread and none of it should be reachable from the server. The Lua VM
/// in particular is not `Send`, which makes that a compile-time guarantee
/// rather than a convention.
#[derive(Default)]
struct Runtime {
    evaluator: Option<Evaluator>,
    /// The config revision the evaluator was built from. `None` until the
    /// first compile, which is why revisions start at 1.
    compiled: Option<u64>,
    compile_error: Option<String>,
    marks: Vec<crate::rules::LineMark>,
    source: String,

    first_tick: Option<Instant>,
    last_action_at: Option<Instant>,
    last_action: Option<String>,
    last_rule: Option<String>,
    since_action: HashMap<String, Instant>,
    since_rule: HashMap<String, Instant>,
    /// Record path -> can it be cast right now. Refreshed on a timer from the
    /// live skill list joined against the parsed database.
    skill_ready: HashMap<String, bool>,
    /// Record path -> milliseconds of cooldown remaining.
    skill_cooldown: HashMap<String, f64>,
    /// When the skill list was last read, for [`SKILL_REFRESH`].
    skills_read_at: Option<Instant>,
    /// Consecutive reads that came back empty from a live character.
    skill_failures: u32,
    /// Whether the character-attribute enum has been probed this session.
    attributes_probed: bool,
    /// Last computed DPS and when, for [`DPS_REFRESH`].
    dps: Option<f32>,
    dps_read_at: Option<Instant>,
    /// Display-name tag -> record path, built once per database revision.
    ///
    /// Without this the join was a linear scan of 1,400 records per live
    /// skill, every tick: several million string comparisons a second on the
    /// game's render thread, to answer a question whose answer rarely changes.
    tag_to_path: HashMap<String, String>,
    tag_index_revision: Option<u64>,

    actions: u64,
    blocked: u64,
    evaluations: u64,
    errors: u64,
    consecutive_errors: u32,
    halted: bool,
    last_error: Option<String>,
    last_eval_us: u64,
}

thread_local! {
    static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::default());
}

/// One evaluation. Runs on the game thread.
///
/// # Safety
/// `engine` must be the live `GameEngine*`, on the frame-hook thread.
pub unsafe fn tick(engine: *mut c_void, frame: u64) {
    RUNTIME.with(|rt| {
        if let Ok(mut rt) = rt.try_borrow_mut() {
            rt.tick(engine, frame);
        }
    });
}

impl Runtime {
    unsafe fn tick(&mut self, engine: *mut c_void, frame: u64) {
        let now = Instant::now();
        self.first_tick.get_or_insert(now);

        let cfg = shared::config();
        let revision = shared::config_revision();
        if self.compiled != Some(revision) {
            self.recompile(&cfg, revision);
        }

        let vitals = state::vitals(engine).map(|v| Vitals {
            life: v.life,
            life_max: v.life_max,
            energy: v.mana,
            energy_max: v.mana_max,
        });

        let in_world = vitals.is_some();
        let potions = self.read_potions(engine, in_world);
        self.read_skills(engine, in_world, now);
        let combat = if in_world {
            crate::live::combat(engine).unwrap_or_default()
        } else {
            crate::live::Combat::default()
        };
        // Probe the attribute enum once, the first time there is a character to
        // read it from. One pass of cheap const getters, then never again.
        if in_world && !self.attributes_probed {
            self.attributes_probed = true;
            if let Some(v) = &vitals {
                self.probe_attributes(engine, v);
            }
        }

        // A character-sheet calculation, so it runs on its own slower clock.
        if !in_world {
            self.dps = None;
            self.dps_read_at = None;
        } else if self.dps_read_at.is_none_or(|t| now.saturating_duration_since(t) >= DPS_REFRESH) {
            self.dps_read_at = Some(now);
            self.dps = crate::live::dps(engine);
        }

        let verdict = gate::evaluate(engine, self.last_action_at);

        // Evaluate when we could act on the answer, or when somebody is
        // watching the editor and wants to see which rule is winning. Neither
        // path can reach the game: the script is handed numbers, not pointers.
        let watched = shared::watchers() > 0;
        let chosen = match (&vitals, verdict.allows_action() || watched) {
            (Some(v), true) => self.evaluate(v, &potions, &combat, now, frame),
            _ => None,
        };

        let mut pending_label = None;
        let mut pending_rule = None;
        if let Some((token, rule)) = &chosen {
            pending_label = Action::parse(token).map(|a| a.label());
            pending_rule = rule.clone();
        }

        if verdict.allows_action() {
            if let Some((token, rule)) = chosen {
                self.perform(engine, &token, rule, now);
            }
        } else if pending_label.is_some()
            && !matches!(verdict, Gate::Disarmed | Gate::NoPlayer)
        {
            // Only count refusals that stopped something we would have done;
            // being disarmed or at the menu is not the gate "holding".
            self.blocked += 1;
        }

        shared::put_snapshot(Snapshot {
            in_world: vitals.is_some(),
            vitals: vitals.unwrap_or_default(),
            gate: Some(verdict),
            gate_reason: verdict.reason().to_string(),
            armed: cfg.armed,
            actions: self.actions,
            blocked: self.blocked,
            last_action: self.last_action.clone(),
            last_rule: self.last_rule.clone(),
            pending_action: pending_label,
            pending_rule,
            potions,
            combat,
            dps: self.dps,
            skills_available: crate::live::available(),
            frame,
        });

        shared::put_script_status(ScriptStatus {
            revision,
            source: self.source.clone(),
            marks: self.marks.clone(),
            ok: self.compile_error.is_none(),
            error: self.compile_error.clone(),
            evaluations: self.evaluations,
            errors: self.errors,
            last_error: self.last_error.clone(),
            halted: self.halted,
            last_eval_us: self.last_eval_us,
        });
    }

    /// Build a new evaluator from the config the browser just sent.
    ///
    /// A failure here is not an error state to recover from: it means no
    /// script, which means no actions. That is the fail-closed behaviour the
    /// gate demands, reached from the other direction.
    fn recompile(&mut self, cfg: &shared::Config, revision: u64) {
        let (source, marks) = cfg.program();
        self.compiled = Some(revision);
        self.source = source;
        self.marks = marks;
        self.halted = false;
        self.consecutive_errors = 0;
        self.last_error = None;

        match Evaluator::compile(&self.source) {
            Ok(ev) => {
                self.evaluator = Some(ev);
                self.compile_error = None;
                log!("script: revision {revision} compiled ({} bytes)", self.source.len());
                shared::event("script", format!("revision {revision} compiled"));
            }
            Err(e) => {
                // Drop the old one. Silently running the previous script after
                // an edit failed would be the worst of both worlds: the editor
                // shows one thing and the game does another.
                self.evaluator = None;
                self.compile_error = Some(e.clone());
                log!("script: revision {revision} will not compile: {e}");
                shared::event("script", format!("will not compile: {e}"));
            }
        }
    }

    /// Read the potion statuses the script is shown.
    ///
    /// # Safety
    /// As [`tick`].
    unsafe fn read_potions(&self, engine: *mut c_void, in_world: bool) -> PotionStatus {
        if !in_world {
            return PotionStatus::default();
        }
        match state::potion_statuses(engine) {
            Some((h, e)) => PotionStatus { health: Some(h as i64), energy: Some(e as i64) },
            None => PotionStatus::default(),
        }
    }

    /// Join the live skill list against the parsed database.
    ///
    /// The game knows a skill by a `Skill*` and its display-name tag; a rule
    /// knows it by its record path. The database has both, so this is where the
    /// two meet -- and it is also what backs the editor's "only skills I have"
    /// filter, since the tags that come back *are* the available set.
    ///
    /// # Safety
    /// As [`tick`].
    unsafe fn read_skills(&mut self, engine: *mut c_void, in_world: bool, now: Instant) {
        if !in_world {
            self.skills_read_at = None;
            self.skill_failures = 0;
            return;
        }

        let due = match self.skills_read_at {
            None => true,
            Some(last) => {
                let wait = if self.skill_failures >= SKILL_FAILURES_BEFORE_BACKOFF {
                    SKILL_BACKOFF
                } else {
                    SKILL_REFRESH
                };
                now.saturating_duration_since(last) >= wait
            }
        };
        if !due {
            return;
        }
        self.skills_read_at = Some(now);

        let live = crate::live::skills(engine);
        if live.is_empty() {
            // A live character with no readable skills means the container
            // did not validate. Count it, and stop hammering a call that is
            // not going to start working.
            self.skill_failures = self.skill_failures.saturating_add(1);
            if self.skill_failures == SKILL_FAILURES_BEFORE_BACKOFF {
                log!("live: skill list unreadable; backing off to every {SKILL_BACKOFF:?}");
                shared::event("script", "could not read the character's skill list");
            }
            return;
        }
        self.skill_failures = 0;

        // Publish the availability set for the editor. Cheap because
        // `put_available` compares before it writes.
        shared::put_available(live.iter().map(|s| s.tag.clone()).collect());

        let Some(index) = shared::skills() else { return };
        self.rebuild_tag_index(&index);

        self.skill_ready.clear();
        self.skill_cooldown.clear();
        for entry in &live {
            let Some(path) = self.tag_to_path.get(&entry.tag) else { continue };
            self.skill_cooldown
                .insert(path.clone(), entry.cooldown_remaining.max(0) as f64);
            self.skill_ready
                .insert(path.clone(), entry.cooldown_remaining <= 0);
        }
    }

    /// Probe `CharAttributeType` and write what it found to the log.
    ///
    /// The enum's values are not in the export table, so the indices have to be
    /// identified by looking at them. Maximum life and maximum energy are known
    /// exactly from confirmed accessors, so they anchor the mapping: if neither
    /// turns up, the probe is wrong and nothing should be read from it.
    ///
    /// # Safety
    /// As [`tick`].
    unsafe fn probe_attributes(&mut self, engine: *mut c_void, v: &Vitals) {
        let probe = crate::live::probe_attributes(engine, v.life_max, v.energy_max);
        if probe.values.is_empty() {
            log!("attrs: GetBaseCharAttribute is unavailable");
            return;
        }

        match (probe.life_index, probe.energy_index) {
            (Some(l), Some(e)) => log!(
                "attrs: anchored -- life_max at index {l}, energy_max at index {e}"
            ),
            (l, e) => log!(
                "attrs: NOT anchored (life {l:?}, energy {e:?}); treat the values below as suspect"
            ),
        }

        // Only the non-zero ones: the enum is sparse for any given character
        // and a wall of zeroes helps nobody.
        let mut line = String::new();
        for (i, value) in probe.values.iter().enumerate() {
            if *value != 0.0 {
                line.push_str(&format!("{i}={value:.1}  "));
            }
        }
        log!("attrs: {line}");
        shared::event("script", format!(
            "probed {} character attributes; see the log",
            probe.values.iter().filter(|v| **v != 0.0).count()
        ));
    }

    /// Build the tag -> record path map, once per database revision.
    ///
    /// One display-name tag can name several records -- a skill and the buff it
    /// applies share one -- and the castable record is what a rule casts, so it
    /// wins.
    fn rebuild_tag_index(&mut self, index: &crate::db::SkillIndex) {
        let revision = shared::skills_revision();
        if self.tag_index_revision == Some(revision) {
            return;
        }
        self.tag_index_revision = Some(revision);
        self.tag_to_path.clear();
        for skill in &index.skills {
            let path = skill.path.to_ascii_lowercase();
            match self.tag_to_path.entry(skill.tag.clone()) {
                std::collections::hash_map::Entry::Vacant(slot) => {
                    slot.insert(path);
                }
                std::collections::hash_map::Entry::Occupied(mut slot) => {
                    if skill.castable {
                        slot.insert(path);
                    }
                }
            }
        }
        log!("live: {} skill tags indexed", self.tag_to_path.len());
    }

    /// Run the script once, returning the action token it chose.
    fn evaluate(
        &mut self,
        v: &Vitals,
        potions: &PotionStatus,
        combat: &crate::live::Combat,
        now: Instant,
        frame: u64,
    ) -> Option<(String, Option<String>)> {
        if self.halted {
            return None;
        }
        let evaluator = self.evaluator.as_ref()?;

        let state = ScriptState {
            life: v.life,
            life_max: v.life_max as f64,
            life_pct: pct(v.life, v.life_max),
            energy: v.energy as f64,
            energy_max: v.energy_max as f64,
            energy_pct: pct(v.energy as f64, v.energy_max),
            frame,
            uptime: self.first_tick.map(|t| secs(now - t)).unwrap_or(0.0),
            since_any: self.last_action_at.map(|t| secs(now - t)).unwrap_or(f64::INFINITY),
            last_action: self.last_action.clone(),
            health_potion_status: potions.health,
            energy_potion_status: potions.energy,
            in_combat: combat.in_combat(IN_COMBAT_WINDOW_MS),
            has_target: combat.has_target,
            under_attack: combat.under_attack,
            moving: combat.moving,
            target: combat.target,
            skill_ready: self.skill_ready.clone(),
            skill_cooldown: self.skill_cooldown.clone(),
            since_action: elapsed_map(&self.since_action, now),
            since_rule: elapsed_map(&self.since_rule, now),
        };

        let started = Instant::now();
        let result = evaluator.evaluate(&state);
        self.last_eval_us = started.elapsed().as_micros().min(u64::MAX as u128) as u64;
        self.evaluations += 1;

        match result {
            Ok(outcome) => {
                self.consecutive_errors = 0;
                shared::console_write(&outcome.prints);
                outcome.action.map(|token| (token, outcome.rule))
            }
            Err(e) => {
                self.errors += 1;
                self.consecutive_errors += 1;
                if self.last_error.as_deref() != Some(e.as_str()) {
                    log!("script: {e}");
                }
                self.last_error = Some(e);
                if self.consecutive_errors >= ERROR_LIMIT {
                    self.halted = true;
                    log!(
                        "script: stopped after {ERROR_LIMIT} consecutive errors -- \
                         edit the script to restart it"
                    );
                }
                None
            }
        }
    }

    /// Validate what the script asked for, then do it.
    ///
    /// This is the only place an action happens, and it is reached from one
    /// place: the `Gate::Clear` branch of [`Runtime::tick`].
    unsafe fn perform(
        &mut self,
        engine: *mut c_void,
        token: &str,
        rule: Option<String>,
        now: Instant,
    ) {
        let Some(action) = Action::parse(token) else {
            self.refuse(format!("the script asked for \"{token}\", which is not an action"), now);
            return;
        };

        let ok = match &action {
            Action::HealthPotion => state::drink_health_potion(engine),
            Action::EnergyPotion => state::drink_energy_potion(engine),
            // The game decides about cooldown, energy and whether the skill is
            // even learned; a refusal here is information, not a failure.
            Action::Skill(path) => match crate::live::cast(engine, path) {
                Ok(()) => true,
                Err(e) => {
                    self.refuse(format!("{}: {}", action.label(), e.reason()), now);
                    return;
                }
            },
        };
        if !ok {
            self.refuse(format!("{} could not be reached", action.label()), now);
            return;
        }

        self.actions += 1;
        self.last_action_at = Some(now);
        self.last_action = Some(action.label());
        self.last_rule = rule.clone();
        self.since_action.insert(action.token(), now);
        if let Some(rule) = rule {
            self.since_rule.insert(rule, now);
        }
        self.last_error = None;

        log!("action: {}", action.label());
        shared::event("action", action.label());
    }

    /// Record a refusal without letting a script spam the log with it.
    ///
    /// The refusal also starts the global cooldown. Without that, a rule whose
    /// action is refused is retried on *every* tick -- twenty attempts a second
    /// at a call the game has already said no to, each one a log line. A
    /// refusal is an attempt; it should cost the same as a success.
    ///
    /// The message reaches the browser through `ScriptStatus::last_error`
    /// either way, which is where someone editing a rule will see it.
    fn refuse(&mut self, message: String, now: Instant) {
        self.last_action_at = Some(now);
        if self.last_error.as_deref() != Some(message.as_str()) {
            log!("script: {message}");
            shared::event("blocked", message.clone());
        }
        self.last_error = Some(message);
    }
}

fn pct(cur: f64, max: f32) -> f64 {
    if max > 0.0 {
        (cur / max as f64) * 100.0
    } else {
        0.0
    }
}

fn secs(d: Duration) -> f64 {
    d.as_secs_f64()
}

/// Turn "when it happened" into "how long ago", which is what a script wants.
fn elapsed_map(map: &HashMap<String, Instant>, now: Instant) -> HashMap<String, f64> {
    map.iter()
        .map(|(k, t)| (k.clone(), secs(now.saturating_duration_since(*t))))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_tokens_round_trip() {
        for action in [
            Action::HealthPotion,
            Action::EnergyPotion,
            Action::Skill("records/skills/playerclass06/totem1.dbr".into()),
        ] {
            assert_eq!(Action::parse(&action.token()), Some(action.clone()), "{action:?}");
        }
    }

    #[test]
    fn rule_tokens_and_action_tokens_are_the_same_language() {
        use crate::rules::ActionSpec;
        for spec in [
            ActionSpec::HealthPotion,
            ActionSpec::EnergyPotion,
            ActionSpec::Skill { path: "records/skills/x.dbr".into() },
        ] {
            assert!(
                Action::parse(&spec.token()).is_some(),
                "the generator emits {:?}, which the host cannot parse",
                spec.token()
            );
        }
    }

    /// A skill and the buff it applies share one display-name tag. The live
    /// game hands us that tag; the rule needs the *castable* record's path, so
    /// the index must resolve the tag to the skill rather than to the buff --
    /// regardless of which one the database happens to list first.
    #[test]
    fn the_tag_index_prefers_the_castable_record() {
        use crate::db::skills::{Kind, Skill};

        let mut buff = Skill { name: "Blood of Dreeg".into(), ..Default::default() };
        buff.path = "records/skills/playerclass03/bloodofdreeg1_buff.dbr".into();
        buff.tag = "tagClass03SkillName04A".into();
        buff.kind = Kind::Buff;

        let mut skill = Skill { name: "Blood of Dreeg".into(), ..Default::default() };
        skill.path = "records/skills/playerclass03/bloodofdreeg1.dbr".into();
        skill.tag = "tagClass03SkillName04A".into();
        skill.kind = Kind::Castable;
        skill.castable = true;

        for order in [vec![buff.clone(), skill.clone()], vec![skill, buff]] {
            let index = crate::db::SkillIndex { skills: order, ..Default::default() };
            let mut rt = Runtime::default();
            rt.rebuild_tag_index(&index);
            assert_eq!(
                rt.tag_to_path.get("tagClass03SkillName04A").map(String::as_str),
                Some("records/skills/playerclass03/bloodofdreeg1.dbr"),
                "the castable record must win whichever order they arrive in"
            );
        }
    }

    #[test]
    fn nonsense_is_not_an_action() {
        for token in ["", "potion", "skill:", "slot:3", "HEALTH_POTION"] {
            assert_eq!(Action::parse(token), None, "{token} parsed");
        }
    }
}
