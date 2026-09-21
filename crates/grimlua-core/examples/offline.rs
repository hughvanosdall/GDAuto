//! A fake Grim Dawn, so the web UI and the scripting layer can be worked on
//! without launching the game.
//!
//!     cargo run --example offline
//!     …then open http://127.0.0.1:7890
//!
//! This is the same idea as the external read mode in CLAUDE.md: keep a way to
//! iterate that does not need a running game. Everything here is real except
//! the game itself -- the real server, the real config path, the real code
//! generator and the real sandboxed Lua VM. Only the vitals are invented and
//! only the actions are pretended.
//!
//! What it is **not** is a test of the frame hook, the gate's UI checks, or
//! any game call. A rule that works here still has to be watched working in a
//! live game before it means anything.

use std::time::{Duration, Instant};

use grimlua_core::script::{Evaluator, ScriptState};
use grimlua_core::shared::{self, Gate, PotionStatus, ScriptStatus, Snapshot, Vitals};
use grimlua_core::{runtime::Action, server};

const TICK: Duration = Duration::from_millis(50);
const LIFE_MAX: f32 = 13_746.0;
const ENERGY_MAX: f32 = 2_591.0;

fn main() {
    println!("grimlua offline simulator");
    println!("  open http://127.0.0.1:{}", server::PORT);
    println!("  arm it, and watch the rules fire against invented vitals");

    std::thread::spawn(server::start);

    // The real skill database, if the game is installed, so the picker and the
    // availability filter can be exercised without launching anything.
    std::thread::spawn(load_skills);

    Character::default().run();
}

/// Parse the installed game's database and pretend a character has some of it.
fn load_skills() {
    let Some(root) = game_root() else {
        println!("skills: Grim Dawn not found, so the picker will be empty");
        return;
    };
    let files = grimlua_core::db::locate(&root);
    if files.archives.is_empty() {
        println!("skills: no archives under {}", root.display());
        return;
    }
    let index = grimlua_core::db::build(&files);

    // Stand in for a character: one mastery's worth of castable skills.
    let mine: Vec<String> = index
        .skills
        .iter()
        .filter(|s| s.mastery == Some(3) && s.castable)
        .map(|s| s.tag.clone())
        .collect();
    println!("skills: {} in the database, pretending {} are on the character", index.skills.len(), mine.len());

    shared::put_skills(index);
    shared::put_available(mine);
}

fn game_root() -> Option<std::path::PathBuf> {
    for base in [
        r"C:\Program Files (x86)\Steam\steamapps\common\Grim Dawn",
        r"C:\Program Files\Steam\steamapps\common\Grim Dawn",
    ] {
        let p = std::path::PathBuf::from(base);
        if p.join("database").join("database.arz").is_file() {
            return Some(p);
        }
    }
    None
}

struct Character {
    life: f64,
    energy: f64,
    evaluator: Option<Evaluator>,
    compiled: Option<u64>,
    compile_error: Option<String>,
    source: String,
    marks: Vec<grimlua_core::rules::LineMark>,

    started: Instant,
    last_action_at: Option<Instant>,
    last_action: Option<String>,
    last_rule: Option<String>,
    since_action: std::collections::HashMap<String, Instant>,
    since_rule: std::collections::HashMap<String, Instant>,

    actions: u64,
    blocked: u64,
    evaluations: u64,
    errors: u64,
    last_error: Option<String>,
    last_eval_us: u64,
    frame: u64,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            life: LIFE_MAX as f64,
            energy: ENERGY_MAX as f64,
            evaluator: None,
            compiled: None,
            compile_error: None,
            source: String::new(),
            marks: Vec::new(),
            started: Instant::now(),
            last_action_at: None,
            last_action: None,
            last_rule: None,
            since_action: Default::default(),
            since_rule: Default::default(),
            actions: 0,
            blocked: 0,
            evaluations: 0,
            errors: 0,
            last_error: None,
            last_eval_us: 0,
            frame: 0,
        }
    }
}

impl Character {
    fn run(&mut self) -> ! {
        loop {
            self.tick();
            std::thread::sleep(TICK);
        }
    }

    fn tick(&mut self) {
        let now = Instant::now();
        self.frame += 1;
        let cfg = shared::config();
        let revision = shared::config_revision();
        if self.compiled != Some(revision) {
            self.recompile(&cfg, revision);
        }

        self.take_damage();

        // A simplified gate: only the parts that exist without a game. The UI
        // checks and the focus check have nothing to look at here.
        let verdict = if !cfg.armed {
            Gate::Disarmed
        } else if self
            .last_action_at
            .is_some_and(|t| now - t < Duration::from_millis(cfg.global_cooldown_ms))
        {
            Gate::Cooldown
        } else {
            Gate::Clear
        };

        let chosen = self.evaluate(now, &cfg);
        let pending_label = chosen
            .as_ref()
            .and_then(|(t, _)| Action::parse(t))
            .map(|a| a.label());
        let pending_rule = chosen.as_ref().and_then(|(_, r)| r.clone());

        if verdict.allows_action() {
            if let Some((token, rule)) = chosen {
                self.perform(&token, rule, now);
            }
        } else if pending_label.is_some() && verdict != Gate::Disarmed {
            self.blocked += 1;
        }

        shared::put_snapshot(Snapshot {
            in_world: true,
            vitals: Vitals {
                life: self.life,
                life_max: LIFE_MAX,
                energy: self.energy as f32,
                energy_max: ENERGY_MAX,
            },
            gate: Some(verdict),
            gate_reason: verdict.reason().into(),
            armed: cfg.armed,
            actions: self.actions,
            blocked: self.blocked,
            last_action: self.last_action.clone(),
            last_rule: self.last_rule.clone(),
            pending_action: pending_label,
            pending_rule,
            potions: PotionStatus { health: Some(0), energy: Some(0) },
            combat: self.fake_combat(),
            skills_available: false,
            // Invented, like the vitals, so the dashboard tile has something
            // to draw. Nothing here talks to a game.
            dps: Some(38_000.0 + (self.started.elapsed().as_secs_f32() * 3.0) % 6_000.0),
            frame: self.frame,
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
            halted: false,
            last_eval_us: self.last_eval_us,
        });
    }

    fn recompile(&mut self, cfg: &shared::Config, revision: u64) {
        let (source, marks) = cfg.program();
        self.compiled = Some(revision);
        self.source = source;
        self.marks = marks;
        self.last_error = None;
        match Evaluator::compile(&self.source) {
            Ok(ev) => {
                self.evaluator = Some(ev);
                self.compile_error = None;
                shared::event("script", format!("revision {revision} compiled"));
                println!("compiled revision {revision}");
            }
            Err(e) => {
                self.evaluator = None;
                shared::event("script", format!("will not compile: {e}"));
                println!("revision {revision} will not compile: {e}");
                self.compile_error = Some(e);
            }
        }
    }

    /// Life drains steadily with a periodic spike, energy drains steadily.
    /// Deterministic rather than random, so a rule that fires once fires every
    /// time and a change in behaviour is a change you made.
    fn take_damage(&mut self) {
        let t = self.started.elapsed().as_secs_f64();
        let spike = if (t % 9.0) < 0.1 { 2_400.0 } else { 0.0 };
        self.life = (self.life - 55.0 - spike).max(1.0);
        self.energy = (self.energy - 9.0).max(0.0);

        // Slow regeneration, so a character left alone climbs back.
        self.life = (self.life + 18.0).min(LIFE_MAX as f64);
        self.energy = (self.energy + 4.0).min(ENERGY_MAX as f64);
    }

    /// Invented combat state on a slow cycle, so a rule keyed on `in_combat`
    /// or `has_target` can be seen switching without a game running.
    fn fake_combat(&self) -> grimlua_core::live::Combat {
        let t = self.started.elapsed().as_secs_f64();
        let fighting = (t % 20.0) < 12.0;
        grimlua_core::live::Combat {
            target: if fighting { 4242 } else { 0 },
            has_target: fighting,
            under_attack: fighting && (t % 20.0) > 4.0,
            moving: !fighting,
            attack_moving: fighting,
            since_attack_ms: fighting.then_some(200),
        }
    }

    fn evaluate(&mut self, now: Instant, _cfg: &shared::Config) -> Option<(String, Option<String>)> {
        let evaluator = self.evaluator.as_ref()?;
        let state = ScriptState {
            life: self.life,
            life_max: LIFE_MAX as f64,
            life_pct: self.life / LIFE_MAX as f64 * 100.0,
            energy: self.energy,
            energy_max: ENERGY_MAX as f64,
            energy_pct: self.energy / ENERGY_MAX as f64 * 100.0,
            frame: self.frame,
            uptime: self.started.elapsed().as_secs_f64(),
            since_any: self
                .last_action_at
                .map(|t| (now - t).as_secs_f64())
                .unwrap_or(f64::INFINITY),
            last_action: self.last_action.clone(),
            health_potion_status: Some(0),
            energy_potion_status: Some(0),
            in_combat: self.fake_combat().in_combat(4_000),
            has_target: self.fake_combat().has_target,
            under_attack: self.fake_combat().under_attack,
            moving: self.fake_combat().moving,
            target: self.fake_combat().target,
            skill_ready: Default::default(),
            skill_cooldown: Default::default(),
            since_action: elapsed(&self.since_action, now),
            since_rule: elapsed(&self.since_rule, now),
        };

        let started = Instant::now();
        let result = evaluator.evaluate(&state);
        self.last_eval_us = started.elapsed().as_micros() as u64;
        self.evaluations += 1;

        match result {
            Ok(outcome) => {
                shared::console_write(&outcome.prints);
                outcome.action.map(|a| (a, outcome.rule))
            }
            Err(e) => {
                self.errors += 1;
                self.last_error = Some(e);
                None
            }
        }
    }

    fn perform(
        &mut self,
        token: &str,
        rule: Option<String>,
        now: Instant,
    ) {
        let Some(action) = Action::parse(token) else {
            self.last_error = Some(format!("\"{token}\" is not an action"));
            return;
        };
        // No game here, so a cast is only ever reported, never performed.
        if let Action::Skill(path) = &action {
            let note = format!("offline: would cast {path}");
            if self.last_error.as_deref() != Some(note.as_str()) {
                shared::event("blocked", note.clone());
            }
            self.last_error = Some(note);
        }

        match &action {
            Action::HealthPotion => self.life = (self.life + 4_500.0).min(LIFE_MAX as f64),
            Action::EnergyPotion => self.energy = (self.energy + 900.0).min(ENERGY_MAX as f64),
            Action::Skill(_) => self.energy = (self.energy - 120.0).max(0.0),
        }

        self.actions += 1;
        shared::event("action", action.label());
        self.last_action_at = Some(now);
        self.last_action = Some(action.label());
        self.last_rule = rule.clone();
        self.since_action.insert(action.token(), now);
        if let Some(rule) = rule {
            self.since_rule.insert(rule, now);
        }
        self.last_error = None;
    }
}

fn elapsed(
    map: &std::collections::HashMap<String, Instant>,
    now: Instant,
) -> std::collections::HashMap<String, f64> {
    map.iter()
        .map(|(k, t)| (k.clone(), now.saturating_duration_since(*t).as_secs_f64()))
        .collect()
}
