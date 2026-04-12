$ErrorActionPreference = 'Stop'
$source = Join-Path $PSScriptRoot 'main.cpp'
$exe = Join-Path $PSScriptRoot 'app_cpp.exe'
clang++ -std=c++20 -Wall -Wextra -pedantic $source -o $exe
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
& $exe
exit $LASTEXITCODE
