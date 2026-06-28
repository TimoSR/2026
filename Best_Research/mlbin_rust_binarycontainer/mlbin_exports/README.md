# ML Binary Container Rust MVPs

This export contains two complete Cargo projects:

- `mlbin_with_crates`: uses common crates for CLI parsing, JSON metadata, SHA-256, and tensor math.
- `mlbin_std_only`: uses only Rust standard library.

Each project can create a demo model, inspect the binary model file, and run dense ReLU inference.
