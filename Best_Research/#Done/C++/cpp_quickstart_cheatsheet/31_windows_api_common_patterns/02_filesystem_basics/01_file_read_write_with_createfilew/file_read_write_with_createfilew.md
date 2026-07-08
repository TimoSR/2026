# File Read/Write with CreateFileW

This file is intentionally narrow: it only shows Win32 file read/write flow.

- UTF-8 path -> wide path conversion
- `CreateFileW` handle open
- `WriteFile` and `ReadFile`
- explicit `CloseHandle`
