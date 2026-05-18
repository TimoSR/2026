param(
    [int]$Frames = 6,
    [switch]$Release
)

$ErrorActionPreference = "Stop"

$workspaceRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..\..")
$profile = if ($Release) { "release" } else { "debug" }
$buildArgs = @("build", "-p", "executable-api-demo")
if ($Release) {
    $buildArgs += "--release"
}

Push-Location $workspaceRoot
try {
    & cargo @buildArgs | Out-Host

    $exePath = Join-Path $workspaceRoot "target\$profile\executable-api-demo.exe"
    if (-not (Test-Path $exePath)) {
        throw "missing executable: $exePath"
    }

    & $exePath run --frames $Frames --json
    exit $LASTEXITCODE
} finally {
    Pop-Location
}
