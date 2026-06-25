param(
    [switch] $Release
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$Root = Resolve-Path (Join-Path $PSScriptRoot "..")
Push-Location $Root
try {
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        throw "Rust/Cargo was not found on PATH. Install Rust from https://rustup.rs, restart PowerShell, then run this script again."
    }

    New-Item -ItemType Directory -Force (Join-Path $Root ".hot") | Out-Null

    Write-Host "Starting Hotgame Live..." -ForegroundColor Cyan
    Write-Host "Edit: crates\game_plugin\src\lib.rs" -ForegroundColor Gray
    Write-Host "Controls: Space pause/resume, Right step, Left scrub, Enter latest, F5 reload, Esc exit" -ForegroundColor Gray

    if ($Release.IsPresent) {
        cargo run -p host --release
    }
    else {
        cargo run -p host
    }
}
finally {
    Pop-Location
}
