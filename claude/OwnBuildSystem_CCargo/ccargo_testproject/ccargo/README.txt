Here’s the **MVP-friendly** version — stripped down to just what’s needed to get `ccargo` built, installed, and usable from the terminal, plus the quick test workflow.

---

# Install & Use `ccargo`

## 1️⃣ Build & Install

```bash
# From ccargo repo root
cargo build --release
cargo install --path .
```

> If `ccargo` isn’t found, add `~/.cargo/bin` to your PATH.
> **Windows PowerShell**:
> `setx PATH "$env:USERPROFILE\.cargo\bin;$env:PATH"`

Verify:

```bash
ccargo --version
```

---

## 2️⃣ Test Project Quickstart

```bash
# Create new project
ccargo init --name demo

# Add a Git dep (with CMake)
ccargo add fmt \
  --git https://github.com/fmtlib/fmt.git \
  --tag 11.0.2 \
  --cmake \
  --cmake-opts -DFMT_TEST=OFF

# (Windows) Add SDL2 via Chocolatey
ccargo add sdl2 --choco sdl2 --version 2.30.0

# Install deps
ccargo resolve

# Build + run
ccargo build --std c++20
ccargo run -- arg1 arg2
```

---

## 3️⃣ Common Commands

| Command          | Description                          |
| ---------------- | ------------------------------------ |
| `ccargo init`    | Create a new C++ project             |
| `ccargo add`     | Add a dependency (Git/Chocolatey)    |
| `ccargo resolve` | Install dependencies                 |
| `ccargo build`   | Compile project                      |
| `ccargo run`     | Build (if needed) and run the binary |

---

Do you want me to also make a **one-liner Windows installer** that downloads the prebuilt `.exe` and sets up PATH automatically? That would make the MVP even easier to try.
