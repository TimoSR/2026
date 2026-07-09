# io_macros_project

Beginner-friendly console input and output macros for Rust.

## Run

The main binary is the default Cargo run target:

```powershell
cargo run
```

This works because `Cargo.toml` sets:

```toml
default-run = "io_macros_project"
```

## Output Syntax

Use `output!` with `<<` to mark each output line:

```rust
output! {
    << [distance details] distance = {distance}, bubels = {centimeters} centimeters, meters = {meters}.
    << [time details] time = {seconds} seconds, source value = {time}, status: accepted;
}
```

Each `<<` starts a new printed line. Semicolons are normal text, not line
terminators.

Large reports can stay in one `output!` block. The output macros parse the
block with a procedural macro, so normal Rust projects do not need a
crate-level `recursion_limit` setting for long text.

Use braces when you want no trailing semicolon after the macro call:

```rust
output! {
    << Please give me input!
}

input! {
    let distance: f64,
}
```

Do not write the no-semicolon form with parentheses:

```rust
output!(<< Please give me input!)
input! {
    let distance: f64,
}
```

Rust parses parenthesized macro calls as expression-style statements, so a
following statement requires a semicolon before macro expansion can run.

Spacing is intentionally preserved for normal output text. For example:

```rust
<< [distance details] distance = {distance}, bubels = {centimeters} centimeters, meters = {meters}.
```

renders as:

```text
[distance details] distance = 10, bubels = 1000 centimeters, meters = 10.
```

Arrays, vectors, and slices can be printed directly without Rust's `:?` debug
marker:

```rust
let lister = [1, 3, 4];

output! {
    << {lister}
}
```

renders as:

```text
[1, 3, 4]
```

String values inside lists use the same output style as normal `{value}`
interpolation, so they print without debug quotes.

The built-in output value support covers any type with `Display`, plus arrays,
vectors, slices, and references to those values. Custom collection-like types
can opt in by implementing `io_macros_project::OutputValue`.

## Performance Comparison

Run all performance checks with one command:

```powershell
just perf
```

The `Justfile` also accepts custom iteration counts:

```powershell
just perf 1000000 10000
```

Without `just`, run the underlying Cargo command directly:

```powershell
$env:RUSTFLAGS='-C opt-level=3'; cargo run --release --bin perf_all -- 1000000 10000 > $null
```

`cargo run --release` already uses Cargo's optimized release profile. The
explicit `RUSTFLAGS` setting makes the `-O` / `opt-level=3` optimization level
unambiguous.

The first number is the writer benchmark iteration count. The second number is
the stdout benchmark iteration count. The command redirects the large benchmark
output text to `$null`; the metrics are printed to stderr so they remain
visible.

The benchmark first renders every compared implementation to byte buffers and
asserts that the bytes are identical. It then reports the metrics a user needs
to judge whether this library is the right fit:

- `ns/report`: elapsed time per full report
- `bytes/report`: rendered output size
- `write ops/r`: exact `Write` calls for wrapped writers; logical stdout-facing calls for stdout benchmarks
- `allocs/r`: allocation calls per report
- `alloc bytes/r`: allocated bytes per report
- `extra alloc/r`: extra allocation calls per report compared with the baseline

Recent result on this machine:

```text
============================================================
io_macros_project performance report
============================================================
proof: PASS - every benchmark implementation renders identical report bytes before timing
report bytes: 1238
build: release profile with requested optimization flags

------------------------------------------------------------
how to read this report
------------------------------------------------------------
- ns/report: lower is faster for one complete report.
- write ops/r: fewer write calls usually means less writer or stdout overhead.
- allocs/r: 0 means no heap allocation inside the measured loop; 1 means one new allocation per report.
- alloc bytes/r: 2048 means a new 2 KiB buffer was allocated for each report.
- extra alloc/r: allocation count compared with that section's baseline.
- output_to! trades zero allocation for more write calls.
- output!, output_buffered_to!, and new-Vec buffering trade one allocation for one write call.
- reused-Vec buffering allocates before timing and reuses that memory.

------------------------------------------------------------
1. writer benchmarks
------------------------------------------------------------
iterations: 1000000
write ops/report: exact counted calls to the wrapped Write target
+---------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+
| implementation                  |    ns/report | bytes/report |  write ops/r | allocs/r    | alloc bytes/r | extra alloc/r |
+---------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+
| output_to!                      |       3107.5 |       1238.0 |        479.0 |       0.000 |           0.0 |        +0.000 |
| output_buffered_to!             |       3842.1 |       1238.0 |          1.0 |       1.000 |        2048.0 |        +1.000 |
| writeln!                        |       1162.4 |       1238.0 |        103.0 |       0.000 |           0.0 |        +0.000 |
| buffered writeln! new Vec       |       1526.8 |       1238.0 |          1.0 |       1.000 |        2048.0 |        +1.000 |
| buffered writeln! reused Vec    |       1360.4 |       1238.0 |          1.0 |       0.000 |           0.0 |        +0.000 |
+---------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+
fastest: writeln!
writeln! is 2.67x faster than output_to!
writeln! is 3.31x faster than output_buffered_to!
writeln! is 1.31x faster than buffered writeln! new Vec
writeln! is 1.17x faster than buffered writeln! reused Vec

------------------------------------------------------------
2. stdout benchmarks
------------------------------------------------------------
iterations: 10000
note: run this command with stdout redirected, for example `> $null` on PowerShell
write ops/report: logical stdout-facing calls per report, not OS syscall count
+---------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+
| implementation                  |    ns/report | bytes/report |  write ops/r | allocs/r    | alloc bytes/r | extra alloc/r |
+---------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+
| output!                         |      31942.5 |       1238.0 |          1.0 |       1.000 |        2048.0 |        +1.000 |
| println!                        |      47970.9 |       1238.0 |         16.0 |       0.000 |           0.0 |        +0.000 |
| locked writeln!                 |      45591.1 |       1238.0 |         16.0 |       0.000 |           0.0 |        +0.000 |
| buffered writeln! new Vec       |      16773.8 |       1238.0 |          1.0 |       1.000 |        2048.0 |        +1.000 |
| buffered writeln! reused Vec    |      17493.6 |       1238.0 |          1.0 |       0.000 |           0.0 |        +0.000 |
+---------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+
fastest: buffered writeln! new Vec
buffered writeln! new Vec is 1.90x faster than output!
buffered writeln! new Vec is 2.86x faster than println!
buffered writeln! new Vec is 2.72x faster than locked writeln!
buffered writeln! new Vec is 1.04x faster than buffered writeln! reused Vec
```

The factual conclusion is mixed, not one-sided. `output!` avoids the worst
repeated-locking cost of naive `println!`, but it allocates one 2048-byte buffer
per report. A carefully written `writeln!` implementation that reuses its buffer
is faster than `output!`. `output_to!` does not allocate, but it performs many
small writes because it preserves the free-form `<<` template syntax and spacing
rules.

## Test

```powershell
cargo test
```
