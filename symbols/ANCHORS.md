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

## Actions (build-order step 5)

The only way grimlua affects the game. Every one is an exported game function
called on the frame hook -- **never synthetic input**. The game therefore
applies its own rules about cooldowns, charges and whether a potion is even
slotted, and grimlua cannot make it do anything the player could not.

| Symbol | Signature | Runtime confirmed |
|---|---|---|
| `?GetPlayerHotSlotCtrl@Player@GAME@@QEAAAEAVPlayerHotSlotCtrl@2@XZ` | `PlayerHotSlotCtrl& Player::GetPlayerHotSlotCtrl()` | ☑ |
| `?ActivateHealthPotionSlot@PlayerHotSlotCtrl@GAME@@QEAAXXZ` | `void ActivateHealthPotionSlot()` | ☑ |
| `?ActivateManaPotionSlot@PlayerHotSlotCtrl@GAME@@QEAAXXZ` | `void ActivateManaPotionSlot()` | ☑ |
| `?GetHealthPotionStatus@PlayerHotSlotCtrl@GAME@@QEBA?AW4HotSlotOptionStatus@2@XZ` | `HotSlotOptionStatus GetHealthPotionStatus() const` | logged only |
| `?GetManaPotionStatus@PlayerHotSlotCtrl@GAME@@QEBA?AW4HotSlotOptionStatus@2@XZ` | `HotSlotOptionStatus GetManaPotionStatus() const` | ☐ passed to scripts |

The full chain, all resolved by name:

    GameEngine::Update            hook; `this` is the GameEngine*
      -> GetMainPlayer()          -> Player*
      -> GetPlayerHotSlotCtrl()   -> PlayerHotSlotCtrl&
      -> ActivateHealthPotionSlot()

`HotSlotOptionStatus` is an enum whose values are not yet decoded, so the
status is written to the log for study rather than acted on. Pressing a slot
the game considers unavailable is a no-op, exactly as it is for the keybind,
so nothing depends on decoding it first.

Neighbours worth knowing when adding actions: `ActivatePrimarySlot`,
`ActivateSecondarySlot`, `ActivateEvadeSlot`, `DeactivateActiveSlot`, and
`PlayerInventoryCtrl::UsePotionOfType`.

### Skill-bar slots: removed, and why

An earlier version cast skills through `PlayerHotSlotCtrl::ActivateHotSlot`,
naming a bar position. That was the wrong design and it is gone.

Going through a slot re-imposes exactly the limits an external macro has: the
skill has to be bound to the bar, the binding has to be known, and a priority
list ends up naming a bar index instead of a skill. Invoking the skill directly,
independent of the bar and of the player's key bindings, is the whole advantage
of being inside the process.

The potion entry points stay, because `ActivateHealthPotionSlot` is the game's
own potion function rather than a bar index -- it was never the thing being
objected to.

### Why the per-slot `HotSlotOption` getters are not used

`HotSlotOption` exports `GetCooldownRemaining`, `GetCooldownCompletion`,
`GetSkillId`, `GetNumberAvailable` and friends, which would be far richer than
a status enum. They are all **virtual**, and an exported virtual's address is
the base-class implementation — several of them share one folded `return 0`
thunk at `Game.dll+0x167c0`. Calling them by name would therefore return
nothing useful for a `HotSlotOptionSkill`; dispatching properly means indexing
the vtable, which is an offset and thus the kind of patch-fragile dependency
the project has exactly one of already. `PlayerHotSlotCtrl::GetSlotStatus` is
non-virtual and does the dispatch internally, so it is the one used.

## Casting a skill: what the trace established

**Method.** Rather than reason about which function ought to be the cast path,
eight candidates were detoured at once and the game was asked to show us: load
in, press a skill key, read the log. Every hook logs whether it installed, so
"nothing fired" and "nothing was hooked" cannot be confused. See
`crates/grimlua-core/src/trace.rs`.

**Result, 8 of 8 hooks live, one skill press:**

    trace ActivateHotSlot: this=0x1fcfc4b1308 | 2 | 0 | 0     <- a dash
    trace ActivateHotSlot: this=0x1fcfc4b1308 | 8 | 0 | 0     <- Ring of Steel

`PlayerHotSlotCtrl::ActivateHotSlot(slot, false, false)` fired. The other seven
stayed **completely silent**:

| Hooked | Fired on a skill press |
|---|---|
| `PlayerHotSlotCtrl::ActivateHotSlot` | **yes** |
| `ControllerPlayer::InstantSkillAction` | no |
| `ControllerPlayer::SendSkillAction` | no |
| `ControllerPlayer::HandleActionFromMouse` | no |
| `ControllerPlayerState::DefaultRequestInstantSkillAction` | no |
| `ControllerPlayerStateIdle::RequestInstantSkillAction` | no |
| `Character::StartSkill` | no |
| `Character::ActivateSkill` | no |

Three conclusions:

1. **`ActivateHotSlot`'s two flags are `false, false`.** They were guessed
   correctly the first time and are now confirmed from the game's own call.
2. **`Character::ActivateSkill` and `StartSkill` are not on the player's cast
   path at all.** That is why calling them did nothing and was refused
   respectively -- they are halves of an operation the engine never reaches
   that way.
3. **Everything between the hot slot and the effect is unexported** -- inlined
   or internal. The engine's only exported player-cast entry point is the bar.

### Still wanted: casting without the hot bar

The requirement stands: a priority list should name a *skill*, and casting
should not depend on the bar. The trace says the exported surface cannot do
that today, so the options are:

- **Skill to slot, then press it.** `GetHotSlotOption(slot)` returns a
  `HotSlotOption*`; `HotSlotOption::GetType()` (non-virtual, `+0x16810`) says
  what kind of slot it is; and `HotSlotOptionSkill::GetSkillId()` is at
  **`+0x170610`**, a real function body rather than the folded `return 0` thunk
  at `+0x167c0` that the base class and a dozen other trivial getters share. So
  the mapping is readable, and checkable against what is already known: slot 8
  held Ring of Steel, whose `FindSkillId` result is 10279. Rules would still
  name skills; the limitation is that the skill must be on the bar *somewhere*.
- **Ghidra.** Decompile `ActivateHotSlot` and follow what it calls. This is the
  only route to a genuinely bar-independent cast, and it is the first thing in
  the project the export table cannot answer.

### The half that does work

| Symbol | Confirmed |
|---|---|
| `?GetSkillManager@Character@GAME@@QEAAAEAVSkillManager@2@XZ` | ☑ |
| `?FindSkillId@SkillManager@GAME@@QEBA?BIPEBD@Z` | ☑ record path -> id (Ring of Steel = 10279) |
| `?IsSkillValidForUse@SkillManager@GAME@@QEBA_NI_N@Z` | ☑ the game's own "castable now" |
| `?GetSkillList@SkillManager@GAME@@QEBAAEBV?$vector@PEAVSkill@GAME@@@mem@@XZ` | ☑ |
| `?GetDisplayNameTag@Skill@GAME@@QEBAAEBV...@XZ` | ☑ joins the live game to the `.arz` index |

## Superseded: casting a skill, the earlier guesswork

A rule names a skill by record path, and the first half of the chain is
confirmed working in a live game:

| Symbol | Signature | Confirmed |
|---|---|---|
| `?GetSkillManager@Character@GAME@@QEAAAEAVSkillManager@2@XZ` | `SkillManager& Character::GetSkillManager()` | ☑ |
| `?FindSkillId@SkillManager@GAME@@QEBA?BIPEBD@Z` | `unsigned FindSkillId(char const*) const` | ☑ resolves record paths |
| `?IsSkillValidForUse@SkillManager@GAME@@QEBA_NI_N@Z` | `bool IsSkillValidForUse(unsigned, bool) const` | ☑ |
| `?GetSkillList@SkillManager@GAME@@QEBAAEBV?$vector@PEAVSkill@GAME@@@mem@@XZ` | `const mem::vector<Skill*>&` | ☑ |
| `?GetDisplayNameTag@Skill@GAME@@QEBAAEBV...@XZ` | `const std::string&` | ☑ joins to the `.arz` index |

**The second half does not work yet.**

`?ActivateSkill@Character@GAME@@QEAAXIAEBVName@2@IAEBVWorldVec3@2@@Z` was the
first attempt. In a live game it does *nothing at all* -- no error, no effect --
which fits it being the lower half of the operation, the part that runs after
the player's controller has already accepted the input. It returns `void`, so
it cannot even report that.

The player-facing entry points are on `ControllerPlayer`:

    bool ControllerPlayer::InstantSkillAction(Character&, unsigned skillId,
                                              WorldVec3 const&, unsigned&)
    bool ControllerPlayer::SendSkillAction(Character&, bool, bool,
                                           unsigned skillId, WorldVec3 const&,
                                           unsigned&)

Both are exported. **Neither is reachable**: nothing in the export table returns
a `ControllerPlayer*`. `Character::GetControllerId()` returns an id, and the
registry that turns an id into a controller is not exported.

Currently tried instead: `?StartSkill@Character@GAME@@QEAA?B_NIIAEBVWorldVec3@2@IAEBUTargetLeadingData@2@@Z`,
the same operation one level up. It is **not known to be correct**, but it
returns a `bool`, so it distinguishes "the game rejected the call" from "the
call landed and something downstream is missing". Every attempt is logged with
the skill id, the target and the answer.

This is the first thing in the project that the export table cannot answer, and
therefore the first genuine Ghidra job: decompile `ActivateSkill` and
`InstantSkillAction` to find what the arguments mean and how the controller is
reached.

## Character sheet numbers

`?GetBaseCharAttribute@Character@GAME@@QEBAMW4CharAttributeType@2@@Z` --
`float Character::GetBaseCharAttribute(CharAttributeType) const` -- is a clean
zero-risk `const` getter, and the route to offense, defense, armour and the
resistances. `CharAttributeType`'s values are not recoverable from the export
table (there is no `EnumConverter` specialisation for it), so the indices are
found by probing 0..128 once on the first in-world tick and logging the
non-zero results.

The probe is self-checking: maximum life and maximum energy are already known
exactly from confirmed accessors, so an index reproducing each of them anchors
the mapping. If neither anchor is found, the probe says so and the values are
not to be trusted.

**What it returned, on a real level-100 character:**

    attrs: NOT anchored (life None, energy None)
    attrs: 1=290.0  2=386.0  3=66.0  4=1210.0  5=282.0  10=300.0
           14=0.9  15=1.2  16=1.2  21=65.0  23=65.0

The refusal to anchor is the useful part, and it is correct: `290 / 386 / 66`
are Physique, Cunning and Spirit, and `1210 / 282` look like base life and
energy — all **before** gear, buffs and level. The character's actual maxima
were 13,746 and 2,591, so they were never going to appear here.

So `GetBaseCharAttribute` is the *base* attribute table and is the wrong
function for a dashboard. Indices 21 and 23 both reading `65.0` are plausibly
resistances, but nothing is wired on a resemblance. What is still needed is the
**modified/total** attribute accessor; `Character::GetAllDefenseAttributes`
takes a `CombatAttributeAccumulator` whose layout is unreversed, and
`DesignerCalculate{Offensive,Defensive}Ability(float)` take an undecoded
argument. Until one of those is understood the dashboard shows `—` and says
why.

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
