"""Check every mangled name the Rust source resolves, **in the module it names**.

    python tools/check_symbols.py

A resolve block is all-or-nothing: one name that does not resolve takes the
whole struct with it, and the symptom is a swathe of unrelated features going
silently blank rather than an error.

Checking that a name exists *somewhere* is not enough, and that is not a
hypothetical: `??0Name@GAME@@QEAA@XZ` is exported from **Engine.dll**, the code
looked for it in Game.dll, and the result was that the skill list, the combat
flags and the DPS readout all went blank at once. So this matches the module
the source actually passes to `GetProcAddress`.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).parent.parent
INDEX = ROOT / "symbols" / "symbols.json"
SRC = ROOT / "crates" / "grimlua-core" / "src"

DEFAULT_MODULE = "Game.dll"


def load_index() -> dict[str, set[str]]:
    """module -> set of exported names."""
    data = json.loads(INDEX.read_text(encoding="utf-8"))
    out: dict[str, set[str]] = {}

    def walk(node, module):
        if isinstance(node, dict):
            for key, value in node.items():
                if isinstance(key, str) and key.lower().endswith(".dll"):
                    walk(value, key)
                    continue
                walk(value, module)
            name = node.get("mangled") or node.get("name")
            if isinstance(name, str) and name.startswith("?"):
                out.setdefault(node.get("module", module) or "?", set()).add(name)
        elif isinstance(node, list):
            for value in node:
                walk(value, module)

    walk(data, None)
    return out


def used_symbols() -> list[tuple[Path, str, str]]:
    """(file, module, symbol) for every name the source resolves."""
    found: list[tuple[Path, str, str]] = []
    for path in sorted(SRC.rglob("*.rs")):
        text = path.read_text(encoding="utf-8")

        # live_api! style: = ("Engine.dll", "?Sym@...")
        for module, sym in re.findall(r'\(\s*"([\w.]+\.dll)"\s*,\s*"(\?\??[^"]+)"\s*\)', text):
            found.append((path, module, sym))

        # game_api! style and bare constants: = "?Sym@..." -> the default module
        explicit = {s for _, s in re.findall(r'\(\s*"([\w.]+\.dll)"\s*,\s*"(\?\??[^"]+)"\s*\)', text)}
        for sym in re.findall(r'=\s*"(\?\??[^"]+)"', text):
            if sym not in explicit:
                found.append((path, DEFAULT_MODULE, sym))
        for sym in re.findall(r'SYMBOL: &str = "([^"]+)"', text):
            found.append((path, DEFAULT_MODULE, sym))
    return found


def main() -> None:
    index = load_index()
    total = sum(len(v) for v in index.values())
    print(f"{total:,} exported names across {', '.join(sorted(index))}\n")

    bad = 0
    seen: set[tuple[str, str]] = set()
    current: Path | None = None
    for path, module, sym in used_symbols():
        if (module, sym) in seen:
            continue
        seen.add((module, sym))
        if path != current:
            current = path
            print(f"── {path.relative_to(SRC)}")

        names = index.get(module, set())
        if sym in names:
            print(f"   ok    {module:<11} {sym[:78]}")
            continue

        bad += 1
        elsewhere = [m for m, v in index.items() if sym in v]
        if elsewhere:
            print(f"   WRONG MODULE            {sym[:70]}")
            print(f"          looked in {module}, but it is exported from {', '.join(elsewhere)}")
        else:
            print(f"   MISSING {module:<11} {sym[:70]}")

    print()
    if bad:
        print(f"{bad} symbol(s) will not resolve -- their whole block fails, blanking every")
        print("feature in it. Fix the module or the name before shipping.")
        sys.exit(1)
    print("all symbols resolve, in the modules the source names")


if __name__ == "__main__":
    main()
