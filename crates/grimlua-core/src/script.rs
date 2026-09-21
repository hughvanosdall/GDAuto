//! The sandboxed Lua evaluator.
//!
//! Four properties this module exists to guarantee, all of them restated from
//! CLAUDE.md because they are the reason the scripting layer is worth having
//! at all rather than a keyboard macro:
//!
//! 1. **A script returns intent, it never acts.** `choose` is handed a table
//!    of plain numbers and returns an action *name*. There is no host function
//!    in the environment that can reach Grim Dawn -- not even indirectly --
//!    so the script is a pure function from a state snapshot to a token, and
//!    the host decides what that token is worth.
//! 2. **At most one action.** `choose` returns one token. A list would be a
//!    design error: the gate serialises everything, so the second element
//!    could never happen.
//! 3. **The gate cannot be bypassed.** Nothing here performs anything, so
//!    there is no per-call opt-out for a script author to forget.
//! 4. **A misbehaving script must not stall the frame hook.** This runs on the
//!    game's render thread, so `while true do end` is a hang *in the game*.
//!    Lua's count hook is re-armed before every call, which turns that into an
//!    error return after a bounded number of VM instructions.
//!
//! The environment is built by subtraction from a small standard library:
//! `math`, `string` and `table` only, then the reachable escape hatches
//! removed by name. There is no Rust<->Lua object bridge at all -- the earlier
//! Python prototype's whole class of `__class__.__mro__` escapes does not
//! exist here, because nothing but numbers, strings and tables ever crosses.

use std::collections::HashMap;

use mlua::{Function, HookTriggers, Lua, LuaOptions, StdLib, Table, Value};

/// The global function a script must define.
pub const ENTRY: &str = "choose";

/// VM instructions one `choose` call may execute before it is killed. A
/// realistic priority list is a few hundred; this is three orders of magnitude
/// of headroom and still returns in well under a frame.
const INSTRUCTION_BUDGET: u32 = 200_000;

/// Hard ceiling on the VM's heap, which is what stops `("x"):rep(1e9)` from
/// taking the game down. `string.rep` is removed as well -- belt and braces,
/// because the cost of being wrong here is someone's hardcore character.
const MEMORY_LIMIT: usize = 4 * 1024 * 1024;

/// Longest script accepted from the browser.
pub const MAX_SOURCE: usize = 64 * 1024;

/// Lines of `print` output kept per evaluation. A script that prints in a loop
/// is a script trying to flood the log.
const MAX_PRINTS_PER_EVAL: usize = 8;

/// Names reachable from the base library that must not be. `load` and friends
/// would let a shared script carry a second, unreviewed script; `package`
/// carries `loadlib`, which is a direct DLL load and therefore a malware
/// channel in a file people trade.
const BANNED: [&str; 11] = [
    "load",
    "loadstring",
    "dofile",
    "loadfile",
    "require",
    "package",
    "io",
    "os",
    "debug",
    "newproxy",
    "collectgarbage",
];

/// Everything a script is shown. Assembled by the caller on the frame hook
/// *before* the VM runs, which is what makes the script pure: by the time Lua
/// has control there is nothing left to ask the game.
#[derive(Clone, Default)]
pub struct ScriptState {
    pub life: f64,
    pub life_max: f64,
    pub life_pct: f64,
    pub energy: f64,
    pub energy_max: f64,
    pub energy_pct: f64,
    pub frame: u64,
    /// Seconds since the DLL started ticking.
    pub uptime: f64,
    /// Seconds since any action fired, `f64::INFINITY` if none has.
    pub since_any: f64,
    pub last_action: Option<String>,
    /// Raw `HotSlotOptionStatus` for the potion slots, if readable.
    pub health_potion_status: Option<i64>,
    pub energy_potion_status: Option<i64>,
    // ── combat, which is the thing a rotation actually keys off ──────────
    pub in_combat: bool,
    pub has_target: bool,
    pub under_attack: bool,
    pub moving: bool,
    /// Entity id of the attack target, 0 for none.
    pub target: u32,

    // ── per-skill, keyed by record path ──────────────────────────────────
    pub skill_ready: HashMap<String, bool>,
    /// Milliseconds of cooldown left; 0 when ready.
    pub skill_cooldown: HashMap<String, f64>,
    /// Seconds since each action token last fired.
    pub since_action: HashMap<String, f64>,
    /// Seconds since each rule last fired.
    pub since_rule: HashMap<String, f64>,
}

/// What one evaluation produced.
#[derive(Clone, Default, PartialEq, Debug)]
pub struct Outcome {
    /// The action token, unvalidated. The host decides whether it means
    /// anything.
    pub action: Option<String>,
    /// The rule that produced it, when the script says. Generated scripts
    /// always do; a hand-written one need not.
    pub rule: Option<String>,
    pub prints: Vec<String>,
}

pub struct Evaluator {
    lua: Lua,
    choose: Function,
}

impl Evaluator {
    /// Build a VM, harden it, run the script's top level, and take its
    /// `choose`.
    ///
    /// The top-level chunk runs under the same instruction budget as a call:
    /// `while true do end` at file scope is just as good a way to hang the
    /// render thread as one inside the function.
    pub fn compile(source: &str) -> Result<Self, String> {
        if source.len() > MAX_SOURCE {
            return Err(format!(
                "script is {} bytes; the limit is {MAX_SOURCE}",
                source.len()
            ));
        }

        let lua = Lua::new_with(
            StdLib::MATH | StdLib::STRING | StdLib::TABLE,
            LuaOptions::default(),
        )
        .map_err(|e| format!("could not start the Lua VM: {e}"))?;

        let _ = lua.set_memory_limit(MEMORY_LIMIT);
        harden(&lua)?;
        arm(&lua)?;

        lua.load(source)
            .set_name("grimlua")
            .exec()
            .map_err(|e| readable(&e))?;

        let choose: Function = lua.globals().get(ENTRY).map_err(|_| {
            format!("the script must define a global function `{ENTRY}(s)`")
        })?;

        Ok(Self { lua, choose })
    }

    /// Run `choose` once.
    ///
    /// Re-arms the instruction hook first: `lua_sethook` resets the countdown,
    /// so every call gets a whole budget rather than inheriting whatever the
    /// previous one left.
    pub fn evaluate(&self, state: &ScriptState) -> Result<Outcome, String> {
        take_prints(); // drop anything a previous failed call left behind
        arm(&self.lua)?;

        let table = build_state(&self.lua, state).map_err(|e| readable(&e))?;
        let (action, rule): (Value, Value) =
            self.choose.call(table).map_err(|e| readable(&e))?;

        let prints = take_prints();
        let rule = match rule {
            Value::String(s) => s.to_str().ok().map(|s| s.to_string()),
            _ => None,
        };

        match action {
            Value::Nil | Value::Boolean(false) => Ok(Outcome { action: None, rule: None, prints }),
            Value::String(s) => {
                let token = s
                    .to_str()
                    .map_err(|_| "the action name was not valid text".to_string())?
                    .to_string();
                Ok(Outcome { action: Some(token), rule, prints })
            }
            other => Err(format!(
                "`{ENTRY}` must return an action name or nil, but returned a {}",
                other.type_name()
            )),
        }
    }
}

/// Remove everything reachable that should not be, and install `print`.
fn harden(lua: &Lua) -> Result<(), String> {
    let globals = lua.globals();
    for name in BANNED {
        globals.set(name, Value::Nil).map_err(|e| readable(&e))?;
    }

    // `string.rep` is a memory bomb and `string.dump` leaks bytecode. Removing
    // them from the `string` table also removes them from the shared string
    // metatable, so `("x"):rep(n)` goes with it.
    if let Ok(string) = globals.get::<Table>("string") {
        let _ = string.set("rep", Value::Nil);
        let _ = string.set("dump", Value::Nil);
    }

    // The one host function in the environment. It reaches a ring buffer and
    // nothing else -- no game state, no file system, no network.
    let print = lua
        .create_function(|_, args: mlua::Variadic<Value>| {
            let parts: Vec<String> = args
                .iter()
                .map(|v| v.to_string().unwrap_or_else(|_| "?".into()))
                .collect();
            push_print(parts.join("\t"));
            Ok(())
        })
        .map_err(|e| readable(&e))?;
    globals.set("print", print).map_err(|e| readable(&e))?;

    Ok(())
}

/// (Re-)install the instruction-count hook, resetting its countdown.
fn arm(lua: &Lua) -> Result<(), String> {
    lua.set_hook(
        HookTriggers::default().every_nth_instruction(INSTRUCTION_BUDGET),
        |_, _| {
            Err(mlua::Error::RuntimeError(format!(
                "script ran for more than {INSTRUCTION_BUDGET} instructions and was stopped \
                 (an endless loop here would freeze the game)"
            )))
        },
    )
    .map_err(|e| readable(&e))
}

/// Turn [`ScriptState`] into the table `choose` receives.
fn build_state(lua: &Lua, state: &ScriptState) -> mlua::Result<Table> {
    let s = lua.create_table()?;
    s.set("life", state.life)?;
    s.set("life_max", state.life_max)?;
    s.set("life_pct", state.life_pct)?;
    s.set("energy", state.energy)?;
    s.set("energy_max", state.energy_max)?;
    s.set("energy_pct", state.energy_pct)?;
    s.set("frame", state.frame as f64)?;
    s.set("uptime", state.uptime)?;
    s.set("since_any", state.since_any)?;
    match &state.last_action {
        Some(a) => s.set("last_action", a.as_str())?,
        None => s.set("last_action", Value::Nil)?,
    }

    let potion = lua.create_table()?;
    potion.set("health", state.health_potion_status)?;
    potion.set("energy", state.energy_potion_status)?;
    s.set("potion", potion)?;

    // Booleans are exposed as 0/1 so a generated condition can compare them
    // with the same operators as everything else.
    s.set("in_combat", if state.in_combat { 1 } else { 0 })?;
    s.set("has_target", if state.has_target { 1 } else { 0 })?;
    s.set("under_attack", if state.under_attack { 1 } else { 0 })?;
    s.set("moving", if state.moving { 1 } else { 0 })?;
    s.set("target", state.target)?;

    let ready = lua.create_table()?;
    for (path, is_ready) in &state.skill_ready {
        ready.set(path.as_str(), if *is_ready { 1 } else { 0 })?;
    }
    // A skill the character does not have reads as "not ready" rather than
    // nil, so a rule naming a skill from another build does not error.
    s.set("skill_ready", default_to(lua, ready, 0.0)?)?;

    let cooldown = lua.create_table()?;
    for (path, ms) in &state.skill_cooldown {
        cooldown.set(path.as_str(), *ms)?;
    }
    // Unknown skill: forever on cooldown, which is the safe reading.
    s.set("skill_cooldown", default_to(lua, cooldown, f64::INFINITY)?)?;

    // Buff state is not yet read from the game; a condition on it is false
    // rather than an error, so a rule using it simply never fires until it is.
    let buffs = lua.create_table()?;
    s.set("buff_active", default_to(lua, buffs, 0.0)?)?;

    let since_action = lua.create_table()?;
    for (k, v) in &state.since_action {
        since_action.set(k.as_str(), *v)?;
    }
    s.set("since", default_to(lua, since_action, f64::INFINITY)?)?;

    let since_rule = lua.create_table()?;
    for (k, v) in &state.since_rule {
        since_rule.set(k.as_str(), *v)?;
    }
    s.set("since_rule", default_to(lua, since_rule, f64::INFINITY)?)?;

    Ok(s)
}

/// Give a lookup table a default, so a miss reads as a number rather than as
/// `nil`.
///
/// Without this, the first evaluation of a generated rule would compare `nil`
/// against a number and error -- a rule that has never fired is exactly the
/// case a cooldown test has to handle, so `math.huge` ("forever ago") is the
/// only sane answer.
fn default_to(lua: &Lua, table: Table, default: f64) -> mlua::Result<Table> {
    let meta = lua.create_table()?;
    meta.set(
        "__index",
        lua.create_function(move |_, (_, _): (Table, Value)| Ok(default))?,
    )?;
    table.set_metatable(Some(meta))?;
    Ok(table)
}

// ── print capture ───────────────────────────────────────────────────────────
//
// Thread-local, because a VM only ever runs on the frame hook and a global
// lock here would be a lock the game thread waits on.

thread_local! {
    static PRINTS: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

fn push_print(line: String) {
    PRINTS.with(|p| {
        let mut p = p.borrow_mut();
        if p.len() < MAX_PRINTS_PER_EVAL {
            p.push(line.chars().take(200).collect());
        } else if p.len() == MAX_PRINTS_PER_EVAL {
            p.push("… further output from this evaluation dropped".into());
        }
    });
}

fn take_prints() -> Vec<String> {
    PRINTS.with(|p| std::mem::take(&mut *p.borrow_mut()))
}

/// Make an mlua error fit on one line of a web page.
///
/// mlua spells the chunk name `[string "grimlua"]` and appends a traceback;
/// neither helps someone looking at a fifteen-line priority list, but the line
/// number very much does.
fn readable(err: &mlua::Error) -> String {
    let text = err.to_string();
    let text = text.replace("[string \"grimlua\"]:", "line ");
    let head = text
        .split("stack traceback:")
        .next()
        .unwrap_or(&text)
        .trim()
        .to_string();
    let head = head.strip_prefix("runtime error: ").unwrap_or(&head).to_string();
    head.chars().take(400).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `Evaluator` is deliberately not `Debug` -- a Lua VM has nothing
    /// useful to print -- so the failure cases go through this.
    fn compile_err(source: &str) -> String {
        Evaluator::compile(source).err().expect("should not have compiled")
    }

    fn state() -> ScriptState {
        ScriptState {
            life: 500.0,
            life_max: 1000.0,
            life_pct: 50.0,
            energy: 90.0,
            energy_max: 300.0,
            energy_pct: 30.0,
            since_any: f64::INFINITY,
            ..Default::default()
        }
    }

    #[test]
    fn a_generated_script_round_trips() {
        use crate::rules::{ActionSpec, Op, Rule, Subject};
        let generated = crate::rules::generate(&[Rule::new(
            "r1",
            "flask",
            ActionSpec::HealthPotion,
        )
        .when(Subject::LifePct, Op::Below, 55.0)]);

        let ev = Evaluator::compile(&generated.source).expect("compiles");
        let out = ev.evaluate(&state()).expect("runs");
        assert_eq!(out.action.as_deref(), Some("health_potion"));
        assert_eq!(out.rule.as_deref(), Some("r1"));
    }

    #[test]
    fn nothing_matching_returns_nothing() {
        let ev = Evaluator::compile("function choose(s) if s.life_pct < 10 then return 'health_potion' end end")
            .expect("compiles");
        assert_eq!(ev.evaluate(&state()).unwrap().action, None);
    }

    #[test]
    fn an_endless_loop_is_stopped_rather_than_hanging() {
        let ev = Evaluator::compile("function choose(s) while true do end end").expect("compiles");
        let err = ev.evaluate(&state()).unwrap_err();
        assert!(err.contains("instructions"), "{err}");
    }

    #[test]
    fn the_budget_is_fresh_for_every_call() {
        // Just under the budget each time: a shared countdown would trip on a
        // later call even though no single call is expensive.
        let ev = Evaluator::compile(
            "function choose(s) local n = 0 for i = 1, 20000 do n = n + i end return nil end",
        )
        .expect("compiles");
        for i in 0..20 {
            assert!(ev.evaluate(&state()).is_ok(), "call {i} was killed");
        }
    }

    #[test]
    fn a_missing_entry_point_is_a_compile_error() {
        let err = compile_err("local x = 1");
        assert!(err.contains("choose"), "{err}");
    }

    #[test]
    fn a_syntax_error_reports_its_line() {
        let err = compile_err("function choose(s)\n  if then\nend");
        assert!(err.contains("line 2"), "{err}");
    }

    #[test]
    fn the_escape_hatches_are_gone() {
        for name in BANNED {
            let src = format!("function choose(s) return {name} end");
            let ev = Evaluator::compile(&src).expect("compiles");
            // Reaching a removed global yields nil, which is "no action".
            assert_eq!(ev.evaluate(&state()).unwrap().action, None, "{name} survived");
        }
    }

    #[test]
    fn string_rep_cannot_be_used_as_a_memory_bomb() {
        let ev = Evaluator::compile("function choose(s) return ('x'):rep(1e9) end").expect("compiles");
        assert!(ev.evaluate(&state()).is_err());
    }

    #[test]
    fn a_never_fired_rule_reads_as_forever_ago() {
        let ev = Evaluator::compile(
            "function choose(s) if s.since_rule['never'] >= 5 then return 'health_potion' end end",
        )
        .expect("compiles");
        assert_eq!(ev.evaluate(&state()).unwrap().action.as_deref(), Some("health_potion"));
    }

    /// A rule can name a skill this character does not have -- a priority list
    /// shared by someone else, or one written for another build. That must read
    /// as "not ready" and "forever on cooldown", never as a Lua error that
    /// takes the whole script down.
    #[test]
    fn a_skill_the_character_lacks_reads_as_unavailable_not_nil() {
        let ev = Evaluator::compile(
            "function choose(s)
               if s.skill_ready['records/skills/nope.dbr'] < 1
                  and s.skill_cooldown['records/skills/nope.dbr'] > 1000 then
                 return 'energy_potion'
               end
             end",
        )
        .expect("compiles");
        assert_eq!(ev.evaluate(&state()).unwrap().action.as_deref(), Some("energy_potion"));
    }

    /// The combat state a rotation keys off is present and numeric.
    #[test]
    fn combat_state_reaches_the_script_as_numbers() {
        let ev = Evaluator::compile(
            "function choose(s) if s.in_combat >= 1 and s.has_target >= 1 then              return 'health_potion' end end",
        )
        .expect("compiles");

        let mut fighting = state();
        fighting.in_combat = true;
        fighting.has_target = true;
        assert_eq!(ev.evaluate(&fighting).unwrap().action.as_deref(), Some("health_potion"));
        assert_eq!(ev.evaluate(&state()).unwrap().action, None);
    }

    #[test]
    fn returning_the_wrong_type_is_an_error_not_an_action() {
        let ev = Evaluator::compile("function choose(s) return 42 end").expect("compiles");
        assert!(ev.evaluate(&state()).unwrap_err().contains("action name"));
    }

    #[test]
    fn print_is_captured_rather_than_lost() {
        let ev = Evaluator::compile("function choose(s) print('life', s.life_pct) end")
            .expect("compiles");
        let out = ev.evaluate(&state()).unwrap();
        assert_eq!(out.prints, vec!["life\t50".to_string()]);
    }

    #[test]
    fn print_flooding_is_capped() {
        let ev = Evaluator::compile("function choose(s) for i = 1, 100 do print(i) end end")
            .expect("compiles");
        assert_eq!(ev.evaluate(&state()).unwrap().prints.len(), MAX_PRINTS_PER_EVAL + 1);
    }

    #[test]
    fn an_oversized_script_is_refused_before_it_is_parsed() {
        let err = compile_err(&"-".repeat(MAX_SOURCE + 1));
        assert!(err.contains("limit"), "{err}");
    }
}
