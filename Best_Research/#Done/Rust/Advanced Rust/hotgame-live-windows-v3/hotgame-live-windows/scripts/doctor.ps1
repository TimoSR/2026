param()

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$Root = Resolve-Path (Join-Path $PSScriptRoot "..")
Push-Location $Root
try {
    Write-Host "Hotgame Live doctor" -ForegroundColor Cyan
    Write-Host "Project: $Root" -ForegroundColor Gray

    $Cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if (-not $Cargo) {
        Write-Host "Cargo: missing" -ForegroundColor Red
        throw "Install Rust from https://rustup.rs, restart PowerShell, then run .\scripts\doctor.ps1 again."
    }

    Write-Host "Cargo: $($Cargo.Source)" -ForegroundColor Green
    cargo --version
    rustc --version

    if (-not (Test-Path "crates\game_plugin\src\lib.rs")) {
        throw "Missing crates\game_plugin\src\lib.rs. Run this script from the project root or re-extract the archive."
    }

    New-Item -ItemType Directory -Force ".hot" | Out-Null
    Write-Host ".hot: ready" -ForegroundColor Green

    Write-Host "Checking workspace..." -ForegroundColor Cyan
    cargo check --workspace

    Write-Host "Doctor passed. Run .\dev.ps1 or .\scripts\run.ps1" -ForegroundColor Green
}
finally {
    Pop-Location
}
