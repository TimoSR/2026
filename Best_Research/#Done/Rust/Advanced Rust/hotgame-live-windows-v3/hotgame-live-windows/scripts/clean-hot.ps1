Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$Root = Resolve-Path (Join-Path $PSScriptRoot "..")
$Hot = Join-Path $Root ".hot"

if (Test-Path $Hot) {
    Remove-Item -Recurse -Force $Hot
}

New-Item -ItemType Directory -Force $Hot | Out-Null
Write-Host "Cleaned $Hot" -ForegroundColor Green
