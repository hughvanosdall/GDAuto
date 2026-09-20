"""MSVC symbol demangling via dbghelp's UnDecorateSymbolName.

dbghelp ships with every Windows install and implements MSVC mangling exactly,
which beats hand-rolling a parser for 31,600 symbols full of templates and
`std::` types. We ask it twice per symbol: once for the complete signature and
once for the bare qualified name, then split the structured fields out of those
two strings rather than parsing the mangled form ourselves.
"""

from __future__ import annotations

import ctypes
from ctypes import wintypes
from dataclasses import asdict, dataclass

UNDNAME_COMPLETE = 0x0000
UNDNAME_NAME_ONLY = 0x1000

_BUF = 8192

_dbghelp = ctypes.WinDLL("dbghelp.dll")
_dbghelp.UnDecorateSymbolName.argtypes = [
    ctypes.c_char_p,
    ctypes.c_char_p,
    wintypes.DWORD,
    wintypes.DWORD,
]
_dbghelp.UnDecorateSymbolName.restype = wintypes.DWORD

_CALLING_CONVENTIONS = ("__cdecl", "__thiscall", "__stdcall", "__fastcall", "__vectorcall")
_ACCESS = ("public:", "protected:", "private:")


def demangle(mangled: str, flags: int = UNDNAME_COMPLETE) -> str:
    """Return the demangled form, or the input unchanged if it is not mangled."""
    if not mangled.startswith("?"):
        return mangled  # plain C export
    buf = ctypes.create_string_buffer(_BUF)
    n = _dbghelp.UnDecorateSymbolName(mangled.encode("latin1"), buf, _BUF, flags)
    if n == 0:
        return mangled
    return buf.value.decode("latin1").strip()


def split_qualified(name: str) -> list[str]:
    """Split `A::B<C::D>::E` on `::` at nesting depth zero -> [A, B<C::D>, E].

    A naive split would shred template arguments, which matters here because
    the heaviest-used classes are templates.
    """
    parts, depth, start = [], 0, 0
    i = 0
    while i < len(name):
        c = name[i]
        if c in "<([":
            depth += 1
        elif c in ">)]":
            depth -= 1
        elif c == ":" and depth == 0 and name[i : i + 2] == "::":
            parts.append(name[start:i])
            i += 2
            start = i
            continue
        i += 1
    parts.append(name[start:])
    return [p for p in parts if p]


def _balanced_span(s: str, open_at: int) -> int:
    """Index just past the `)` matching the `(` at open_at, or -1."""
    depth = 0
    for i in range(open_at, len(s)):
        if s[i] == "(":
            depth += 1
        elif s[i] == ")":
            depth -= 1
            if depth == 0:
                return i + 1
    return -1


def _split_params(params: str) -> list[str]:
    """Split a parameter list on top-level commas."""
    if params.strip() in ("", "void"):
        return []
    out, depth, start = [], 0, 0
    for i, c in enumerate(params):
        if c in "<([":
            depth += 1
        elif c in ">)]":
            depth -= 1
        elif c == "," and depth == 0:
            out.append(params[start:i].strip())
            start = i + 1
    out.append(params[start:].strip())
    return [p for p in out if p]


@dataclass
class Symbol:
    mangled: str
    demangled: str
    qualified: str          # GAME::GameEngine::Update
    namespace: str          # GAME
    cls: str                # GameEngine ("" for free functions)
    method: str             # Update
    is_function: bool
    access: str = ""        # public / protected / private
    is_static: bool = False
    is_virtual: bool = False
    is_const: bool = False
    calling_convention: str = ""
    return_type: str = ""
    params: list[str] | None = None

    def as_dict(self) -> dict:
        return asdict(self)


def parse(mangled: str) -> Symbol:
    full = demangle(mangled, UNDNAME_COMPLETE)
    qualified = demangle(mangled, UNDNAME_NAME_ONLY)

    parts = split_qualified(qualified)
    namespace = parts[0] if len(parts) > 2 else ""
    cls = parts[-2] if len(parts) > 1 else ""
    method = parts[-1] if parts else qualified

    sym = Symbol(
        mangled=mangled,
        demangled=full,
        qualified=qualified,
        namespace=namespace,
        cls=cls,
        method=method,
        is_function=False,
    )

    head = full
    for a in _ACCESS:
        if head.startswith(a):
            sym.access = a.rstrip(":")
            head = head[len(a) :].strip()
            break
    for kw, attr in (("static ", "is_static"), ("virtual ", "is_virtual")):
        if head.startswith(kw):
            setattr(sym, attr, True)
            head = head[len(kw) :].strip()

    # Locate the argument list: the first `(` after the qualified name. Data
    # exports (vftables, statics) have none, and template arguments before the
    # name must not be mistaken for it.
    anchor = full.rfind(qualified)
    open_at = full.find("(", anchor + len(qualified)) if anchor >= 0 else -1
    if open_at < 0:
        return sym  # data export

    close_at = _balanced_span(full, open_at)
    if close_at < 0:
        return sym

    sym.is_function = True
    sym.params = _split_params(full[open_at + 1 : close_at - 1])
    sym.is_const = "const" in full[close_at:].split()

    for cc in _CALLING_CONVENTIONS:
        pos = head.find(cc)
        if pos >= 0:
            sym.calling_convention = cc
            sym.return_type = head[:pos].strip()
            break
    else:
        # Constructors, destructors and operators have no return type.
        sym.return_type = ""

    return sym
