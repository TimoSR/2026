# What We Can Do With The `.exe` (Code-Wise)

Treat the `.exe` as a **process API**.  
Code-wise, you can:

- Define a stable CLI contract (`launcher.exe run --frames 120 --config game.toml`)
- Return meaningful exit codes (`0` success, non-zero error classes)
- Emit machine-readable output (`--json` for scripts/CI/tools)
- Accept files/stdin and produce files/stdout
- Add subcommands (`run`, `benchmark`, `validate-assets`, `headless-server`)
- Run it from other code (PowerShell/Python/C#/Rust) via `Process`/`Command`
- Turn it into a long-running service mode (HTTP/TCP) for external callers
- Write integration tests that spawn the `.exe` and assert output/exit status

## Important Limit

- You **cannot import/call functions from an `.exe`** directly.
- For in-process calls, keep logic in `lib` crates (like `game-client`).
- If you need binary-level callable APIs, compile a `cdylib`/`staticlib` instead.

## Practical Next Step

Add `clap` subcommands and a `--json` mode to `launcher.exe` so scripts and tools can consume it as a stable interface.

## Implemented Example Folder

A full working implementation lives in:

- `examples/executable-api-demo/`

Start here:

- `examples/executable-api-demo/README.md`
