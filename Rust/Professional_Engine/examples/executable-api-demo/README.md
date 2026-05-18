# Executable API Demo

This folder implements concrete examples of how a compiled `.exe` can expose a process-style API.

## What is implemented

- Stable CLI contract with subcommands:
  - `run`
  - `benchmark`
  - `validate-assets`
  - `headless-server`
- Explicit non-zero exit codes for error categories
- Machine-readable output with `--json`
- File and stdin inputs (`run --config ...` and `run --config-stdin`)
- Service-style mode over TCP (`headless-server --live`)
- Integration tests that spawn the executable process
- Caller scripts for PowerShell and Python

## Build

```powershell
cargo build -p executable-api-demo
cargo build -p executable-api-demo --release
```

Artifacts:
- `target\debug\executable-api-demo.exe`
- `target\release\executable-api-demo.exe`

## Try the commands

From the workspace root (`C:\Users\timot\code\2026\Rust\Professional_Engine`):

```powershell
cargo run -p executable-api-demo -- run --frames 8
cargo run -p executable-api-demo -- run --frames 8 --json
cargo run -p executable-api-demo -- benchmark --iterations 500000 --json
cargo run -p executable-api-demo -- validate-assets --manifest examples/executable-api-demo/examples/assets.manifest
```

From inside this folder (`examples/executable-api-demo`):

```powershell
cargo run -p executable-api-demo -- run --frames 8
cargo run -p executable-api-demo -- run --frames 8 --json
cargo run -p executable-api-demo -- benchmark --iterations 500000 --json
cargo run -p executable-api-demo -- validate-assets --manifest examples/assets.manifest
```

Read config from stdin:

```powershell
'{"frames":4,"scene":"stdin-scene"}' | cargo run -p executable-api-demo -- run --config-stdin --json
```

Simulated server mode (no socket binding):

```powershell
cargo run -p executable-api-demo -- headless-server --json
```

Live TCP server mode:

```powershell
cargo run -p executable-api-demo -- headless-server --live --bind 127.0.0.1 --port 5001 --max-clients 1
```

From another terminal:

```powershell
"ping" | nc 127.0.0.1 5001
```

## Exit code mapping

- `0`: success
- `2`: CLI/input error
- `10`: runtime/serialization error
- `20`: validation error
- `30`: I/O error
- `40`: server/network error

## Caller scripts

- PowerShell: `examples/executable-api-demo/callers/call_from_powershell.ps1`
- Python: `examples/executable-api-demo/callers/call_from_python.py`

## Tests

```powershell
cargo test -p executable-api-demo
```
