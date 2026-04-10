$ErrorActionPreference = "Stop"

$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$integrationScript = Join-Path $scriptRoot "27_database_connectivity\02_containerized_integration\run_db_integration_checks.ps1"

if (!(Test-Path $integrationScript)) {
    throw "Missing integration script: $integrationScript"
}

& $integrationScript @args
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

