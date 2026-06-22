#!/usr/bin/env python3
"""Verify xmake and install this workspace's recommended VS Code extensions."""

import json
from pathlib import Path
import shutil
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[1]


def run(command: list[str]) -> None:
    print("+", " ".join(command))
    subprocess.run(command, check=True)


def require_command(name: str, installation_hint: str) -> str:
    command = shutil.which(name)
    if not command:
        sys.exit(f"{name} was not found. {installation_hint}")
    return command


require_command("xmake", "Install xmake and ensure it is on PATH, then run this script again.")
code = require_command("code", "Install VS Code and add its 'code' command to PATH, then run this script again.")

extensions_file = ROOT / ".vscode" / "extensions.json"
extensions = json.loads(extensions_file.read_text(encoding="utf-8"))["recommendations"]

for extension in extensions:
    run([code, "--install-extension", extension])

print("\nSetup complete.")
