param(
    [switch]$KeepRunning
)

$ErrorActionPreference = "Stop"
$scriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path

if ($null -eq (Get-Command docker -ErrorAction SilentlyContinue)) {
    throw "Docker is not available on PATH. Install Docker Desktop and reopen PowerShell."
}

docker compose version | Out-Null
if ($LASTEXITCODE -ne 0) {
    throw "Docker Compose is not available. Enable Docker Compose v2 in Docker Desktop."
}

Write-Host "Starting Postgres + MySQL containers..."
docker compose -f (Join-Path $scriptRoot "docker-compose.yml") up -d

$postgresContainer = "cpp_learning_postgres"
$mysqlContainer = "cpp_learning_mysql"

Write-Host "Waiting for containers to become healthy..."
docker inspect --format='{{.State.Health.Status}}' $postgresContainer | Out-Null
docker inspect --format='{{.State.Health.Status}}' $mysqlContainer | Out-Null

for ($i = 0; $i -lt 40; $i++) {
    $pg = docker inspect --format='{{.State.Health.Status}}' $postgresContainer
    $my = docker inspect --format='{{.State.Health.Status}}' $mysqlContainer
    if ($pg -eq "healthy" -and $my -eq "healthy") {
        break
    }
    Start-Sleep -Milliseconds 500
}

$pgStatus = docker inspect --format='{{.State.Health.Status}}' $postgresContainer
$myStatus = docker inspect --format='{{.State.Health.Status}}' $mysqlContainer
if ($pgStatus -ne "healthy" -or $myStatus -ne "healthy") {
    throw "Containers did not become healthy in time. Postgres=$pgStatus MySQL=$myStatus"
}

Write-Host "Running Postgres integration query..."
$pgResult = docker exec $postgresContainer psql -U app -d app -t -c "SELECT COUNT(*) FROM users WHERE email='seed@example.com';"
$pgCount = ($pgResult | Out-String).Trim()
if ($pgCount -ne "1") {
    throw "Postgres integration check failed. Expected 1, got '$pgCount'"
}

Write-Host "Running MySQL integration query..."
$myResult = docker exec $mysqlContainer mysql -uapp -papp -D app -N -e "SELECT COUNT(*) FROM users WHERE email='seed@example.com';"
$myCount = ($myResult | Out-String).Trim()
if ($myCount -ne "1") {
    throw "MySQL integration check failed. Expected 1, got '$myCount'"
}

Write-Host "Integration checks passed for Postgres and MySQL."

if (-not $KeepRunning) {
    Write-Host "Stopping containers..."
    docker compose -f (Join-Path $scriptRoot "docker-compose.yml") down
}
