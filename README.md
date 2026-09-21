# grimlua

**A skill rotation engine that lives inside Grim Dawn.**

One DLL. It reads the game's own state, evaluates a priority list you build in
a browser, and presses the game's own buttons — on the game's own thread, with
a single veto point that everything has to pass through.

---

## What it actually is

Grim Dawn has no combat automation and no scripting interface. The usual
workaround is a keyboard macro: something outside the process watching pixels
or timers and hammering keys. Macros are blind. They cannot tell whether you
are in a menu, whether a skill is off cooldown, or whether you are even alive,
so they fire into loading screens and vendor windows and hope.

grimlua is the same idea done from the inside. Because it runs in the game's
address space it can *ask* — what is my health, is a panel open, can this skill
be cast right now — and because it calls the game's own functions rather than
faking input, the game applies all its own rules about cooldowns, costs and
charges. It cannot make your character do anything you could not do yourself.

You write rules like **"when life is below 35%, drink a health potion"** or
**"when in combat and Blood of Dreeg is ready, cast it"**, in a browser, while
the game runs. They take effect on the next frame.

## The three ideas it rests on

**The game will tell you everything, if you ask by name.** Grim Dawn ships
modding-enabled builds that export around 31,600 mangled C++ symbols — the
modding tools link against the same modules. Demangling those recovers class
names, method names and parameter types. So reading your health is a call to
the game's own `Character::GetCurrentLife()`, resolved by name at runtime, not
a memory offset that dies at the next patch. In the whole project there is
exactly **one** raw offset, it is pinned to a build stamp, and it refuses to
answer on any other build.

**One veto point, in the host, that scripts cannot reach.** Every action
funnels through a single gate: disarmed, no character, a UI panel open, the
game unfocused, or inside the global cooldown, and nothing happens. Anything
the gate cannot *establish* also blocks — a state it is unsure of is a refusal,
not a shrug. A rule may only name an action; the host decides whether it
happens. That is the entire difference between grimlua and a macro, and it is
why the gate lives in Rust rather than in the scripting layer.

**Hardcore characters exist.** A crash deletes someone's character
permanently, and that one fact settles a lot of arguments: the tool starts
disarmed every launch, a panic stops grimlua rather than the game, a runaway
script is killed rather than allowed to hang a frame, and a number that cannot
be read honestly is shown blank rather than guessed at.

## How a rule becomes an action

```
  browser  ──► rules (typed IR) ──► generated Lua ──► sandboxed VM
                                                          │
  frame hook: read vitals, combat state, skill cooldowns   │
              └──► gate ──► if and only if clear ──────────┴──► one action
```

The priority list is ordered and **first match wins** — the model is
SimulationCraft's action priority list, not a flowchart. Exactly one action
happens per evaluation, about twenty times a second, because the gate
serialises everything; a rule that tried to do two things would silently drop
one.

Rules compile to Lua rather than being interpreted directly, which means the
form builder and a hand-written script are the same thing downstream. You can
switch to the Lua and take ownership of it whenever you like. The generated
source is shown with the firing line lit up live, so the rule you built and the
code that runs are never two separate mysteries.

The sandbox is built by subtraction: `math`, `string` and `table`, then every
escape hatch removed by name. A script is handed a table of plain numbers and
returns an action name. **Nothing in its environment can reach the game** — not
even indirectly — so a shared script is data rather than a liability. It also
runs under an instruction budget, because this is the render thread and
`while true do end` would be a hang *in the game*.

## Getting it running

Needs the `x86_64-pc-windows-msvc` Rust toolchain. 32-bit Grim Dawn is out of
scope.

```
cargo build --release --target x86_64-pc-windows-msvc
python tools/install.py              # copies one file into <game>\x64\
python tools/install.py --uninstall  # deletes it again
```

Install is a single file copy and uninstall is a single delete. Nothing the
game ships is renamed or modified, so Steam cloud saves, the overlay and
achievements all keep working — grimlua rides in as `dinput8.dll`, which the
game's own `DirectInput.dll` imports and which the game does not ship itself.

Launch the game and press **Ctrl+Shift+G**, or open `http://127.0.0.1:7890`.
The server binds loopback only and there is no setting to widen it.

It starts **disarmed every single run**. Arming is a deliberate act; a tool
that armed itself would be acting before its owner was watching.

Your rules are saved beside the DLL in `grimlua.config.json` and come back next
launch. `armed` never does.

## Working on it without launching the game

```
cargo run --example offline              # a fake Grim Dawn behind the real UI
cargo run --release --example runaway    # the sandbox containment check
python tools/check_symbols.py            # every symbol resolves, in its module
```

`offline` serves the genuine UI, config path, code generator, skill database
and Lua sandbox against invented vitals. It is how the web editor gets built,
and it proves nothing whatsoever about the frame hook or any game call.

`runaway` throws endless loops, runaway recursion and memory bombs at the
sandbox and checks the process survives. It cannot be a unit test: Cargo
ignores the `panic` profile setting for tests, and that setting is exactly what
this exercises.

`check_symbols.py` matters more than it looks. A symbol resolve block is
all-or-nothing, so one name that fails to resolve blanks every feature in the
block with no error at all — and checking that a name exists *somewhere* is not
enough. `GAME::Name`'s constructor is exported from `Engine.dll`, the code
looked for it in `Game.dll`, and the skill list, the combat flags and the DPS
readout all went dark together.

## Where things stand

The architecture is proven end to end in a live game: the DLL loads, hooks the
frame, reads state, passes the gate and takes real actions. Potions work.

The skill database works. Both of Grim Dawn's container formats are parsed from
scratch — 1,403 skills across the base game and three expansions in under a
second, cached until the game patches or a mod is enabled. The editor knows
which skills your character actually has and which can be cast at all, so a
rule naming a passive says so at the point you write it.

**Casting a skill does not work yet.** Tracing the game's own calls showed that
the only exported path a player cast takes is the hot bar, and everything past
it is inlined. Doing that without making rules depend on your bar layout is the
next real piece of work, and it is the first question the export table cannot
answer. `symbols/ANCHORS.md` has the trace and the options.

Also outstanding: offense, defense, armour and the nine resistances on the
dashboard — reachable in principle, not yet honestly readable, and so shown
blank with the reason rather than filled with plausible numbers. Skill icons.
And the node-canvas editor, which will be a second front end onto the same IR
the form builder already produces.

## The map

```
tools/            symbol harvesting, format probes, install — stdlib Python only
symbols/          the generated index; ANCHORS.md is the part worth reading
crates/
  grimlua-core/   everything, independent of how the DLL got loaded
    hook.rs         the frame hook — the only thread that touches the game
    gate.rs         the safety gate; one veto point for every action
    runtime.rs      the tick: read, gate, at most one action
    state.rs        game accessors, resolved by exported name
    live.rs         skills, combat state and casting
    db/             .arz and .arc parsers, and the skill index
    rules.rs        the rule IR and its Lua code generator
    script.rs       the Lua sandbox; the VM never sees a game pointer
    shared.rs       snapshot out / config in — the hook-server boundary
    server.rs       HTTP + WebSocket, loopback only
    trace.rs        detour the game's own functions and watch it work
    abi.rs          the only two C++ layouts we read, and both are Microsoft's
    web/index.html  the UI, embedded at compile time
  grimlua-dinput8/  the proxy shim that gets us into the process
```

[CLAUDE.md](CLAUDE.md) carries the design and, more usefully, the reasoning —
including what was tried, rejected, and why. `symbols/ANCHORS.md` is the short
list of symbols the runtime actually depends on, and how each one was
confirmed.
