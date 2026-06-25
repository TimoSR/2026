#![deny(unsafe_op_in_unsafe_fn)]

use hot_api::{
    FrameContext, PluginStatus, HOT_API_VERSION, KEY_ACTION, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP,
};

const FILE_ID_GAME_PLUGIN: u32 = 1;

#[no_mangle]
pub extern "C" fn hot_api_version() -> u32 {
    HOT_API_VERSION
}

#[no_mangle]
pub unsafe extern "C" fn hot_update(ctx: *mut FrameContext) -> PluginStatus {
    if ctx.is_null() {
        return PluginStatus::null_context();
    }

    let ctx = unsafe { &mut *ctx };
    let input = ctx.input;

    if ctx.state.is_null() {
        return PluginStatus::null_state();
    }

    let mut axis_x = 0.0_f32;
    let mut axis_y = 0.0_f32;

    if input.is_key_down(KEY_LEFT) {
        axis_x -= 1.0;
    }
    if input.is_key_down(KEY_RIGHT) {
        axis_x += 1.0;
    }
    if input.is_key_down(KEY_UP) {
        axis_y -= 1.0;
    }
    if input.is_key_down(KEY_DOWN) {
        axis_y += 1.0;
    }

    let speed = 800.0;
    let damping = 0.82;

    let (player_x, player_y) = {
        let state = unsafe { &mut *ctx.state };
        state.total_time_seconds += f64::from(input.dt_seconds);

        state.player_vx = state.player_vx * damping + axis_x * speed * (1.0 - damping);
        state.player_vy = state.player_vy * damping + axis_y * speed * (1.0 - damping);

        if input.mouse_down != 0 || input.is_key_down(KEY_ACTION) {
            state.player_x += (input.mouse_x - state.player_x) * 0.12;
            state.player_y += (input.mouse_y - state.player_y) * 0.12;
        } else {
            state.player_x += state.player_vx * input.dt_seconds;
            state.player_y += state.player_vy * input.dt_seconds;
        }

        state.player_x = state.player_x.clamp(24.0, 936.0);
        state.player_y = state.player_y.clamp(24.0, 516.0);

        (state.player_x, state.player_y)
    };

    let t = input.time_seconds as f32;
    let pulse = 0.5 + 0.5 * (t * 4.0).sin();
    let player_size = 34.0 + pulse * 10.0;

    let bg_r = 0.055;
    let bg_g = 0.060;
    let bg_b = 0.085;

    unsafe {
        ctx.emit_rect(0.0, 0.0, 960.0, 540.0, bg_r, bg_g, bg_b, 1.0);
    }

    draw_grid(ctx, t);

    let shadow_size = player_size + 18.0;
    unsafe {
        ctx.emit_rect(
            player_x - shadow_size * 0.5 + 6.0,
            player_y - shadow_size * 0.5 + 8.0,
            shadow_size,
            shadow_size,
            0.0,
            0.0,
            0.0,
            1.0,
        );
    }

    let player_r = 0.95;
    let player_g = 0.42 + 0.25 * pulse;
    let player_b = 0.18;

    unsafe {
        ctx.emit_rect(
            player_x - player_size * 0.5,
            player_y - player_size * 0.5,
            player_size,
            player_size,
            player_r,
            player_g,
            player_b,
            1.0,
        );
    }

    let aim_size = if input.mouse_down != 0 { 22.0 } else { 14.0 };
    unsafe {
        ctx.emit_rect(
            input.mouse_x - aim_size * 0.5,
            input.mouse_y - 2.0,
            aim_size,
            4.0,
            0.9,
            0.9,
            0.25,
            1.0,
        );
        ctx.emit_rect(
            input.mouse_x - 2.0,
            input.mouse_y - aim_size * 0.5,
            4.0,
            aim_size,
            0.9,
            0.9,
            0.25,
            1.0,
        );
    }

    unsafe {
        ctx.emit_debug_f64(
            FILE_ID_GAME_PLUGIN,
            line!(),
            column!(),
            "player_x",
            f64::from(player_x),
        );
        ctx.emit_debug_f64(
            FILE_ID_GAME_PLUGIN,
            line!(),
            column!(),
            "player_y",
            f64::from(player_y),
        );
        ctx.emit_debug_f64(
            FILE_ID_GAME_PLUGIN,
            line!(),
            column!(),
            "speed",
            f64::from(speed),
        );
    }

    PluginStatus::ok()
}

fn draw_grid(ctx: &mut FrameContext, t: f32) {
    let offset = (t * 12.0) % 48.0;

    let mut x = -offset;
    while x < 960.0 {
        unsafe {
            ctx.emit_rect(x, 0.0, 1.0, 540.0, 0.11, 0.12, 0.16, 1.0);
        }
        x += 48.0;
    }

    let mut y = offset;
    while y < 540.0 {
        unsafe {
            ctx.emit_rect(0.0, y, 960.0, 1.0, 0.11, 0.12, 0.16, 1.0);
        }
        y += 48.0;
    }
}
