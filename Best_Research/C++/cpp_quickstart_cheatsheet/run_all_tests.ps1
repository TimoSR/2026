$ErrorActionPreference = "Stop"

$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
& (Join-Path $scriptRoot 'install_dependencies.ps1') -Profile core -AutoInstall

function Get-CppCompiler {
    $gxx = Get-Command g++.exe -ErrorAction SilentlyContinue
    if ($null -ne $gxx) {
        return $gxx.Path
    }

    $clangxx = Get-Command clang++.exe -ErrorAction SilentlyContinue
    if ($null -ne $clangxx) {
        return $clangxx.Path
    }

    throw "No C++ compiler found. Run setup_cpp_tooling.ps1 or install LLVM/WinLibs."
}

$compiler = Get-CppCompiler
Write-Host "Using compiler: $compiler"

$buildDir = Join-Path $scriptRoot '.build'
if (!(Test-Path $buildDir)) {
    New-Item -ItemType Directory -Path $buildDir | Out-Null
}

$testFiles = Get-ChildItem -Path $scriptRoot -Recurse -Filter '*.test.cpp' | Sort-Object FullName

if ($testFiles.Count -eq 0) {
    throw "No test files found (*.test.cpp)."
}

$moduleTest = $testFiles | Where-Object {
    $_.Name -eq 'bank_account_module_usage.test.cpp'
}

$regularTests = $testFiles | Where-Object {
    $_.FullName -ne $moduleTest.FullName
}

foreach ($testFile in $regularTests) {
    $relativePath = $testFile.FullName.Substring($scriptRoot.Length + 1)
    $exeName = ($relativePath -replace '[\\/\.:]', '_') + '.exe'
    $outputExe = Join-Path $buildDir $exeName

    $cppStandard = "c++17"
    if ($testFile.Name -like "*.cpp23.test.cpp") {
        $cppStandard = "c++23"
    }

    Write-Host "Compiling $relativePath"
    & $compiler "-std=$cppStandard" $testFile.FullName -o $outputExe
    if ($LASTEXITCODE -ne 0) {
        throw "Compile failed for $relativePath"
    }

    Write-Host "Running $relativePath"
    & $outputExe
    if ($LASTEXITCODE -ne 0) {
        throw "Test failed for $relativePath"
    }
}

if ($null -ne $moduleTest) {
    $clangxx = Get-Command clang++.exe -ErrorAction SilentlyContinue
    if ($null -eq $clangxx) {
        throw "Module test found, but clang++ is required and was not found on PATH."
    }

    $moduleCompiler = $clangxx.Path
    $moduleTestPath = $moduleTest.FullName
    $moduleRelativePath = $moduleTestPath.Substring($scriptRoot.Length + 1)
    $moduleDir = Split-Path -Parent $moduleTestPath
    $moduleInterface = Join-Path $moduleDir 'bank_account_module.ixx'

    if (!(Test-Path $moduleInterface)) {
        throw "Missing module interface file: $moduleInterface"
    }

    $moduleBuildDir = Join-Path $buildDir 'modules'
    if (!(Test-Path $moduleBuildDir)) {
        New-Item -ItemType Directory -Path $moduleBuildDir | Out-Null
    }

    $runStamp = Get-Date -Format 'yyyyMMdd_HHmmss_fff'
    $modulePcm = Join-Path $moduleBuildDir "bank_account_module_$runStamp.pcm"
    $moduleObj = Join-Path $moduleBuildDir "bank_account_module_$runStamp.obj"
    $moduleTestObj = Join-Path $moduleBuildDir "bank_account_module_usage_test_$runStamp.obj"
    $moduleExe = Join-Path $moduleBuildDir "bank_account_module_usage_test_$runStamp.exe"
    $moduleArg = "-fmodule-file=bank_account_module=$modulePcm"

    Write-Host "Compiling module interface $($moduleInterface.Substring($scriptRoot.Length + 1))"
    & $moduleCompiler -std=c++20 -x c++-module --precompile $moduleInterface -o $modulePcm
    if ($LASTEXITCODE -ne 0) {
        throw "Module interface precompile failed."
    }

    # Keep a stable PCM path for editor tooling (clangd / compile_commands.json).
    $stableModulePcm = Join-Path $moduleBuildDir 'bank_account_module.pcm'
    try {
        Copy-Item -LiteralPath $modulePcm -Destination $stableModulePcm -Force
    } catch {
        Write-Host "Warning: could not update stable module PCM path for editor tooling."
    }

    & $moduleCompiler -std=c++20 -x c++-module -c $moduleInterface -o $moduleObj
    if ($LASTEXITCODE -ne 0) {
        throw "Module interface object compile failed."
    }

    Write-Host "Compiling $moduleRelativePath"
    & $moduleCompiler -std=c++20 -c $moduleTestPath $moduleArg -o $moduleTestObj
    if ($LASTEXITCODE -ne 0) {
        throw "Module test compile failed for $moduleRelativePath"
    }

    & $moduleCompiler $moduleObj $moduleTestObj -o $moduleExe
    if ($LASTEXITCODE -ne 0) {
        throw "Module test link failed for $moduleRelativePath"
    }

    Write-Host "Running $moduleRelativePath"
    & $moduleExe
    if ($LASTEXITCODE -ne 0) {
        throw "Test failed for $moduleRelativePath"
    }
}

Write-Host "All tests passed."
