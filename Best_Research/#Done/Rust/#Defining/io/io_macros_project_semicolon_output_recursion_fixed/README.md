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

The built-in output value support covers numbers, booleans, characters,
strings, arrays, vectors, slices, and references to those values. Custom types
can opt in by implementing `io_macros_project::OutputValue`.

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
output_to!: 2.8531065s, 2853.1 ns/report, 1238000000 bytes, 461000000 writes
writeln!: 1.1950106s, 1195.0 ns/report, 1238000000 bytes, 103000000 writes
writeln! was 2.39x faster
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
output!: 163.8296ms, 16383.0 ns/report
println!: 353.3016ms, 35330.2 ns/report
output! was 2.16x faster
```

`output!` uses a whole-block buffered stdout path: it renders the block into a
memory buffer, locks stdout once, and writes the rendered bytes once.
`output_to!` uses a direct writer path, which avoids the extra memory buffer for
caller-provided writers but performs more small writes.

## Test

```powershell
cargo test
```
