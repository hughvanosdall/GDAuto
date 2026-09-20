//! Best-effort file logging.
//!
//! Every function here swallows its own errors. A logger that can fail the
//! caller is a logger that can crash the game, and this code runs inside a
//! process where a crash deletes hardcore characters.

use std::fmt::Write as _;
use std::fs::File;
use std::io::Write as _;
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

static SINK: OnceLock<Mutex<File>> = OnceLock::new();

/// Point the logger at `dir/grimlua.log`, truncating any previous run.
///
/// Safe to call more than once; only the first call takes effect.
pub fn init(dir: &Path) {
    if SINK.get().is_some() {
        return;
    }
    if let Ok(file) = File::create(dir.join("grimlua.log")) {
        let _ = SINK.set(Mutex::new(file));
    }
}

pub fn write(line: &str) {
    let Some(sink) = SINK.get() else { return };
    // A poisoned lock means another thread panicked mid-write. The log is not
    // worth propagating that, so recover the guard and carry on.
    let mut file = match sink.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let _ = writeln!(file, "[{}] {}", timestamp(), line);
    let _ = file.flush();
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => { $crate::log::write(&format!($($arg)*)) };
}

/// `YYYY-MM-DD HH:MM:SS.mmm` in UTC, without pulling in a date crate.
fn timestamp() -> String {
    let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d,
        Err(_) => return "--------- --:--:--.---".into(),
    };
    let secs = now.as_secs();
    let millis = now.subsec_millis();

    let (hour, minute, second) = (secs / 3600 % 24, secs / 60 % 60, secs % 60);
    let (year, month, day) = civil_from_days((secs / 86_400) as i64);

    let mut out = String::with_capacity(23);
    let _ = write!(
        out,
        "{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}.{millis:03}"
    );
    out
}

/// Howard Hinnant's days-from-civil, inverted. Days are since 1970-01-01.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}
