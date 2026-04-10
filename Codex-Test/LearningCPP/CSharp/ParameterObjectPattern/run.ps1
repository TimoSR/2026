param(
    [switch]$Clean
)

$ErrorActionPreference = "Stop"

$source = Join-Path $PSScriptRoot "main.cpp"
$output = Join-Path $PSScriptRoot "parameter_object_demo.exe"

if ($Clean -and (Test-Path $output)) {
    Remove-Item -LiteralPath $output
}

function Get-Compiler {
    $clangOnPath = Get-Command clang++ -ErrorAction SilentlyContinue
    if ($clangOnPath) {
        return @{ kind = "gcc-like"; path = $clangOnPath.Source }
    }

    $bundledClang = "C:\Program Files\LLVM\bin\clang++.exe"
    if (Test-Path $bundledClang) {
        return @{ kind = "gcc-like"; path = $bundledClang }
    }

    $gpp = Get-Command g++ -ErrorAction SilentlyContinue
    if ($gpp) {
        return @{ kind = "gcc-like"; path = $gpp.Source }
    }

    $cl = Get-Command cl -ErrorAction SilentlyContinue
    if ($cl) {
        return @{ kind = "msvc"; path = $cl.Source }
    }

    return $null
}

$compiler = Get-Compiler
if (-not $compiler) {
    throw "No C++ compiler found. Install LLVM/Clang, GCC, or Visual Studio Build Tools."
}

Write-Host "Compiling with $($compiler.path)..."

if ($compiler.kind -eq "gcc-like") {
    & $compiler.path -std=c++20 -Wall -Wextra -pedantic $source -o $output
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
} else {
    & $compiler.path /nologo /std:c++20 /EHsc /W4 /Fe:$output $source
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}

Write-Host "Running $output..."
& $output
