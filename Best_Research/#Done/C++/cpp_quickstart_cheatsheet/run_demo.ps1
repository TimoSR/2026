param(
    [Parameter(Mandatory = $true)]
    [string]$ConceptCppPath
)

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

$fullConceptPath = Join-Path $scriptRoot $ConceptCppPath
if (!(Test-Path $fullConceptPath)) {
    throw "Concept file not found: $ConceptCppPath"
}

$buildDir = Join-Path $scriptRoot '.build'
if (!(Test-Path $buildDir)) {
    New-Item -ItemType Directory -Path $buildDir | Out-Null
}

$exeName = ((Split-Path $ConceptCppPath -Leaf) -replace '\.cpp$', '') + '.demo.exe'
$outputExe = Join-Path $buildDir $exeName

Write-Host "Compiling demo: $ConceptCppPath"
& $compiler -std=c++17 -DRUN_DEMO $fullConceptPath -o $outputExe
if ($LASTEXITCODE -ne 0) {
    throw "Demo compile failed: $ConceptCppPath"
}

Write-Host "Running demo: $ConceptCppPath"
& $outputExe
