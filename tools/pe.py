"""Minimal PE reader for export/import tables. Stdlib only.

Grim Dawn ships modding-enabled builds, so Game.dll / Engine.dll / Widget.dll
export ~31,600 mangled C++ symbols. That export table is the project's primary
source of game structure, which is why reading it has no third-party
dependency: it must keep working on a bare machine years from now.
"""

from __future__ import annotations

import hashlib
import struct
from dataclasses import dataclass, field
from pathlib import Path

IMAGE_FILE_MACHINE = {0x014C: "x86", 0x8664: "x64", 0xAA64: "arm64"}

DIR_EXPORT = 0
DIR_IMPORT = 1


class PEError(Exception):
    pass


@dataclass(frozen=True)
class Export:
    name: str          # mangled name as it appears in the export table
    ordinal: int
    rva: int           # 0 for forwarders
    forwarder: str | None = None


@dataclass
class PEFile:
    path: Path
    machine: str
    timestamp: int
    image_base: int
    sha256: str
    sections: list[tuple[str, int, int, int, int]] = field(default_factory=list)

    _data: bytes = field(repr=False, default=b"")
    _dirs: list[tuple[int, int]] = field(repr=False, default_factory=list)

    def rva_to_offset(self, rva: int) -> int | None:
        for _name, va, vsize, raw_size, raw_ptr in self.sections:
            if va <= rva < va + max(vsize, raw_size):
                return raw_ptr + (rva - va)
        return None

    def _cstr(self, rva: int) -> str:
        off = self.rva_to_offset(rva)
        if off is None:
            return ""
        end = self._data.index(b"\0", off)
        return self._data[off:end].decode("latin1")

    def exports(self) -> list[Export]:
        """Every named export, including forwarders and ordinal-only entries."""
        dir_rva, dir_size = self._dirs[DIR_EXPORT]
        if not dir_rva:
            return []
        base_off = self.rva_to_offset(dir_rva)
        if base_off is None:
            raise PEError(f"{self.path.name}: export directory RVA unmapped")

        ordinal_base, n_funcs, n_names = struct.unpack_from("<III", self._data, base_off + 16)
        eat_rva, ent_rva, eot_rva = struct.unpack_from("<III", self._data, base_off + 28)

        eat = self.rva_to_offset(eat_rva)
        ent = self.rva_to_offset(ent_rva) if n_names else None
        eot = self.rva_to_offset(eot_rva) if n_names else None

        # Map export-address-table index -> name, so ordinal-only exports are
        # still reported rather than silently dropped.
        names_by_index: dict[int, str] = {}
        for i in range(n_names or 0):
            name_rva = struct.unpack_from("<I", self._data, ent + 4 * i)[0]
            index = struct.unpack_from("<H", self._data, eot + 2 * i)[0]
            names_by_index[index] = self._cstr(name_rva)

        out: list[Export] = []
        for index in range(n_funcs):
            rva = struct.unpack_from("<I", self._data, eat + 4 * index)[0]
            if rva == 0:
                continue  # hole in the ordinal space
            name = names_by_index.get(index, f"#{ordinal_base + index}")
            # An RVA landing inside the export directory is a forwarder string
            # ("OTHERDLL.Symbol"), not code.
            if dir_rva <= rva < dir_rva + dir_size:
                out.append(Export(name, ordinal_base + index, 0, self._cstr(rva)))
            else:
                out.append(Export(name, ordinal_base + index, rva))
        return out

    def imports(self) -> list[str]:
        """Names of DLLs in the import table (not the delay-load table)."""
        dir_rva, _ = self._dirs[DIR_IMPORT]
        if not dir_rva:
            return []
        off = self.rva_to_offset(dir_rva)
        out = []
        while True:
            _oft, _ts, _fc, name_rva, _ft = struct.unpack_from("<IIIII", self._data, off)
            if name_rva == 0:
                break
            out.append(self._cstr(name_rva))
            off += 20
        return out


def load(path: str | Path) -> PEFile:
    path = Path(path)
    data = path.read_bytes()
    if data[:2] != b"MZ":
        raise PEError(f"{path.name}: not a PE file")

    pe = struct.unpack_from("<I", data, 0x3C)[0]
    if data[pe : pe + 4] != b"PE\0\0":
        raise PEError(f"{path.name}: bad PE signature")

    machine, n_sections = struct.unpack_from("<HH", data, pe + 4)
    timestamp = struct.unpack_from("<I", data, pe + 8)[0]
    opt_size = struct.unpack_from("<H", data, pe + 20)[0]

    opt = pe + 24
    magic = struct.unpack_from("<H", data, opt)[0]
    if magic == 0x20B:      # PE32+
        image_base = struct.unpack_from("<Q", data, opt + 24)[0]
        dirs_off = opt + 112
    elif magic == 0x10B:    # PE32
        image_base = struct.unpack_from("<I", data, opt + 28)[0]
        dirs_off = opt + 96
    else:
        raise PEError(f"{path.name}: unknown optional header magic {magic:#x}")

    n_dirs = struct.unpack_from("<I", data, dirs_off - 4)[0]
    dirs = [struct.unpack_from("<II", data, dirs_off + 8 * i) for i in range(n_dirs)]

    sections = []
    sec_off = opt + opt_size
    for i in range(n_sections):
        b = sec_off + 40 * i
        name = data[b : b + 8].rstrip(b"\0").decode("latin1")
        vsize, va, raw_size, raw_ptr = struct.unpack_from("<IIII", data, b + 8)
        sections.append((name, va, vsize, raw_size, raw_ptr))

    return PEFile(
        path=path,
        machine=IMAGE_FILE_MACHINE.get(machine, f"{machine:#x}"),
        timestamp=timestamp,
        image_base=image_base,
        sha256=hashlib.sha256(data).hexdigest(),
        sections=sections,
        _data=data,
        _dirs=dirs,
    )
