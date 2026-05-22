const MAX_WAVES: usize = 16;

#[derive(Clone, Copy, Debug)]
pub struct Wave {
    pub amp: f32,
    pub freq: f32,
    pub phase: f32,
}

static mut AMPS: [f32; MAX_WAVES] = [0.0; MAX_WAVES];
static mut FREQS: [f32; MAX_WAVES] = [0.0; MAX_WAVES];
static mut PHASES: [f32; MAX_WAVES] = [0.0; MAX_WAVES];

pub fn combine_optimized_tensor(waves: &[Wave], t: f32, step: f32) -> f32 {
    waves.iter().map(|wave| combine_wave(*wave, t, step)).sum()
}

fn combine_wave(wave: Wave, t: f32, step: f32) -> f32 {
    let initial_angle = wave.freq * t + wave.phase;
    let angle_delta = wave.freq * step;
    let cos_delta = angle_delta.cos();
    let sin_delta = angle_delta.sin();
    let cos_angle = initial_angle.cos();
    let sin_angle = initial_angle.sin();

    let _next_cos = cos_angle * cos_delta - sin_angle * sin_delta;
    let _next_sin = sin_angle * cos_delta + cos_angle * sin_delta;

    wave.amp * cos_angle
}

#[no_mangle]
pub extern "C" fn max_waves() -> usize {
    MAX_WAVES
}

#[no_mangle]
pub extern "C" fn amps_ptr() -> *mut f32 {
    core::ptr::addr_of_mut!(AMPS).cast::<f32>()
}

#[no_mangle]
pub extern "C" fn freqs_ptr() -> *mut f32 {
    core::ptr::addr_of_mut!(FREQS).cast::<f32>()
}

#[no_mangle]
pub extern "C" fn phases_ptr() -> *mut f32 {
    core::ptr::addr_of_mut!(PHASES).cast::<f32>()
}

#[no_mangle]
pub extern "C" fn combine(count: usize, t: f32, step: f32) -> f32 {
    let capped_count = count.min(MAX_WAVES);
    let mut sum = 0.0;
    let mut index = 0;

    while index < capped_count {
        let amp = unsafe { AMPS[index] };
        let freq = unsafe { FREQS[index] };
        let phase = unsafe { PHASES[index] };
        let wave = Wave { amp, freq, phase };

        sum += combine_wave(wave, t, step);
        index += 1;
    }

    sum
}
