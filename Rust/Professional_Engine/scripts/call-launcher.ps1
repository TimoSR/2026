param(
    [ValidateRange(1, 100000)]
    [int]$Frames = 6,
    [switch]$Release,
    [switch]$AsJson
)

$ErrorActionPreference = "Stop"

$workspaceRoot = Split-Path -Parent $PSScriptRoot
$profile = if ($Release) { "release" } else { "debug" }
$buildArgs = @("build", "-p", "launcher")
if ($Release) {
    $buildArgs += "--release"
}

Write-Host "[script] building launcher ($profile)"
& cargo @buildArgs | Out-Host

$exePath = Join-Path $workspaceRoot "target\$profile\launcher.exe"
if (-not (Test-Path $exePath)) {
    throw "compiled executable not found: $exePath"
}

Write-Host "[script] calling compiled executable: $exePath $Frames"

$process = New-Object System.Diagnostics.Process
$process.StartInfo = New-Object System.Diagnostics.ProcessStartInfo
$process.StartInfo.FileName = $exePath
$process.StartInfo.Arguments = "$Frames"
$process.StartInfo.WorkingDirectory = $workspaceRoot
$process.StartInfo.UseShellExecute = $false
$process.StartInfo.RedirectStandardOutput = $true
$process.StartInfo.RedirectStandardError = $true

$null = $process.Start()
$stdout = $process.StandardOutput.ReadToEnd()
$stderr = $process.StandardError.ReadToEnd()
$process.WaitForExit()

if ($AsJson) {
    [pscustomobject]@{
        executable = $exePath
        frames = $Frames
        exit_code = $process.ExitCode
        stdout = $stdout.TrimEnd()
        stderr = $stderr.TrimEnd()
    } | ConvertTo-Json -Depth 4
} else {
    if ($stdout) {
        Write-Host "--- stdout ---"
        Write-Host $stdout.TrimEnd()
    }

    if ($stderr) {
        Write-Host "--- stderr ---"
        Write-Host $stderr.TrimEnd()
    }

    Write-Host ("[script] exit code: {0}" -f $process.ExitCode)
}

exit $process.ExitCode
