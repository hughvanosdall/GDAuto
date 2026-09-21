//! The local HTTP + WebSocket server.
//!
//! **Binds to 127.0.0.1 and nothing else.** A game process listening on all
//! interfaces is a real risk, not a theoretical one, so the address is a
//! literal here and there is no setting to widen it.
//!
//! This thread never calls into Grim Dawn. It reads [`shared::SNAPSHOT`] and
//! writes [`shared::CONFIG`]; the frame hook does the reverse. That is the
//! only channel between them, which is what stops a slow browser from
//! stalling the game thread.

use std::io::{ErrorKind, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::time::{Duration, Instant};

use serde::Serialize;
use tungstenite::{Message, WebSocket};

use crate::log;
use crate::shared::{self, Config, Snapshot};

pub const PORT: u16 = 7890;

const INDEX_HTML: &str = include_str!("web/index.html");

/// How often a connected browser is pushed a new snapshot.
const PUSH_INTERVAL: Duration = Duration::from_millis(100);

#[derive(Serialize)]
struct Envelope<'a> {
    snapshot: &'a Snapshot,
    config: &'a Config,
}

pub fn start() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, PORT));
    let listener = match TcpListener::bind(addr) {
        Ok(l) => l,
        Err(e) => {
            log!("server: cannot bind {addr} ({e}) -- is another grimlua running?");
            return;
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

fn serve_websocket(stream: TcpStream) {
    // A short read timeout lets one thread both poll for inbound config and
    // push snapshots on a clock, without needing to split the socket.
    let _ = stream.set_read_timeout(Some(Duration::from_millis(40)));
    let mut ws = match tungstenite::accept(stream) {
        Ok(ws) => ws,
        Err(e) => {
            log!("server: websocket handshake failed: {e}");
            return;
        }
    };
    log!("server: browser connected");

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
            if !push(&mut ws) {
                break;
            }
        }
    }
    log!("server: browser disconnected");
}

fn push(ws: &mut WebSocket<TcpStream>) -> bool {
    let snapshot = shared::snapshot();
    let config = shared::config();
    let payload = match serde_json::to_string(&Envelope { snapshot: &snapshot, config: &config }) {
        Ok(p) => p,
        Err(_) => return true,
    };
    match ws.send(Message::Text(payload.into())) {
        Ok(()) => true,
        Err(tungstenite::Error::Io(e))
            if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) =>
        {
            true
        }
        Err(_) => false,
    }
}

/// Config arriving from the browser. Applied straight away; the frame hook
/// picks it up on its next tick, which is what "no restart" means.
fn handle_message(text: &str) {
    match serde_json::from_str::<Config>(text) {
        Ok(cfg) => {
            log!(
                "config: armed={} health={}@{:.0}% energy={}@{:.0}%",
                cfg.armed,
                cfg.health_potion.enabled,
                cfg.health_potion.threshold,
                cfg.energy_potion.enabled,
                cfg.energy_potion.threshold
            );
            shared::put_config(cfg);
        }
        Err(e) => log!("config: rejected malformed message ({e})"),
    }
}
