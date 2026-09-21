# grimlua

A single DLL that loads into Grim Dawn, parses the game's skill database, runs
a user-configured skill priority list against live game state, and serves a
local web UI for building those priority lists visually.

See [CLAUDE.md](CLAUDE.md) for the design and the reasoning behind it.

## State

Build-order steps 0-5 complete and confirmed in a live game.
See the build order in CLAUDE.md.

- [x] **Step 0 — symbol index.** Grim Dawn ships modding-enabled builds that
      export 31,645 mangled C++ symbols. Harvested into `symbols/`.
- [x] **Step 1 — proxy DLL.** `dinput8.dll` loads, forwards all six exports to
      the real System32 copy, and logs. Confirmed in game, clean detach.
- [x] **Step 2 — frame hook.** `GameEngine::Update` detoured; 10,800 ticks
      across live gameplay, `GameEngine*` captured from RCX.
- [x] **Step 3 — read health.** Life and mana read through the game's own
      `const` accessors, tracked through damage and regeneration.
- [x] **Step 4 — UI-open flag.** `IsTransferOpen` (exported) for stash and
      vendor, plus one byte inside the `GetUI()` object for panel visibility,
      which is not exported. The project's only raw offset: build-pinned and
      fails closed. See `symbols/ANCHORS.md`.
- [x] **Step 5 — HTTP + websocket.** Local UI on 127.0.0.1:7890 with live
      vitals and gate state, plus the first real action: auto-potions, run
      through the safety gate on the frame hook. Ctrl+Shift+G opens it.
- [ ] Step 6 — `.arz` parser and skill index
- [ ] Step 7 — mlua and the priority list evaluator
      (see "Where the script evaluator plugs in" in CLAUDE.md)
- [ ] Step 8 — web editor
- [ ] Step 9 — in-game escape-menu button (hotkey stands in for now)

## Layout

    tools/          symbol harvesting, probe generation, install (stdlib only)
    symbols/        the generated symbol index; ANCHORS.md is the short version
    crates/
      grimlua-core/     all behaviour, independent of how the DLL was loaded
        hook.rs           the frame hook - the only thread that touches the game
        gate.rs           the safety gate; one veto point for every action
        runtime.rs        the tick: read, gate, at most one action
        state.rs          game accessors, resolved by exported name
        shared.rs         snapshot out / config in; the hook-server boundary
        server.rs         HTTP + WebSocket, loopback only
        hotkey.rs         Ctrl+Shift+G opens the UI
        web/index.html    the UI, embedded at compile time
        probe.rs, scan.rs discovery scaffolding, not used at runtime
      grimlua-dinput8/  the proxy shim that gets us into the process

## Build

Needs the `x86_64-pc-windows-msvc` toolchain. The 32-bit game build is out of
scope: symbol addresses differ per build and almost nobody runs it.

    cargo build --release --target x86_64-pc-windows-msvc
    python tools/install.py            # copy into <game>\x64\dinput8.dll
    python tools/install.py --uninstall

Uninstalling is a single file delete. Nothing the game ships is renamed or
modified.

The log lands next to the DLL, at `<game>\x64\grimlua.log`.

## Using it

Launch the game, then press **Ctrl+Shift+G**. grimlua starts **disarmed every
run** on purpose; arming is a deliberate act.

Nothing fires unless the gate is clear: armed, a character in the world, no UI
panel open, game focused, off cooldown. Anything it cannot establish - a game
patch moving the UI offset, for instance - blocks every action rather than
guessing.

## Symbols

`Game.dll`, `Engine.dll` and `Widget.dll` export nearly everything, because the
game directory also ships the modding tools that link against them. That makes
`GetProcAddress` with a mangled name the primary way to reach game code —
no signature scanning, no pointer chains, no offsets.

    python tools/dump_exports.py                          # regenerate the index
    python tools/query_symbols.py --class Character --name life
    python tools/query_symbols.py --classes Skill
    python tools/query_symbols.py --regex Cooldown --full

Regenerate the index after a game patch; `grimlua.log` warns at startup when
the running modules no longer match what was harvested.
