# io_macros_project

Small Rust project with `input!` and `output!` macros.

## Run

```bash
cargo run
```

## Test

```bash
cargo test
```

## Example

```rust
use io_macros_project::{input, output};

fn main() {
    let mut distance = 0.0_f64;
    let mut time = 0.0_f64;

    input! {
        distance,
        time,
    }

    let velocity = distance / time;

    output! {
        distance = {distance}
        time = {time}
        velocity = {velocity}
    }
}
```
