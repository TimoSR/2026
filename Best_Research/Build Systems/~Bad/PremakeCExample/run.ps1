param(
    [ValidateSet("Debug", "Release")]
    [string]$Configuration = "Debug"
)

$ErrorActionPreference = "Stop"

function Find-Premake {
    $cmd = Get-Command premake5 -ErrorAction SilentlyContinue
    if ($cmd) { return $cmd.Source }

    $candidates = @(
        "C:\Users\$env:USERNAME\AppData\Local\Microsoft\WinGet\Packages\Premake.Premake.5.Beta_Microsoft.Winget.Source_8wekyb3d8bbwe\premake5.exe",
        "C:\Program Files\premake\premake5.exe"
    )
    foreach ($path in $candidates) {
        if (Test-Path $path) {
            try {
                Get-Item $path | Out-Null
                return $path
            } catch {
            }
        }
    }
    return $null
}

$premakeExe = Find-Premake
if (-not $premakeExe) {
    throw "premake5.exe not found. Install Premake or add it to PATH."
}

& $premakeExe vs2022

function Find-VsWhere {
    $cmd = Get-Command vswhere -ErrorAction SilentlyContinue
    if ($cmd) { return $cmd.Source }

    $candidates = @(
        "C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe",
        "C:\Program Files\Microsoft Visual Studio\Installer\vswhere.exe"
    )
    foreach ($path in $candidates) {
        if (Test-Path $path) { return $path }
    }
    return $null
}

$vswhereExe = Find-VsWhere
if (-not $vswhereExe) {
    throw "vswhere.exe not found. Install Visual Studio Build Tools or Visual Studio Installer."
}

$msbuild = & $vswhereExe -latest -products * -requires Microsoft.Component.MSBuild -find MSBuild\**\Bin\MSBuild.exe | Select-Object -First 1
if (-not $msbuild) {
    throw "MSBuild.exe not found via vswhere. Install Visual Studio Build Tools."
}

& $msbuild .\assembly_subroutines_c_premake.sln /p:Configuration=$Configuration

$exe = ".\build\bin\$Configuration\assembly_subroutines_c_premake.exe"
if (-not (Test-Path $exe)) {
    throw "Built executable not found: $exe"
}

& $exe
