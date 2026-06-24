#![allow(clippy::needless_return)]

mod cube;
mod graphics;
mod window;

use std::time::Instant;
use windows::{
    core::Result,
    Win32::UI::WindowsAndMessaging::MSG,
};

fn main() -> Result<()>
{
    // application constants
    const WINDOW_WIDTH: i32 = 1280;
    const WINDOW_HEIGHT: i32 = 720;
    const FIRST_CUBE_IDENTIFIER: u64 = 1;
    const SECOND_CUBE_IDENTIFIER: u64 = 2;
    const FIRST_CUBE_POSITION: [f32; 3] = [-1.3, 0.0, 5.0];
    const SECOND_CUBE_POSITION: [f32; 3] = [1.3, 0.0, 5.0];
    const FIRST_CUBE_ROTATION_RADIANS_PER_SECOND: [f32; 3] = [0.8, 1.2, 0.0];
    const SECOND_CUBE_ROTATION_RADIANS_PER_SECOND: [f32; 3] = [1.1, -0.7, 0.4];
    // application constants

    let window = window::create_window(WINDOW_WIDTH, WINDOW_HEIGHT)?;

    let first_cube = cube::SpinningCube::new(
        FIRST_CUBE_IDENTIFIER,
        FIRST_CUBE_POSITION,
        FIRST_CUBE_ROTATION_RADIANS_PER_SECOND,
    );
    let second_cube = cube::SpinningCube::new(
        SECOND_CUBE_IDENTIFIER,
        SECOND_CUBE_POSITION,
        SECOND_CUBE_ROTATION_RADIANS_PER_SECOND,
    );

    let mut graphics = graphics::create_direct3d_graphics(
        window.handle(),
        WINDOW_WIDTH as u32,
        WINDOW_HEIGHT as u32,
    )?;

    graphics.add_object(first_cube)?;
    graphics.add_object(second_cube)?;

    let started_at = Instant::now();
    let mut message = MSG::default();

    loop
    {
        let should_close = window::process_pending_messages(&mut message);

        if should_close
        {
            return Ok(());
        }

        let elapsed_seconds = started_at.elapsed().as_secs_f32();
        graphics.render(elapsed_seconds)?;
    }
}
