"""Search the harvested Grim Dawn symbol index.

The index is 31,600 symbols, so dumping it wholesale is never useful. This
narrows it to the handful you need and prints the mangled name, which is what
`GetProcAddress` takes.

    python tools/query_symbols.py --class Character --name Attribute
    python tools/query_symbols.py --regex 'Cooldown' --module Game.dll
    python tools/query_symbols.py --class GameEngine --name ^Get --full
    python tools/query_symbols.py --classes Skill      # list matching classes
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from collections import Counter
from pathlib import Path

INDEX = Path(__file__).parent.parent / "symbols" / "symbols.json"


def load(path: Path) -> dict:
    if not path.is_file():
        raise SystemExit(f"{path} not found -- run `python tools/dump_exports.py` first")
    return json.loads(path.read_text(encoding="utf-8"))


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--index", type=Path, default=INDEX)
    ap.add_argument("--class", dest="cls", help="class name (regex, anchored loosely)")
    ap.add_argument("--name", help="method name (regex)")
    ap.add_argument("--regex", help="regex against the full demangled signature")
    ap.add_argument("--module", help="Game.dll / Engine.dll / Widget.dll")
    ap.add_argument("--returns", help="regex against the return type")
    ap.add_argument("--param", help="regex against any parameter type")
    ap.add_argument("--static", action="store_true", help="only static members")
    ap.add_argument("--virtual", action="store_true", help="only virtual members")
    ap.add_argument("--const", action="store_true", help="only const members (safe to call)")
    ap.add_argument("--data", action="store_true", help="only data exports, not functions")
    ap.add_argument("--classes", nargs="?", const="", metavar="RE",
                    help="list matching class names with member counts instead of symbols")
    ap.add_argument("--full", action="store_true", help="print full signatures")
    ap.add_argument("--json", action="store_true", help="emit matching records as JSON")
    ap.add_argument("-n", "--limit", type=int, default=100, help="max rows (0 = all)")
    args = ap.parse_args()

    data = load(args.index)
    rows = data["symbols"]

    def flt(pattern: str | None, key):
        nonlocal rows
        if pattern is None:
            return
        rx = re.compile(pattern, re.IGNORECASE)
        rows = [s for s in rows if rx.search(key(s) or "")]

    # Applied before the --classes branch so that listing classes respects
    # --module and the other narrowing flags too.
    flt(args.module, lambda s: s["module"])
    flt(args.cls, lambda s: s["cls"])
    flt(args.name, lambda s: s["method"])
    flt(args.regex, lambda s: s["demangled"])
    flt(args.returns, lambda s: s["return_type"])

    if args.classes is not None:
        rx = re.compile(args.classes, re.IGNORECASE)
        counts = Counter(
            (s["module"], s["cls"]) for s in rows if s["cls"] and rx.search(s["cls"])
        )
        for (module, cls), n in counts.most_common(args.limit or None):
            print(f"{n:6}  {cls:50} {module}")
        print(f"\n{len(counts)} classes", file=sys.stderr)
        return
    if args.param:
        rx = re.compile(args.param, re.IGNORECASE)
        rows = [s for s in rows if any(rx.search(p) for p in (s["params"] or []))]
    for flag, key in (("static", "is_static"), ("virtual", "is_virtual"), ("const", "is_const")):
        if getattr(args, flag):
            rows = [s for s in rows if s[key]]
    if args.data:
        rows = [s for s in rows if not s["is_function"]]

    total = len(rows)
    shown = rows if args.limit == 0 else rows[: args.limit]

    if args.json:
        print(json.dumps(shown, indent=1))
    else:
        for s in shown:
            if args.full:
                print(f"{s['demangled']}\n    {s['mangled']}  [{s['module']} +{s['rva']:#x}]\n")
            else:
                mark = "".join((
                    "S" if s["is_static"] else "",
                    "V" if s["is_virtual"] else "",
                    "C" if s["is_const"] else "",
                ))
                print(f"{s['mangled']}\n    {s['qualified']}{f'  [{mark}]' if mark else ''}")

    if total > len(shown):
        print(f"\n{total} matches, showing {len(shown)} (use -n 0 for all)", file=sys.stderr)
    else:
        print(f"\n{total} matches", file=sys.stderr)


if __name__ == "__main__":
    main()
