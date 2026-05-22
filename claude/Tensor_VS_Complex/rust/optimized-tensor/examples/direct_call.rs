use optimized_tensor::{combine_optimized_tensor, Wave};

fn main() {
    let waves = [
        Wave {
            amp: 1.0,
            freq: 1.0,
            phase: 0.0,
        },
        Wave {
            amp: 0.5,
            freq: 2.0,
            phase: 1.0,
        },
        Wave {
            amp: 0.3,
            freq: 3.0,
            phase: 2.5,
        },
    ];

    for t in [0.0_f32, 0.5, 1.0, 1.5, 2.0] {
        let value = combine_optimized_tensor(&waves, t, 0.01);
        println!("t={t:.2} -> {value:.6}");
    }
}
