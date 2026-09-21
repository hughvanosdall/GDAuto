//! The browser/host wire contract, exercised against the real server.
//!
//! Everything else about the scripting layer is unit-tested, but the seam most
//! likely to break silently is this one: serde renames a variant, the editor
//! keeps sending the old spelling, and the only symptom is a rule that never
//! fires. So these tests speak the exact JSON the page sends, byte for byte,
//! rather than round-tripping Rust structs through serde and proving nothing.
//!
//! The frame hook is not running here, so snapshots stay empty. What is under
//! test is the config path: browser -> server -> `shared::CONFIG`.

use std::net::TcpStream;
use std::time::{Duration, Instant};

use grimlua_core::rules::ActionSpec;
use grimlua_core::shared::{self, ScriptMode};
use tungstenite::{Message, WebSocket};

/// What the editor sends when a user adds a rule with two conditions and a
/// per-rule cooldown, copied from the shape `index.html` builds.
const BROWSER_CONFIG: &str = r#"{
  "type": "config",
  "config": {
    "armed": true,
    "global_cooldown_ms": 900,
    "mode": "rules",
    "rules": [
      {
        "id": "r1",
        "name": "Emergency flask",
        "enabled": true,
        "action": { "kind": "health_potion" },
        "conditions": [
          { "subject": "life_pct", "op": "below", "value": 35 },
          { "subject": "since_any_action", "op": "at_least", "value": 2.5 }
        ],
        "cooldown_ms": 1500
      },
      {
        "id": "r2",
        "name": "Rotation",
        "enabled": true,
        "action": {
          "kind": "skill",
          "path": "records/skills/playerclass03/bloodofdreeg1.dbr"
        },
        "conditions": [],
        "cooldown_ms": 8000
      }
    ],
    "script": "function choose(s) return nil end",
    "editor": { "only_available": true }
  }
}"#;

/// Connect over a plain `TcpStream` rather than `tungstenite::connect`, which
/// would hand back a TLS-capable stream type. The server is loopback-only and
/// speaks no TLS, so there is nothing to negotiate.
/// A port of this test's own, never grimlua's real one.
///
/// Binding 7890 here would find a *running game's* server if one is up, and
/// this test sends a config -- which would overwrite the live rules of whoever
/// is playing. It has happened. The test now owns its port or fails.
const TEST_PORT: u16 = 7899;

fn connect() -> WebSocket<TcpStream> {
    let addr = format!("127.0.0.1:{TEST_PORT}");
    let url = format!("ws://{addr}/ws");
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Ok(stream) = TcpStream::connect(&addr) {
            if let Ok((ws, _)) = tungstenite::client(url.as_str(), stream) {
                // Only after the handshake: a timeout during it would look
                // like a failed upgrade.
                let _ = ws.get_ref().set_read_timeout(Some(Duration::from_millis(200)));
                return ws;
            }
        }
        assert!(Instant::now() < deadline, "the server never came up");
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn next_envelope(ws: &mut WebSocket<TcpStream>) -> serde_json::Value {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if let Ok(Message::Text(text)) = ws.read() {
            return serde_json::from_str(&text).expect("the server sent invalid JSON");
        }
    }
    panic!("the server sent nothing");
}

/// One test, not several: the server binds a fixed port, so a second test
/// running beside this one would race it for the socket.
#[test]
fn the_browser_contract_holds_end_to_end() {
    // Prove the port is ours before anything else runs. A failed bind here
    // must stop the test, not silently redirect it at someone else's server.
    match std::net::TcpListener::bind(("127.0.0.1", TEST_PORT)) {
        Ok(probe) => drop(probe),
        Err(e) => panic!("port {TEST_PORT} is not free ({e}); refusing to talk to another server"),
    }
    std::thread::spawn(|| {
        grimlua_core::server::start_on(TEST_PORT).expect("the test server must bind its own port")
    });
    let mut ws = connect();

    // ── the first push carries everything a fresh page needs ──
    let first = next_envelope(&mut ws);
    for field in ["snapshot", "config", "config_revision", "rules_source", "stats"] {
        assert!(!first[field].is_null(), "the first push had no {field}: {first}");
    }
    assert_eq!(
        first["config"]["armed"], false,
        "a page opening on a fresh launch must be shown a disarmed tool"
    );
    assert!(
        first["rules_source"].as_str().unwrap().contains("function choose(s)"),
        "the generated Lua was not sent with the config"
    );

    // ── a config the editor built is accepted verbatim ──
    // Waiting on the revision rather than on the contents: the stock config
    // also has two rules, so "two rules are live" would pass before the
    // browser's config had been applied at all.
    let before_send = shared::config_revision();
    ws.send(Message::Text(BROWSER_CONFIG.into())).unwrap();

    let deadline = Instant::now() + Duration::from_secs(5);
    let cfg = loop {
        assert!(Instant::now() < deadline, "the config was never applied");
        if shared::config_revision() != before_send {
            break shared::config();
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    assert_eq!(cfg.rules.len(), 2);

    assert!(cfg.armed, "arming from the browser did not take");
    assert_eq!(cfg.global_cooldown_ms, 900);
    assert_eq!(cfg.mode, ScriptMode::Rules);
    assert_eq!(cfg.rules[0].conditions.len(), 2);
    assert_eq!(cfg.rules[0].cooldown_ms, 1500);
    assert_eq!(
        cfg.rules[1].action,
        ActionSpec::Skill { path: "records/skills/playerclass03/bloodofdreeg1.dbr".into() }
    );
    assert!(cfg.editor.only_available);

    // The rules the editor sent must produce a script that actually runs --
    // this is the whole chain: browser JSON, IR, code generation, sandbox.
    let (source, marks) = cfg.program();
    assert!(
        grimlua_core::script::Evaluator::compile(&source).is_ok(),
        "the generated script does not compile:\n{source}"
    );
    assert_eq!(marks.len(), 2, "both rules should be in the source map");

    // ── the config comes back to the page ──
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        assert!(Instant::now() < deadline, "the new config was never echoed");
        let msg = next_envelope(&mut ws);
        if let Some(config) = msg.get("config").filter(|c| !c.is_null()) {
            assert_eq!(config["rules"][1]["action"]["kind"], "skill");
            assert_eq!(
                config["rules"][1]["action"]["path"],
                "records/skills/playerclass03/bloodofdreeg1.dbr"
            );
            assert_eq!(config["rules"][0]["conditions"][1]["subject"], "since_any_action");
            break;
        }
    }

    // ── nonsense is rejected rather than half-applied ──
    let before = shared::config_revision();
    ws.send(Message::Text("{\"type\":\"config\",\"config\":\"not a config\"}".into()))
        .unwrap();
    ws.send(Message::Text("{\"type\":\"nonsense\"}".into())).unwrap();
    std::thread::sleep(Duration::from_millis(300));
    assert_eq!(shared::config_revision(), before, "a malformed message changed the config");
    assert_eq!(shared::config().rules.len(), 2, "the good config was lost");
}
