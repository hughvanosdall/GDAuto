//! The `.arz` record database.
//!
//! Grim Dawn's whole record set — every skill, item, monster and template — is
//! one LZ4-compressed archive per campaign: `database/database.arz` plus one
//! per expansion, plus the active mod's own.
//!
//! The format was pinned down against the shipped files by
//! `tools/arz_probe.py`, which stays in the repo as the reference
//! implementation to compare against. Two facts make it cheap to read:
//! everything is little-endian, and every string in every record is an index
//! into one shared table, so a record is a list of (type, count, name-index)
//! headers followed by raw `u32` words.
//!
//! Nothing here touches the game. It reads files off disk on a worker thread.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Header is 24 bytes, and every record's payload offset is relative to its end.
const HEADER_LEN: usize = 24;

#[derive(Debug)]
pub enum ArzError {
    Io(std::io::Error),
    Truncated(&'static str),
    BadVersion(u16),
    Layout(String),
    Decompress(String),
}

impl std::fmt::Display for ArzError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArzError::Io(e) => write!(f, "{e}"),
            ArzError::Truncated(what) => write!(f, "file ends early, reading {what}"),
            ArzError::BadVersion(v) => write!(f, "unsupported .arz version {v}"),
            ArzError::Layout(m) => write!(f, "unexpected layout: {m}"),
            ArzError::Decompress(m) => write!(f, "decompression failed: {m}"),
        }
    }
}

impl From<std::io::Error> for ArzError {
    fn from(e: std::io::Error) -> Self {
        ArzError::Io(e)
    }
}

type Result<T> = std::result::Result<T, ArzError>;

/// A cursor that refuses to read past the end rather than panicking.
///
/// These files are user-supplied in every practical sense — a mod ships its
/// own — so a malformed one has to be an error, not an index-out-of-bounds in
/// a DLL living inside someone's game.
struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(data: &'a [u8], pos: usize) -> Self {
        Self { data, pos }
    }

    fn u16(&mut self) -> Result<u16> {
        let b = self.take(2, "u16")?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    fn u32(&mut self) -> Result<u32> {
        let b = self.take(4, "u32")?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn take(&mut self, n: usize, what: &'static str) -> Result<&'a [u8]> {
        let end = self.pos.checked_add(n).ok_or(ArzError::Truncated(what))?;
        let slice = self.data.get(self.pos..end).ok_or(ArzError::Truncated(what))?;
        self.pos = end;
        Ok(slice)
    }

    fn skip(&mut self, n: usize) -> Result<()> {
        self.take(n, "padding").map(|_| ())
    }
}

/// One entry in the record table: where a record lives and what it is.
#[derive(Clone, Debug)]
pub struct RecordHeader {
    /// Full record path, e.g. `records/skills/playerclass03/blooddreeg.dbr`.
    pub path: String,
    /// The record's class, e.g. `Skill_BuffSelfDuration`. This is what tells a
    /// passive from something castable, without reading the record at all.
    pub class: String,
    offset: u32,
    compressed: u32,
    decompressed: u32,
}

/// A parsed record: field name to value.
#[derive(Clone, Debug, Default)]
pub struct Record {
    pub fields: HashMap<String, Value>,
}

impl Record {
    pub fn string(&self, key: &str) -> Option<&str> {
        match self.fields.get(key) {
            Some(Value::Strings(v)) => v.first().map(|s| s.as_str()),
            _ => None,
        }
    }

    pub fn float(&self, key: &str) -> Option<f32> {
        match self.fields.get(key) {
            Some(Value::Floats(v)) => v.first().copied(),
            Some(Value::Ints(v)) => v.first().map(|i| *i as f32),
            _ => None,
        }
    }

    pub fn int(&self, key: &str) -> Option<i64> {
        match self.fields.get(key) {
            Some(Value::Ints(v)) => v.first().map(|i| *i as i64),
            Some(Value::Floats(v)) => v.first().map(|f| *f as i64),
            _ => None,
        }
    }

    /// Values indexed by skill rank, which is how most skill numbers are
    /// stored: `skillCooldownTime` is an array, one entry per level.
    pub fn floats(&self, key: &str) -> Option<&[f32]> {
        match self.fields.get(key) {
            Some(Value::Floats(v)) => Some(v),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Ints(Vec<u32>),
    Floats(Vec<f32>),
    Strings(Vec<String>),
    Bools(Vec<bool>),
}

pub struct Arz {
    pub path: PathBuf,
    data: Vec<u8>,
    strings: Vec<String>,
    pub records: Vec<RecordHeader>,
}

impl Arz {
    pub fn open(path: &Path) -> Result<Self> {
        let data = std::fs::read(path)?;
        Self::parse(path.to_path_buf(), data)
    }

    fn parse(path: PathBuf, data: Vec<u8>) -> Result<Self> {
        let mut c = Cursor::new(&data, 0);
        let _magic = c.u16()?;
        let version = c.u16()?;
        if version != 3 {
            return Err(ArzError::BadVersion(version));
        }
        let record_start = c.u32()? as usize;
        let record_size = c.u32()? as usize;
        let record_count = c.u32()? as usize;
        let string_start = c.u32()? as usize;
        let string_size = c.u32()? as usize;

        // The record table runs exactly up to the string table. Checking it
        // turns a wrong guess about the layout into a clear error rather than
        // a table full of nonsense.
        if record_start + record_size != string_start {
            return Err(ArzError::Layout(format!(
                "record table {record_start}+{record_size} does not meet the string table at {string_start}"
            )));
        }
        if string_start + string_size > data.len() {
            return Err(ArzError::Layout("string table runs past the end".into()));
        }

        let strings = Self::read_strings(&data, string_start)?;
        let records = Self::read_record_table(&data, record_start, record_count, &strings)?;

        Ok(Self { path, data, strings, records })
    }

    fn read_strings(data: &[u8], start: usize) -> Result<Vec<String>> {
        let mut c = Cursor::new(data, start);
        let count = c.u32()? as usize;
        // A corrupt count would otherwise ask for a huge allocation up front.
        if count > 4_000_000 {
            return Err(ArzError::Layout(format!("{count} strings is not plausible")));
        }
        let mut out = Vec::with_capacity(count);
        for _ in 0..count {
            let len = c.u32()? as usize;
            let bytes = c.take(len, "string")?;
            out.push(String::from_utf8_lossy(bytes).into_owned());
        }
        Ok(out)
    }

    fn read_record_table(
        data: &[u8],
        start: usize,
        count: usize,
        strings: &[String],
    ) -> Result<Vec<RecordHeader>> {
        let mut c = Cursor::new(data, start);
        let mut out = Vec::with_capacity(count);
        for _ in 0..count {
            let name_idx = c.u32()? as usize;
            let class_len = c.u32()? as usize;
            let class = String::from_utf8_lossy(c.take(class_len, "class")?).into_owned();
            let offset = c.u32()?;
            let compressed = c.u32()?;
            let decompressed = c.u32()?;
            c.skip(8)?; // FILETIME, unused

            let path = strings
                .get(name_idx)
                .cloned()
                .ok_or_else(|| ArzError::Layout(format!("string index {name_idx} out of range")))?;
            out.push(RecordHeader { path, class, offset, compressed, decompressed });
        }
        Ok(out)
    }

    /// Decompress and decode one record.
    pub fn record(&self, header: &RecordHeader) -> Result<Record> {
        let start = HEADER_LEN + header.offset as usize;
        let end = start
            .checked_add(header.compressed as usize)
            .ok_or(ArzError::Truncated("record payload"))?;
        let blob = self
            .data
            .get(start..end)
            .ok_or(ArzError::Truncated("record payload"))?;

        let raw = lz4_flex::block::decompress(blob, header.decompressed as usize)
            .map_err(|e| ArzError::Decompress(format!("{}: {e}", header.path)))?;

        self.decode(&raw)
    }

    /// A record body is a flat sequence of fields; there is no length prefix,
    /// so it is read until the buffer runs out.
    fn decode(&self, raw: &[u8]) -> Result<Record> {
        let mut c = Cursor::new(raw, 0);
        let mut fields = HashMap::new();

        while c.pos + 8 <= raw.len() {
            let kind = c.u16()?;
            let count = c.u16()? as usize;
            let name_idx = c.u32()? as usize;

            let mut words = Vec::with_capacity(count);
            for _ in 0..count {
                words.push(c.u32()?);
            }

            let name = match self.strings.get(name_idx) {
                Some(n) => n.clone(),
                None => continue, // a field we cannot name is a field we skip
            };

            let value = match kind {
                1 => Value::Floats(words.into_iter().map(f32::from_bits).collect()),
                2 => Value::Strings(
                    words
                        .into_iter()
                        .map(|w| self.strings.get(w as usize).cloned().unwrap_or_default())
                        .collect(),
                ),
                3 => Value::Bools(words.into_iter().map(|w| w != 0).collect()),
                _ => Value::Ints(words),
            };
            fields.insert(name, value);
        }

        Ok(Record { fields })
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
