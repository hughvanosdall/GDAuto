# grimlua

A single DLL that loads into Grim Dawn, parses the game's skill database, runs
a user-configured skill priority list against live game state, and serves a
local web UI for building those priority lists visually.

Status: build-order steps 0–5 done and confirmed in a live game — symbol
index, proxy DLL, frame hook, live health read, UI-open gate, local web UI
taking real actions. Step 6 (the `.arz`/`.arc` parsers and the skill index),
step 7 (mlua) and the form half of step 8 are built and exercised offline but
**not yet watched running inside the game**. See README.md for current state
and `symbols/ANCHORS.md` for the confirmed entry points.

**The web UI's design is not described here.** It lives in a Design-canvas
artifact the user maintains — "grimlua Web UI — Layout Explorations", option A,
"stone and moss". Read it with the Artifact tool before building or changing
the UI. Its notes deliberately override parts of this document; where the two
disagree, the artifact wins and this file should be corrected. The page now
implements option A: four tabs, the stone-and-moss dressing, and one-line rule
rows.

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
    ├── live layer ─────  skills the character has, combat, casting [built]
    ├── rules → Lua ────  typed IR, code generator, source map      [built]
    ├── mlua ───────────  sandboxed priority-list evaluation        [built]
    ├── .arz/.arc ──────  skill index: names, classes, timings      [built]
    │                     (own worker thread; icons still to do)
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

**Unwind, and catch at our own boundary.** A panic must never cross into Grim
Dawn's C++ frames. The original answer was `panic = "abort"`, which guarantees
that by making every panic fatal — including to the player's hardcore
character. It is now `panic = "unwind"` with `catch_unwind` wrapping everything
inside `hook::update_hook`, so a panic stops grimlua and leaves the game
running. That change was forced as well as preferred: mlua signals an error
from a Rust callback by longjmping out through `lua_error`, which under abort
trips `panic_cannot_unwind` and kills the process — so the script
instruction-budget guard, the one thing protecting the game from an endless
loop, *was itself* the thing killing the game. The obligation this creates is
that no panic may escape an `extern` boundary: the frame hook catches, the
proxy's exports are naked tail-jumps with no Rust frame, and mlua catches its
own callbacks. `examples/runaway.rs` is the regression check, and it cannot be
a unit test because Cargo ignores the `panic` setting for the test profile.

**Pin the module; never let it be unloaded.** grimlua runs threads of its own
-- server, hotkey pump, autosave, database worker -- and they execute code
inside the DLL. A `FreeLibrary` while any of them is live unmaps that code
underneath them, which is an access violation in someone's game. The deferred
init thread calls `GetModuleHandleExW` with `GET_MODULE_HANDLE_EX_FLAG_PIN`
before spawning anything, and refuses to start the workers if that fails. Grim
Dawn never unloads `dinput8.dll`, so this is belt and braces -- but it was
found by a harness that *did* unload it, and it segfaulted every time.

Pinning happens on the deferred thread, not in `DllMain`: `GetModuleHandleEx`
takes the loader lock, which the loader already holds there.

**Nothing heavy in `DllMain`.** It runs under the loader lock. No hooks, no
large stack frames (a 64 KB buffer there is a silent stack overflow waiting to
happen on a small game thread), no work beyond recording a handle and opening
a log. The frame hook is installed from a worker thread, which cannot begin
executing until the loader lock is released.

**Config hot-applies.** Edit in the browser → websocket → runtime recompiles
and swaps the active script next frame. No restart, no reload command.
Retrofitting this is painful; building for it is easy.

**Rules persist, `armed` does not.** The config is saved to
`grimlua.config.json` beside the DLL by a one-second autosave thread and
restored at startup. Nobody rebuilds a priority list every launch, so without
this the scripting layer would be unusable. `armed` is forced false both when
loading and when saving — a tool that remembers it was armed is a tool that
acts before its owner is watching.

**A config that will not load must never be overwritten.** This was learned the
hard way: a stray byte-order mark made the file unparseable, the loader fell
back to defaults, and the autosave wrote those defaults over the user's rules a
second later. A file we cannot understand is still their work. So a failed load
now sets a flag that blocks autosave entirely, renames the file to
`grimlua.config.bad.json` rather than replacing it, and clears only when a
config arrives from the browser — by which point someone has looked at the
state of it. The loader also tolerates a BOM, because hand-editing that file is
a reasonable thing to do and Notepad leaves one.

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

**Invoke skills directly; never press the hot bar.** An earlier version cast
skills through `PlayerHotSlotCtrl::ActivateHotSlot`, naming a bar position.
That was wrong and it was rejected outright: going through a slot re-imposes
exactly the limitations an external macro has — the skill has to be bound to
the bar, the binding has to be known, and a priority list ends up naming a bar
index instead of a skill. Casting directly, independent of the bar and of the
player's key bindings, **is the main advantage of being inside the process**,
and throwing it away to save a little reverse engineering is a bad trade.

So a rule names a *skill*, as a database record path. `SkillManager::
FindSkillId` turns that into a runtime id, `IsSkillValidForUse` is the game's
own answer on whether it can be cast this instant, and `Character::
ActivateSkill` casts it. Potions stay as they were: `ActivateHealthPotionSlot`
is the game's own potion entry point, not a bar index, so it was never the
thing being objected to.

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
- **Check the module, not just the name**, and run
  `python tools/check_symbols.py` before shipping. A resolve block is
  all-or-nothing, so one name that does not resolve blanks every feature in the
  block, silently and with no error. That is not hypothetical: `GAME::Name`'s
  constructor is exported from **Engine.dll**, the lookup searched Game.dll, and
  the result was the skill list, the combat flags and the DPS readout all going
  blank together in a live game. Group symbols by the feature they serve so a
  loss stays local, and name the module each one belongs to.
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

**Two C++ types are read by layout, and only two.** `abi.rs` knows how MSVC
lays out `std::string` and `mem::vector<T>`, because the game hands both back
across the export boundary — `Skill::GetDisplayNameTag` returns a string, and
`SkillManager::GetSkillList` returns a vector. This is a much weaker assumption
than a struct offset: the layouts belong to Microsoft's standard library, not
to Grim Dawn, so they do not move when Crate recompiles. Every read is
validated (size ≤ capacity, sane bounds, correct alignment, printable bytes)
and yields `None` rather than a guess. Nothing there writes.

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

1. recompile if the config revision moved
2. read vitals and hot-slot statuses through the game's own `const` accessors
3. `gate::evaluate` → a `Gate`
4. run the script, if the gate is clear or a browser is watching — it is a pure
   function of what step 2 gathered and cannot touch the game
5. if and only if `Gate::Clear`, validate the action it named and perform it
6. publish a `Snapshot` and a `ScriptStatus` for the browser

All of it inside a `catch_unwind` in `hook::update_hook`. A panic there stops
grimlua for the session and leaves the game running.

**Not everything runs at tick rate.** The character's skill list is re-read
every 2 s and DPS every 250 ms, both cached in between. Two reasons, and the
first is the important one: walking the skill list is the only code in the
project that reads a game container by assumed layout, so every call is
exposure that buys nothing when the answer changes on levelling and gear swaps
rather than per frame. The second is cost -- `CalculateDps` is the character
sheet's own calculation, and the tag-to-record join was several million string
comparisons a second before it was turned into a map built once per database
revision. If the skill list comes back unreadable five times running, the
reader backs off to every 30 s and says so in the log.

### The gate

`gate::evaluate(engine, last_action) -> Gate`. Blocks on: disarmed, no
character, UI panel open, game unfocused, global cooldown (default 750ms).
Only `Gate::Clear` permits an action, and `Gate::Unknown` — returned when the
UI flag cannot be trusted on the running build — blocks everything.

`armed` defaults to **false on every launch**. A tool that arms itself acts
before its owner is watching.

### The script evaluator, as built

The hardcoded `choose_action` stand-in is gone; `script.rs` is a Lua call with
the same contract. The four properties it had to preserve, and how:

1. **A script returns intent, it does not act.** `Action` is a closed enum the
   host owns. The VM is handed a table of plain numbers, built on the frame
   hook *before* Lua gets control, and returns an action *name* which the host
   parses and may refuse. There is **no host function in the environment that
   can reach the game** — not even indirectly — so a script is a pure function
   from a snapshot to a token. `print` is the only host function at all, and it
   reaches a ring buffer.
2. **At most one action per tick.** `choose` returns one token. A second return
   value names the rule, which is how per-rule cooldowns and the editor's live
   highlight work.
3. **The gate cannot be bypassed.** `runtime::perform` is reachable from
   exactly one place: inside the `Gate::Clear` branch.
4. **A misbehaving script must not stall the frame hook.** Lua's count hook is
   **re-armed before every call** (`lua_sethook` resets the countdown, so a
   call inherits a whole budget rather than the remains of the last one), plus
   a 4 MB VM memory ceiling.

**One refinement to point 3.** The original rule was that a script is not
consulted at all when the gate is closed. That is relaxed for one case: when a
browser is connected, the script is evaluated even while blocked, so the editor
can show which rule is currently winning while you are still writing it — which
is precisely when you do not want to be armed. Nothing about who may *act*
changes, because evaluation cannot touch the game. Without it the live feedback
would only work while armed.

The sandbox is built by subtraction: `math`, `string` and `table` only, then
`load`, `loadstring`, `dofile`, `loadfile`, `require`, `package`, `io`, `os`,
`debug`, `newproxy`, `collectgarbage`, `string.rep` and `string.dump` removed
by name. Removing `rep` from the `string` table also removes it from the shared
string metatable, so `("x"):rep(n)` goes with it.

### The two front ends

`rules.rs` holds the IR and one code generator; `shared::ScriptMode` picks
which front end owns the script.

* **Rules mode.** An ordered `Vec<Rule>`, each with conditions joined by `and`,
  one action, and a per-rule cooldown. `rules::generate` emits the Lua *and* a
  source map (rule id → line), which the UI uses to badge each rule with its
  line and to light up the line that just fired.
* **Script mode.** The user owns the Lua. Generation stops. Both are kept in
  the config, so switching back and forth loses nothing — but there is no
  decompiler and there will not be one.

`Config::sanitize` is the single trust boundary for both. The editor is a
client like any other; a hand-written websocket frame or an edited config file
reaches exactly the same code, so every limit is enforced there once.

### Adding an action

1. add a variant to `runtime::Action`, with a token and a label
2. add the matching `rules::ActionSpec` variant so the form builder can emit it
   — the two must agree on the token spelling, which `runtime`'s tests check
3. resolve the exported game function in `state.rs`'s `game_api!` block
4. call it from `runtime::perform`
5. add it to `actionChoices()` in `web/index.html`

Actions must be **exported game functions**, never synthetic input. The potion
rules call `PlayerHotSlotCtrl::ActivateHealthPotionSlot`, the same entry point
the keybind reaches, so the game applies its own rules about cooldowns and
charges and grimlua cannot make it do anything the player could not. See
`symbols/ANCHORS.md`.

**Skill-bar slots are the first action shipped unconfirmed.**
`ActivateHotSlot(unsigned int, bool, bool)` is exported like the rest, but the
two flags are a guess (`false, false`) and the game does not bounds-check the
index against the bar. So the whole feature is off until the user switches it
on and states how many slots their bar has, and `Config::allows_slot` refuses
any slot action while it is off. Confirm it live, then take this paragraph out.

### Config and hot-apply

The browser sends a whole `Config` over the websocket; `shared::put_config`
stores it and bumps `CONFIG_REVISION`; the hook reads it on the next tick.
That is what "no restart, no reload command" means in practice.

### The UI

One page, served from `src/web/index.html` and embedded with `include_str!`.
Vanilla JS, no build step, no framework. It renders stale data greyed out and
labelled rather than letting frozen numbers pass as live.

Option A, as built: four tabs — **DASHBOARD**, **RULES**, **SCRIPTS**,
**LOG** — over a shared command bar and gate banner.

* **DASHBOARD** — vitals, a four-tile row (DPS, offense, defense, armour), the
  nine resistances with a tick at the 80 cap, combat state, the potion rules,
  and the script's state.
* **RULES** — the priority list as numbered one-line rows: toggle, name, arrow,
  action, and a badge showing either the generated line number or FIRING. A bad
  rule is marked in rust with the reason under it.
* **SCRIPTS** — the compiled Lua with the firing line highlighted, or the
  hand-written editor, plus the `print` console.
* **LOG** — what grimlua did and what stopped it, colour-coded by kind.

**Numbers that are not read are shown blank, never invented.** Offense, defense
and armour need the character-attribute enum, which is not in the export table;
the resistances need `GetAllDefenseAttributes`, whose
`CombatAttributeAccumulator` argument has not been reversed. Those tiles say
`—` and the panel says why. Only DPS is wired, through `Player::CalculateDps`.
This is the same rule the original document stated for the same numbers: a
dashboard that invents values is worse than one that admits a gap.

**APPLY does not gate hot-apply.** Edits still go out on a short debounce by
themselves. The button reports whether anything is still in flight and sends it
immediately when clicked, which is what the design's APPLY means next to a
runtime that swaps the list next frame.

Two habits worth keeping when extending it:

* **Heavy fields are pushed on change, not on the clock.** The config, the
  compiled source and the console go out only when they differ from what that
  connection was last sent; only the snapshot and a few counters go at 10Hz.
* **The page recognises its own echo.** Every edit mutates a local mirror of
  the config and sends it; when the server echoes it back, a key-sorted
  stringify comparison tells the page it is looking at its own change, and the
  DOM is left alone. Without that, the rules list would rebuild under the
  caret every time someone typed a rule name.

`cargo run --example offline` serves this page against a fake game, which is
how it is developed. `examples/runaway.rs` is the sandbox containment check.

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

### As built

`db/arz.rs`, `db/arc.rs` and `db/skills.rs`. Both formats were pinned down
against the shipped files by `tools/arz_probe.py` and `tools/arc_probe.py`,
which stay in the repo as stdlib-only reference implementations to check the
Rust against. 1,403 skills across the base game and three expansions in about
650 ms, cached to `grimlua.skills.json` beside the DLL and keyed on the size
and mtime of every archive, so a patch or a newly enabled mod rebuilds it
without anyone asking.

Layout notes worth not rediscovering:

- `.arz`: 24-byte header, then payload, then the record table, then the string
  table. `record_start + record_size == string_start` exactly, which is a good
  assertion to keep — it turns a wrong guess into an error instead of a table
  of plausible nonsense.
- `.arc`: the **string table sits between** the part table and the file
  entries, not after them. Getting that order wrong yields entries with empty
  names and sizes that are ASCII read as integers.

All three gotchas were real:

1. Names aren't in records — `.dbr` holds tags like `tagClass03SkillName04A`,
   strings live in `text_en`. Handled; mods ship their own tag files and later
   archives overwrite earlier ones.
2. `.tpl` templates are the schema; each `.dbr` declares its `templateName`.
   Not needed so far: field names come through in the record itself.
3. **Records chain, and this one bites hardest.** Some castable skills are four
   fields and a pointer: `Blood of Dreeg` is a `Skill_BuffRadius` record whose
   only content is `buffSkillName`, pointing at a `SkillBuff_Passive` record
   that holds the name, the icon, the 12s cooldown and the 60s duration. Read
   naively, the game's most famous buff looks like an uncastable passive. The
   collector resolves through the link but keeps the *parent* as the castable
   identity. Many values are also arrays indexed by rank, so rank 1 is what the
   picker shows and resolving a specific rank stays a traversal.

`skills::Kind` — `Castable`, `Passive`, `Buff`, `Modifier` — is what produces
the editor's "that is a passive, it cannot be cast" message at the point of the
mistake.

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

**The form builder is built** (`rules.rs` plus the PRIORITY LIST panel), and
the settled points above survived contact: IR then Lua, one direction only, and
the source map is a `Vec<LineMark>` pushed over the existing websocket. Two
things the canvas will inherit rather than invent:

- The generator is in Rust and there is exactly one of it. The browser never
  builds Lua; when the Lua editor needs "start from my rules", the host sends
  the generated text.
- `Config::sanitize` clamps everything a client can send before it reaches the
  generator. A canvas is another untrusted client, not a privileged one.

Still open for the canvas: `Skill` as a pin type, which needs step 6.

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
6. **`.arz` parser and skill index** — ✅ built, verified against the
   installed game. Icons (`.tex` → PNG) are the one piece still outstanding
7. **mlua and the priority list evaluator** — ✅ built, exercised offline.
   Sandboxed Lua 5.4 compiled and run on the frame hook under a per-call
   instruction budget. Not yet watched running inside the game
8. **Web editor** — ✅ form half built: ordered rules, conditions, per-rule
   cooldowns, a Lua view with the firing line highlighted, a hand-written-Lua
   mode and a `print` console. The node canvas is still to come, as a second
   front end on the same IR
9. Escape menu button

### Open, as of the last session

- **Casting without the hot bar.** The only exported player-cast path is
  `ActivateHotSlot`. Unresolved, and the next thing to pick up. See "Casting a
  skill" above and `symbols/ANCHORS.md`.
- **The dashboard's numbers.** DPS is wired. Offense, defense, armour and the
  nine resistances are not: `GetBaseCharAttribute` turns out to return *base*
  values, so the modified/total accessor still has to be found. Shown blank
  with the reason rather than guessed.
- **Skill icons.** `.tex` inside `.arc`, close to DDS with a custom header.
  Cosmetic, deferred.
- **`buff_active`.** Plumbed end to end through the IR and the sandbox, but
  always reads false: `SkillManager::IsSkillBuffActive` is resolved and not yet
  called.

Steps 1–5 are the risky infrastructure and end with something visibly working:
a browser tab showing your health updating live from inside the game. **That
point has been reached** — the architecture is proven end to end, including a
real action taken through the safety gate.

Step 7 was mostly the sandbox, not the language. The evaluator itself is a
`Function::call`; the work was making a runaway script survivable, which turned
out to require changing the panic strategy (see "Unwind, and catch at our own
boundary" above). Budget for that shape of problem rather than for Lua.

Steps 0–4 turned out far cheaper than this list implies, because the export
table removed nearly all the offset hunting. Step 4 was the first to need any:
no disassembler, but one byte that had to be found by diffing memory rather
than read from a name.

**Casting a skill: the engine's only exported path is the hot bar.** A trace
of eight candidate entry points, with the player pressing a skill key, fired
exactly one: `PlayerHotSlotCtrl::ActivateHotSlot(slot, false, false)`.
`Character::ActivateSkill`, `Character::StartSkill`,
`ControllerPlayer::InstantSkillAction`, `SendSkillAction` and the controller
state machine were all silent, which is why calling them did nothing — they are
not on the player's path. Everything between the slot and the effect is inlined
or internal.

Everything *up to* the cast does work by name: a record path resolves through
`FindSkillId`, and `IsSkillValidForUse` is the game's own answer on whether it
can be cast now. Both confirmed live.

**This is unfinished, and deliberately so.** The requirement — a priority list
names a skill, and casting does not depend on the bar — has not been met, and
using the bar would not meet it. Two routes remain: resolve skill to slot at
runtime and press that slot (works, but the skill must be on the bar), or
decompile `ActivateHotSlot` and follow it to whatever actually performs the
cast. The second is the first genuine Ghidra job in this project. See
`symbols/ANCHORS.md` for the trace output and the symbols involved.

**On tooling.** Nothing so far has needed Ghidra or Cheat Engine. Four
techniques did the work, in increasing cost: search the export table; call
exported `const` getters live from the frame hook and watch what changes
(`probe.rs`); **detour the game's own functions and watch it call them**
(`trace.rs`); diff the interior of objects we can already name (`scan.rs`).
Prefer them in that order.

The third is new and earned its place immediately. Being *inside* the process
means a question like "how does the game cast a skill" does not have to be
reasoned about at all — hook every candidate, press the key, read the log. One
run ruled out seven of eight hypotheses and confirmed the arguments of the
eighth, which no amount of reading demangled signatures had managed. Two rules
make it trustworthy: **log whether each hook installed**, so silence is
unambiguous, and declare the hooks uniformly as four integer arguments rather
than reconstructing prototypes — on x64 the first four arrive in RCX, RDX, R8
and R9 whatever their types, and inventing a signature for a function that may
not even be on the path is the wrong way round. A memory scanner yields an absolute address that
dies at the next patch, whereas scanning inside a named object yields an
offset from it. Reach for Ghidra when the question is what code *does* rather
than what a value *is* — realistically the `.dbr` record loader and the
escape-menu command dispatcher.
