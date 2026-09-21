//! The rule IR and the Lua it compiles to.
//!
//! This is the "form-based rule builder" half of the scripting layer, and it
//! is deliberately the *first* front end rather than the node canvas: same IR,
//! same code generator, a fraction of the work. When the canvas arrives it
//! produces these same [`Rule`] values and nothing downstream changes.
//!
//! One direction only. The rules are the source of truth and the Lua is a
//! build artefact -- there is no decompiler and there will not be one. A user
//! who wants to edit the Lua switches the whole config to script mode and owns
//! it from then on.
//!
//! Generation also emits a source map (rule id -> line number) so the web UI
//! can point at the line a rule became, and light up the rule that just fired.

use serde::{Deserialize, Serialize};

/// Ceilings on anything the browser can send. These are not stylistic: the
/// generated script runs on the game's render thread, so an unbounded rule
/// list is an unbounded per-frame cost.
pub const MAX_RULES: usize = 64;
pub const MAX_CONDITIONS: usize = 8;
/// Longest record path a rule may name. Real ones are ~50 characters.
pub const MAX_PATH: usize = 200;

/// What a rule does when it fires. A closed set the host owns -- a rule names
/// an action, it never carries one.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ActionSpec {
    HealthPotion,
    EnergyPotion,
    /// Cast a skill, named by its database record path.
    ///
    /// A record path, never a hot-bar position: grimlua invokes the skill
    /// directly, so a rule does not care whether the skill is on the bar, where
    /// it sits, or what the player bound it to.
    Skill { path: String },
}

impl ActionSpec {
    /// The token the generated Lua returns and [`crate::runtime::Action`]
    /// parses. Keeping one spelling for both directions means an action added
    /// to the enum is immediately writable by hand in script mode too.
    pub fn token(&self) -> String {
        match self {
            ActionSpec::HealthPotion => "health_potion".into(),
            ActionSpec::EnergyPotion => "energy_potion".into(),
            ActionSpec::Skill { path } => format!("skill:{path}"),
        }
    }

    /// The skill this action casts, if it casts one.
    pub fn skill_path(&self) -> Option<&str> {
        match self {
            ActionSpec::Skill { path } => Some(path.as_str()),
            _ => None,
        }
    }

    fn clamp(&mut self) {
        if let ActionSpec::Skill { path } = self {
            sanitize_path(path);
        }
    }
}

/// What a condition looks at. Every one of these is a plain number already
/// present in the state table, so a condition can never reach into the game.
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Subject {
    LifePct,
    EnergyPct,
    Life,
    Energy,
    /// Seconds since *any* action last fired.
    SinceAnyAction,

    // ── combat state ──────────────────────────────────────────────────────
    /// 1 while there is a target, recent outgoing damage, or incoming damage.
    InCombat,
    /// 1 while the character has an attack target.
    HasTarget,
    /// 1 while something is attacking us.
    UnderAttack,
    Moving,

    // ── per-skill, and so requiring `Condition::skill` ────────────────────
    /// 1 when the game says the skill can be cast right now.
    SkillReady,
    /// Milliseconds left on the skill's cooldown; 0 when ready.
    SkillCooldown,
    /// 1 while the buff the skill grants is on the character.
    BuffActive,
}

impl Subject {
    /// Whether this subject asks about a particular skill, and therefore needs
    /// `Condition::skill` filled in.
    pub fn needs_skill(self) -> bool {
        matches!(self, Subject::SkillReady | Subject::SkillCooldown | Subject::BuffActive)
    }
}

impl Subject {
    fn lua(self) -> &'static str {
        match self {
            Subject::LifePct => "s.life_pct",
            Subject::EnergyPct => "s.energy_pct",
            Subject::Life => "s.life",
            Subject::Energy => "s.energy",
            Subject::SinceAnyAction => "s.since_any",
            Subject::InCombat => "s.in_combat",
            Subject::HasTarget => "s.has_target",
            Subject::UnderAttack => "s.under_attack",
            Subject::Moving => "s.moving",
            // Per-skill subjects index a table by record path; the caller
            // substitutes the path, so this is only the table name.
            Subject::SkillReady => "s.skill_ready",
            Subject::SkillCooldown => "s.skill_cooldown",
            Subject::BuffActive => "s.buff_active",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Op {
    Below,
    AtMost,
    Above,
    AtLeast,
    /// For the yes/no subjects. Reads better than `>= 1` in the editor and
    /// generates exactly that.
    IsTrue,
    IsFalse,
}

impl Op {
    fn lua(self) -> &'static str {
        match self {
            Op::Below => "<",
            Op::AtMost => "<=",
            Op::Above => ">",
            Op::AtLeast => ">=",
            Op::IsTrue => ">=",
            Op::IsFalse => "<",
        }
    }

    /// The yes/no ops carry their own value, so the editor does not show a
    /// number box for them.
    fn fixed_value(self) -> Option<f64> {
        match self {
            Op::IsTrue | Op::IsFalse => Some(1.0),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Condition {
    pub subject: Subject,
    /// The skill a per-skill subject asks about, as a record path.
    #[serde(default)]
    pub skill: Option<String>,
    pub op: Op,
    pub value: f64,
}

impl Condition {
    fn clamp(&mut self) {
        if !self.value.is_finite() {
            self.value = 0.0;
        }
        self.value = self.value.clamp(-1.0e9, 1.0e9);
        match &mut self.skill {
            Some(path) => sanitize_path(path),
            None => {}
        }
        if !self.subject.needs_skill() {
            self.skill = None;
        }
    }

    /// The Lua for this condition's left-hand side.
    fn lua_subject(&self) -> String {
        if self.subject.needs_skill() {
            let path = self.skill.as_deref().unwrap_or("");
            format!("{}[\"{}\"]", self.subject.lua(), path)
        } else {
            self.subject.lua().to_string()
        }
    }
}

/// Record paths come from a browser, so they are reduced to the characters a
/// real one uses before being embedded in generated Lua as a table key.
fn sanitize_path(path: &mut String) {
    path.retain(|c| c.is_ascii_alphanumeric() || matches!(c, '/' | '_' | '-' | '.'));
    path.truncate(MAX_PATH);
    path.make_ascii_lowercase();
}

/// One line of the priority list: all conditions must hold, and then exactly
/// one action happens. There is no "and then" -- the gate serialises
/// everything, so a rule that tried to do two things would silently drop one.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Rule {
    /// Stable across edits, assigned by the editor. Used as the source-map key
    /// and as the per-rule cooldown key, so it must survive reordering.
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub action: ActionSpec,
    /// Joined with `and`. An empty list means "always".
    pub conditions: Vec<Condition>,
    /// Minimum gap between two firings *of this rule*, on top of the global
    /// cooldown. This is what makes a rotation expressible at all.
    pub cooldown_ms: u64,
}

impl Rule {
    pub fn new(id: &str, name: &str, action: ActionSpec) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            enabled: true,
            action,
            conditions: Vec::new(),
            cooldown_ms: 0,
        }
    }

    pub fn when(mut self, subject: Subject, op: Op, value: f64) -> Self {
        self.conditions.push(Condition { subject, skill: None, op, value });
        self
    }

    pub fn when_skill(mut self, subject: Subject, skill: &str, op: Op, value: f64) -> Self {
        self.conditions.push(Condition {
            subject,
            skill: Some(skill.to_string()),
            op,
            value,
        });
        self
    }
}

/// Where a rule ended up in the generated source, 1-based.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct LineMark {
    pub rule: String,
    pub line: u32,
}

pub struct Generated {
    pub source: String,
    pub marks: Vec<LineMark>,
}

/// Bring a rule list sent by a browser inside every limit, in place.
///
/// The host does not trust the editor: it is a client like any other, and a
/// hand-written websocket message is exactly as likely. Everything that could
/// reach the code generator or the game is clamped here, once.
pub fn sanitize(rules: &mut Vec<Rule>) {
    rules.truncate(MAX_RULES);
    let mut seen: Vec<String> = Vec::with_capacity(rules.len());
    for (i, rule) in rules.iter_mut().enumerate() {
        rule.id = unique_id(&rule.id, i, &seen);
        seen.push(rule.id.clone());
        rule.name = clean_name(&rule.name);
        rule.action.clamp();
        rule.conditions.truncate(MAX_CONDITIONS);
        for c in &mut rule.conditions {
            c.clamp();
        }
        rule.cooldown_ms = rule.cooldown_ms.min(600_000);
    }
}

/// Reduce an id to something that is safe as a Lua table key and unambiguous
/// as a source-map key. Editor-generated ids already satisfy this, so in
/// practice the value comes back unchanged; this only catches hand-edited
/// configs.
fn unique_id(raw: &str, index: usize, taken: &[String]) -> String {
    let mut id: String = raw
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .take(32)
        .collect();
    if id.is_empty() {
        id = format!("rule{index}");
    }
    if taken.iter().any(|t| *t == id) {
        id = format!("{id}_{index}");
    }
    id
}

/// Rule names land in Lua comments, so anything that could end a comment or
/// start a new line has to go.
fn clean_name(raw: &str) -> String {
    let cleaned: String = raw
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .take(60)
        .collect();
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        "unnamed rule".into()
    } else {
        trimmed.into()
    }
}

/// Render a number as a Lua literal. Callers must have clamped it first;
/// a non-finite value here would emit `inf`, which is not Lua syntax.
fn num(v: f64) -> String {
    if v.is_finite() && v == v.trunc() && v.abs() < 1.0e15 {
        format!("{}", v as i64)
    } else if v.is_finite() {
        format!("{v}")
    } else {
        "0".into()
    }
}

/// Compile a rule list into the Lua the evaluator actually runs.
///
/// The shape is fixed: one `if` per enabled rule, in order, first match wins,
/// returning the action token and the rule id. That second return value is
/// what lets the host attribute a firing to a rule -- for the per-rule
/// cooldown, and for lighting the rule up in the editor.
pub fn generate(rules: &[Rule]) -> Generated {
    let enabled = rules.iter().filter(|r| r.enabled).count();
    let mut out = String::new();
    let mut marks = Vec::new();

    out.push_str("-- Generated by grimlua from the rule list. Do not edit here:\n");
    out.push_str("-- switch the editor to Lua mode to take ownership of this script.\n");
    out.push_str(&format!(
        "-- {} rule{} enabled of {}.\n\n",
        enabled,
        if enabled == 1 { "" } else { "s" },
        rules.len()
    ));
    out.push_str("function choose(s)\n");

    for rule in rules.iter().filter(|r| r.enabled) {
        out.push_str(&format!("  -- {}\n", rule.name));

        let mut tests: Vec<String> = rule
            .conditions
            .iter()
            .map(|c| {
                let value = c.op.fixed_value().unwrap_or(c.value);
                format!("{} {} {}", c.lua_subject(), c.op.lua(), num(value))
            })
            .collect();
        if rule.cooldown_ms > 0 {
            tests.push(format!(
                "s.since_rule[\"{}\"] >= {}",
                rule.id,
                num(rule.cooldown_ms as f64 / 1000.0)
            ));
        }
        let test = if tests.is_empty() { "true".into() } else { tests.join(" and ") };

        // Recorded before the line is pushed, so the mark points at the `if`.
        marks.push(LineMark { rule: rule.id.clone(), line: line_count(&out) + 1 });
        out.push_str(&format!("  if {test} then\n"));
        out.push_str(&format!(
            "    return \"{}\", \"{}\"\n",
            rule.action.token(),
            rule.id
        ));
        out.push_str("  end\n\n");
    }

    out.push_str("  return nil\n");
    out.push_str("end\n");

    Generated { source: out, marks }
}

/// Lines already written. `out` always ends in a newline at call time, so the
/// next line's number is this plus one.
fn line_count(out: &str) -> u32 {
    out.bytes().filter(|b| *b == b'\n').count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marks_point_at_the_generated_if() {
        let rules = vec![
            Rule::new("r1", "flask", ActionSpec::HealthPotion)
                .when(Subject::LifePct, Op::Below, 55.0),
            Rule::new("r2", "tonic", ActionSpec::EnergyPotion)
                .when(Subject::EnergyPct, Op::Below, 30.0),
        ];
        let g = generate(&rules);
        let lines: Vec<&str> = g.source.lines().collect();
        assert_eq!(g.marks.len(), 2);
        for mark in &g.marks {
            let line = lines[mark.line as usize - 1];
            assert!(line.trim_start().starts_with("if "), "line {} was {line:?}", mark.line);
        }
        assert!(lines[g.marks[0].line as usize - 1].contains("s.life_pct < 55"));
    }

    #[test]
    fn disabled_rules_are_not_generated() {
        let mut rules = vec![Rule::new("r1", "flask", ActionSpec::HealthPotion)];
        rules[0].enabled = false;
        let g = generate(&rules);
        assert!(g.marks.is_empty());
        assert!(!g.source.contains("health_potion"));
    }

    #[test]
    fn a_rule_with_no_conditions_is_unconditional() {
        let g = generate(&[Rule::new("r1", "always", ActionSpec::Skill { path: "records/skills/playerclass03/bloodofdreeg1.dbr".into() })]);
        assert!(g.source.contains("if true then"));
        assert!(g.source.contains("return \"skill:records/skills/playerclass03/bloodofdreeg1.dbr\", \"r1\""));
    }

    #[test]
    fn a_cooldown_becomes_a_test_on_since_rule() {
        let mut rule = Rule::new("r1", "rotation", ActionSpec::Skill { path: "records/skills/playerclass06/totem1.dbr".into() });
        rule.cooldown_ms = 2500;
        let g = generate(&[rule]);
        assert!(g.source.contains("s.since_rule[\"r1\"] >= 2.5"), "{}", g.source);
    }

    #[test]
    fn sanitize_clamps_everything_a_browser_could_send() {
        let mut rules = vec![Rule::new(
            "has spaces and ; quotes\"",
            "bad\nname",
            ActionSpec::Skill { path: "RECORDS/Skills/Bad Path!.dbr".into() },
        )
        .when(Subject::LifePct, Op::Below, f64::INFINITY)];
        rules[0].cooldown_ms = u64::MAX;
        sanitize(&mut rules);
        assert_eq!(rules[0].id, "hasspacesandquotes");
        assert_eq!(rules[0].name, "bad name");
        assert_eq!(
            rules[0].action,
            ActionSpec::Skill { path: "records/skills/badpath.dbr".into() }
        );
        assert!(rules[0].conditions[0].value.is_finite());
        assert_eq!(rules[0].cooldown_ms, 600_000);
    }

    #[test]
    fn duplicate_ids_are_separated() {
        let mut rules = vec![
            Rule::new("r1", "a", ActionSpec::HealthPotion),
            Rule::new("r1", "b", ActionSpec::HealthPotion),
        ];
        sanitize(&mut rules);
        assert_ne!(rules[0].id, rules[1].id);
    }
}
