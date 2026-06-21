#!/usr/bin/env python3
"""Install Rustup and this workspace's recommended VS Code extensions."""

import json
from pathlib import Path
import platform
import shutil
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[1]


def run(command):
    print("+", command if isinstance(command, str) else " ".join(command))
    subprocess.run(command, check=True, shell=isinstance(command, str))


if not shutil.which("rustup"):
    if platform.system() == "Windows":
        run(["winget", "install", "--id", "Rustlang.Rustup", "--exact"])
    else:
        run("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y")

code = shutil.which("code")
if not code:
    sys.exit("VS Code's 'code' command was not found. Install VS Code and add 'code' to PATH, then run this again.")

extensions_file = ROOT / ".vscode" / "extensions.json"
extensions = json.loads(extensions_file.read_text(encoding="utf-8"))["recommendations"]

for extension in extensions:
    run([code, "--install-extension", extension])

print("\nSetup complete.")
