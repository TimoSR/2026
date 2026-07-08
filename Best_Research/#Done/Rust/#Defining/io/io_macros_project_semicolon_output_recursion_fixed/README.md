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

## Performance Comparison

The project includes a benchmark-style comparison binary:

```powershell
$env:RUSTFLAGS='-C opt-level=3'
cargo run --release --bin perf_compare -- 1000000
```

`cargo run --release` already uses Cargo's optimized release profile. The
explicit `RUSTFLAGS` setting makes the `-O` / `opt-level=3` optimization level
unambiguous.

Recent result on this machine:

```text
iterations: 1000000
output_to!: 2.9490697s, 2949.1 ns/report, 1238000000 bytes, 461000000 writes
writeln!: 1.3115585s, 1311.6 ns/report, 1238000000 bytes, 103000000 writes
writeln! was 2.25x faster
```

Both implementations wrote the same byte count. The traditional `writeln!`
version is faster because Rust's formatting machinery uses direct format
strings. `output_to!` writes literals and values directly, but still has to
restore spacing around token boundaries for the free-form `<<` syntax.

The comparison intentionally uses `output_to!` and `writeln!` with the same
in-memory counting writer. That keeps the benchmark focused on formatting and
template-processing overhead. Comparing `output!` with `println!` would mostly
measure stdout locking, buffering, terminal speed, and OS behavior.

`println!` writes to stdout and appends a newline. `writeln!` writes to any
`Write` target, also appends a newline, and returns a `Result`.

There is also a stdout comparison between `output!` and the same report written
with `println!`:

```powershell
$env:RUSTFLAGS='-C opt-level=3'
cargo run --release --bin perf_stdout_compare -- 10000 > $null
```

The benchmark prints timing results to stderr, so stdout can be redirected away.
Recent result on this machine with stdout redirected to `$null`:

```text
iterations: 10000
output!: 358.2989ms, 35829.9 ns/report
println!: 335.8473ms, 33584.7 ns/report
println! was 1.07x faster
```

`output!` uses a buffered stdout path: it locks stdout once and reuses a line
buffer for the whole block. `output_to!` uses a direct writer path, which is
better for the CPU-focused counting-writer benchmark.

## Test

```powershell
cargo test
```
