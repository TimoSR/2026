$ErrorActionPreference = 'Stop'
$source = Join-Path $PSScriptRoot 'main.c'
$exe = Join-Path $PSScriptRoot 'app_c.exe'
clang -std=c11 -Wall -Wextra -pedantic $source -o $exe
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
& $exe
exit $LASTEXITCODE
