"""Copy the built proxy into Grim Dawn, or remove it again.

    python tools/install.py              # install (builds first if needed)
    python tools/install.py --uninstall  # remove, leaving the game untouched
    python tools/install.py --status

Installing is one file copy and uninstalling is one delete: no shipped file is
renamed or modified, which is the whole reason the dinput8 slot was chosen.
"""

from __future__ import annotations

import argparse
import shutil
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

from dump_exports import SUBDIR, find_game_dir  # noqa: E402

BUILT = Path("target/x86_64-pc-windows-msvc/release/dinput8.dll")
TARGET_NAME = "dinput8.dll"
LOG_NAME = "grimlua.log"


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--game-dir", type=Path, default=None)
    ap.add_argument("--uninstall", action="store_true")
    ap.add_argument("--status", action="store_true")
    args = ap.parse_args()

    game_dir = args.game_dir or find_game_dir()
    if game_dir is None:
        raise SystemExit("Grim Dawn not found; pass --game-dir")

    dest = game_dir / SUBDIR / TARGET_NAME
    log = game_dir / SUBDIR / LOG_NAME

    if args.status:
        print(f"target : {dest}")
        print(f"present: {dest.is_file()}")
        if dest.is_file():
            print(f"size   : {dest.stat().st_size:,} bytes")
        print(f"log    : {log if log.is_file() else '(none)'}")
        return

    if args.uninstall:
        removed = False
        for path in (dest, log):
            if path.is_file():
                path.unlink()
                print(f"removed {path}")
                removed = True
        if not removed:
            print("nothing installed")
        return

    src = Path(__file__).parent.parent / BUILT
    if not src.is_file():
        raise SystemExit(
            f"{src} not found -- run:\n"
            f"  cargo build --release --target x86_64-pc-windows-msvc"
        )

    # Refuse to clobber a dinput8.dll we did not put there. Grim Dawn ships
    # none, so anything already present belongs to another tool.
    if dest.is_file() and dest.stat().st_size and not _looks_like_ours(dest):
        raise SystemExit(
            f"{dest} already exists and was not built here.\n"
            f"Another mod may own this slot -- move it aside manually first."
        )

    try:
        shutil.copy2(src, dest)
    except PermissionError:
        raise SystemExit(
            f"permission denied writing {dest}\n"
            f"Close Grim Dawn, or run this shell as administrator."
        )
    print(f"installed {src.name} -> {dest}  ({dest.stat().st_size:,} bytes)")
    print(f"log will appear at {log}")


def _looks_like_ours(path: Path) -> bool:
    """Our build embeds the crate name in its debug info and rdata."""
    try:
        return b"grimlua" in path.read_bytes()
    except OSError:
        return False


if __name__ == "__main__":
    main()
