param(
    [Parameter(Mandatory = $true)]
    [string]$File,
    [ValidateSet("c++17", "c++20", "c++23")]
    [string]$Std = "c++17",
    [switch]$Demo
)

$ErrorActionPreference = "Stop"
$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
& (Join-Path $scriptRoot 'install_dependencies.ps1') -Profile core -AutoInstall
$compiler = "C:\Program Files\LLVM\bin\clang++.exe"

$fullFile = Join-Path $scriptRoot $File
if (!(Test-Path $fullFile)) {
    throw "File not found: $File"
}

$buildDir = Join-Path $scriptRoot ".build\single_runs"
if (!(Test-Path $buildDir)) {
    New-Item -ItemType Directory -Path $buildDir | Out-Null
}

$safeName = ($File -replace '[\\/\.:]', '_')
$exe = Join-Path $buildDir ($safeName + ".exe")

if ($Demo) {
    & $compiler "-std=$Std" -DRUN_DEMO $fullFile -o $exe
} else {
    & $compiler "-std=$Std" $fullFile -o $exe
}

if ($LASTEXITCODE -ne 0) {
    throw "Compile failed: $File"
}

& $exe
