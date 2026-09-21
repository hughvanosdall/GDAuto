//! State shared between the frame hook and the web server.
//!
//! The division is strict, and it is the project's central safety rule: the
//! frame hook is the only thing that ever touches the game. The server never
//! calls into Grim Dawn. They meet only here, through locks holding plain
//! data:
//!
//! * [`SNAPSHOT`] and [`SCRIPT`] — written by the hook, read by the server.
//! * [`CONFIG`] — written by the server, read by the hook.
//!
//! Nothing in here holds a game pointer, so a slow or wedged HTTP client can
//! never stall or corrupt the game thread. Note which way the script travels:
//! the browser sends *rules or source text*, and the hook is what compiles and
//! runs them. A Lua VM never exists on a server thread.

use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{OnceLock, RwLock};

use serde::{Deserialize, Serialize};

use crate::rules::{self, ActionSpec, LineMark, Op, Rule, Subject};
use crate::script;

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

/// Raw `HotSlotOptionStatus` values, as the game reports them. The enum is not
/// decoded, so these are passed through to the script and the UI verbatim
/// rather than being interpreted here.
#[derive(Clone, Serialize, Default)]
pub struct PotionStatus {
    pub health: Option<i64>,
    pub energy: Option<i64>,
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
    /// The rule that produced it, when the script named one.
    pub last_rule: Option<String>,
    /// What the script chose on this tick, whether or not the gate allowed it.
    /// This is what lets the editor light up the rule that is currently
    /// winning while you are still disarmed and editing.
    pub pending_action: Option<String>,
    pub pending_rule: Option<String>,
    pub potions: PotionStatus,
    /// What the character is doing and to whom.
    pub combat: crate::live::Combat,
    /// False when the skill exports did not resolve on this build, so the UI
    /// can say casting is unavailable rather than silently never firing.
    pub skills_available: bool,
    /// Damage per second as the game computes it, when it can be read.
    pub dps: Option<f32>,
    pub frame: u64,
}

/// Which front end owns the script.
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScriptMode {
    /// The rule list is the source of truth and the Lua is generated from it.
    Rules,
    /// The user has taken the Lua. Generation stops; there is no way back
    /// other than discarding the script, because there is no decompiler.
    Script,
}

/// Editor preferences that ride along with the config so they survive a
/// restart and follow the user between browsers.
#[derive(Clone, Serialize, Deserialize)]
pub struct Editor {
    /// Show only skills the character actually has, rather than every skill in
    /// the database. Off means the full list, which is what you want when
    /// building a priority list for a character you are not currently playing.
    pub only_available: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Self { only_available: true }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// The master switch. Starts **off** every run: a tool that arms itself on
    /// load is a tool that acts before its owner is watching, and hardcore
    /// characters do not get a second chance. This is the one field that is
    /// deliberately *not* restored from the saved config.
    pub armed: bool,
    /// Minimum gap between any two actions, in milliseconds.
    pub global_cooldown_ms: u64,
    pub mode: ScriptMode,
    pub rules: Vec<Rule>,
    /// Hand-written Lua, used when `mode` is [`ScriptMode::Script`]. Kept even
    /// while in rules mode so switching back and forth does not lose work.
    pub script: String,
    pub editor: Editor,
    /// Hook the game's own skill calls and log what it passes.
    ///
    /// A diagnostic, not a feature: extra detours on hot paths, there to answer
    /// how a player cast actually reaches the engine. Off by default, and each
    /// hook falls silent after a couple of dozen calls.
    pub trace_skills: bool,
}

/// Lowest global cooldown accepted. Twenty actions a second is the tick rate,
/// and a config asking for that is a config asking to be throttled.
pub const MIN_GLOBAL_COOLDOWN_MS: u64 = 100;
pub const MAX_GLOBAL_COOLDOWN_MS: u64 = 60_000;

impl Default for Config {
    /// The stock config reproduces exactly what grimlua did before it had a
    /// scripting layer: drink a health flask below 55%, everything else off.
    fn default() -> Self {
        let mut energy = Rule::new("r2", "Energy tonic", ActionSpec::EnergyPotion)
            .when(Subject::EnergyPct, Op::Below, 30.0);
        energy.enabled = false;

        Self {
            armed: false,
            global_cooldown_ms: 750,
            mode: ScriptMode::Rules,
            rules: vec![
                Rule::new("r1", "Health flask", ActionSpec::HealthPotion)
                    .when(Subject::LifePct, Op::Below, 55.0),
                energy,
            ],
            script: DEFAULT_SCRIPT.into(),
            editor: Editor::default(),
            trace_skills: false,
        }
    }
}

/// What the Lua editor starts with when a user first switches to script mode.
/// It is the stock rule list written by hand, so the first thing they see is
/// something that already works and can be poked at.
pub const DEFAULT_SCRIPT: &str = r#"-- grimlua priority list.
--
-- `choose` is called about twenty times a second and returns the ONE action
-- to take, or nil for none. First match wins, exactly like a SimulationCraft
-- action priority list -- there is no "do A then B", because the safety gate
-- allows one action per evaluation.
--
-- The state table `s` holds:
--   s.life, s.life_max, s.life_pct       numbers
--   s.energy, s.energy_max, s.energy_pct numbers
--   s.since["health_potion"]             seconds since that action fired
--   s.since_rule["my_rule"]              seconds since that rule fired
--   s.since_any                          seconds since anything fired
--   s.slot[n]                            raw hot-slot status, -1 if unknown
--   s.potion.health, s.potion.energy     raw potion slot status
--   s.uptime, s.frame                    numbers
--
-- Actions: "health_potion", "energy_potion", "slot:0" .. "slot:11".
-- Return a second value to name the rule, which shows up in the UI.
--
-- The gate still decides whether anything actually happens. Nothing you write
-- here can act while a panel is open, the game is unfocused, or you are
-- disarmed. print() goes to the console below.

function choose(s)
  if s.life_pct < 55 then
    return "health_potion", "flask"
  end

  return nil
end
"#;

impl Config {
    /// Bring anything a browser sent inside every limit.
    ///
    /// Called on receipt and again after loading from disk. The editor is a
    /// client like any other -- a hand-written websocket frame or an edited
    /// config file reaches exactly the same code -- so this is the only place
    /// that decides what is acceptable.
    pub fn sanitize(&mut self) {
        self.global_cooldown_ms = self
            .global_cooldown_ms
            .clamp(MIN_GLOBAL_COOLDOWN_MS, MAX_GLOBAL_COOLDOWN_MS);
        rules::sanitize(&mut self.rules);
        if self.script.len() > script::MAX_SOURCE {
            // On a char boundary: a config file is not necessarily ASCII, and
            // slicing through a multi-byte character would panic.
            let end = (0..=script::MAX_SOURCE)
                .rev()
                .find(|i| self.script.is_char_boundary(*i))
                .unwrap_or(0);
            self.script.truncate(end);
        }
    }

    /// The Lua that will actually be compiled, and where its rules came from.
    ///
    /// This is the single seam between the two front ends: everything
    /// downstream sees source text and a source map, and neither knows nor
    /// cares whether a human or the generator wrote it.
    pub fn program(&self) -> (String, Vec<LineMark>) {
        match self.mode {
            ScriptMode::Rules => {
                let generated = rules::generate(&self.rules);
                (generated.source, generated.marks)
            }
            ScriptMode::Script => (self.script.clone(), Vec::new()),
        }
    }

    /// The Lua the rules would generate, whatever mode is active.
    ///
    /// In rules mode this is the same text as [`Config::program`] produces. It
    /// is sent to the browser in both modes so the Lua editor can offer "start
    /// from my rules" without a second copy of the code generator living in
    /// JavaScript -- there is one generator, in Rust, and the editor is handed
    /// its output.
    pub fn generated_source(&self) -> String {
        rules::generate(&self.rules).source
    }

}

/// How the script the hook compiled is faring, published back to the browser.
#[derive(Clone, Serialize, Default)]
pub struct ScriptStatus {
    /// The config revision this was compiled from, so the UI can tell a stale
    /// status from a current one.
    pub revision: u64,
    /// The exact source that was compiled — generated or hand-written.
    pub source: String,
    /// Rule id to line number, empty in script mode.
    pub marks: Vec<LineMark>,
    pub ok: bool,
    /// Why it would not compile.
    pub error: Option<String>,
    pub evaluations: u64,
    pub errors: u64,
    /// The most recent runtime complaint: a Lua error, or the host refusing an
    /// action the script named.
    pub last_error: Option<String>,
    /// Set after repeated runtime errors. Evaluation stops until the config
    /// changes, because a script erroring every tick is a script burning the
    /// render thread.
    pub halted: bool,
    /// Microseconds the last evaluation took. Shown because this runs inside
    /// the game's frame.
    pub last_eval_us: u64,
}

pub static SNAPSHOT: RwLock<Option<Snapshot>> = RwLock::new(None);
pub static CONFIG: RwLock<Option<Config>> = RwLock::new(None);
pub static SCRIPT: RwLock<Option<ScriptStatus>> = RwLock::new(None);

/// Bumped by the server whenever the config changes, so the hook can notice a
/// hot-apply and recompile. Starts at 1 so a hook that has compiled nothing
/// yet can hold `None` and always disagree.
pub static CONFIG_REVISION: AtomicU64 = AtomicU64::new(1);

/// Browsers currently connected. The hook evaluates the script even when the
/// gate is closed *if* somebody is watching, so the editor can show which rule
/// is winning while you are disarmed and still editing. Nothing is performed
/// on that path -- see `runtime::tick`.
pub static WATCHERS: AtomicUsize = AtomicUsize::new(0);

pub fn snapshot() -> Snapshot {
    SNAPSHOT.read().ok().and_then(|g| g.clone()).unwrap_or_default()
}

pub fn put_snapshot(s: Snapshot) {
    if let Ok(mut g) = SNAPSHOT.write() {
        *g = Some(s);
    }
}

pub fn script_status() -> ScriptStatus {
    SCRIPT.read().ok().and_then(|g| g.clone()).unwrap_or_default()
}

pub fn put_script_status(s: ScriptStatus) {
    if let Ok(mut g) = SCRIPT.write() {
        *g = Some(s);
    }
}

pub fn config() -> Config {
    CONFIG.read().ok().and_then(|g| g.clone()).unwrap_or_default()
}

/// Install a new config and tell the hook to recompile.
///
/// The revision is bumped *after* the write, so a hook that sees the new
/// number is guaranteed to read the new config.
pub fn put_config(mut c: Config) {
    // A config arriving from the browser is a deliberate act by someone who can
    // see what the rules currently are, so it is safe to start saving again.
    LOAD_FAILED.store(false, Ordering::Release);
    c.sanitize();
    if let Ok(mut g) = CONFIG.write() {
        *g = Some(c);
    }
    CONFIG_REVISION.fetch_add(1, Ordering::Release);
}

pub fn config_revision() -> u64 {
    CONFIG_REVISION.load(Ordering::Acquire)
}

pub fn watchers() -> usize {
    WATCHERS.load(Ordering::Relaxed)
}

// ── the skill database, and what the character actually has ─────────────────
//
// Two different things, deliberately kept apart:
//
//   SKILLS    every skill in the parsed database. Built once on a worker
//             thread at startup, never changes, shared by Arc so the server
//             can serve it without cloning 1,400 records per request.
//   AVAILABLE the tags of the skills *this character* has right now. Written
//             by the frame hook, changes when you level or swap gear.
//
// The editor's "only skills I have" checkbox is the intersection: a database
// entry is offered when its tag appears in AVAILABLE.

pub static SKILLS: RwLock<Option<std::sync::Arc<crate::db::SkillIndex>>> = RwLock::new(None);

pub fn put_skills(index: crate::db::SkillIndex) {
    if let Ok(mut g) = SKILLS.write() {
        *g = Some(std::sync::Arc::new(index));
    }
    SKILLS_REVISION.fetch_add(1, Ordering::Release);
}

pub fn skills() -> Option<std::sync::Arc<crate::db::SkillIndex>> {
    SKILLS.read().ok().and_then(|g| g.clone())
}

pub static SKILLS_REVISION: AtomicU64 = AtomicU64::new(0);

pub fn skills_revision() -> u64 {
    SKILLS_REVISION.load(Ordering::Acquire)
}

/// Display-name tags of the skills the character currently has.
#[derive(Clone, Default, Serialize)]
pub struct Available {
    /// Bumped only when the set actually changes, so the browser is not sent
    /// a few hundred strings ten times a second.
    pub revision: u64,
    pub tags: Vec<String>,
}

pub static AVAILABLE: RwLock<Option<Available>> = RwLock::new(None);
static AVAILABLE_REVISION: AtomicU64 = AtomicU64::new(0);

pub fn available() -> Available {
    AVAILABLE.read().ok().and_then(|g| g.clone()).unwrap_or_default()
}

/// Replace the available set, but only bump the revision when it differs.
///
/// Called every tick from the frame hook, so the comparison is what keeps it
/// cheap: the set changes when you level or change gear, not per frame.
pub fn put_available(mut tags: Vec<String>) {
    tags.sort_unstable();
    tags.dedup();
    if let Ok(guard) = AVAILABLE.read() {
        if guard.as_ref().is_some_and(|a| a.tags == tags) {
            return;
        }
    }
    let revision = AVAILABLE_REVISION.fetch_add(1, Ordering::Release) + 1;
    if let Ok(mut g) = AVAILABLE.write() {
        *g = Some(Available { revision, tags });
    }
}

// ── the event log ───────────────────────────────────────────────────────────
//
// What the LOG tab shows. Deliberately not the same thing as the script
// console: `print` is the script talking, this is grimlua reporting what it
// did and what stopped it.

/// Events kept. A few hundred is a session's worth at the rate actions fire.
const EVENT_LINES: usize = 200;

#[derive(Clone, Serialize)]
pub struct Event {
    /// Seconds since the DLL loaded, so the browser can show a clock without
    /// the host needing one.
    pub at: f64,
    /// `action`, `blocked`, `script`, `config` -- the UI colours by this.
    pub kind: &'static str,
    pub text: String,
}

pub static EVENTS: RwLock<VecDeque<Event>> = RwLock::new(VecDeque::new());
static EVENT_SEQ: AtomicU64 = AtomicU64::new(0);
static STARTED: RwLock<Option<std::time::Instant>> = RwLock::new(None);

fn uptime() -> f64 {
    let mut guard = match STARTED.write() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    };
    let start = guard.get_or_insert_with(std::time::Instant::now);
    start.elapsed().as_secs_f64()
}

pub fn event(kind: &'static str, text: impl Into<String>) {
    let entry = Event { at: uptime(), kind, text: text.into() };
    if let Ok(mut e) = EVENTS.write() {
        if e.len() >= EVENT_LINES {
            e.pop_front();
        }
        e.push_back(entry);
    }
    EVENT_SEQ.fetch_add(1, Ordering::Relaxed);
}

pub fn events() -> (u64, Vec<Event>) {
    let lines = EVENTS.read().ok().map(|e| e.iter().cloned().collect()).unwrap_or_default();
    (EVENT_SEQ.load(Ordering::Relaxed), lines)
}

pub fn events_clear() {
    if let Ok(mut e) = EVENTS.write() {
        e.clear();
    }
    EVENT_SEQ.fetch_add(1, Ordering::Relaxed);
}

// ── the script console ──────────────────────────────────────────────────────

/// Lines kept from `print`. Enough to debug a priority list, small enough that
/// a runaway script cannot grow it without bound.
const CONSOLE_LINES: usize = 60;

pub static CONSOLE: RwLock<VecDeque<String>> = RwLock::new(VecDeque::new());
static CONSOLE_SEQ: AtomicU64 = AtomicU64::new(0);

pub fn console_write(lines: &[String]) {
    if lines.is_empty() {
        return;
    }
    if let Ok(mut c) = CONSOLE.write() {
        for line in lines {
            if c.len() >= CONSOLE_LINES {
                c.pop_front();
            }
            c.push_back(line.clone());
        }
    }
    CONSOLE_SEQ.fetch_add(lines.len() as u64, Ordering::Relaxed);
}

pub fn console_clear() {
    if let Ok(mut c) = CONSOLE.write() {
        c.clear();
    }
    CONSOLE_SEQ.fetch_add(1, Ordering::Relaxed);
}

/// The console plus a counter, so the browser can skip re-rendering when
/// nothing has been printed since the last push.
pub fn console() -> (u64, Vec<String>) {
    let lines = CONSOLE
        .read()
        .ok()
        .map(|c| c.iter().cloned().collect())
        .unwrap_or_default();
    (CONSOLE_SEQ.load(Ordering::Relaxed), lines)
}

// ── persistence ─────────────────────────────────────────────────────────────
//
// Rules survive a restart; `armed` never does. Without this, a scripting layer
// would be unusable -- nobody rebuilds a priority list every launch -- but
// restoring the master switch would mean the tool acts before its owner has
// looked at it.

static CONFIG_PATH: OnceLock<PathBuf> = OnceLock::new();

/// Set when the config on disk could not be read.
///
/// The autosave thread refuses to write while this is set, because the
/// alternative is what actually happened: a file that failed to parse fell back
/// to defaults, the autosave wrote those defaults a second later, and the
/// user's rules were gone. A file we could not understand is still the user's
/// work, and losing it is worse than not saving.
static LOAD_FAILED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn load_failed() -> bool {
    LOAD_FAILED.load(Ordering::Acquire)
}

pub fn config_path() -> Option<&'static Path> {
    CONFIG_PATH.get().map(|p| p.as_path())
}

/// Point persistence at `dir/grimlua.config.json` and load whatever is there.
///
/// Errors are reported and swallowed: a corrupt config file must not stop the
/// DLL from loading, it must fall back to the defaults.
pub fn load_from(dir: &Path) {
    let path = dir.join("grimlua.config.json");
    let _ = CONFIG_PATH.set(path.clone());

    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            crate::log!("config: no saved config, starting from defaults");
            return;
        }
        Err(e) => {
            LOAD_FAILED.store(true, Ordering::Release);
            crate::log!(
                "config: cannot read {} ({e}); using defaults and NOT saving over it",
                path.display()
            );
            return;
        }
    };

    // A byte-order mark is what a Windows text editor leaves behind, and
    // serde_json refuses it with an error pointing at column 1 rather than at
    // the invisible character causing it. Hand-editing this file is a
    // reasonable thing to do, so tolerate it.
    let text = text.strip_prefix('\u{feff}').unwrap_or(&text);

    match serde_json::from_str::<Config>(text) {
        Ok(mut cfg) => {
            cfg.armed = false; // never restored, by design
            cfg.sanitize();
            let n = cfg.rules.len();
            let mode = if cfg.mode == ScriptMode::Rules { "rules" } else { "lua" };
            put_config(cfg);
            crate::log!("config: loaded {n} rule(s) in {mode} mode, disarmed");
        }
        Err(e) => {
            // Keep the unreadable file rather than letting defaults replace it.
            let kept = path.with_extension("bad.json");
            let note = match std::fs::rename(&path, &kept) {
                Ok(()) => format!("kept a copy at {}", kept.display()),
                Err(e) => format!("could not set it aside: {e}"),
            };
            LOAD_FAILED.store(true, Ordering::Release);
            crate::log!("config: {} is not readable ({e}); {note}", path.display());
        }
    }
}

/// Write the live config out. Returns false if there is nowhere to write it.
pub fn save() -> bool {
    if load_failed() {
        return false;
    }
    let Some(path) = config_path() else { return false };
    let mut cfg = config();
    cfg.armed = false; // so a crash while armed cannot arm the next launch
    let Ok(text) = serde_json::to_string_pretty(&cfg) else { return false };
    match std::fs::write(path, text) {
        Ok(()) => true,
        Err(e) => {
            crate::log!("config: could not save to {} ({e})", path.display());
            false
        }
    }
}

/// Persist the config a second after it last changed.
///
/// A separate thread rather than a write on the websocket's message path: a
/// slider being dragged sends a config per frame, and none of them should
/// touch the disk.
pub fn run_autosave() {
    let mut saved = 0u64;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        // A config that could not be read is not overwritten by defaults. The
        // user can still edit in the browser; the first deliberate change
        // clears the block, because by then they have seen the state of it.
        let current = config_revision();
        if current != saved && save() {
            saved = current;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_stock_config_generates_a_script_that_compiles() {
        let (source, marks) = Config::default().program();
        assert!(script::Evaluator::compile(&source).is_ok(), "{source}");
        // Only the health rule is enabled out of the box.
        assert_eq!(marks.len(), 1);
    }

    #[test]
    fn the_stock_hand_written_script_compiles_too() {
        assert!(script::Evaluator::compile(DEFAULT_SCRIPT).is_ok());
    }

    #[test]
    fn sanitize_pulls_a_silly_cooldown_into_range() {
        let mut cfg = Config { global_cooldown_ms: 0, ..Default::default() };
        cfg.sanitize();
        assert_eq!(cfg.global_cooldown_ms, MIN_GLOBAL_COOLDOWN_MS);
    }

    #[test]
    fn sanitize_truncates_an_oversized_script_on_a_char_boundary() {
        let mut cfg = Config { script: "é".repeat(script::MAX_SOURCE), ..Default::default() };
        cfg.sanitize();
        assert!(cfg.script.len() <= script::MAX_SOURCE);
    }

    /// The editor opens filtered to what the character actually has, because
    /// that is the common case; the full database is one click away.
    #[test]
    fn the_editor_defaults_to_showing_only_available_skills() {
        assert!(Config::default().editor.only_available);
    }

    /// A skill action names a record path, never a bar position -- the whole
    /// point of invoking skills directly.
    #[test]
    fn a_skill_action_round_trips_as_a_record_path() {
        let spec = ActionSpec::Skill { path: "records/skills/playerclass03/bloodofdreeg1.dbr".into() };
        assert_eq!(
            spec.token(),
            "skill:records/skills/playerclass03/bloodofdreeg1.dbr"
        );
        assert_eq!(
            crate::runtime::Action::parse(&spec.token()),
            Some(crate::runtime::Action::Skill(
                "records/skills/playerclass03/bloodofdreeg1.dbr".into()
            ))
        );
    }

    #[test]
    fn a_config_round_trips_through_json() {
        let cfg = Config::default();
        let text = serde_json::to_string(&cfg).unwrap();
        let back: Config = serde_json::from_str(&text).unwrap();
        assert_eq!(back.rules, cfg.rules);
        assert_eq!(back.mode, cfg.mode);
    }

    /// The one test that touches the process-wide config and the one that sets
    /// `CONFIG_PATH`, which is a `OnceLock`. Kept as a single test so it cannot
    /// interleave with another that does the same.
    #[test]
    fn saved_rules_come_back_but_armed_never_does() {
        let dir = std::env::temp_dir().join(format!("grimlua-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();

        let mut saved = Config {
            armed: true,
            global_cooldown_ms: 1234,
            mode: ScriptMode::Script,
            ..Default::default()
        };
        saved.rules.push(Rule::new("r9", "Rotation", ActionSpec::Skill { path: "records/skills/playerclass03/bloodofdreeg1.dbr".into() }));
        std::fs::write(
            dir.join("grimlua.config.json"),
            serde_json::to_string(&saved).unwrap(),
        )
        .unwrap();

        load_from(&dir);
        let live = config();
        assert!(!live.armed, "a saved config must never arm the tool on load");
        assert_eq!(live.global_cooldown_ms, 1234);
        assert_eq!(live.mode, ScriptMode::Script);
        assert_eq!(live.rules.len(), 3);
        assert_eq!(live.rules[2].action, ActionSpec::Skill { path: "records/skills/playerclass03/bloodofdreeg1.dbr".into() });

        // And writing it back out keeps `armed` false even if it is on now.
        put_config(Config { armed: true, ..live });
        assert!(save());
        let text = std::fs::read_to_string(dir.join("grimlua.config.json")).unwrap();
        let back: Config = serde_json::from_str(&text).unwrap();
        assert!(!back.armed, "an armed session must not be written to disk");

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A Windows text editor leaves a byte-order mark, and the error serde
    /// gives for one points at column 1 rather than at the invisible cause.
    #[test]
    fn a_config_with_a_byte_order_mark_still_loads() {
        let body = serde_json::to_string(&Config::default()).unwrap();
        let with_bom = format!("{}{}", '\u{feff}', body);

        assert!(
            serde_json::from_str::<Config>(&with_bom).is_err(),
            "if serde ever accepts a mark itself, the strip below can go"
        );
        let stripped = with_bom.strip_prefix('\u{feff}').unwrap();
        assert!(serde_json::from_str::<Config>(stripped).is_ok());
    }

    #[test]
    fn a_config_missing_fields_falls_back_to_defaults() {
        let back: Config = serde_json::from_str("{\"armed\":true}").unwrap();
        assert!(back.armed);
        assert_eq!(back.global_cooldown_ms, Config::default().global_cooldown_ms);
        assert_eq!(back.rules.len(), Config::default().rules.len());
    }
}
