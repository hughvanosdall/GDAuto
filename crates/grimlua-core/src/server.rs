//! The local HTTP + WebSocket server.
//!
//! **Binds to 127.0.0.1 and nothing else.** A game process listening on all
//! interfaces is a real risk, not a theoretical one, so the address is a
//! literal here and there is no setting to widen it.
//!
//! This thread never calls into Grim Dawn, and it never runs Lua either. It
//! reads [`shared::SNAPSHOT`] and [`shared::SCRIPT`] and writes
//! [`shared::CONFIG`]; the frame hook does the reverse, and the hook is what
//! compiles and runs the script. That is the only channel between them, which
//! is what stops a slow browser from stalling the game thread.

use std::io::{ErrorKind, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tungstenite::{Message, WebSocket};

use crate::log;
use crate::shared::{self, Config, ScriptStatus, Snapshot};

pub const PORT: u16 = 7890;

const INDEX_HTML: &str = include_str!("web/index.html");

/// How often a connected browser is pushed a new snapshot.
const PUSH_INTERVAL: Duration = Duration::from_millis(100);

/// Largest websocket frame accepted. The script is capped well below this;
/// the limit exists so a client cannot make the server allocate freely.
const MAX_MESSAGE: usize = 256 * 1024;

/// The small, always-changing half of the script status. Split out from the
/// heavy half so the source text is not re-sent ten times a second, and the
/// browser is not asked to re-render an editor full of text that has not
/// changed.
#[derive(Serialize)]
struct Stats {
    ok: bool,
    halted: bool,
    evaluations: u64,
    errors: u64,
    last_error: Option<String>,
    last_eval_us: u64,
}

#[derive(Serialize)]
struct Envelope<'a> {
    snapshot: &'a Snapshot,
    /// Only when it has changed. Sending it on every push would fight the
    /// control the user is dragging.
    config: Option<&'a Config>,
    config_revision: u64,
    /// The Lua the rules would generate, sent with the config. The editor uses
    /// it for "start from my rules" while in script mode, where the compiled
    /// source is the hand-written one.
    rules_source: Option<String>,
    /// Only when it has changed: carries the compiled source and source map.
    script: Option<&'a ScriptStatus>,
    stats: Stats,
    /// Only when something new has been printed.
    console: Option<Vec<String>>,
    /// What grimlua did and what stopped it, for the LOG tab. Sent on change.
    events: Option<Vec<shared::Event>>,
    /// The parsed skill database, sent once per connection when it is ready.
    /// ~1,400 entries, so it is emphatically not part of the 10Hz push.
    skills: Option<SkillCatalogue<'a>>,
    /// The tags of the skills this character currently has, sent when the set
    /// changes. This is what the editor's "only skills I have" box filters on.
    available: Option<&'a [String]>,
}

/// The database as the editor needs it. A trimmed projection of
/// [`crate::db::Skill`]: the picker does not need every field, and this is the
/// one large payload on the socket.
#[derive(Serialize)]
struct SkillCatalogue<'a> {
    revision: u64,
    built_ms: u64,
    skills: Vec<CatalogueEntry<'a>>,
}

#[derive(Serialize)]
struct CatalogueEntry<'a> {
    path: &'a str,
    name: &'a str,
    tag: &'a str,
    castable: bool,
    kind: crate::db::skills::Kind,
    mastery: Option<u32>,
    cooldown: Option<f32>,
    duration: Option<f32>,
    /// Why it cannot be cast, ready to show at the point of the mistake.
    reason: Option<&'a str>,
}

/// What a browser may send. Everything is validated before it reaches the
/// config: the editor is a client like any other.
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Inbound {
    Config { config: Config },
    ClearConsole,
    ClearLog,
    /// Write the config out now rather than waiting for the autosave.
    Save,
}

pub fn start() {
    if start_on(PORT).is_err() {
        log!("server: no UI will be served");
    }
}

/// Serve on a specific port, reporting whether the bind succeeded.
///
/// Tests use this with a port of their own. That is not tidiness: `start`
/// used to swallow a failed bind and return, so a test run while a real
/// grimlua held 7890 would quietly connect to *that* server instead of its
/// own -- and then send it a config, overwriting the live game's rules. A
/// caller that cares must be able to tell the difference.
pub fn start_on(port: u16) -> std::io::Result<()> {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, port));
    let listener = match TcpListener::bind(addr) {
        Ok(l) => l,
        Err(e) => {
            log!("server: cannot bind {addr} ({e}) -- is another grimlua running?");
            return Err(e);
        }
    };
    log!("server: listening on http://{addr}");

    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        std::thread::Builder::new()
            .name("grimlua-conn".into())
            .spawn(move || serve(stream))
            .ok();
    }
    Ok(())
}

fn serve(stream: TcpStream) {
    // Peek rather than read, so a WebSocket upgrade can be handed to
    // tungstenite with the request bytes still unconsumed.
    let mut probe = [0u8; 1024];
    let Ok(n) = stream.peek(&mut probe) else { return };
    let head = String::from_utf8_lossy(&probe[..n]).to_ascii_lowercase();

    if head.contains("upgrade: websocket") {
        serve_websocket(stream);
    } else {
        serve_http(stream, &head);
    }
}

fn serve_http(mut stream: TcpStream, head: &str) {
    // Consume the request we only peeked at.
    let mut sink = [0u8; 4096];
    let _ = stream.read(&mut sink);

    let wants_index = head.starts_with("get / ") || head.starts_with("get /index.html ");
    let (status, body, mime) = if wants_index {
        ("200 OK", INDEX_HTML, "text/html; charset=utf-8")
    } else {
        ("404 Not Found", "not found", "text/plain; charset=utf-8")
    };

    let response = format!(
        "HTTP/1.1 {status}\r\n\
         Content-Type: {mime}\r\n\
         Content-Length: {}\r\n\
         Cache-Control: no-store\r\n\
         Connection: close\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();
}

/// Counts a connected browser for as long as this value lives.
///
/// The hook reads this to decide whether to evaluate the script while the gate
/// is closed, so it has to come back down on every exit path -- including the
/// several early returns below.
struct Watcher;

impl Watcher {
    fn new() -> Self {
        shared::WATCHERS.fetch_add(1, Ordering::Relaxed);
        Self
    }
}

impl Drop for Watcher {
    fn drop(&mut self) {
        shared::WATCHERS.fetch_sub(1, Ordering::Relaxed);
    }
}

fn serve_websocket(stream: TcpStream) {
    // A short read timeout lets one thread both poll for inbound config and
    // push snapshots on a clock, without needing to split the socket.
    let _ = stream.set_read_timeout(Some(Duration::from_millis(40)));

    let config = tungstenite::protocol::WebSocketConfig {
        max_message_size: Some(MAX_MESSAGE),
        max_frame_size: Some(MAX_MESSAGE),
        ..Default::default()
    };
    let mut ws = match tungstenite::accept_with_config(stream, Some(config)) {
        Ok(ws) => ws,
        Err(e) => {
            log!("server: websocket handshake failed: {e}");
            return;
        }
    };
    let _watcher = Watcher::new();
    log!("server: browser connected");

    // What this connection has already been told, so the heavy fields are sent
    // on change rather than on every tick.
    let mut sent_config: Option<u64> = None;
    let mut sent_script: Option<(u64, bool, Option<String>)> = None;
    let mut sent_console: Option<u64> = None;
    let mut sent_skills: Option<u64> = None;
    let mut sent_available: Option<u64> = None;
    let mut sent_events: Option<u64> = None;

    let mut next_push = Instant::now();
    loop {
        match ws.read() {
            Ok(Message::Text(text)) => handle_message(&text),
            Ok(Message::Close(_)) => break,
            Ok(_) => {}
            Err(tungstenite::Error::Io(e))
                if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(_) => break,
        }

        if Instant::now() >= next_push {
            next_push = Instant::now() + PUSH_INTERVAL;
            if !push(
                &mut ws,
                &mut sent_config,
                &mut sent_script,
                &mut sent_console,
                &mut sent_skills,
                &mut sent_available,
                &mut sent_events,
            ) {
                break;
            }
        }
    }
    log!("server: browser disconnected");
}

fn push(
    ws: &mut WebSocket<TcpStream>,
    sent_config: &mut Option<u64>,
    sent_script: &mut Option<(u64, bool, Option<String>)>,
    sent_console: &mut Option<u64>,
    sent_skills: &mut Option<u64>,
    sent_available: &mut Option<u64>,
    sent_events: &mut Option<u64>,
) -> bool {
    let snapshot = shared::snapshot();
    let config = shared::config();
    let revision = shared::config_revision();
    let script = shared::script_status();
    let (console_seq, console_lines) = shared::console();
    let (event_seq, event_lines) = shared::events();

    let script_sig = (script.revision, script.ok, script.error.clone());

    let catalogue = shared::skills();
    let skills_revision = shared::skills_revision();
    let available = shared::available();

    let fresh_config = *sent_config != Some(revision);
    let envelope = Envelope {
        snapshot: &snapshot,
        config: fresh_config.then_some(&config),
        config_revision: revision,
        rules_source: fresh_config.then(|| config.generated_source()),
        script: (sent_script.as_ref() != Some(&script_sig)).then_some(&script),
        stats: Stats {
            ok: script.ok,
            halted: script.halted,
            evaluations: script.evaluations,
            errors: script.errors,
            last_error: script.last_error.clone(),
            last_eval_us: script.last_eval_us,
        },
        console: (*sent_console != Some(console_seq)).then_some(console_lines),
        events: (*sent_events != Some(event_seq)).then_some(event_lines),
        skills: match catalogue.as_deref() {
            Some(index) if *sent_skills != Some(skills_revision) => Some(SkillCatalogue {
                revision: skills_revision,
                built_ms: index.built_ms,
                skills: index
                    .skills
                    .iter()
                    .map(|s| CatalogueEntry {
                        path: &s.path,
                        name: &s.name,
                        tag: &s.tag,
                        castable: s.castable,
                        kind: s.kind,
                        mastery: s.mastery,
                        cooldown: s.cooldown,
                        duration: s.duration,
                        reason: (!s.castable)
                            .then(|| crate::db::skills::not_castable_reason(&s.class)),
                    })
                    .collect(),
            }),
            _ => None,
        },
        available: (*sent_available != Some(available.revision)).then_some(&available.tags),
    };

    let payload = match serde_json::to_string(&envelope) {
        Ok(p) => p,
        Err(_) => return true,
    };

    match ws.send(Message::Text(payload.into())) {
        Ok(()) => {
            // Only record what was actually handed to the socket.
            *sent_config = Some(revision);
            *sent_script = Some(script_sig);
            *sent_console = Some(console_seq);
            *sent_events = Some(event_seq);
            if catalogue.is_some() {
                *sent_skills = Some(skills_revision);
            }
            *sent_available = Some(available.revision);
            true
        }
        Err(tungstenite::Error::Io(e))
            if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) =>
        {
            true
        }
        Err(_) => false,
    }
}

/// A message from the browser. A config is applied straight away; the frame
/// hook recompiles and picks it up on its next tick, which is what "no
/// restart, no reload command" means in practice.
fn handle_message(text: &str) {
    match serde_json::from_str::<Inbound>(text) {
        Ok(Inbound::Config { config }) => {
            let mode = match config.mode {
                shared::ScriptMode::Rules => "rules",
                shared::ScriptMode::Script => "lua",
            };
            log!(
                "config: armed={} mode={mode} rules={}",
                config.armed,
                config.rules.len(),
            );
            shared::event("config", format!("{} rule(s) applied in {mode} mode", config.rules.len()));
            shared::put_config(config);
        }
        Ok(Inbound::ClearConsole) => shared::console_clear(),
        Ok(Inbound::ClearLog) => shared::events_clear(),
        Ok(Inbound::Save) => {
            if shared::save() {
                log!("config: saved");
            }
        }
        Err(e) => log!("server: rejected malformed message ({e})"),
    }
}
