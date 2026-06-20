# Assembly Subroutines in C

A C version of the Rust example: hand-written assembly subroutine selected per architecture.

## Project Structure

```
CExample/
├── asm/
│   ├── x86_64_windows/add.asm
│   ├── x86_64/add.s
│   └── aarch64/add.s
├── src/main.c
└── CMakeLists.txt
```

## Build & Run

```sh
cmake -S . -B build
cmake --build build
./build/assembly_subroutines_c
```
