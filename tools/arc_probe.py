"""Prototype .arc reader, the companion to arz_probe.py.

    python tools/arc_probe.py                       # list Text_EN.arc
    python tools/arc_probe.py --extract skills      # pull matching entries
    python tools/arc_probe.py --arc "<path>" --list

`.arc` is Grim Dawn's container for loose resources: the `text_en` tag files
that turn `tagSkillB011Name` into "Blood of Dreeg", and the `.tex` skill icons.
Same LZ4 blocks as `.arz`, different index. Stdlib only.
"""

from __future__ import annotations

import argparse
import struct
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from arz_probe import lz4_decompress  # noqa: E402
from dump_exports import find_game_dir  # noqa: E402

ARC_MAGIC = 0x435241  # "ARC\0" little-endian


class Arc:
    def __init__(self, path: Path):
        self.path = path
        data = path.read_bytes()
        self.data = data

        magic, version, entry_count, record_count, record_size, string_size, table_off = \
            struct.unpack_from("<IIIIIII", data, 0)
        if magic != ARC_MAGIC:
            raise ValueError(f"{path.name}: not an .arc (magic {magic:#x})")
        self.version = version

        # Layout after the payload, confirmed against Text_EN.arc by the
        # section sizes adding up to exactly the file length:
        #   table_off -- part table   (record_size bytes)
        #             -- string table (string_size bytes), NUL-separated
        #             -- file entries (entry_count * 44), indexing both
        # The string table sits BETWEEN the two tables, not after them.
        parts = []

        pos = table_off
        for _ in range(record_count):
            off, csize, dsize = struct.unpack_from("<III", data, pos)
            parts.append((off, csize, dsize))
            pos += 12

        strings_off = table_off + record_size
        entries_off = strings_off + string_size
        end = entries_off + entry_count * 44
        if end != len(data):
            print(f"  !! sections end at {end:,} but the file is {len(data):,}")

        self.files = []
        pos = entries_off
        for _ in range(entry_count):
            (etype, offset, csize, dsize, _hash, _ftime,
             nparts, first_part, name_len, name_off) = struct.unpack_from("<IIIIIQIIII", data, pos)
            pos += 44
            name = data[strings_off + name_off: strings_off + name_off + name_len]
            name = name.split(bytes([0]))[0]
            self.files.append({
                "name": name.decode("utf-8", "replace"),
                "type": etype,
                "offset": offset,
                "csize": csize,
                "dsize": dsize,
                "parts": nparts,
                "first_part": first_part,
            })
        self.parts = parts

    def read(self, entry: dict) -> bytes:
        """Entries are stored as one or more independently compressed parts."""
        if entry["type"] == 1:  # stored whole, uncompressed
            return self.data[entry["offset"]: entry["offset"] + entry["dsize"]]
        out = bytearray()
        for i in range(entry["first_part"], entry["first_part"] + entry["parts"]):
            off, csize, dsize = self.parts[i]
            blob = self.data[off: off + csize]
            out += blob if csize == dsize else lz4_decompress(blob, dsize)
        return bytes(out)


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--arc", type=Path, default=None)
    ap.add_argument("--list", action="store_true")
    ap.add_argument("--extract", help="substring of an entry name to decode")
    ap.add_argument("--tag", help="look this tag up and print its text")
    args = ap.parse_args()

    path = args.arc
    if path is None:
        game = find_game_dir()
        if game is None:
            raise SystemExit("Grim Dawn not found; pass --arc")
        path = game / "resources" / "Text_EN.arc"

    arc = Arc(path)
    print(f"{path.name}: version={arc.version} entries={len(arc.files):,} parts={len(arc.parts):,}")

    if args.list or not (args.extract or args.tag):
        for f in arc.files[:40]:
            print(f"  {f['dsize']:>9,}  {f['name']}")
        if len(arc.files) > 40:
            print(f"  ... {len(arc.files) - 40} more")

    if args.extract:
        for f in arc.files:
            if args.extract.lower() in f["name"].lower():
                body = arc.read(f)
                print(f"\n--- {f['name']}  ({len(body):,} bytes)")
                text = body.decode("utf-8-sig", "replace")
                for line in text.splitlines()[:12]:
                    print(f"    {line[:100]}")

    if args.tag:
        # Tag files are `tagName=Some Text` lines, UTF-8 with a BOM.
        found = 0
        for f in arc.files:
            body = arc.read(f).decode("utf-8-sig", "replace")
            for line in body.splitlines():
                if line.startswith(args.tag + "="):
                    print(f"  {f['name']}: {line}")
                    found += 1
        if not found:
            print(f"  {args.tag} not found")


if __name__ == "__main__":
    main()
