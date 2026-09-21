//! Does a runaway script get stopped without taking the process with it?
//!
//!     cargo run --example runaway        # must print "survived"
//!
//! **This cannot be a unit test.** Cargo ignores the `panic` profile setting
//! for the `test` profile, so `cargo test` builds with unwinding whatever
//! `Cargo.toml` says, and the failure this guards against only appears under
//! the profile the DLL is actually built with. It has to be a real binary.
//!
//! The failure it guards against: mlua signals an error from a Rust callback
//! by longjmping out through `lua_error`. Under `panic = "abort"` on MSVC that
//! trips `panic_cannot_unwind` and kills the process — so the instruction
//! budget, the one thing standing between an endless loop and a frozen game,
//! became the thing that killed the game. Run this after touching the sandbox,
//! the mlua version, or the panic strategy.

use grimlua_core::script::{Evaluator, ScriptState};

/// Every way a script can misbehave that the host is supposed to survive.
const HOSTILE: [(&str, &str); 4] = [
    ("endless loop", "function choose(s) while true do end end"),
    ("endless recursion", "function choose(s) return choose(s) end"),
    ("memory bomb", "function choose(s) local t = {} for i = 1, 1e9 do t[i] = i end end"),
    ("error thrown", "function choose(s) error('boom') end"),
];

fn main() {
    let mut failures = 0;

    for (name, source) in HOSTILE {
        let ev = match Evaluator::compile(source) {
            Ok(ev) => ev,
            Err(e) => {
                println!("  {name:<20} refused at compile time: {e}");
                continue;
            }
        };
        // Repeated, because a budget that is not reset per call would let the
        // first one through and kill a later one.
        for _ in 0..5 {
            match ev.evaluate(&ScriptState::default()) {
                Ok(outcome) => {
                    println!("  {name:<20} !! RAN TO COMPLETION: {outcome:?}");
                    failures += 1;
                }
                Err(e) => println!("  {name:<20} stopped: {}", first_line(&e)),
            }
        }
    }

    // A well-behaved script must still work after all that.
    let ev = Evaluator::compile("function choose(s) return 'health_potion', 'ok' end").unwrap();
    match ev.evaluate(&ScriptState::default()) {
        Ok(o) if o.action.as_deref() == Some("health_potion") => {}
        other => {
            println!("  !! the VM is unusable afterwards: {other:?}");
            failures += 1;
        }
    }

    if failures == 0 {
        println!("survived");
    } else {
        println!("{failures} failure(s)");
        std::process::exit(1);
    }
}

fn first_line(s: &str) -> String {
    s.lines().next().unwrap_or("").chars().take(90).collect()
}
