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

Very large output blocks still have a compile-time cost. The macro moves text
parsing and spacing work out of runtime and into compilation, then emits normal
Rust write calls. That can make runtime faster than repeated `println!`, but a
project with many huge `output!` blocks can compile slower than the same project
written with `println!`.

The intentionally large compile-stress example is kept in an opt-in binary:

```powershell
just check-output-stress
cargo run --manifest-path stress_cases/output_stress_case/Cargo.toml
```

The default `cargo run` path stays small so normal development does not compile
the stress case every time. The stress case is outside the main Cargo package
targets, so `perf_all`, `cargo test`, and `cargo clippy --all-targets` do not
compile it unless you call the stress command explicitly.

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

`output!` and `output_buffered_to!` use a 2048-byte starting buffer by default.
For larger reports, reserve more space up front with a concrete buffer size:

```rust
output! {
    buffer: 8192 BYTES,
    << [large report] distance = {distance}, meters = {meters}.
}

output! {
    buffer: 8 KB,
    << [large report] distance = {distance}, meters = {meters}.
}

output_buffered_to! {
    writer: &mut writer,
    buffer: 8 KB,
    << [large report] distance = {distance}, meters = {meters}.
}
```

This is a CPU / memory tradeoff. A larger capacity can avoid buffer growth for
large reports, but it allocates more memory per report. The macro accepts
`BYTES`, `KB`, and `MB`. Uppercase units tend to get better editor coloring,
and the parser also accepts lowercase unit names. For buffer sizing, `KB` means
1024 bytes.

For hot paths where one allocation per report is still too much, reuse a buffer
owned by the caller:

```rust
let mut writer = Vec::new();
let mut output_buffer = Vec::with_capacity(8 * 1024);

output_reusing_to! {
    writer: &mut writer,
    buffer: &mut output_buffer,
    << [large report] distance = {distance}, meters = {meters}.
}
```

This keeps the buffer allocation outside the measured or repeated output path.

## Performance Comparison

Run all performance checks with one command:

```powershell
just perf
```

The `Justfile` also accepts custom iteration counts:

```powershell
just perf_prop 1000000 10000
```

Without `just`, run the underlying Cargo command directly:

```powershell
$env:RUSTFLAGS='-C opt-level=3'; cargo run --release --bin perf_all -- 1000000 10000 > $null
```

`cargo run --release` already uses Cargo's optimized release profile. The
explicit `RUSTFLAGS` setting makes the `-O` / `opt-level=3` optimization level
unambiguous.

The first number is the practical writer benchmark iteration count. The second
number is the practical stdout benchmark iteration count. Stress benchmarks use
one tenth of those counts because each stress report is larger. The command
redirects the large benchmark output text to `$null`; the metrics are printed to
stderr so they remain visible.

The benchmark first renders every compared implementation to byte buffers and
asserts that the bytes are identical. It then reports the metrics a user needs
to judge whether this library is the right fit:

- `ns/report`: elapsed time per full report
- `bytes/report`: rendered output size
- `write ops/r`: exact `Write` calls for wrapped writers; logical stdout-facing calls for stdout benchmarks
- `allocs/r`: allocation calls per report
- `alloc bytes/r`: bytes requested from the allocator per report
- `extra alloc/r`: extra allocation calls per report compared with the baseline

The fastest row in each table is highlighted green in the terminal. The
project's own `output!`, `output_to!`, `output_buffered_to!`, and
`output_reusing_to!` rows are highlighted yellow unless one of them is also the
winner. Set `NO_COLOR=1` to disable ANSI color output.

Recent result on this machine:

```text
============================================================
io_macros_project performance report
============================================================
proof: PASS - every benchmark implementation renders identical bytes before timing
practical report bytes: 1238
stress report bytes: 4952
stress report blocks: 4 practical reports in one large output block
build: release profile with requested optimization flags

------------------------------------------------------------
how to read this report
------------------------------------------------------------
- ns/report: lower is faster for one complete report.
- write ops/r: fewer write calls usually means less writer or stdout overhead.
- allocs/r: 0 means no heap allocation inside the measured loop; 1 means one new allocation per report.
- alloc bytes/r: bytes requested from the allocator per report; 2048 is the default output buffer, 8192 is the large-buffer test.
- extra alloc/r: allocation count compared with that section's baseline.
- vs output...: percent faster or slower than the output macro row in that table.
- output_to! writes literals and values directly without per-report allocation.
- single println!/writeln! is the optimized traditional baseline for one big interpolated format string.
- output!, output_buffered_to!, and new-Vec buffering trade one allocation for one final write call.
- output_reusing_to! rows allocate before timing and reuse caller-owned Vec memory.
- buffer-size rows compare explicit output_buffered_to! stress buffers: 2 KB, 4 KB, 8 KB, 16 KB, 32 KB, and 64 KB.
- reused-Vec buffering allocates before timing and reuses that memory.

------------------------------------------------------------
1. practical writer benchmarks
------------------------------------------------------------
iterations: 1000000
write ops/report: exact counted calls to the wrapped Write target
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| implementation                                   |    ns/report | bytes/report |  write ops/r | allocs/r    | alloc bytes/r | extra alloc/r |          vs output_to! |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output_to!                                       |       1166.4 |       1238.0 |        103.0 |       0.000 |           0.0 |        +0.000 |               baseline |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output_buffered_to!                              |       1282.7 |       1238.0 |          1.0 |       1.000 |        2048.0 |        +1.000 |           10.0% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output_buffered_to! buffer 8 KB                  |       1396.6 |       1238.0 |          1.0 |       1.000 |        8192.0 |        +1.000 |           19.7% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output_reusing_to! reused Vec                    |       1435.4 |       1238.0 |          1.0 |       0.000 |           0.0 |        +0.000 |           23.1% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| writeln!                                         |       1225.0 |       1238.0 |        103.0 |       0.000 |           0.0 |        +0.000 |            5.0% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| single writeln!                                  |       1207.7 |       1238.0 |         88.0 |       0.000 |           0.0 |        +0.000 |            3.5% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| buffered writeln! new Vec                        |       1433.1 |       1238.0 |          1.0 |       1.000 |        2048.0 |        +1.000 |           22.9% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| buffered writeln! reused Vec                     |       1345.9 |       1238.0 |          1.0 |       0.000 |           0.0 |        +0.000 |           15.4% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+

------------------------------------------------------------
2. practical stdout benchmarks
------------------------------------------------------------
iterations: 10000
note: run this command with stdout redirected, for example `> $null` on PowerShell
write ops/report: logical stdout-facing calls per report, not OS syscall count
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| implementation                                   |    ns/report | bytes/report |  write ops/r | allocs/r    | alloc bytes/r | extra alloc/r |             vs output! |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output!                                          |      32289.8 |       1238.0 |          1.0 |       1.000 |        2048.0 |        +1.000 |               baseline |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output! buffer 8 KB                              |      36592.1 |       1238.0 |          1.0 |       1.000 |        8192.0 |        +1.000 |           13.3% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output_reusing_to! stdout reused Vec             |      36498.2 |       1238.0 |          1.0 |       0.000 |           0.0 |        +0.000 |           13.0% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| println!                                         |      48979.1 |       1238.0 |         16.0 |       0.000 |           0.0 |        +0.000 |           51.7% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| single println!                                  |      48226.4 |       1238.0 |          1.0 |       0.000 |           0.0 |        +0.000 |           49.4% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| locked writeln!                                  |      49871.1 |       1238.0 |         16.0 |       0.000 |           0.0 |        +0.000 |           54.4% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| buffered writeln! new Vec                        |      17268.0 |       1238.0 |          1.0 |       1.000 |        2048.0 |        +1.000 |           87.0% faster |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| buffered writeln! reused Vec                     |      18225.9 |       1238.0 |          1.0 |       0.000 |           0.0 |        +0.000 |           77.2% faster |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+

------------------------------------------------------------
3. stress writer benchmarks
------------------------------------------------------------
iterations: 100000
stress shape: one large output macro block matching 4 practical reports
write ops/report: exact counted calls to the wrapped Write target
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| implementation                                   |    ns/report | bytes/report |  write ops/r | allocs/r    | alloc bytes/r | extra alloc/r |   vs output_to! stress |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output_to! stress                                |       5082.1 |       4952.0 |        412.0 |       0.000 |           0.0 |        +0.000 |               baseline |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output_buffered_to! stress                       |       5439.3 |       4952.0 |          1.0 |       3.000 |       14336.0 |        +3.000 |            7.0% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output_buffered_to! stress buffer 8 KB           |       5118.9 |       4952.0 |          1.0 |       1.000 |        8192.0 |        +1.000 |            0.7% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output_reusing_to! stress reused Vec             |       5858.0 |       4952.0 |          1.0 |       0.000 |           0.0 |        +0.000 |           15.3% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| writeln! stress                                  |       5111.9 |       4952.0 |        412.0 |       0.000 |           0.0 |        +0.000 |            0.6% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| single writeln! stress                           |       4382.8 |       4952.0 |        349.0 |       0.000 |           0.0 |        +0.000 |           16.0% faster |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| buffered writeln! stress                         |       6437.9 |       4952.0 |          1.0 |       1.000 |        8192.0 |        +1.000 |           26.7% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| buffered writeln! reused Vec stress              |       5625.8 |       4952.0 |          1.0 |       0.000 |           0.0 |        +0.000 |           10.7% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+

------------------------------------------------------------
4. stress stdout benchmarks
------------------------------------------------------------
iterations: 1000
stress shape: one large output! block compared with repeated println! and one large single println!
note: run this command with stdout redirected, for example `> $null` on PowerShell
write ops/report: logical stdout-facing calls per report, not OS syscall count
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| implementation                                   |    ns/report | bytes/report |  write ops/r | allocs/r    | alloc bytes/r | extra alloc/r |      vs output! stress |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output! stress                                   |      68277.3 |       4952.0 |          1.0 |       3.000 |       14336.0 |        +3.000 |               baseline |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output! stress buffer 8 KB                       |      62270.1 |       4952.0 |          1.0 |       1.000 |        8192.0 |        +1.000 |            9.6% faster |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output_reusing_to! stress stdout reused Vec      |      61638.1 |       4952.0 |          1.0 |       0.000 |           0.0 |        +0.000 |           10.8% faster |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| println! stress                                  |     198603.0 |       4952.0 |         64.0 |       0.000 |           0.0 |        +0.000 |          190.9% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| single println! stress                           |     193792.0 |       4952.0 |          1.0 |       0.000 |           0.0 |        +0.000 |          183.8% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| locked writeln! stress                           |     197763.1 |       4952.0 |         64.0 |       0.000 |           0.0 |        +0.000 |          189.6% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| buffered writeln! stress                         |      59807.3 |       4952.0 |          1.0 |       1.000 |        8192.0 |        +1.000 |           14.2% faster |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| buffered writeln! reused Vec stress              |      59583.6 |       4952.0 |          1.0 |       0.000 |           0.0 |        +0.000 |           14.6% faster |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+

------------------------------------------------------------
5. output_buffered_to! buffer size benchmarks
------------------------------------------------------------
iterations: 100000
stress shape: one large output_buffered_to! block matching 4 practical reports
write ops/report: exact counted calls to the wrapped Write target
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| implementation                                   |    ns/report | bytes/report |  write ops/r | allocs/r    | alloc bytes/r | extra alloc/r |                vs 2 KB |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output_buffered_to! stress buffer 2 KB           |       5692.1 |       4952.0 |          1.0 |       3.000 |       14336.0 |        +0.000 |               baseline |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output_buffered_to! stress buffer 4 KB           |       7069.8 |       4952.0 |          1.0 |       2.000 |       12288.0 |        -1.000 |           24.2% slower |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output_buffered_to! stress buffer 8 KB           |       5460.9 |       4952.0 |          1.0 |       1.000 |        8192.0 |        -2.000 |            4.2% faster |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output_buffered_to! stress buffer 16 KB          |       5168.9 |       4952.0 |          1.0 |       1.000 |       16384.0 |        -2.000 |           10.1% faster |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output_buffered_to! stress buffer 32 KB          |       5158.5 |       4952.0 |          1.0 |       1.000 |       32768.0 |        -2.000 |           10.3% faster |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
| output_buffered_to! stress buffer 64 KB          |       5285.5 |       4952.0 |          1.0 |       1.000 |       65536.0 |        -2.000 |            7.7% faster |
+--------------------------------------------------+--------------+--------------+--------------+-------------+---------------+---------------+------------------------+
```

The factual conclusion is mixed, not one-sided. The `output_to!` improvement is
not about avoiding stdout flushing: the writer benchmark uses an in-memory
`CountingWriter`, and none of the compared writer implementations flush. The
speedup came from moving literal text and spacing work into the procedural macro
at compile time. That removed the old runtime scanning path and reduced the
measured write calls for this report from 479 to 103.

The closest traditional comparison is a single interpolated multi-line
`writeln!` or `println!`. For writer targets, `output_to!` is close to both
traditional forms for this report. For stdout, `output!` beats repeated
`println!` calls in both practical and stress cases.

`output_reusing_to!` removes per-report heap allocation by reusing a caller-owned
buffer. That helped in the stress stdout benchmark here, but it was not a
universal win in the in-memory writer benchmark. The buffer-size table shows
the CPU / memory tradeoff directly: for a 4952-byte stress report, `2 KB`
allocates 3 times, `4 KB` allocates 2 times, and `8 KB` through `64 KB` allocate
once. In this run, `16 KB`, `32 KB`, and `64 KB` were close, with `32 KB` fastest
by a small margin. Larger buffers mainly buy fewer growth allocations at the
cost of reserving more memory per report.

## Test

```powershell
cargo test
```

## Rust Analyzer

This project uses procedural macros for `output!`, `output_to!`,
`output_buffered_to!`, and `output_reusing_to!`. If rust-analyzer shows a
warning like this while the code still builds and runs:

```text
proc macro server error: Cannot create expander for ...io_macros_project_macros-....dll
```

the editor is usually holding a stale path to an old proc-macro DLL. The checked
in VS Code setting uses `target/rust-analyzer` for rust-analyzer builds so
normal Cargo commands do not replace the DLL rust-analyzer is trying to load.

After pulling this setting, restart rust-analyzer once from the command palette:

```text
Rust Analyzer: Restart Server
```
