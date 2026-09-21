# grimlua

A single DLL that loads into Grim Dawn, parses the game's skill database, runs
a user-configured skill priority list against live game state, and serves a
local web UI for building those priority lists visually.

Status: build-order steps 0–3 done and confirmed in a live game — symbol
index, proxy DLL, frame hook, live health read. See README.md for current
state and `symbols/ANCHORS.md` for the confirmed entry points.

**No memory offsets were ever needed.** Grim Dawn ships modding-enabled builds
that export ~31,600 mangled C++ symbols; see "The export table" below. That
single fact invalidated several decisions in the original draft of this
document, and the affected sections have been rewritten.

## Architecture

```
x64/Grim Dawn.exe
└── dinput8.dll (ours, proxy)
    ├── proxy exports ──→ real System32\dinput8.dll
    ├── frame hook ─────  the ONLY thread that touches the game   [built]
    │   ├── state reader   life, energy, UI state                  [built]
    │   ├── safety gate    one veto point for every action         [built]
    │   └── runtime tick   evaluate, then at most one action       [built]
    ├── shared state ───  snapshot out, config in; no game pointers [built]
    ├── .arz parser ────  skills, durations, costs, icons
    ├── mlua ───────────  sandboxed priority-list evaluation
    └── 127.0.0.1:7890 ─  own thread, never calls into the game    [built]
        ├── HTTP ──────── the UI page (+ skill icons later)        [built]
        └── WebSocket ─── live state out, config in                [built]
```

Rust, x64 only. `retour` for detours, `windows-sys` for Win32, `mlua` for Lua,
`axum` or `tiny_http` + `tungstenite` for the server.

The 32-bit build is out of scope: addresses differ per build and almost nobody
runs it. The installed game reports x64 + direct3d11.

## Hard constraints

**Bind to 127.0.0.1, never 0.0.0.0.** A game process listening on all
interfaces is a real risk, not a theoretical one.

**All game interaction happens on the frame hook.** Game engines are not
thread-safe. The web server and any worker threads queue work; they never call
into the game directly. This is the difference between a tool that works and
one that corrupts state randomly.

**Lua scripts never get raw memory access.** No read/write primitives exposed,
ever. Scripts are shared between users; a script that can poke memory makes the
library a malware channel. Scripts declare intent, the host validates.

**The safety gate lives in the host.** One veto point for every action:
disarmed, game unfocused, any UI panel open, in town, or under the global
cooldown → blocked. Script authors cannot opt out per-call. This is the entire
value proposition over a keyboard macro.

**Fail closed.** If the UI-open flag isn't resolved, perform no actions at all.

**One action per evaluation.** The gate serializes everything. Any design
implying "do A then B then C in one tick" is wrong — see priority lists below.

**Hardcore characters exist.** A crash or misfire permanently deletes someone's
character. Bias toward inaction.

**Link the CRT statically.** Grim Dawn ships its own `x64\ucrtbase.dll` from
2017 (10.0.14393.795), and Windows searches the application directory before
System32 -- so a dynamically linked DLL binds its CRT to that 2017 UCRT while
picking up a 2026 `VCRUNTIME140.dll` from System32. That pairing killed the
game at startup with no dialog, no WER report and no crash dump. `+crt-static`
in `.cargo/config.toml`; our imports are now kernel32 and ntdll only. More
generally: **take nothing from the host process that we can bring ourselves.**

**Nothing heavy in `DllMain`.** It runs under the loader lock. No hooks, no
large stack frames (a 64 KB buffer there is a silent stack overflow waiting to
happen on a small game thread), no work beyond recording a handle and opening
a log. The frame hook is installed from a worker thread, which cannot begin
executing until the loader lock is released.

**Config hot-applies.** Edit in the browser → websocket → runtime swaps the
active list next frame. No restart, no reload command. Retrofitting this is
painful; building for it is easy.

## Decisions and rejected alternatives

**Proxy DLL, not a launcher, not a runtime injector.** Ship `dinput8.dll`
into `<game>\x64\`, forward all exports to the real System32 copy. Windows'
search order does the rest (the ReShade/ENB approach). Install is one file
copy; uninstall is one delete; nothing the game ships is renamed.

`version.dll` does **not** work here — nothing in the process dependency
closure imports it. `dinput8.dll` does: Grim Dawn's own `DirectInput.dll`
imports `DirectInput8Create` from `DINPUT8.dll`, and the game ships no
`dinput8.dll`, so the application-directory-first search finds ours. Verified
in game.

Fallback if that slot is ever lost: `CrashReport.dll`, which the exe imports
statically (so it loads at process init, earlier than dinput8) and which has
only 9 trivially-forwardable exports. It costs a rename of the shipped file.

Forward with **runtime-bound thunks, not linker forwarders.** A forwarder
string names a module, and the module named `dinput8` is us — it would forward
to itself. For the same reason the real DLL must be loaded by *full path*: a
bare `LoadLibrary("dinput8.dll")` returns the already-loaded module of that
name, which is us, and every thunk silently becomes an infinite loop.
`tools/verify_proxy.py` exists to catch exactly that.

NOT a renderer-named proxy like `d3d9.dll` — Grim Dawn ships both DX9 and DX11
renderers and the user picks in options, so a renderer-named proxy breaks or
needs two builds. `dinput8.dll` is renderer-agnostic.

NOT a launcher: Grim Internals uses one, and its own documentation warns Steam
Cloud Save doesn't work properly through it. Proxy means the game starts
normally through Steam — cloud saves, overlay, achievements, playtime all
intact. Also no admin required, and uninstall is deleting one file.

A launcher that starts the game through the Steam API and then injects is a
reasonable *fallback*, and the code is structured so it stays cheap: all
behaviour lives in `grimlua-core`, which knows nothing about how it was
loaded, and the proxy is a thin shim around `init()`. But it is Steam-only,
which works against later GOG and Epic support that a proxy gets for free, so
it stays a fallback rather than the plan.

**Rust, specifically for `mlua`.** The earlier Python prototype used lupa,
whose whole purpose is reflective Python↔Lua bridging — a script reaching any
host object can walk `__class__.__mro__.__subclasses__()` to `__builtins__` and
RCE. `mlua` has no such bridge: build the environment from an empty table,
insert exactly the functions we wrote, nothing else is reachable. It also
exposes Lua's debug hook, so an instruction-count limit against
`while true do end` hanging the frame hook is a few lines.

Still needed regardless: no `load`/`loadstring`/`dofile`/`require`, absolutely
no `package.loadlib` (direct DLL load), `string.rep` as a memory bomb, no shared
string metatable.

**No Electron, no Tauri.** The DLL serves HTTP, so the UI is a browser tab.
Launch with `--app=http://localhost:7890` for a chromeless window. One
`ShellExecute`. Tauri (WebView2, ~5MB) only if that proves insufficient.

**The export table is the offset dumper.** The original draft of this
document argued no general dumper was possible: Grim Dawn is native C++ on the
Titan Quest engine, with no managed runtime, no metadata and no reflection, so
the information could not be in the binary. That reasoning was sound and the
conclusion was wrong.

The game directory ships the modding tools — `Editor.exe`, `DBREditor.exe`,
`QuestEditor.exe` — which link against the game's own modules. So those modules
export nearly everything:

| Module | Named exports |
|---|---|
| `Game.dll` | 25,100 |
| `Engine.dll` | 6,283 |
| `Widget.dll` | 262 |

Full MSVC mangling, so demangling recovers class name, method name, parameter
types, const-ness and calling convention. This is *better* than IL2CPP
metadata: it carries type signatures, not just addresses.

Consequences:

- **Resolve by mangled name via `GetProcAddress`.** Not signature scanning,
  not pointer chains. An exported name survives patches unless Crate actually
  renames the function; a signature may not survive a recompile.
- `tools/dump_exports.py` harvests all of it into `symbols/symbols.json`;
  `tools/query_symbols.py` searches it. Regenerate after a game patch — the
  runtime logs a warning at startup when the loaded modules no longer match
  what was harvested.
- **RTTI recovery is moot.** RTTI is largely stripped (`Widget.dll` keeps 4
  descriptors, all `type_info`). Exports replace it entirely, so nothing is
  lost. Do not spend time on an RTTI scanner.
- Signature scanning stays in reserve for the genuinely non-exported
  internals: the `.dbr` record loader and the UI-open state, neither of which
  appears in any export table.

**Hooking an exported member function hands you the object.** `this` arrives
in RCX on x64, so hooking `GameEngine::Update(int)` yields the `GameEngine*`
with no pointer chain to resolve. `GetMainPlayer()` reaches the player from
there, and health is a `const` getter call rather than a struct offset. No
struct layouts have had to be reversed at all.

**Keep an external read mode behind a trait.** ~50 extra lines, and it lets you
iterate on offsets without restarting the game and gives a safe fallback when
unsure whether an offset is right. You'll use it constantly during discovery.

## The runtime, as built

Everything below exists and is confirmed in a live game. This is the part to
read before touching the scripting layer.

### Threads, and the one rule that matters

| Thread | Does | May touch the game |
|---|---|---|
| frame hook (`hook.rs`) | the whole runtime tick | **yes — only this one** |
| `grimlua-server` (`server.rs`) | accept loop on 127.0.0.1:7890 | no |
| `grimlua-conn` (one per browser) | push snapshots, take config | no |
| `grimlua-hotkey` (`hotkey.rs`) | Ctrl+Shift+G, opens the UI | no |
| `grimlua-init` (`lib.rs`) | installs the hook, starts the rest | no |

The server and the hook never call each other. They meet only in `shared.rs`,
through two locks holding plain data: `SNAPSHOT` (hook writes, server reads)
and `CONFIG` (server writes, hook reads). Neither holds a game pointer, so a
wedged browser cannot stall or corrupt the game thread. **Do not add a path
that lets the server reach a `GameEngine*`.**

### The tick

`runtime::tick(engine, frame)`, every 6 frames (~20Hz at 120fps):

1. read vitals through the game's own `const` accessors
2. `gate::evaluate` → a `Gate`
3. if and only if `Gate::Clear`, pick **one** action and perform it
4. publish a `Snapshot` for the browser

### The gate

`gate::evaluate(engine, last_action) -> Gate`. Blocks on: disarmed, no
character, UI panel open, game unfocused, global cooldown (default 750ms).
Only `Gate::Clear` permits an action, and `Gate::Unknown` — returned when the
UI flag cannot be trusted on the running build — blocks everything.

`armed` defaults to **false on every launch**. A tool that arms itself acts
before its owner is watching.

### Where the script evaluator plugs in

`runtime::choose_action(&Config, &Vitals) -> Option<Action>` is a hardcoded
stand-in for the priority list: ordered checks, first match wins, returns at
most one action. **That signature is the contract mlua has to satisfy.**

Four things the scripting layer must preserve:

1. **A script returns intent, it does not act.** `Action` is a closed enum the
   host owns. A script names an action; the host validates it and performs it.
   Scripts never receive a function pointer or a game pointer.
2. **At most one action per tick.** The gate serialises everything, so
   "do A then B this tick" is not expressible. Returning a list would be a
   design error, not a feature.
3. **The gate runs before the script and cannot be bypassed.** A script is not
   consulted at all when the gate is closed, so there is no per-call opt-out to
   forget to check.
4. **A script that misbehaves must not stall the frame hook.** mlua exposes
   Lua's debug hook; use it for an instruction-count limit. This runs on the
   game's render thread — a hang here is a hang in the game.

### Adding an action

1. add a variant to `runtime::Action` and a label for it
2. resolve the exported game function in `state.rs`'s `game_api!` block
3. call it from `runtime::perform`

Actions must be **exported game functions**, never synthetic input. The potion
rules call `PlayerHotSlotCtrl::ActivateHealthPotionSlot`, the same entry point
the keybind reaches, so the game applies its own rules about cooldowns and
charges and grimlua cannot make it do anything the player could not. See
`symbols/ANCHORS.md`.

### Config and hot-apply

The browser sends a whole `Config` over the websocket; `shared::put_config`
stores it and bumps `CONFIG_REVISION`; the hook reads it on the next tick.
That is what "no restart, no reload command" means in practice.

### The UI

One page, served from `src/web/index.html` and embedded with `include_str!`.
Vanilla JS, no build step, no framework. It renders stale data greyed out and
labelled rather than letting frozen numbers pass as live.

Deliberately **not** shown: DPS and resistances. Both are reachable
(`Player::CalculateDps`, `Character::GetAllDefenseAttributes`) but are not
wired, and inventing numbers on a live dashboard is worse than omitting them.

## The database

Parse base `database.arz`, expansion archives, and the active mod's
`database/<mod>/database.arz`. Extract `records/skills/**`, the `.tpl`
templates that define field schemas, and `text_en` tag files.

**In memory, not SQLite.** Skills-only is a few thousand records across ten
masteries plus devotions and item skills. Parse to structs, serialize the index
to one cache file, serve JSON. SQLite only becomes worth it if items get
indexed later, which is out of scope.

**Rebuild triggers automatically.** Cache key = hash of the `.arz` file set plus
active mod name. Mismatch on startup → rebuild, show progress in the web UI.
The user never clicks anything. Being in-process means we can read the loaded
mod directly rather than guessing from config.

**Icons.** `.tex` inside `.arc`, close to DDS with a custom header. Convert once
at build time, cache as PNG, serve at `/icons/<id>.png`. Web UI gets them via
plain `<img>`.

**End users never run AssetManager.** That's a dev tool for understanding the
format. Use it now to explore; the shipped DLL parses natively. References:
atom0s's `grimarz`/`grimarc`, ARZExplorer in TQVaultAE (works because Grim Dawn
runs on the Titan Quest engine).

Three gotchas:
1. Names aren't in records — `.dbr` holds tags like `tagSkillNameB011`, strings
   live in `text_en`. Mods ship their own tag files.
2. `.tpl` templates are the schema; each `.dbr` declares its `templateName`.
3. Records chain — buff skills point at a separate buff record, and many values
   are arrays indexed by skill rank. Resolving "Pneumatic Burst duration at rank
   12" is a traversal, not a field read.

## Priority lists, not flowcharts

The runtime does one action per evaluation, so execution-flow chains silently
drop actions. The model is an Action Priority List (as in SimulationCraft): an
ordered selector, first matching condition wins.

The web editor may look like Unreal Blueprints — node canvas, typed colored
pins — but has no exec pins. Settled points:

- Graph → typed IR → Lua. Not graph → string. The IR catches type errors and
  unconnected pins with node-level messages.
- One direction only. Graph is source of truth, Lua is a build artifact. Never
  attempt decompilation.
- Emit a source map (node id → line range), push the firing node over the
  existing websocket, highlight live on the canvas.
- Node definitions are data, not classes. One registry, three sources: built-in
  bundle, skill nodes generated from the parsed `.arz`, user Lua nodes. This is
  what makes mod support fall out for free.
- `Skill` is a pin type carrying a record reference, not a string. Enables
  compile-time validation: passive wired into a Cast node, refresh-at longer
  than the real buff duration, skill removed by a patch.
- Don't build the canvas — React Flow / Litegraph / Rete.
- **Ship a form-based rule builder first** ("When [health below] [55%] → [drink
  flask]"). Same IR, same codegen, one week, actually shippable. The canvas
  becomes a second front end on a proven backend.

Sharing trust tiers: a graph using only built-in and DB-derived nodes is pure
data, safe by construction, no sandbox needed. Embedded Lua nodes and raw
scripts are code — show the source, require acknowledgement, never auto-install.

## Escape menu button

Wanted: a link in the in-game escape menu that opens the web UI.

**Not as a mod.** Grim Dawn's UI is partly data-driven so native buttons are
possible in principle, but GD runs only one mod at a time via Custom Game —
shipping UI changes as a mod conflicts with every other mod. Self-defeating.

**Not a renderer-hook overlay, and not a separate always-on-top window.** Both
draw *over* the game. We're injected, so the engine's own UI system is
available — make the game draw the button itself. It then matches art style,
scales with resolution, gets hit-testing free, and is unaffected by the
DX9/DX11 renderer choice.

**Approaches, by tractability:**

1. **Hook the record loader.** The engine resolves `.dbr` records by path
   string. Hook that lookup; when it requests the escape menu record, return a
   patched copy from memory with the button added. No disk changes, no mod, no
   widget class knowledge needed — we feed data to code that already knows how
   to build buttons. Most patch-resilient, since record schemas change less
   than code addresses. Find it by cross-referencing a `.dbr` path string.
2. **Hook menu construction, append a widget** via the engine's own widget
   constructor. Needs the constructor address, calling convention, and enough
   class layout to pass sane args.
3. **Clone an existing button** from the built menu's child list. Needs widget
   struct layout.
4. **Hook an existing menu item's handler.** Ugly fallback, one hour.

**The real blocker is command dispatch, not the button.** UI records almost
certainly reference engine-defined command IDs, not arbitrary callbacks — a
data file can't carry our function pointer. Workaround: use an unused/out-of-
range command ID and hook the dispatcher to intercept before the default case,
or point at a harmless existing command and hook that handler, checking sender.

**Find the dispatcher early.** If command routing is closed, approaches 1–3 all
produce a button that does nothing, and that should be discovered before a week
is spent.

**Two leads to chase first:**

- **The exported class list** (was: RTTI — now unnecessary, see above). Note
  that `Widget.dll` is a red herring: its 19 classes are `TreeView`,
  `DirectoryBrowser`, `FileBrowserWindow`, `StatusBar` and friends, a
  Win32-style toolkit for the *modding tools*, not the in-game HUD. The game
  UI lives in `Game.dll` as `*Ctrl` classes such as `PlayerInventoryCtrl`.
  Start there.
- **The `settings/` folder.** GD already loads loose files from `settings/`
  with no mod — that's how the Rainbow filter tool ships `text_en` overrides.
  So override logic exists in the resource manager. Reportedly localization-
  only (forum reports say loose `/database` and `/resources` are ignored in
  favour of the original `.arc` files), but worth re-testing directly. Either
  way, finding where the localization path diverges from the record path points
  straight at the function to hook for approach 1.

Pair whatever ships with a global hotkey; that's what people use after week one.

## Reverse engineering workflow

**Static: Ghidra + GhidraMCP** (bethington/ghidra-mcp, Apache 2.0, ~110 MCP
tools / 132 REST endpoints / 70+ batch scripts). Claude Code queries the
disassembler directly — decompile, cross-reference, rename, apply struct types.

The important part is that it writes back. Renames and applied types accumulate
in the Ghidra project rather than in a chat log, and the decompiler output gets
more legible with each pass. Treat the Ghidra project as the durable artifact.

Needs Ghidra 12.0.2, Java 21, Python 3.8+. Binds to 127.0.0.1. Decompile
timeout defaults to 60s and is configurable per call — some Grim Dawn functions
will need more.

**Dynamic: Cheat Engine / x64dbg.** Ghidra does not run the game. Same author
ships `cheat-engine-server-python` (MCP for dynamic memory analysis) if the
live half should also be agent-driven.

**Discipline: every static conclusion is a hypothesis until confirmed at
runtime.** Optimized C++ decompiles ambiguously and an LLM will confidently
mislabel `GetPlayerHealthRegenModifier` as `GetPlayerHealth`. Found an
accessor? Read the value live and watch it change. Found the record loader?
Breakpoint it and check the string it receives. Confirm before anything
depends on it.

Be surgical about decompiling — output is verbose and broad exploratory passes
burn context fast.

Start from `symbols/symbols.json`, not from the disassembler: most questions
about where something lives are now answered by a name search in a few
seconds. Reach for Ghidra only for the non-exported internals.

Tasks this is well suited to:
- `.dbr` path strings → xref → the record loader function
- The UI command dispatcher (likely a large switch; decompiles well)
- The UI `*Ctrl` classes in Game.dll → the widget tree → the escape menu
  (the export table already names them; Ghidra is for their internals)
- **Signature generation**: emit byte patterns with relative operands correctly
  wildcarded. Mechanical, error-prone by hand, and signatures are the entire
  patch-resilience strategy.

## Build order

Each step proves the previous one. Don't skip ahead to the UI.

0. **Symbol index** — harvest and demangle the export tables. ✅ done
1. **Proxy DLL** that logs one line proving it loaded, exports correctly
   forwarded. ✅ confirmed in game
2. **Frame hook** — proves code runs safely on the game thread. ✅ confirmed:
   10,800 ticks across live gameplay, `GameEngine*` captured from RCX
3. **Read health** — ✅ confirmed: life and mana tracked through damage and
   regeneration. Not a "memory value" as originally assumed — it is a call to
   the game's own `const` accessor
4. **UI-open flag** — ✅ confirmed. Half of it is exported (`IsTransferOpen`
   for stash and vendor); panel visibility is genuinely not, so it is a byte
   inside the `GetUI()` object found by in-process diffing. **The project's
   only raw offset**, pinned to a build stamp and failing closed on any other.
   See `symbols/ANCHORS.md`
5. **HTTP + websocket** — ✅ confirmed. Also the first step that *acts*:
   auto-potions run through the gate on the frame hook. Global hotkey
   (Ctrl+Shift+G) opens the UI, standing in for step 9
6. `.arz` parser and skill index
7. mlua and the priority list evaluator
8. Web editor
9. Escape menu button

Steps 1–5 are the risky infrastructure and end with something visibly working:
a browser tab showing your health updating live from inside the game. **That
point has been reached** — the architecture is proven end to end, including a
real action taken through the safety gate.

Steps 0–4 turned out far cheaper than this list implies, because the export
table removed nearly all the offset hunting. Step 4 was the first to need any:
no disassembler, but one byte that had to be found by diffing memory rather
than read from a name.

**On tooling.** Nothing so far has needed Ghidra or Cheat Engine. Three
techniques did the work, in increasing cost: search the export table; call
exported `const` getters live from the frame hook and watch what changes
(`probe.rs`); diff the interior of objects we can already name (`scan.rs`).
Prefer them in that order. A memory scanner yields an absolute address that
dies at the next patch, whereas scanning inside a named object yields an
offset from it. Reach for Ghidra when the question is what code *does* rather
than what a value *is* — realistically the `.dbr` record loader and the
escape-menu command dispatcher.
