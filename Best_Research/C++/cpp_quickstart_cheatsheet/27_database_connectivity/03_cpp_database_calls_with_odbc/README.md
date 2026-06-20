# Folder: 27_database_connectivity/03_cpp_database_calls_with_odbc

All commands below are run from project root:  
`C:\Users\timot\code\2026\Codex-Test\LearningCPP\cpp_quickstart_cheatsheet`

## Run Concept File (real C++ DB calls)

```powershell
.\run_file.ps1 -File '27_database_connectivity\03_cpp_database_calls_with_odbc\postgres_mysql_odbc_calls.cpp' -Std c++17 -Demo
```

## Run Test File

```powershell
.\run_file.ps1 -File '27_database_connectivity\03_cpp_database_calls_with_odbc\postgres_mysql_odbc_calls.test.cpp' -Std c++17
```

## Practical Setup

1. Start DB containers:

```powershell
.\27_database_connectivity\02_containerized_integration\run_db_integration_checks.ps1 -KeepRunning
```

2. Ensure ODBC drivers are installed:
- PostgreSQL ODBC driver (psqlODBC)
- MySQL ODBC 8.x driver

3. Run the C++ demo command above to execute real SQL from C++.
