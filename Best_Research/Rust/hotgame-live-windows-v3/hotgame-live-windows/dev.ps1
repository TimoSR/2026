Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$Root = $PSScriptRoot
Push-Location $Root
try {
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        throw "Rust/Cargo was not found on PATH. Install Rust from https://rustup.rs, restart PowerShell, then run this script again."
    }

    New-Item -ItemType Directory -Force (Join-Path $Root ".hot") | Out-Null

    Write-Host "Starting Hotgame Live..." -ForegroundColor Cyan
    Write-Host "Edit this while running:" -ForegroundColor Gray
    Write-Host "  crates\game_plugin\src\lib.rs" -ForegroundColor Gray
    Write-Host "" 
    Write-Host "Controls:" -ForegroundColor Gray
    Write-Host "  Space      Pause/resume" -ForegroundColor Gray
    Write-Host "  Right      Step one frame while paused" -ForegroundColor Gray
    Write-Host "  Left       Scrub backward captured frames" -ForegroundColor Gray
    Write-Host "  Enter      Jump to latest frame" -ForegroundColor Gray
    Write-Host "  F5         Force rebuild/reload plugin" -ForegroundColor Gray
    Write-Host "  Esc        Exit" -ForegroundColor Gray
    Write-Host ""

    cargo run -p host
}
finally {
    Pop-Location
}
