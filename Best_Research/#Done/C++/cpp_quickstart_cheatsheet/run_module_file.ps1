param(
    [Parameter(Mandatory = $true)]
    [ValidateSet("demo", "test")]
    [string]$Mode
)

$ErrorActionPreference = "Stop"
$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
& (Join-Path $scriptRoot 'install_dependencies.ps1') -Profile core -AutoInstall
$compiler = "C:\Program Files\LLVM\bin\clang++.exe"
$moduleDir = Join-Path $scriptRoot "06_code_organization_approaches\04_cpp20_modules"

$modulePcm = Join-Path $moduleDir "bank_account_module.pcm"
$moduleObj = Join-Path $moduleDir "bank_account_module.obj"
$moduleArg = "-fmodule-file=bank_account_module=$modulePcm"

& $compiler -std=c++20 -x c++-module --precompile (Join-Path $moduleDir "bank_account_module.ixx") -o $modulePcm
if ($LASTEXITCODE -ne 0) { throw "Module precompile failed" }

& $compiler -std=c++20 -x c++-module -c (Join-Path $moduleDir "bank_account_module.ixx") -o $moduleObj
if ($LASTEXITCODE -ne 0) { throw "Module object compile failed" }

if ($Mode -eq "demo") {
    $usageObj = Join-Path $moduleDir "bank_account_module_usage.obj"
    $exe = Join-Path $moduleDir "bank_account_module_usage.demo.exe"
    & $compiler -std=c++20 -c (Join-Path $moduleDir "bank_account_module_usage.cpp") $moduleArg -o $usageObj
    if ($LASTEXITCODE -ne 0) { throw "Module demo compile failed" }
    & $compiler $moduleObj $usageObj -o $exe
    if ($LASTEXITCODE -ne 0) { throw "Module demo link failed" }
    & $exe
} else {
    $testObj = Join-Path $moduleDir "bank_account_module_usage.test.obj"
    $exe = Join-Path $moduleDir "bank_account_module_usage.test.exe"
    & $compiler -std=c++20 -c (Join-Path $moduleDir "bank_account_module_usage.test.cpp") $moduleArg -o $testObj
    if ($LASTEXITCODE -ne 0) { throw "Module test compile failed" }
    & $compiler $moduleObj $testObj -o $exe
    if ($LASTEXITCODE -ne 0) { throw "Module test link failed" }
    & $exe
}

