#![deny(unsafe_op_in_unsafe_fn)]

//! Stable host/plugin ABI for Hotgame Live.
//!
//! Keep this crate deliberately small. The host and plugin are compiled separately,
//! so shared structs crossing the reload boundary must be `#[repr(C)]`, non-generic,
//! and layout-stable while the host process is running.
//!
//! During live development, edit `crates/game_plugin/src/lib.rs`. Restart the host
//! after changing this crate.

use core::ffi::c_void;

pub const HOT_API_VERSION: u32 = 1;
pub const DEBUG_LABEL_BYTES: usize = 64;

pub const KEY_LEFT: u32 = 1 << 0;
pub const KEY_RIGHT: u32 = 1 << 1;
pub const KEY_UP: u32 = 1 << 2;
pub const KEY_DOWN: u32 = 1 << 3;
pub const KEY_ACTION: u32 = 1 << 4;

pub const DRAW_KIND_RECT: u32 = 1;

pub type HotApiVersionFn = unsafe extern "C" fn() -> u32;
pub type HotUpdateFn = unsafe extern "C" fn(*mut FrameContext) -> PluginStatus;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GameState {
    pub player_x: f32,
    pub player_y: f32,
    pub player_vx: f32,
    pub player_vy: f32,
    pub camera_x: f32,
    pub camera_y: f32,
    pub total_time_seconds: f64,
    pub plugin_generation: u64,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            player_x: 320.0,
            player_y: 240.0,
            player_vx: 0.0,
            player_vy: 0.0,
            camera_x: 0.0,
            camera_y: 0.0,
            total_time_seconds: 0.0,
            plugin_generation: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Input {
    pub dt_seconds: f32,
    pub time_seconds: f64,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_down: u32,
    pub key_bits: u32,
    pub frame_index: u64,
}

impl Input {
    #[must_use]
    pub const fn is_key_down(self, key: u32) -> bool {
        self.key_bits & key != 0
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct DrawCommand {
    pub kind: u32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DebugEvent {
    pub file_id: u32,
    pub line: u32,
    pub column: u32,
    pub value: f64,
    pub label_len: u32,
    pub label: [u8; DEBUG_LABEL_BYTES],
}

impl Default for DebugEvent {
    fn default() -> Self {
        Self {
            file_id: 0,
            line: 0,
            column: 0,
            value: 0.0,
            label_len: 0,
            label: [0; DEBUG_LABEL_BYTES],
        }
    }
}

impl DebugEvent {
    #[must_use]
    pub fn label_string(&self) -> String {
        let len = (self.label_len as usize).min(self.label.len());
        String::from_utf8_lossy(&self.label[..len]).into_owned()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PluginStatus {
    pub code: i32,
}

impl PluginStatus {
    #[must_use]
    pub const fn ok() -> Self {
        Self { code: 0 }
    }

    #[must_use]
    pub const fn null_context() -> Self {
        Self { code: 1 }
    }

    #[must_use]
    pub const fn null_state() -> Self {
        Self { code: 2 }
    }

    #[must_use]
    pub const fn buffer_full() -> Self {
        Self { code: 3 }
    }
}

#[repr(C)]
pub struct FrameContext {
    pub input: Input,
    pub state: *mut GameState,
    pub user_data: *mut c_void,

    pub draw_commands: *mut DrawCommand,
    pub draw_len: *mut usize,
    pub draw_capacity: usize,

    pub debug_events: *mut DebugEvent,
    pub debug_len: *mut usize,
    pub debug_capacity: usize,
}

impl FrameContext {
    /// # Safety
    ///
    /// The host must provide a valid `state` pointer for the duration of the call.
    pub unsafe fn state_mut(&mut self) -> Option<&mut GameState> {
        if self.state.is_null() {
            None
        } else {
            Some(unsafe { &mut *self.state })
        }
    }

    /// # Safety
    ///
    /// The host must provide valid draw buffer pointers and capacity.
    pub unsafe fn emit_rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) -> bool {
        if self.draw_commands.is_null() || self.draw_len.is_null() {
            return false;
        }

        let draw_len = unsafe { &mut *self.draw_len };

        if *draw_len >= self.draw_capacity {
            return false;
        }

        let command = DrawCommand {
            kind: DRAW_KIND_RECT,
            x,
            y,
            w,
            h,
            r,
            g,
            b,
            a,
        };

        unsafe {
            self.draw_commands.add(*draw_len).write(command);
        }

        *draw_len += 1;
        true
    }

    /// # Safety
    ///
    /// The host must provide valid debug buffer pointers and capacity.
    pub unsafe fn emit_debug_f64(
        &mut self,
        file_id: u32,
        line: u32,
        column: u32,
        label: &str,
        value: f64,
    ) -> bool {
        if self.debug_events.is_null() || self.debug_len.is_null() {
            return false;
        }

        let debug_len = unsafe { &mut *self.debug_len };

        if *debug_len >= self.debug_capacity {
            return false;
        }

        let mut event = DebugEvent {
            file_id,
            line,
            column,
            value,
            label_len: 0,
            label: [0; DEBUG_LABEL_BYTES],
        };

        let label_bytes = label.as_bytes();
        let copy_len = label_bytes.len().min(DEBUG_LABEL_BYTES);
        event.label[..copy_len].copy_from_slice(&label_bytes[..copy_len]);
        event.label_len = copy_len as u32;

        unsafe {
            self.debug_events.add(*debug_len).write(event);
        }

        *debug_len += 1;
        true
    }
}
