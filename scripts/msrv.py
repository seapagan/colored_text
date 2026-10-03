"""Verify the exact manifest-declared compiler floor, including tests."""

import subprocess
import sys
import tomllib
from pathlib import Path


def main() -> None:
    root = Path(__file__).resolve().parent.parent
    manifest = tomllib.loads((root / "Cargo.toml").read_text())
    version = manifest["package"]["rust-version"]
    if sys.argv[1:] == ["--print"]:
        print(version)
        return
    if sys.argv[1:]:
        raise SystemExit("usage: msrv.py [--print]")
    common = ["--all-targets", "--all-features", "--locked"]
    commands = [
        ["fmt", "--all", "--", "--check"],
        ["check", *common],
        ["clippy", *common, "--", "-D", "warnings"],
        ["test", *common],
        ["test", "--doc", "--all-features", "--locked"],
        ["doc", "--no-deps", "--all-features", "--locked"],
        ["build", *common],
        ["package", "--locked"],
    ]
    for args in commands:
        subprocess.run(["cargo", f"+{version}", *args], cwd=root, check=True)


if __name__ == "__main__":
    main()
