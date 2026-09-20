# Confirmed anchor symbols

The handful of exports the runtime actually depends on, lifted out of the
31,645 in `symbols.json` so they are not re-derived every session.

Per CLAUDE.md's discipline, a symbol is a hypothesis until it has been called
in a live game and watched producing a sane, *changing* value. The
"runtime confirmed" column records which ones have crossed that line.

Build these were read from: Game.dll / Engine.dll / Widget.dll, x64, built
2026-08-19. RVAs are build-specific and will move; **resolve by name at
runtime via `GetProcAddress`, never by RVA.** The RVAs below are for
cross-referencing in a disassembler only.

## Entry point

| Symbol | Signature | Runtime confirmed |
|---|---|---|
| `?Update@GameEngine@GAME@@QEAAXH@Z` | `void GameEngine::Update(int)` | ☑ |

The frame hook. A non-static member function, so on x64 the `this` pointer
arrives in `RCX`: hooking this yields the `GameEngine*` with no pointer chain,
no signature scan and no offset hunting. Everything else hangs off it.

Game.dll +0x2c1a50, ordinal 24203.

## Player reachability

| Symbol | Signature | Runtime confirmed |
|---|---|---|
| `?GetMainPlayer@GameEngine@GAME@@QEBAPEAVPlayer@2@XZ` | `Player* GameEngine::GetMainPlayer() const` | ☑ |

Returns null at the main menu and during load. Every consumer must null-check
and fail closed (CLAUDE.md L49).

`Player` derives from `Character` — `Player` has 297 exported members of its
own, `Character` has 714 — so the `Character` accessors below are callable on
the `Player*` directly.

## Vitals

All are zero-argument `const` getters, which is the safest possible shape for
a first call: no arguments to marshal, and no state mutated.

| Symbol | Signature | Runtime confirmed |
|---|---|---|
| `?GetCurrentLife@Character@GAME@@QEBA?BNXZ` | `double Character::GetCurrentLife() const` | ☑ |
| `?GetLifeLimit@Character@GAME@@QEBA?BMXZ` | `float Character::GetLifeLimit() const` | ☑ |
| `?GetCurrentLifeInt@Character@GAME@@QEBAIXZ` | `unsigned Character::GetCurrentLifeInt() const` | ☐ |
| `?GetCurrentMana@Character@GAME@@QEBA?BMXZ` | `float Character::GetCurrentMana() const` | ☑ |
| `?GetManaLimit@Character@GAME@@QEBA?BMXZ` | `float Character::GetManaLimit() const` | ☑ |

Note the inconsistent widths: current life is `double`, every other vital is
`float`. Getting this wrong reads garbage, so the FFI declarations must match
exactly.

"Energy" in the UI is **Mana** in the code.

Confirmed live on 2026-09-20: `13745.6 / 13746` life and `2591` max mana on a
real character, with two damage events (-187.3 and -71.6) and mana spend and
regeneration tracked across 44 samples. The maxima stayed constant throughout,
which is what establishes that `Player*` is directly usable as a `Character*`:
the base subobject sits at offset zero with no inheritance adjustment.

### Superseded approach

The plan expected health to require `Character::GetBaseCharAttribute(enum
CharAttributeType)` plus discovering the right enum value by brute force. That
is unnecessary — the direct getters above exist. `GetBaseCharAttribute` is
still the route for the wider attribute set, and there is **no**
`EnumConverter::GetStringAsEnum<CharAttributeType>` specialisation exported
(only `ActorMountType`, `CharacterPathGenerationStyle` and
`PlayerCharacterClassType`), so that enum still has to be recovered by other
means when it is eventually needed.

## UI state (build-order step 4)

Two halves, with very different durability.

**Exported, patch-stable:**

| Symbol | Signature | Covers | Runtime confirmed |
|---|---|---|---|
| `?IsTransferOpen@GameEngine@GAME@@QEBA_NXZ` | `bool GameEngine::IsTransferOpen() const` | stash, vendor | ☑ |
| `?IsSaveEnabled@GameEngine@GAME@@QEBA_NXZ` | `bool GameEngine::IsSaveEnabled() const` | inverted; transfer panels and (probably) the escape menu | ☑ |

**A raw offset, and the project's only one:**

    ui + 0x1ef1   (also mirrored at 0x1ef4 and 0x1fb8)

where `ui` is the pointer from
`?GetUI@GameEngine@GAME@@QEBAPEAVGameUIInterface@2@XZ`. 1 while a UI panel is
open, 0 otherwise.

### Why an offset, when everything else is a name

Panel visibility is genuinely not in the export table. `GameUIInterface` is
abstract with no exported members; `PlayerInventoryCtrl` describes inventory
*contents* and nothing returns one; Engine.dll's `Window`/`Display` classes are
the OS-window layer; Widget.dll is the modding tools' toolkit. All 128
zero-argument `const` getters on `GameEngine`, `Player` and `Character` were
probed live while panels were opened and closed, and not one of them moves.

So it was found by diffing the interior of the objects we *can* name, which is
what `scan.rs` does -- deliberately scoped to those objects, so that a hit is
expressed as `named anchor + offset` rather than the absolute address a
memory scanner would give.

### Evidence

Across a scripted run of repeated open/close cycles on two different panel
groups, four bytes moved in lockstep to the same millisecond, and no other
byte in 24 KB across three anchors moved in both phases:

| State | `0x17a9` | `0x1ef1` | `0x1ef4` | `0x1fb8` |
|---|---|---|---|---|
| no panel (15 resting samples) | 0 | 0 | 0 | 0 |
| panel group A | 1 | 1 | 1 | 1 |
| panel group B | 0 | 1 | 1 | 1 |

`0x17a9` is panel-specific and therefore not used. The other three are
equivalent; `0x1ef1` is the one wired up.

Not yet exercised against: NPC dialogue, the map, and the transfer panels
(which `IsTransferOpen` already covers by name).

### How the fragility is contained

`state::ui_panel_open` returns `Option<bool>` and yields `None` -- meaning
**fail closed, take no action** -- when any of these hold:

- `Game.dll`'s PE timestamp differs from `0x6A85FBB3`, the build the offset was
  verified on. A patch therefore disables the runtime rather than letting it
  act on a stale layout.
- the UI object is unreachable
- the byte holds anything other than 0 or 1

Re-verify after every game patch by re-running the scan, then update
`UI_OPEN_OFFSET` and `UI_OPEN_VERIFIED_BUILD` together in `state.rs`.

## Largest classes, for orientation

| Class | Module | Exported members |
|---|---|---:|
| `GameEngine` | Game.dll | 809 |
| `Character` | Game.dll | 714 |
| `Skill` | Game.dll | 422 |
| `Player` | Game.dll | 297 |
| `Engine` | Engine.dll | 238 |
| `SkillManager` | Game.dll | 212 |
| `Item` | Game.dll | 181 |
| `Entity` | Engine.dll | 165 |
| `Actor` | Engine.dll | 161 |
| `SkillProfile` | Game.dll | 160 |

## Dead ends already checked

- **`Widget.dll` is the modding tools' UI toolkit, not the game HUD.** Its 19
  classes are `TreeView`, `Splitter`, `DirectoryBrowser`, `FileBrowserWindow`,
  `Toolbar`, `StatusBar`, `SpinButton` and friends — a Win32-style editor
  toolkit used by `Editor.exe` and `DBREditor.exe`. The in-game UI is in
  Game.dll as `*Ctrl` classes (e.g. `PlayerInventoryCtrl`). Build-order step 9
  should start there, not in Widget.dll.
- **No exported record/`.dbr` loader.** `LoadRecord` exists only on
  `CombatManager`. The resource manager's record lookup stays genuine Ghidra
  work.
- **No exported UI-open / pause / focus flag.** Build-order step 4 is still
  real reverse engineering; the `*Ctrl` classes in Game.dll are the lead.
- **RTTI is stripped.** Game.dll and Widget.dll retain only `type_info`
  descriptors. Exports replace RTTI entirely, so this costs nothing.
