$ErrorActionPreference = "Stop"

$llvmBin = "C:\Program Files\LLVM\bin"
$winLibsBin = "C:\Users\timot\AppData\Local\Microsoft\WinGet\Packages\BrechtSanders.WinLibs.POSIX.UCRT_Microsoft.Winget.Source_8wekyb3d8bbwe\mingw64\bin"

function Add-PathIfExists([string]$pathToAdd) {
    if (!(Test-Path $pathToAdd)) {
        return
    }

    $pathItems = $env:PATH -split ";"
    if ($pathItems -contains $pathToAdd) {
        return
    }

    $env:PATH = "$pathToAdd;$env:PATH"
}

Add-PathIfExists $llvmBin
Add-PathIfExists $winLibsBin

$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($null -eq $userPath) {
    $userPath = ""
}

function Persist-UserPathIfMissing([string]$pathToAdd) {
    if (!(Test-Path $pathToAdd)) {
        return
    }

    $userPathItems = $userPath -split ";" | Where-Object { $_ -ne "" }
    if ($userPathItems -contains $pathToAdd) {
        return
    }

    $newUserPath = "$pathToAdd;$userPath"
    [Environment]::SetEnvironmentVariable("Path", $newUserPath, "User")
    $script:userPath = $newUserPath
}

Persist-UserPathIfMissing $llvmBin
Persist-UserPathIfMissing $winLibsBin

Write-Host "Tooling setup complete."
Write-Host "Current clang++:" 
try {
    clang++ --version
} catch {
    Write-Host "clang++ not currently available on PATH in this shell."
}

try {
    g++ --version
} catch {
    Write-Host "g++ not currently available on PATH in this shell."
}
