# Real C++ Calls to Postgres/MySQL via ODBC

This example does real database I/O in C++:

- `SQLDriverConnectA` for opening a DB connection
- `SQLExecDirectA` for running SQL
- `SQLFetch` + `SQLGetData` for reading rows

## What This Example Queries

Both probes run:

`SELECT COUNT(*) FROM users WHERE email='seed@example.com';`

against:

- Postgres container: `127.0.0.1:55432`
- MySQL container: `127.0.0.1:53306`

## Why ODBC Here

- No project-specific C++ ORM required.
- One API shape for both Postgres and MySQL.
- Good for showing raw, practical DB flow in C++.

## InDepths

- Connection strings are DSN-less (`Driver={...};Server=...;Port=...;...`) so learners can see all required fields.
- Driver names differ by installed ODBC driver package. The code tries multiple common driver names.
- Tests are stable even when local ODBC drivers or DB containers are missing; success paths activate automatically when available.
