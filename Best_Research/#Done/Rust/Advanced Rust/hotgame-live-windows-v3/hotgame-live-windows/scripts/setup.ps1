Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$Root = Resolve-Path (Join-Path $PSScriptRoot "..")
Push-Location $Root
try {
    Write-Host "Hotgame Live setup" -ForegroundColor Cyan
    Write-Host "Project: $Root"

    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        throw "Rust/Cargo was not found on PATH. Install Rust from https://rustup.rs, restart PowerShell, then run this script again."
    }

    New-Item -ItemType Directory -Force (Join-Path $Root ".hot") | Out-Null

    Write-Host "Fetching dependencies..." -ForegroundColor Cyan
    cargo fetch

    Write-Host "Building host and initial plugin..." -ForegroundColor Cyan
    cargo build -p host
    cargo build -p game_plugin

    Write-Host "Setup complete." -ForegroundColor Green
    Write-Host "Run with: .\scripts\run.ps1" -ForegroundColor Green
}
finally {
    Pop-Location
}
