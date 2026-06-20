# Folder: 27_database_connectivity/02_containerized_integration

All commands below are run from project root:  
`C:\Users\timot\code\2026\Codex-Test\LearningCPP\cpp_quickstart_cheatsheet`

## Run DB Integration Checks (Containerized)

```powershell
.\27_database_connectivity\02_containerized_integration\run_db_integration_checks.ps1
```

This starts Postgres + MySQL containers, verifies seeded data, and tears down by default.

Keep containers running after checks:

```powershell
.\27_database_connectivity\02_containerized_integration\run_db_integration_checks.ps1 -KeepRunning
```

## Manual Start/Stop

```powershell
docker compose -f .\27_database_connectivity\02_containerized_integration\docker-compose.yml up -d
docker compose -f .\27_database_connectivity\02_containerized_integration\docker-compose.yml down
```

## Notes
- This is a testcontainers-style workflow using Docker Compose.
- It validates DB integration behavior without needing local Postgres/MySQL installations.

