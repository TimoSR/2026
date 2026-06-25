Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$Root = Resolve-Path (Join-Path $PSScriptRoot "..")
Push-Location $Root
try {
    cargo fmt --all -- --check
    cargo check --workspace
}
finally {
    Pop-Location
}
