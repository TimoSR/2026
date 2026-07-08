# C++ Organization Approaches Map

## 1) Header + Source Split (`.h` + `.cpp`)

- Most common in traditional C++ codebases
- Good default for medium/large projects
- Example: `01_header_source_split`

## 2) Single-File Learning Style (`.cpp` only)

- Best for teaching and rapid prototyping
- Used in this package for quick scanning
- Example: `02_single_file_learning_style`

## 3) Header-Only Libraries (`.h` only)

- Everything in headers (often templates)
- Easy distribution, but can increase compile times
- Example: `03_header_only_library`

## 4) C++20 Modules (`import`/`module`)

- Modern alternative to many header issues
- Cleaner boundaries and potentially faster builds
- Toolchain/ecosystem support still varies by setup
- Example: `04_cpp20_modules`

## Practical Recommendation

- Learning phase: single-file style
- Real app phase: header + source split
- Utility/template phase: header-only is practical
- Performance/scale phase: evaluate modules where toolchain allows

## InDepths

- Organization choice depends on team size, build speed constraints, and tooling maturity.
- Single-file examples optimize learning speed; split/header/module layouts optimize long-term maintenance.
- Re-evaluate structure when pain appears (slow builds, hard navigation, merge conflicts).

