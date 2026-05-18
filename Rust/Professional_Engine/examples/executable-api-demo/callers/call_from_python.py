import json
import subprocess
from pathlib import Path

workspace_root = Path(__file__).resolve().parents[3]
exe_path = workspace_root / "target" / "release" / "executable-api-demo.exe"

subprocess.run(
    ["cargo", "build", "-p", "executable-api-demo", "--release"],
    cwd=workspace_root,
    check=True,
)

result = subprocess.run(
    [str(exe_path), "benchmark", "--iterations", "250000", "--json"],
    cwd=workspace_root,
    capture_output=True,
    text=True,
    check=False,
)

print("exit_code:", result.returncode)
if result.stdout:
    payload = json.loads(result.stdout)
    print("command:", payload["command"])
    print("iterations:", payload["iterations"])
if result.stderr:
    print("stderr:", result.stderr.strip())
