# Database Connectivity: Postgres / MySQL Patterns

This section focuses on practical patterns you use before choosing a specific client library.

Covered:
- connection string shape differences
- parameterized SQL query shape
- repository abstraction for testable business logic
- for real C++ database calls, see `03_cpp_database_calls_with_odbc/postgres_mysql_odbc_calls.cpp`

## InDepths

- Never concatenate raw user input into SQL strings; use parameterized queries/placeholders.
- Keep DB library code at infrastructure boundaries; business logic should depend on repository interfaces.
- Start with one DB adapter per database type (Postgres/MySQL) behind the same repository contract.
