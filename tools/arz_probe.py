"""Prototype .arz reader, used to pin the format down before writing it in Rust.

    python tools/arz_probe.py                     # summarise the base database
    python tools/arz_probe.py --dump records/skills/playerclass01/...dbr

The shipped DLL parses `.arz` natively; this exists only so the format can be
checked against the real files cheaply, and so the Rust parser has something to
be compared against. Stdlib only, including the LZ4 block decoder.
"""

from __future__ import annotations

import argparse
import struct
import sys
from collections import Counter
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from dump_exports import find_game_dir  # noqa: E402


def lz4_decompress(src: bytes, expected: int) -> bytes:
    """LZ4 block format. No frame header, no checksums -- .arz stores raw blocks."""
    out = bytearray()
    i = 0
    n = len(src)
    while i < n:
        token = src[i]
        i += 1

        literal_len = token >> 4
        if literal_len == 15:
            while True:
                b = src[i]
                i += 1
                literal_len += b
                if b != 255:
                    break
        out += src[i:i + literal_len]
        i += literal_len
        if i >= n:
            break

        offset = src[i] | (src[i + 1] << 8)
        i += 2
        match_len = token & 0x0F
        if match_len == 15:
            while True:
                b = src[i]
                i += 1
                match_len += b
                if b != 255:
                    break
        match_len += 4

        start = len(out) - offset
        for k in range(match_len):
            out.append(out[start + k])

    if len(out) != expected:
        raise ValueError(f"lz4: got {len(out)} bytes, header said {expected}")
    return bytes(out)


FIELD_TYPES = {0: "int", 1: "float", 2: "string", 3: "bool"}


class Arz:
    def __init__(self, path: Path):
        self.path = path
        data = path.read_bytes()
        self.data = data

        (magic, version, rec_start, rec_size, rec_count,
         str_start, str_size) = struct.unpack_from("<HHIIIII", data, 0)
        self.version = version
        self.rec_count = rec_count
        print(f"{path.name}: magic={magic} version={version} records={rec_count:,}")

        if rec_start + rec_size != str_start:
            print("  !! record table does not run up to the string table")

        self.strings = self._read_strings(data, str_start, str_size)
        print(f"  string table: {len(self.strings):,} entries")

        self.records = self._read_record_table(data, rec_start, rec_count)
        print(f"  record table: {len(self.records):,} entries")

    @staticmethod
    def _read_strings(data: bytes, start: int, size: int) -> list[str]:
        out = []
        pos = start
        (count,) = struct.unpack_from("<I", data, pos)
        pos += 4
        for _ in range(count):
            (length,) = struct.unpack_from("<I", data, pos)
            pos += 4
            out.append(data[pos:pos + length].decode("utf-8", "replace"))
            pos += length
        return out

    def _read_record_table(self, data: bytes, start: int, count: int):
        out = []
        pos = start
        for _ in range(count):
            (name_idx,) = struct.unpack_from("<I", data, pos)
            pos += 4
            (type_len,) = struct.unpack_from("<I", data, pos)
            pos += 4
            rec_type = data[pos:pos + type_len].decode("utf-8", "replace")
            pos += type_len
            offset, csize, dsize = struct.unpack_from("<III", data, pos)
            pos += 12
            pos += 8  # FILETIME
            out.append((self.strings[name_idx], rec_type, offset, csize, dsize))
        return out

    def record(self, index: int) -> dict:
        name, rec_type, offset, csize, dsize = self.records[index]
        blob = self.data[24 + offset: 24 + offset + csize]
        raw = lz4_decompress(blob, dsize)

        fields: dict[str, object] = {}
        pos = 0
        while pos < len(raw):
            ftype, fcount, name_idx = struct.unpack_from("<HHI", raw, pos)
            pos += 8
            values = []
            for _ in range(fcount):
                (word,) = struct.unpack_from("<I", raw, pos)
                pos += 4
                if ftype == 1:
                    values.append(struct.unpack_from("<f", raw, pos - 4)[0])
                elif ftype == 2:
                    values.append(self.strings[word])
                elif ftype == 3:
                    values.append(bool(word))
                else:
                    values.append(word)
            key = self.strings[name_idx]
            fields[key] = values[0] if len(values) == 1 else values
        fields["__name"] = name
        fields["__type"] = rec_type
        return fields


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--arz", type=Path, default=None)
    ap.add_argument("--dump", help="substring of a record path to dump")
    ap.add_argument("--skills", action="store_true",
                    help="what the skill index will actually be built from")
    ap.add_argument("--limit", type=int, default=3)
    args = ap.parse_args()

    path = args.arz
    if path is None:
        game = find_game_dir()
        if game is None:
            raise SystemExit("Grim Dawn not found; pass --arz")
        path = game / "database" / "database.arz"

    arz = Arz(path)

    if args.skills:
        # The distinction Option A's editor needs: which record types are
        # castable and which are passives that can only ever be a condition.
        skill_types = Counter(
            r[1] for r in arz.records
            if r[0].lower().startswith("records/skills/") and r[1].startswith("Skill")
        )
        print("\n  skill record types:")
        for t, c in skill_types.most_common(20):
            print(f"    {c:>6,}  {t}")

        interesting = ("skillDisplayName", "skillBaseDescription", "skillUpBitmapName",
                       "Class", "templateName", "skillCooldownTime", "skillManaCost",
                       "skillActiveDuration", "skillTargetRadius", "skillMaxLevel",
                       "buffSkillName", "petSkillName")
        shown = 0
        for i, r in enumerate(arz.records):
            if not r[0].lower().startswith("records/skills/"):
                continue
            if not r[1].startswith("Skill"):
                continue
            rec = arz.record(i)
            if "skillDisplayName" not in rec:
                continue
            print(f"\n--- {rec['__name']}   [{rec['__type']}]")
            for k in interesting:
                if k in rec:
                    print(f"    {k:<24} {str(rec[k])[:66]}")
            shown += 1
            if shown >= args.limit:
                break
        return

    if args.dump:
        hits = [i for i, r in enumerate(arz.records) if args.dump.lower() in r[0].lower()]
        print(f"\n{len(hits):,} records matching {args.dump!r}")
        for i in hits[:args.limit]:
            rec = arz.record(i)
            print(f"\n--- {rec['__name']}   [{rec['__type']}]")
            for k, v in list(rec.items())[:40]:
                if k.startswith("__"):
                    continue
                print(f"    {k:<38} {str(v)[:70]}")
        return

    types = Counter(r[1] for r in arz.records)
    print("\n  record types, most common:")
    for t, c in types.most_common(12):
        print(f"    {c:>7,}  {t}")

    skills = [r for r in arz.records if r[0].lower().startswith("records/skills/")]
    print(f"\n  records/skills/**: {len(skills):,}")

    # Prove decompression works across a spread of records, not just one.
    print("\n  decompressing a sample...")
    ok = bad = 0
    step = max(1, len(arz.records) // 400)
    for i in range(0, len(arz.records), step):
        try:
            arz.record(i)
            ok += 1
        except Exception as e:  # noqa: BLE001
            bad += 1
            if bad <= 3:
                print(f"    FAIL {arz.records[i][0]}: {e}")
    print(f"    {ok} ok, {bad} failed")


if __name__ == "__main__":
    main()
