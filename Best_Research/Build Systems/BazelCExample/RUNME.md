# RUNME

## Prerequisites

- Bazelisk
- Visual Studio Build Tools (MSVC + MASM)

## Run

```powershell
cd C:\Users\timot\code\2026\claude\AssemblySubroutines\BazelCExample
bazelisk run //:app
```

## Build Only

```powershell
cd C:\Users\timot\code\2026\claude\AssemblySubroutines\BazelCExample
bazelisk build //:app
```

## Release-style Build/Run

```powershell
cd C:\Users\timot\code\2026\claude\AssemblySubroutines\BazelCExample
bazelisk build -c opt //:app
bazelisk run -c opt //:app
```
