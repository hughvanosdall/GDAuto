"""Check that the installed proxy actually forwards to the real System32 DLL.

Loads the installed dinput8.dll in this process, decodes each exported thunk's
`jmp qword ptr [rip+disp32]`, and compares the pointer it reads against the
genuine export address.

This exists because a plausible-looking change once made every thunk jump to
itself: `LoadLibraryExW("dinput8.dll", ...)` returns an already-loaded module
of that name, which is the proxy. The log still said "forwarding" and the
binding "succeeded" -- only the jump targets showed the truth. Run this after
any change to how the real module is resolved.

    python tools/verify_proxy.py
"""

from __future__ import annotations

import ctypes
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

from dump_exports import SUBDIR, find_game_dir  # noqa: E402

EXPORTS = [
    "DirectInput8Create",
    "DllCanUnloadNow",
    "DllGetClassObject",
    "DllRegisterServer",
    "DllUnregisterServer",
    "GetdfDIJoystick",
]
REAL = r"C:\Windows\System32\dinput8.dll"


def main() -> int:
    game_dir = find_game_dir()
    if game_dir is None:
        raise SystemExit("Grim Dawn not found")
    installed = game_dir / SUBDIR / "dinput8.dll"
    if not installed.is_file():
        raise SystemExit(f"{installed} not installed -- run tools/install.py")

    ours = ctypes.WinDLL(str(installed))
    real = ctypes.WinDLL(REAL)
    if ours._handle == real._handle:
        print("FAIL: the proxy and the real DLL are the same module")
        return 1
    print(f"proxy {ours._handle:#x}   real {real._handle:#x}\n")

    failures = 0
    for name in EXPORTS:
        thunk = ctypes.cast(getattr(ours, name), ctypes.c_void_p).value
        want = ctypes.cast(getattr(real, name), ctypes.c_void_p).value
        code = bytes((ctypes.c_ubyte * 6).from_address(thunk))

        if code[:2] != b"\xff\x25":
            print(f"FAIL {name:22} not a jmp [rip+disp32]: {code.hex(' ')}")
            failures += 1
            continue

        disp = int.from_bytes(code[2:6], "little", signed=True)
        slot = thunk + 6 + disp           # RIP is the next instruction
        got = ctypes.c_uint64.from_address(slot).value

        if got == thunk:
            print(f"FAIL {name:22} jumps to ITSELF ({got:#x}) -- infinite loop")
            failures += 1
        elif got != want:
            print(f"FAIL {name:22} -> {got:#x}, expected {want:#x}")
            failures += 1
        else:
            print(f"ok   {name:22} -> {got:#x}")

    print()
    if failures:
        print(f"{failures}/{len(EXPORTS)} thunks WRONG -- do not launch the game")
        return 1
    print(f"all {len(EXPORTS)} thunks forward correctly")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
