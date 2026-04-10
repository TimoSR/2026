param(
    [ValidateSet("core", "graphics")]
    [string]$Profile = "core",
    [switch]$AutoInstall
)

$ErrorActionPreference = "Stop"
$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path

. (Join-Path $scriptRoot 'setup_cpp_tooling.ps1')

function CommandExists([string]$commandName) {
    return $null -ne (Get-Command $commandName -ErrorAction SilentlyContinue)
}

function InstallWithWinget([string]$packageId, [string]$friendlyName) {
    if (!(CommandExists "winget")) {
        throw "Missing dependency: $friendlyName. winget is not available for auto-install."
    }

    Write-Host "Installing $friendlyName via winget..."
    winget install --id $packageId --exact --accept-package-agreements --accept-source-agreements
    if ($LASTEXITCODE -ne 0) {
        throw "Failed installing $friendlyName via winget ($packageId)."
    }
}

function EnsureDependency([string]$friendlyName, [string]$commandName, [string]$wingetPackageId) {
    if (CommandExists $commandName) {
        return
    }

    if ($AutoInstall -and $wingetPackageId -ne "") {
        InstallWithWinget $wingetPackageId $friendlyName
        if (CommandExists $commandName) {
            return
        }
    }

    if ($wingetPackageId -ne "") {
        throw "Missing dependency: $friendlyName. Install with: winget install --id $wingetPackageId --exact"
    }

    throw "Missing dependency: $friendlyName."
}

function EnsureCoreDependencies {
    if (!(CommandExists "clang++.exe") -and !(CommandExists "g++.exe")) {
        if ($AutoInstall) {
            InstallWithWinget "LLVM.LLVM" "LLVM clang++"
            . (Join-Path $scriptRoot 'setup_cpp_tooling.ps1')
        }
    }

    if (!(CommandExists "clang++.exe") -and !(CommandExists "g++.exe")) {
        throw "No C++ compiler found. Install LLVM or WinLibs."
    }
}

function EnsureGraphicsDependencies {
    EnsureCoreDependencies
    EnsureDependency "CMake" "cmake" "Kitware.CMake"
    EnsureDependency "Ninja" "ninja" "Ninja-build.Ninja"
    EnsureDependency "Git" "git" "Git.Git"
    EnsureDependency "Rust toolchain (cargo) for wgpu workflows" "cargo" "Rustlang.Rustup"

    if (!(Test-Path "C:\Program Files (x86)\Windows Kits\10")) {
        Write-Host "Warning: Windows SDK not found at expected location."
        Write-Host "DirectX examples may fail until Windows SDK is installed via Visual Studio Build Tools."
    }

    if ($env:VULKAN_SDK -eq $null -or $env:VULKAN_SDK -eq "") {
        Write-Host "Warning: VULKAN_SDK is not set."
        Write-Host "Install Vulkan SDK from LunarG for Vulkan examples."
    }
}

if ($Profile -eq "core") {
    EnsureCoreDependencies
    Write-Host "Dependency check complete (core)."
} else {
    EnsureGraphicsDependencies
    Write-Host "Dependency check complete (graphics)."
}
