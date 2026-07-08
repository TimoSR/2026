[CmdletBinding()]
param(
    [switch]$Run
)

$ErrorActionPreference = "Stop"

$projectRoot = Split-Path -Parent $MyInvocation.MyCommand.Path

Push-Location $projectRoot
try
{
    $cmakeCommand = Get-Command cmake -ErrorAction SilentlyContinue

    if ($null -ne $cmakeCommand)
    {
        Write-Host "[build] configuring with CMake"
        & $cmakeCommand.Source -S . -B build

        Write-Host "[build] building with CMake"
        & $cmakeCommand.Source --build build

        if ($Run)
        {
            $executable = Get-ChildItem -Path "build" -Recurse -Filter "engine_config_example.exe" |
                Select-Object -First 1

            if ($null -eq $executable)
            {
                throw "Build succeeded, but engine_config_example.exe was not found in build/."
            }

            Write-Host "[run] launching $($executable.FullName)"
            & $executable.FullName
        }

        return
    }

    $clangCommand = Get-Command clang++ -ErrorAction SilentlyContinue
    if ($null -eq $clangCommand)
    {
        throw "Neither cmake nor clang++ was found in PATH."
    }

    if (-not (Test-Path "build"))
    {
        New-Item -ItemType Directory -Path "build" | Out-Null
    }

    $sources = @(
        "src/main.cpp",
        "src/file_system.cpp",
        "src/asset_store.cpp",
        "src/render_engine.cpp",
        "src/audio_engine.cpp",
        "src/physics_engine.cpp",
        "src/window.cpp",
        "src/config/app_configs.cpp"
    )

    $arguments = @(
        "-std=c++20",
        "-Wall",
        "-Wextra",
        "-Wpedantic",
        "-Iinclude"
    )
    $arguments += $sources
    $arguments += @(
        "-o",
        "build/engine_config_example.exe"
    )

    Write-Host "[build] compiling with clang++"
    & $clangCommand.Source @arguments

    if ($LASTEXITCODE -ne 0)
    {
        throw "clang++ compilation failed with exit code $LASTEXITCODE."
    }

    if ($Run)
    {
        Write-Host "[run] launching build/engine_config_example.exe"
        & ".\build\engine_config_example.exe"
    }
}
finally
{
    Pop-Location
}
