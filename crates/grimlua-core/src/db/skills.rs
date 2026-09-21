//! The skill index: what the rule editor picks from, and what validates a rule.
//!
//! Built from the `.arz` record set plus the `text_en` tag files. Two things
//! come out of it that the runtime cannot get any other way:
//!
//! * **a name** — records hold `tagClass03SkillName04A`, not "Blood of Dreeg";
//! * **castable or not** — a rule that tries to cast a passive is a rule that
//!   can never fire, and Option A's editor says so at the point of the mistake
//!   rather than letting it sit silently in a priority list.

use serde::{Deserialize, Serialize};

use super::arz::{Arz, RecordHeader};

/// Record classes that are *not* something a priority list can cast.
///
/// `Skill_Passive` and friends are always-on; `SkillBuff_*` records are the
/// buff half of a skill, pointed at by the castable record rather than cast
/// themselves; `Skill_Modifier` alters another skill. All three appear under
/// `records/skills/`, so the path alone cannot tell them apart.
fn classify(class: &str) -> Kind {
    if class.starts_with("SkillBuff") {
        return Kind::Buff;
    }
    if class.contains("Modifier") {
        return Kind::Modifier;
    }
    if class.contains("Passive") {
        return Kind::Passive;
    }
    if class.starts_with("Skill_") {
        return Kind::Castable;
    }
    Kind::Passive
}

/// Why a skill cannot be cast, phrased for the editor to show verbatim.
pub fn not_castable_reason(class: &str) -> &'static str {
    if class.contains("Passive") {
        "That is a passive — it cannot be cast. Choose an activated skill, or use it as a condition instead."
    } else if class.contains("Modifier") {
        "That is a modifier to another skill, not a skill you can cast."
    } else if class.starts_with("SkillBuff") {
        "That is the buff a skill applies, not the skill. Cast the skill that grants it."
    } else {
        "That record is not a castable skill."
    }
}

/// What a record is, for the editor's purposes.
///
/// The distinction matters because a skill and the buff it applies are two
/// separate records that share a display name. Picking the wrong one is how
/// "Blood of Dreeg" ends up looking like an uncastable passive.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    /// A priority list can cast this.
    Castable,
    /// Always on. Usable as a condition, never as an action.
    Passive,
    /// The buff half of a skill, reached through the skill that grants it.
    /// This is what a "expires within 3s" condition actually watches.
    Buff,
    /// Modifies another skill.
    Modifier,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Skill {
    /// The record path, which is the stable identity across patches and the
    /// key a rule stores. Never the display name: names are localised and mods
    /// rename things.
    pub path: String,
    /// Resolved display name, or the raw tag if the tag file has no entry.
    pub name: String,
    /// The raw `skillDisplayName` tag, e.g. `tagClass03SkillName04A`.
    ///
    /// This is the join key against the *live* game: a `Skill*` on the
    /// character exposes the same tag through `Skill::GetDisplayNameTag`, so
    /// matching on it is how "only skills I actually have" is decided without
    /// guessing at record paths.
    pub tag: String,
    pub class: String,
    pub kind: Kind,
    pub castable: bool,
    /// The buff record this skill applies, when it has one. A "expires within"
    /// condition watches the buff; the rule still casts the skill.
    pub buff_path: Option<String>,
    /// `ui/skills/icons/...tex`, for the editor once icons are converted.
    pub icon: Option<String>,
    /// Mastery number from the path, 1-10, or `None` for item and default skills.
    pub mastery: Option<u32>,
    /// Cooldown at rank 1, in seconds. Records store one value per rank.
    pub cooldown: Option<f32>,
    pub mana_cost: Option<f32>,
    /// How long the buff lasts, which is what makes "refresh within 3s"
    /// checkable at edit time rather than only at runtime.
    pub duration: Option<f32>,
    pub max_level: Option<i64>,
}

/// Pull the mastery number out of a record path.
///
/// `records/skills/playerclass03/blooddreeg.dbr` -> 3.
fn mastery_of(path: &str) -> Option<u32> {
    let at = path.find("playerclass")? + "playerclass".len();
    let digits: String = path[at..].chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

/// Whether a record is a skill the player could conceivably have.
///
/// Excludes the large mass of monster and pet skills, which would otherwise
/// drown the picker: 4,580 records under `records/skills/` but only a few
/// hundred are a player's own.
fn is_player_facing(path: &str) -> bool {
    let p = path.to_ascii_lowercase();
    if !p.starts_with("records/skills/") {
        return false;
    }
    if p.contains("/pets/") || p.contains("petskill") {
        return false;
    }
    p.contains("playerclass") || p.contains("itemskills") || p.contains("/default/")
}

/// Build the skill list from one archive's records.
///
/// `tags` resolves `skillDisplayName`; a record whose tag is missing keeps the
/// raw tag as its name so the entry is still usable and the gap is visible.
pub fn collect(
    arz: &Arz,
    tags: &std::collections::HashMap<String, String>,
    out: &mut Vec<Skill>,
) -> usize {
    use std::collections::HashMap;

    let mut failures = 0;

    // Pass one: decompress every candidate and keep it, because resolving a
    // skill can require a *different* record. Only player-facing skills are
    // parsed, so this is a few thousand records, not the whole 34,000.
    let mut parsed: Vec<(&super::arz::RecordHeader, super::arz::Record)> = Vec::new();
    for header in &arz.records {
        if !header.class.starts_with("Skill") || !is_player_facing(&header.path) {
            continue;
        }
        match arz.record(header) {
            Ok(r) => parsed.push((header, r)),
            Err(_) => failures += 1,
        }
    }

    let by_path: HashMap<String, usize> = parsed
        .iter()
        .enumerate()
        .map(|(i, (h, _))| (h.path.to_ascii_lowercase(), i))
        .collect();

    // Pass two: emit a skill per record, following the buff link where the
    // record itself is nameless.
    for (header, record) in &parsed {
        let kind = classify(&header.class);

        // The link a wrapper record uses to reach its real content.
        let buff_path = record
            .string("buffSelfSkillName")
            .or_else(|| record.string("buffSkillName"))
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());

        // Some castable records are four fields and a pointer: `Blood of
        // Dreeg` is `Skill_BuffRadius` with nothing but `buffSkillName`, and
        // the name, icon and timings all live on the buff it names. Resolve
        // through it, but keep this record's path and class -- *this* is the
        // thing a rule casts.
        let detail: &super::arz::Record = match record.string("skillDisplayName") {
            Some(tag) if !tag.is_empty() => record,
            _ => buff_path
                .as_deref()
                .and_then(|p| by_path.get(&p.to_ascii_lowercase()))
                .map(|i| &parsed[*i].1)
                .unwrap_or(record),
        };

        let Some(tag) = detail.string("skillDisplayName") else { continue };
        if tag.is_empty() {
            continue;
        }
        let name = tags.get(tag).cloned().unwrap_or_else(|| tag.to_string());

        out.push(Skill {
            path: header.path.clone(),
            name,
            tag: tag.to_string(),
            class: header.class.clone(),
            kind,
            castable: kind == Kind::Castable,
            buff_path,
            icon: detail.string("skillUpBitmapName").map(|s| s.to_string()),
            mastery: mastery_of(&header.path),
            cooldown: first_rank(detail, "skillCooldownTime"),
            mana_cost: first_rank(detail, "skillManaCost"),
            duration: first_rank(detail, "skillActiveDuration")
                .or_else(|| first_rank(detail, "buffDuration")),
            max_level: detail.int("skillMaxLevel"),
        });
    }

    failures
}

/// Most skill numbers are arrays indexed by rank. Rank 1 is the useful one for
/// a picker; resolving a specific rank is a traversal the runtime does later.
fn first_rank(record: &super::arz::Record, key: &str) -> Option<f32> {
    if let Some(values) = record.floats(key) {
        return values.first().copied().filter(|v| *v != 0.0);
    }
    record.float(key).filter(|v| *v != 0.0)
}

/// Sort for display and drop duplicates, keeping the last definition.
///
/// Expansions and mods redefine base records by path, and they are collected in
/// load order, so the last one to mention a path wins.
pub fn finish(mut skills: Vec<Skill>) -> Vec<Skill> {
    let mut seen: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for (i, s) in skills.iter().enumerate() {
        seen.insert(s.path.to_ascii_lowercase(), i);
    }
    let mut keep: Vec<usize> = seen.into_values().collect();
    keep.sort_unstable();
    let mut out: Vec<Skill> = keep.into_iter().map(|i| std::mem::take(&mut skills[i])).collect();

    // Mastery skills first and in mastery order, then item and default skills.
    // `Option::cmp` puts `None` first, which is the wrong way round here: the
    // player's own skills are what a picker should open on.
    out.sort_by(|a, b| {
        let rank = |m: Option<u32>| m.unwrap_or(u32::MAX);
        rank(a.mastery)
            .cmp(&rank(b.mastery))
            .then_with(|| a.name.to_ascii_lowercase().cmp(&b.name.to_ascii_lowercase()))
            .then_with(|| a.path.cmp(&b.path))
    });

    // Item skills repeat the same name across near-identical records. Keeping
    // every one turns the picker into a list of apparent duplicates, so the
    // first of each (name, kind) wins -- deterministic because of the sort.
    let mut seen_name = std::collections::HashSet::new();
    out.retain(|s| seen_name.insert((s.name.to_ascii_lowercase(), s.kind)));
    out
}

impl Default for Skill {
    fn default() -> Self {
        Self {
            path: String::new(),
            name: String::new(),
            tag: String::new(),
            class: String::new(),
            kind: Kind::Passive,
            castable: false,
            buff_path: None,
            icon: None,
            mastery: None,
            cooldown: None,
            mana_cost: None,
            duration: None,
            max_level: None,
        }
    }
}

#[allow(dead_code)]
fn unused(_: &RecordHeader) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passives_and_buff_records_are_not_castable() {
        assert_eq!(classify("Skill_Passive"), Kind::Passive);
        assert_eq!(classify("SkillBuff_Passive"), Kind::Buff);
        assert_eq!(classify("SkillBuff_Debuf"), Kind::Buff);
        assert_eq!(classify("Skill_Modifier"), Kind::Modifier);
        assert!(not_castable_reason("Skill_Passive").contains("passive"));
    }

    /// The trap this fixes: a skill and the buff it applies share a display
    /// name, so looking one up by name can land on the uncastable half.
    #[test]
    fn a_skill_wins_over_the_buff_that_shares_its_name() {
        let mut skill = Skill { name: "Blood of Dreeg".into(), ..Default::default() };
        skill.path = "records/skills/playerclass03/bloodofdreeg1.dbr".into();
        skill.kind = Kind::Castable;
        skill.castable = true;
        skill.mastery = Some(3);

        let mut buff = Skill { name: "Blood of Dreeg".into(), ..Default::default() };
        buff.path = "records/skills/playerclass03/bloodofdreeg1_buff.dbr".into();
        buff.kind = Kind::Buff;
        buff.mastery = Some(3);

        let out = finish(vec![buff, skill]);
        let castable: Vec<&Skill> = out.iter().filter(|s| s.castable).collect();
        assert_eq!(castable.len(), 1, "the castable record must survive");
        assert!(castable[0].path.ends_with("bloodofdreeg1.dbr"));
    }

    #[test]
    fn attacks_and_self_buffs_are_castable() {
        for class in [
            "Skill_AttackProjectile",
            "Skill_BuffSelfDuration",
            "Skill_AttackRadius",
            "Skill_SpawnPet",
            "Skill_BuffSelfToggled",
            "Skill_TargetedSpawnPet",
        ] {
            assert_eq!(classify(class), Kind::Castable, "{class} should be castable");
        }
    }

    #[test]
    fn masteries_sort_before_item_skills() {
        let mut item = Skill { name: "Aether Burst".into(), ..Default::default() };
        item.path = "records/skills/itemskills/a.dbr".into();
        let mut mastery = Skill { name: "Zzz".into(), ..Default::default() };
        mastery.path = "records/skills/playerclass01/z.dbr".into();
        mastery.mastery = Some(1);

        let out = finish(vec![item, mastery]);
        assert_eq!(out[0].mastery, Some(1), "mastery skills come first");
    }

    #[test]
    fn mastery_comes_out_of_the_path() {
        assert_eq!(mastery_of("records/skills/playerclass03/blooddreeg.dbr"), Some(3));
        assert_eq!(mastery_of("records/skills/playerclass10/x.dbr"), Some(10));
        assert_eq!(mastery_of("records/skills/itemskills/x.dbr"), None);
    }

    #[test]
    fn monster_and_pet_skills_are_left_out() {
        assert!(is_player_facing("records/skills/playerclass03/blooddreeg.dbr"));
        assert!(is_player_facing("records/skills/itemskills/item_x.dbr"));
        assert!(!is_player_facing("records/skills/playerclass03/pets/pet_x.dbr"));
        assert!(!is_player_facing("records/skills/monsters/bite.dbr"));
    }
}
