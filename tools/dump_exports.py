"""Harvest Grim Dawn's exported C++ symbols into a committed index.

Grim Dawn ships modding-enabled builds: Game.dll, Engine.dll and Widget.dll
export ~31,600 mangled C++ symbols between them, which recovers class names,
method names, parameter types and const-ness without any disassembly. This is
the project's substitute for the offset dumper that CLAUDE.md assumed was
impossible.

Writes symbols/symbols.json (the durable artifact) and symbols/index.md.

    python tools/dump_exports.py [--game-dir PATH] [--out-dir PATH]
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

import demangle  # noqa: E402
import pe  # noqa: E402

MODULES = ("Game.dll", "Engine.dll", "Widget.dll")

# The x64 build is the target: the installed game reports x64 + direct3d11, and
# symbol addresses differ per build, so mixing the two would be meaningless.
SUBDIR = "x64"

STEAM_LIBRARIES = (
    r"C:\Program Files (x86)\Steam\steamapps\common",
    r"C:\SteamLibrary\steamapps\common",
    r"D:\SteamLibrary\steamapps\common",
    r"E:\SteamLibrary\steamapps\common",
)


def find_game_dir() -> Path | None:
    for lib in STEAM_LIBRARIES:
        candidate = Path(lib) / "Grim Dawn"
        if (candidate / SUBDIR / "Game.dll").is_file():
            return candidate
    return None


def harvest(game_dir: Path) -> dict:
    modules, symbols = {}, []

    for module in MODULES:
        path = game_dir / SUBDIR / module
        if not path.is_file():
            raise SystemExit(f"missing {path} -- is --game-dir correct?")

        binary = pe.load(path)
        exports = binary.exports()
        modules[module] = {
            "machine": binary.machine,
            "timestamp": binary.timestamp,
            "timestamp_utc": dt.datetime.fromtimestamp(
                binary.timestamp, dt.timezone.utc
            ).isoformat(),
            "image_base": binary.image_base,
            "sha256": binary.sha256,
            "export_count": len(exports),
            "imports": binary.imports(),
        }

        for export in exports:
            sym = demangle.parse(export.name)
            record = sym.as_dict()
            record.update(
                module=module,
                ordinal=export.ordinal,
                rva=export.rva,
                forwarder=export.forwarder,
            )
            symbols.append(record)

        print(f"  {module:12} {len(exports):6,} exports  "
              f"(built {modules[module]['timestamp_utc'][:10]})")

    return {
        "generated_utc": dt.datetime.now(dt.timezone.utc).isoformat(timespec="seconds"),
        "game_dir": str(game_dir),
        "subdir": SUBDIR,
        "modules": modules,
        "symbols": symbols,
    }


def write_json(data: dict, out: Path) -> None:
    """Valid JSON, but with one symbol per line.

    Pretty-printing 31,600 records costs ~20 MB of indentation, while fully
    compact output turns every game patch into a single unreadable diff line.
    One record per line gives a diff that names exactly which symbols Crate
    added, removed or re-signed.
    """
    header = {k: v for k, v in data.items() if k != "symbols"}
    with out.open("w", encoding="utf-8", newline="\n") as f:
        f.write(json.dumps(header, indent=1)[:-2])  # drop the closing "\n}"
        f.write(',\n "symbols": [\n')
        last = len(data["symbols"]) - 1
        for i, s in enumerate(data["symbols"]):
            f.write("  " + json.dumps(s, separators=(",", ":")))
            f.write(",\n" if i != last else "\n")
        f.write(" ]\n}\n")


def write_index(data: dict, out: Path) -> None:
    """A browsable companion to the JSON: classes by size, then their members."""
    by_class: dict[tuple[str, str], list[dict]] = defaultdict(list)
    for s in data["symbols"]:
        key = (s["module"], s["cls"] or "<free functions>")
        by_class[key].append(s)

    lines = [
        "# Grim Dawn exported symbol index",
        "",
        f"Generated {data['generated_utc']} from `{data['game_dir']}\\{data['subdir']}`.",
        "",
        "Do not edit by hand -- regenerate with `python tools/dump_exports.py`.",
        "Search with `python tools/query_symbols.py`.",
        "",
        "## Modules",
        "",
        "| Module | Machine | Exports | Built (UTC) | SHA-256 |",
        "|---|---|---:|---|---|",
    ]
    for name, m in data["modules"].items():
        lines.append(
            f"| `{name}` | {m['machine']} | {m['export_count']:,} | "
            f"{m['timestamp_utc'][:10]} | `{m['sha256'][:16]}…` |"
        )

    lines += ["", "## Largest classes", "", "| Class | Module | Members |", "|---|---|---:|"]
    for (module, cls), members in sorted(
        by_class.items(), key=lambda kv: -len(kv[1])
    )[:60]:
        lines.append(f"| `{cls}` | {module} | {len(members)} |")

    lines += ["", "## All classes", ""]
    for (module, cls), members in sorted(by_class.items(), key=lambda kv: (kv[1][0]["cls"] or "~", kv[0][0])):
        lines.append(f"### `{cls}` ({module}, {len(members)})")
        lines.append("")
        for s in sorted(members, key=lambda s: s["method"]):
            mark = "".join(
                (
                    "S" if s["is_static"] else "",
                    "V" if s["is_virtual"] else "",
                    "C" if s["is_const"] else "",
                )
            )
            suffix = f" `{mark}`" if mark else ""
            lines.append(f"- `{s['method']}`{suffix}")
        lines.append("")

    out.write_text("\n".join(lines), encoding="utf-8")


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--game-dir", type=Path, default=None)
    ap.add_argument("--out-dir", type=Path, default=Path(__file__).parent.parent / "symbols")
    args = ap.parse_args()

    game_dir = args.game_dir or find_game_dir()
    if game_dir is None:
        raise SystemExit("Grim Dawn not found; pass --game-dir")

    print(f"Reading {game_dir}\\{SUBDIR}")
    data = harvest(game_dir)

    args.out_dir.mkdir(parents=True, exist_ok=True)
    json_path = args.out_dir / "symbols.json"
    write_json(data, json_path)
    write_index(data, args.out_dir / "index.md")

    total = len(data["symbols"])
    functions = sum(1 for s in data["symbols"] if s["is_function"])
    classes = len({(s["module"], s["cls"]) for s in data["symbols"] if s["cls"]})
    print(f"\n{total:,} symbols  ({functions:,} functions, {total - functions:,} data)")
    print(f"{classes:,} classes")
    print(f"-> {json_path}  ({json_path.stat().st_size / 1e6:.1f} MB)")
    print(f"-> {args.out_dir / 'index.md'}")


if __name__ == "__main__":
    main()
