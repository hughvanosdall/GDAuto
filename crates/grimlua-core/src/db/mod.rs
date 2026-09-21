//! The skill database: locate Grim Dawn's data files, parse them, cache the
//! result.
//!
//! This is build-order step 6. It runs on its own worker thread at startup and
//! **never touches the game** — it reads files off disk, exactly as an
//! external tool would. The frame hook only ever sees the finished index
//! through `shared`.
//!
//! Load order matters and is the base game, then each expansion, then the
//! active mod. Later archives redefine records by path, which is how an
//! expansion retitles a base skill and how a mod adds its own.

pub mod arc;
pub mod arz;
pub mod skills;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::log;
pub use skills::Skill;

/// Bumped whenever the shape of the cache file changes, so a stale one from an
/// older build is rebuilt rather than misread.
///
/// **Bump this whenever [`Skill`] gains, loses or renames a field.** Version 2
/// is `tag`, `kind` and `buff_path`. Relying on deserialisation to fail is not
/// the same thing: it happens to rebuild, but only by accident, and a field
/// whose *meaning* changes without its type changing would slip straight
/// through and be believed.
const CACHE_VERSION: u32 = 2;

#[derive(Serialize, Deserialize)]
pub struct SkillIndex {
    pub version: u32,
    /// Identifies the archive set this was built from: name, size and mtime of
    /// every file that fed it. Any change rebuilds.
    pub fingerprint: String,
    pub skills: Vec<Skill>,
    /// Milliseconds the build took, reported in the UI.
    pub built_ms: u64,
}

impl Default for SkillIndex {
    fn default() -> Self {
        Self {
            version: CACHE_VERSION,
            fingerprint: String::new(),
            skills: Vec::new(),
            built_ms: 0,
        }
    }
}

impl SkillIndex {
    pub fn castable(&self) -> impl Iterator<Item = &Skill> {
        self.skills.iter().filter(|s| s.castable)
    }

    pub fn by_path(&self, path: &str) -> Option<&Skill> {
        self.skills.iter().find(|s| s.path.eq_ignore_ascii_case(path))
    }
}

/// Where the game's data lives, in load order.
pub struct DataFiles {
    pub archives: Vec<PathBuf>,
    pub text: Vec<PathBuf>,
}

/// Find the game's `.arz` and text archives relative to the install root.
///
/// The DLL sits in `<game>\x64`, so the root is its parent. Expansions are
/// `gdx1`..`gdx3`; a mod adds `mods\<name>\database\<name>.arz`.
pub fn locate(game_root: &Path) -> DataFiles {
    let mut archives = Vec::new();
    let mut text = Vec::new();

    let base = game_root.join("database").join("database.arz");
    if base.is_file() {
        archives.push(base);
    }
    let base_text = game_root.join("resources").join("Text_EN.arc");
    if base_text.is_file() {
        text.push(base_text);
    }

    for expansion in ["gdx1", "gdx2", "gdx3"] {
        let dir = game_root.join(expansion);
        if !dir.is_dir() {
            continue;
        }
        // The expansion's archive is named after it, in upper case.
        let arz = dir.join("database").join(format!("{}.arz", expansion.to_uppercase()));
        if arz.is_file() {
            archives.push(arz);
        }
        let txt = dir.join("resources").join("Text_EN.arc");
        if txt.is_file() {
            text.push(txt);
        }
    }

    DataFiles { archives, text }
}

/// A cheap identity for a file set: path, size and modification time.
///
/// Hashing 180 MB of archives on every launch would cost more than the parse
/// it is meant to avoid, and size-plus-mtime is what actually changes when a
/// patch or a mod lands.
fn fingerprint(files: &DataFiles) -> String {
    let mut parts = Vec::new();
    for path in files.archives.iter().chain(files.text.iter()) {
        let meta = match std::fs::metadata(path) {
            Ok(m) => m,
            Err(_) => continue,
        };
        let stamp = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        parts.push(format!("{name}:{}:{stamp}", meta.len()));
    }
    parts.join("|")
}

/// Parse everything and build the index. Minutes of work becomes seconds
/// because only skill records are decompressed.
pub fn build(files: &DataFiles) -> SkillIndex {
    let started = Instant::now();

    let mut tags: HashMap<String, String> = HashMap::new();
    for path in &files.text {
        match arc::Arc::open(path) {
            Ok(archive) => archive.collect_tags(&mut tags),
            Err(e) => log!("db: cannot read {} ({e})", path.display()),
        }
    }
    log!("db: {} text tags", tags.len());

    let mut collected: Vec<Skill> = Vec::new();
    for path in &files.archives {
        match arz::Arz::open(path) {
            Ok(archive) => {
                let before = collected.len();
                let failures = skills::collect(&archive, &tags, &mut collected);
                log!(
                    "db: {} -> {} records, {} skills{}",
                    path.file_name().unwrap_or_default().to_string_lossy(),
                    archive.len(),
                    collected.len() - before,
                    if failures > 0 { format!(", {failures} unreadable") } else { String::new() }
                );
            }
            Err(e) => log!("db: cannot read {} ({e})", path.display()),
        }
    }

    let skills = skills::finish(collected);
    let castable = skills.iter().filter(|s| s.castable).count();
    let built_ms = started.elapsed().as_millis() as u64;
    log!(
        "db: {} skills ({castable} castable) in {built_ms} ms",
        skills.len()
    );

    SkillIndex {
        version: CACHE_VERSION,
        fingerprint: fingerprint(files),
        skills,
        built_ms,
    }
}

/// Load the cache if it still matches the game's files, otherwise rebuild and
/// write it back.
///
/// The user never asks for this: a patch or a newly enabled mod changes the
/// fingerprint and the index rebuilds on the next launch by itself.
pub fn load_or_build(game_root: &Path, cache_path: &Path) -> SkillIndex {
    let files = locate(game_root);
    if files.archives.is_empty() {
        log!("db: no .arz archives found under {}", game_root.display());
        return SkillIndex::default();
    }
    let want = fingerprint(&files);

    if let Ok(text) = std::fs::read_to_string(cache_path) {
        match serde_json::from_str::<SkillIndex>(&text) {
            Ok(cached) if cached.version == CACHE_VERSION && cached.fingerprint == want => {
                log!("db: {} skills from cache", cached.skills.len());
                return cached;
            }
            Ok(_) => log!("db: cache is stale, rebuilding"),
            Err(e) => log!("db: cache unreadable ({e}), rebuilding"),
        }
    }

    let index = build(&files);
    match serde_json::to_string(&index) {
        Ok(text) => {
            if let Err(e) = std::fs::write(cache_path, text) {
                log!("db: could not write cache ({e})");
            }
        }
        Err(e) => log!("db: could not serialise cache ({e})"),
    }
    index
}
