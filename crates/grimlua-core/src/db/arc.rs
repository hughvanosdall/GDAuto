//! The `.arc` resource container.
//!
//! Where `.arz` holds records, `.arc` holds loose files: the `text_en` tag
//! files that turn `tagClass03SkillName04A` into "Blood of Dreeg", and the
//! `.tex` skill icons.
//!
//! Layout, confirmed against the shipped `Text_EN.arc` by the section sizes
//! summing to exactly the file length (see `tools/arc_probe.py`):
//!
//! ```text
//!   [ payload ......................................... ]
//!   table_off ─ part table    record_size bytes
//!             ─ string table  string_size bytes, NUL-separated names
//!             ─ file entries  entry_count * 44 bytes, indexing both
//! ```
//!
//! The string table sits *between* the two tables, which is the one thing
//! about this format that is easy to get wrong and produces entries full of
//! plausible-looking garbage rather than an error.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

const MAGIC: u32 = 0x0043_5241; // "ARC\0"
const ENTRY_LEN: usize = 44;

#[derive(Debug)]
pub enum ArcError {
    Io(std::io::Error),
    NotAnArchive,
    BadVersion(u32),
    Layout(String),
    Truncated,
    Decompress(String),
}

impl std::fmt::Display for ArcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArcError::Io(e) => write!(f, "{e}"),
            ArcError::NotAnArchive => write!(f, "not an .arc archive"),
            ArcError::BadVersion(v) => write!(f, "unsupported .arc version {v}"),
            ArcError::Layout(m) => write!(f, "unexpected layout: {m}"),
            ArcError::Truncated => write!(f, "file ends early"),
            ArcError::Decompress(m) => write!(f, "decompression failed: {m}"),
        }
    }
}

impl From<std::io::Error> for ArcError {
    fn from(e: std::io::Error) -> Self {
        ArcError::Io(e)
    }
}

type Result<T> = std::result::Result<T, ArcError>;

#[derive(Clone, Debug)]
pub struct Entry {
    pub name: String,
    kind: u32,
    offset: u32,
    decompressed: u32,
    parts: u32,
    first_part: u32,
}

/// One independently compressed chunk of a file.
#[derive(Clone, Copy)]
struct Part {
    offset: u32,
    compressed: u32,
    decompressed: u32,
}

pub struct Arc {
    pub path: PathBuf,
    data: Vec<u8>,
    parts: Vec<Part>,
    pub entries: Vec<Entry>,
}

fn u32_at(data: &[u8], at: usize) -> Result<u32> {
    let b = data.get(at..at + 4).ok_or(ArcError::Truncated)?;
    Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

impl Arc {
    pub fn open(path: &Path) -> Result<Self> {
        let data = std::fs::read(path)?;
        Self::parse(path.to_path_buf(), data)
    }

    fn parse(path: PathBuf, data: Vec<u8>) -> Result<Self> {
        if u32_at(&data, 0)? != MAGIC {
            return Err(ArcError::NotAnArchive);
        }
        let version = u32_at(&data, 4)?;
        if version != 3 {
            return Err(ArcError::BadVersion(version));
        }
        let entry_count = u32_at(&data, 8)? as usize;
        let part_count = u32_at(&data, 12)? as usize;
        let part_size = u32_at(&data, 16)? as usize;
        let string_size = u32_at(&data, 20)? as usize;
        let table_off = u32_at(&data, 24)? as usize;

        let strings_off = table_off + part_size;
        let entries_off = strings_off + string_size;
        let end = entries_off + entry_count * ENTRY_LEN;
        if end > data.len() {
            return Err(ArcError::Layout(format!(
                "sections end at {end} but the file is {}",
                data.len()
            )));
        }

        let mut parts = Vec::with_capacity(part_count);
        for i in 0..part_count {
            let at = table_off + i * 12;
            parts.push(Part {
                offset: u32_at(&data, at)?,
                compressed: u32_at(&data, at + 4)?,
                decompressed: u32_at(&data, at + 8)?,
            });
        }

        let mut entries = Vec::with_capacity(entry_count);
        for i in 0..entry_count {
            let at = entries_off + i * ENTRY_LEN;
            let kind = u32_at(&data, at)?;
            let offset = u32_at(&data, at + 4)?;
            let decompressed = u32_at(&data, at + 12)?;
            // +16 hash, +20..28 FILETIME
            let nparts = u32_at(&data, at + 28)?;
            let first_part = u32_at(&data, at + 32)?;
            let name_len = u32_at(&data, at + 36)? as usize;
            let name_off = u32_at(&data, at + 40)? as usize;

            let raw = data
                .get(strings_off + name_off..strings_off + name_off + name_len)
                .ok_or(ArcError::Truncated)?;
            // Names are stored NUL-terminated; the length may or may not
            // include it depending on the packer.
            let raw = match raw.iter().position(|b| *b == 0) {
                Some(n) => &raw[..n],
                None => raw,
            };

            entries.push(Entry {
                name: String::from_utf8_lossy(raw).into_owned(),
                kind,
                offset,
                decompressed,
                parts: nparts,
                first_part,
            });
        }

        Ok(Self { path, data, parts, entries })
    }

    pub fn find(&self, name: &str) -> Option<&Entry> {
        self.entries.iter().find(|e| e.name.eq_ignore_ascii_case(name))
    }

    /// Decompress one entry, reassembling it from its parts.
    pub fn read(&self, entry: &Entry) -> Result<Vec<u8>> {
        // Kind 1 is stored whole and uncompressed.
        if entry.kind == 1 {
            let start = entry.offset as usize;
            let end = start + entry.decompressed as usize;
            return self.data.get(start..end).map(|s| s.to_vec()).ok_or(ArcError::Truncated);
        }

        let mut out = Vec::with_capacity(entry.decompressed as usize);
        let first = entry.first_part as usize;
        for i in first..first + entry.parts as usize {
            let part = *self.parts.get(i).ok_or(ArcError::Truncated)?;
            let start = part.offset as usize;
            let end = start + part.compressed as usize;
            let blob = self.data.get(start..end).ok_or(ArcError::Truncated)?;

            // A part whose sizes match was stored rather than compressed.
            if part.compressed == part.decompressed {
                out.extend_from_slice(blob);
            } else {
                let chunk = lz4_flex::block::decompress(blob, part.decompressed as usize)
                    .map_err(|e| ArcError::Decompress(format!("{}: {e}", entry.name)))?;
                out.extend_from_slice(&chunk);
            }
        }
        Ok(out)
    }

    /// Read every `tags_*.txt` entry as `tag=text` pairs.
    ///
    /// Later archives overwrite earlier ones, which is how an expansion or a
    /// mod retitles a base-game skill.
    pub fn collect_tags(&self, into: &mut HashMap<String, String>) {
        for entry in &self.entries {
            let lower = entry.name.to_ascii_lowercase();
            if !(lower.starts_with("tags") && lower.ends_with(".txt")) {
                continue;
            }
            let Ok(bytes) = self.read(entry) else { continue };
            // UTF-8 with a BOM.
            let text = String::from_utf8_lossy(&bytes);
            let text = text.strip_prefix('\u{feff}').unwrap_or(&text);
            for line in text.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with("//") {
                    continue;
                }
                if let Some((tag, value)) = line.split_once('=') {
                    into.insert(tag.trim().to_string(), value.trim().to_string());
                }
            }
        }
    }
}
