# Folder: .

All commands below are run from project root:
C:\Users\timot\code\2026\Codex-Test\LearningCPP\cpp_quickstart_cheatsheet

## Alternative References (And Why This Repo Exists)

This repository is an example-first, pattern-based learning path for people who want to start writing C++ quickly.

If you want traditional references or book-style material, use these too:
- C++ reference: https://cppreference.com/
- Book-style C++ tutorials: https://www.learncpp.com/
- C++ Core Guidelines: https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines

If you found this while searching for C++ docs, beginner material, books, or reference content, this repo is designed as a faster practical companion with runnable examples + tests.

## Run Entire Package
```powershell
.\\run_all_tests.ps1
```

## Dependency Setup

No manual setup command is required for normal use.

- `run_all_tests.ps1`, `run_file.ps1`, `run_demo.ps1`, and `run_module_file.ps1` auto-check and auto-install core dependencies on first run.
- If a dependency is missing, the scripts use `winget` automatically.
- Graphics and container tooling are still optional and only needed for those specific sections.

## Run DB Container Integration Checks
```powershell
.\run_db_integration_checks.ps1
```

Keep containers running after checks:
```powershell
.\run_db_integration_checks.ps1 -KeepRunning
```

## Important Note (`using namespace std;`)

These learning examples often use `using namespace std;` to reduce visual noise.

In real projects, this is usually discouraged:
- prefer explicit `std::` (`std::string`, `std::vector`, `std::cout`)
- it makes symbol origin clear
- it reduces naming collisions as the codebase and dependencies grow

Production-style C++ usually avoids global `using namespace std;` in source files and especially in headers.

## Important Note (Headers, Header / Source Split and Header Only)

Section 06 (`06_code_organization_approaches`) is about **development structure standards**, not just syntax.

- `single-file` style is best for learning, tiny experiments, and fast iteration.
- `header + source split` is the normal standard for medium/large production codebases.
- `header-only` is useful for small reusable utilities and templates, but can increase compile times.
- `Modules` still have uneven IDE/editor support today, so setup is often harder than header-based approaches.

Practical team standard:
- learning/prototyping: single-file is fine
- production apps/libraries: prefer header/source split
- header-only: use intentionally, not by default

New C++ developers are often surprised by this transition from tutorial style to production structure.

## Important Note (Public API Stability)

When a header is part of a public API, changing it can break consumers.

- keep public headers small and intentional
- avoid exposing internals unless needed
- prefer stable interfaces and evolve implementations behind them

This matters more as soon as your code is reused by other modules, teams, or external users.

