#![deny(unsafe_op_in_unsafe_fn)]

use anyhow::{anyhow, bail, Context, Result};
use hot_api::{
    DebugEvent, DrawCommand, FrameContext, GameState, HotApiVersionFn, HotUpdateFn, Input,
    DRAW_KIND_RECT, HOT_API_VERSION, KEY_ACTION, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP,
};
use libloading::Library;
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use std::{
    collections::VecDeque,
    ffi::OsStr,
    fs::{self, File},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    time::{Duration, Instant, SystemTime},
};

const WIDTH: usize = 960;
const HEIGHT: usize = 540;
const MAX_DRAW_COMMANDS: usize = 8192;
const MAX_DEBUG_EVENTS: usize = 2048;
const MAX_FRAME_CAPTURES: usize = 420;
const REBUILD_DEBOUNCE: Duration = Duration::from_millis(250);

#[cfg(target_os = "windows")]
const DYLIB_FILE_NAME: &str = "game_plugin.dll";

#[cfg(target_os = "macos")]
const DYLIB_FILE_NAME: &str = "libgame_plugin.dylib";

#[cfg(all(unix, not(target_os = "macos")))]
const DYLIB_FILE_NAME: &str = "libgame_plugin.so";

#[derive(Clone)]
struct FrameCapture {
    frame_index: u64,
    input: Input,
    state: GameState,
    draw_commands: Vec<DrawCommand>,
    debug_events: Vec<DebugEvent>,
}

struct Plugin {
    generation: u64,
    update: HotUpdateFn,
    _library: Library,
    loaded_from: PathBuf,
}

impl Plugin {
    fn load(generation: u64, path: PathBuf) -> Result<Self> {
        let library = unsafe { Library::new(&path) }
            .with_context(|| format!("failed to load plugin DLL: {}", path.display()))?;

        let version_fn: HotApiVersionFn = unsafe {
            *library
                .get::<HotApiVersionFn>(b"hot_api_version\0")
                .context("plugin is missing exported function hot_api_version")?
        };

        let update: HotUpdateFn = unsafe {
            *library
                .get::<HotUpdateFn>(b"hot_update\0")
                .context("plugin is missing exported function hot_update")?
        };

        let plugin_api_version = unsafe { version_fn() };
        if plugin_api_version != HOT_API_VERSION {
            bail!(
                "plugin API version mismatch: host={}, plugin={}",
                HOT_API_VERSION,
                plugin_api_version
            );
        }

        Ok(Self {
            generation,
            update,
            _library: library,
            loaded_from: path,
        })
    }

    fn update(&self, ctx: *mut FrameContext) -> i32 {
        let status = unsafe { (self.update)(ctx) };
        status.code
    }
}

struct BuildJob {
    child: Child,
    stdout_path: PathBuf,
    stderr_path: PathBuf,
    generation: u64,
    started_at: Instant,
}

struct BuildSystem {
    workspace_root: PathBuf,
    plugin_source_root: PathBuf,
    plugin_manifest: PathBuf,
    target_library: PathBuf,
    hot_dir: PathBuf,
    last_seen_source_mtime: Option<SystemTime>,
    pending_rebuild_after: Option<Instant>,
    queued_after_current: bool,
    job: Option<BuildJob>,
    next_generation: u64,
    last_status: String,
}

impl BuildSystem {
    fn new(workspace_root: PathBuf) -> Result<Self> {
        let plugin_source_root = workspace_root.join("crates").join("game_plugin").join("src");
        let plugin_manifest = workspace_root
            .join("crates")
            .join("game_plugin")
            .join("Cargo.toml");
        let target_library = workspace_root.join("target").join("debug").join(DYLIB_FILE_NAME);
        let hot_dir = workspace_root.join(".hot");

        fs::create_dir_all(&hot_dir)
            .with_context(|| format!("failed to create {}", hot_dir.display()))?;

        Ok(Self {
            workspace_root,
            plugin_source_root,
            plugin_manifest,
            target_library,
            hot_dir,
            last_seen_source_mtime: None,
            pending_rebuild_after: None,
            queued_after_current: false,
            job: None,
            next_generation: 1,
            last_status: "waiting for initial build".to_string(),
        })
    }

    fn request_build(&mut self, now: Instant, immediate: bool, reason: &str) {
        if self.job.is_some() {
            self.queued_after_current = true;
            self.last_status = format!("build queued: {reason}");
            return;
        }

        self.pending_rebuild_after = Some(if immediate { now } else { now + REBUILD_DEBOUNCE });
        self.last_status = format!("build scheduled: {reason}");
    }

    fn scan_for_changes(&mut self, now: Instant) -> Result<()> {
        let latest = self.latest_plugin_modified_time()?;
        if self.last_seen_source_mtime != Some(latest) {
            self.last_seen_source_mtime = Some(latest);
            self.request_build(now, false, "plugin source changed");
        }
        Ok(())
    }

    fn tick(&mut self, now: Instant) -> Result<Option<BuildOutput>> {
        if let Some(job) = &mut self.job {
            if let Some(status) = job.child.try_wait().context("failed to poll cargo build")? {
                let elapsed = job.started_at.elapsed();
                let stdout = read_lossy(&job.stdout_path);
                let stderr = read_lossy(&job.stderr_path);
                let generation = job.generation;
                self.job = None;

                if status.success() {
                    let copied = self.copy_fresh_plugin(generation)?;
                    self.last_status = format!("build ok: gen {generation}, {:.2?}", elapsed);

                    if self.queued_after_current {
                        self.queued_after_current = false;
                        self.request_build(now, true, "queued change after completed build");
                    }

                    return Ok(Some(BuildOutput::Success {
                        generation,
                        plugin_path: copied,
                        stdout,
                        stderr,
                    }));
                }

                self.last_status = format!("build failed: gen {generation}, {:.2?}", elapsed);
                if self.queued_after_current {
                    self.queued_after_current = false;
                    self.request_build(now, true, "queued change after failed build");
                }

                return Ok(Some(BuildOutput::Failure {
                    generation,
                    stdout,
                    stderr,
                }));
            }
        }

        if self.job.is_none() && self.pending_rebuild_after.is_some_and(|deadline| now >= deadline) {
            self.pending_rebuild_after = None;
            self.start_build(now)?;
        }

        Ok(None)
    }

    fn start_build(&mut self, now: Instant) -> Result<()> {
        let generation = self.next_generation;
        self.next_generation += 1;

        fs::create_dir_all(&self.hot_dir)
            .with_context(|| format!("failed to create {}", self.hot_dir.display()))?;

        let stdout_path = self.hot_dir.join(format!("build-{generation}-stdout.log"));
        let stderr_path = self.hot_dir.join(format!("build-{generation}-stderr.log"));

        let stdout = File::create(&stdout_path)
            .with_context(|| format!("failed to create {}", stdout_path.display()))?;
        let stderr = File::create(&stderr_path)
            .with_context(|| format!("failed to create {}", stderr_path.display()))?;

        let child = Command::new("cargo")
            .args(["build", "-p", "game_plugin", "--color", "never"])
            .current_dir(&self.workspace_root)
            .stdin(Stdio::null())
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .spawn()
            .context("failed to start cargo build for game_plugin")?;

        self.last_status = format!("building plugin gen {generation}");
        self.job = Some(BuildJob {
            child,
            stdout_path,
            stderr_path,
            generation,
            started_at: now,
        });

        Ok(())
    }

    fn copy_fresh_plugin(&self, generation: u64) -> Result<PathBuf> {
        if !self.target_library.exists() {
            bail!(
                "cargo build succeeded but plugin DLL was not found: {}",
                self.target_library.display()
            );
        }

        let copied_path = self.hot_dir.join(format!(
            "game_plugin-{}-{generation}.{}",
            std::process::id(),
            dylib_extension()
        ));

        fs::copy(&self.target_library, &copied_path).with_context(|| {
            format!(
                "failed to copy {} to {}",
                self.target_library.display(),
                copied_path.display()
            )
        })?;

        Ok(copied_path)
    }

    fn latest_plugin_modified_time(&self) -> Result<SystemTime> {
        let mut latest = SystemTime::UNIX_EPOCH;
        collect_latest_modified_time(&self.plugin_source_root, &mut latest)?;

        if self.plugin_manifest.exists() {
            let manifest_modified = fs::metadata(&self.plugin_manifest)
                .with_context(|| format!("failed to read {}", self.plugin_manifest.display()))?
                .modified()
                .with_context(|| format!("failed to read modified time for {}", self.plugin_manifest.display()))?;
            latest = latest.max(manifest_modified);
        }

        Ok(latest)
    }
}

#[derive(Debug)]
enum BuildOutput {
    Success {
        generation: u64,
        plugin_path: PathBuf,
        stdout: String,
        stderr: String,
    },
    Failure {
        generation: u64,
        stdout: String,
        stderr: String,
    },
}

fn workspace_root() -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| anyhow!("failed to resolve workspace root from CARGO_MANIFEST_DIR"))
}

fn dylib_extension() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "dll"
    }

    #[cfg(target_os = "macos")]
    {
        "dylib"
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        "so"
    }
}

fn collect_latest_modified_time(path: &Path, latest: &mut SystemTime) -> Result<()> {
    let metadata = fs::metadata(path)
        .with_context(|| format!("failed to read metadata for {}", path.display()))?;

    if metadata.is_dir() {
        for entry in fs::read_dir(path)
            .with_context(|| format!("failed to read directory {}", path.display()))?
        {
            let entry = entry.with_context(|| format!("failed to read entry in {}", path.display()))?;
            collect_latest_modified_time(&entry.path(), latest)?;
        }
    } else if path.extension().is_some_and(|ext| ext == OsStr::new("rs")) {
        let modified = metadata
            .modified()
            .with_context(|| format!("failed to read modified time for {}", path.display()))?;
        *latest = (*latest).max(modified);
    }

    Ok(())
}

fn read_lossy(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| format!("<failed to read {}: {error}>", path.display()))
}

fn key_bits(window: &Window) -> u32 {
    let mut bits = 0;

    if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
        bits |= KEY_LEFT;
    }
    if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
        bits |= KEY_RIGHT;
    }
    if window.is_key_down(Key::Up) || window.is_key_down(Key::W) {
        bits |= KEY_UP;
    }
    if window.is_key_down(Key::Down) || window.is_key_down(Key::S) {
        bits |= KEY_DOWN;
    }
    if window.is_key_down(Key::LeftShift) {
        bits |= KEY_ACTION;
    }

    bits
}

fn clear_buffer(buffer: &mut [u32], color: u32) {
    buffer.fill(color);
}

fn draw_rect(buffer: &mut [u32], command: DrawCommand) {
    if command.kind != DRAW_KIND_RECT {
        return;
    }

    let min_x = command.x.floor().max(0.0) as usize;
    let min_y = command.y.floor().max(0.0) as usize;
    let max_x = (command.x + command.w).ceil().clamp(0.0, WIDTH as f32) as usize;
    let max_y = (command.y + command.h).ceil().clamp(0.0, HEIGHT as f32) as usize;

    if min_x >= max_x || min_y >= max_y {
        return;
    }

    let color = rgba_to_u32(command.r, command.g, command.b, command.a);

    for y in min_y..max_y {
        let row_start = y * WIDTH;
        for x in min_x..max_x {
            buffer[row_start + x] = color;
        }
    }
}

fn rgba_to_u32(r: f32, g: f32, b: f32, _a: f32) -> u32 {
    let r = float_to_u8(r);
    let g = float_to_u8(g);
    let b = float_to_u8(b);
    ((r as u32) << 16) | ((g as u32) << 8) | b as u32
}

fn float_to_u8(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn push_capture(captures: &mut VecDeque<FrameCapture>, capture: FrameCapture) {
    captures.push_back(capture);
    while captures.len() > MAX_FRAME_CAPTURES {
        captures.pop_front();
    }
}

fn print_step_debug(capture: &FrameCapture, plugin_generation: Option<u64>) {
    println!(
        "frame={} plugin_gen={} state=({:.2}, {:.2}) dt={:.4}",
        capture.frame_index,
        plugin_generation.unwrap_or(0),
        capture.state.player_x,
        capture.state.player_y,
        capture.input.dt_seconds,
    );

    for event in capture.debug_events.iter().take(8) {
        println!(
            "  debug {}:{}:{} {} = {:.4}",
            event.file_id,
            event.line,
            event.column,
            event.label_string(),
            event.value
        );
    }
}

fn render_capture(buffer: &mut [u32], capture: Option<&FrameCapture>, paused: bool, building: bool) {
    clear_buffer(buffer, 0x0d0e14);

    if let Some(capture) = capture {
        for command in &capture.draw_commands {
            draw_rect(buffer, *command);
        }
    } else {
        draw_rect(
            buffer,
            DrawCommand {
                kind: DRAW_KIND_RECT,
                x: 0.0,
                y: 0.0,
                w: WIDTH as f32,
                h: HEIGHT as f32,
                r: 0.04,
                g: 0.045,
                b: 0.07,
                a: 1.0,
            },
        );
    }

    if paused {
        draw_rect(
            buffer,
            DrawCommand {
                kind: DRAW_KIND_RECT,
                x: 0.0,
                y: 0.0,
                w: 10.0,
                h: HEIGHT as f32,
                r: 0.9,
                g: 0.55,
                b: 0.1,
                a: 1.0,
            },
        );
    }

    if building {
        draw_rect(
            buffer,
            DrawCommand {
                kind: DRAW_KIND_RECT,
                x: 0.0,
                y: 0.0,
                w: WIDTH as f32,
                h: 8.0,
                r: 0.1,
                g: 0.45,
                b: 0.95,
                a: 1.0,
            },
        );
    }
}

fn capture_title(
    capture: Option<&FrameCapture>,
    plugin: Option<&Plugin>,
    build_status: &str,
    paused: bool,
    scrub_back: usize,
) -> String {
    let mode = if paused { "paused" } else { "live" };
    let generation = plugin.map_or(0, |plugin| plugin.generation);
    let frame = capture.map_or(0, |capture| capture.frame_index);

    let debug = capture
        .and_then(|capture| capture.debug_events.first())
        .map_or_else(
            || "no debug".to_string(),
            |event| format!("{}={:.2}", event.label_string(), event.value),
        );

    let scrub = if paused && scrub_back > 0 {
        format!(" | scrub -{scrub_back}")
    } else {
        String::new()
    };

    format!(
        "Hotgame Live | {mode}{scrub} | frame {frame} | plugin gen {generation} | {debug} | {build_status}"
    )
}

fn main() -> Result<()> {
    let root = workspace_root()?;
    println!("workspace: {}", root.display());
    println!("edit crates\\game_plugin\\src\\lib.rs while this is running");
    println!("controls: Space pause/resume, Right step, Left scrub back, Enter latest, F5 reload, Esc quit");

    let mut build_system = BuildSystem::new(root)?;
    let mut window = Window::new("Hotgame Live", WIDTH, HEIGHT, WindowOptions::default())
        .context("failed to create window")?;

    let mut pixels = vec![0_u32; WIDTH * HEIGHT];
    let mut draw_buffer = vec![DrawCommand::default(); MAX_DRAW_COMMANDS];
    let mut debug_buffer = vec![DebugEvent::default(); MAX_DEBUG_EVENTS];
    let mut captures = VecDeque::<FrameCapture>::new();

    let started_at = Instant::now();
    let mut previous_frame = Instant::now();
    let mut frame_index = 0_u64;
    let mut game_state = GameState::default();
    let mut plugin: Option<Plugin> = None;
    let mut paused = false;
    let mut scrub_back = 0_usize;
    let mut last_title_update = Instant::now();

    let now = Instant::now();
    build_system.request_build(now, true, "initial build");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();

        if let Err(error) = build_system.scan_for_changes(now) {
            eprintln!("source scan failed: {error:?}");
        }

        if window.is_key_pressed(Key::F5, KeyRepeat::No) {
            build_system.request_build(now, true, "F5 pressed");
        }

        match build_system.tick(now) {
            Ok(Some(BuildOutput::Success {
                generation,
                plugin_path,
                stdout,
                stderr,
            })) => {
                if !stdout.trim().is_empty() {
                    println!("cargo stdout:\n{}", stdout.trim());
                }
                if !stderr.trim().is_empty() {
                    println!("cargo stderr:\n{}", stderr.trim());
                }

                match Plugin::load(generation, plugin_path) {
                    Ok(new_plugin) => {
                        game_state.plugin_generation = generation;
                        println!(
                            "reloaded plugin gen {} from {}",
                            generation,
                            new_plugin.loaded_from.display()
                        );
                        plugin = Some(new_plugin);
                    }
                    Err(error) => {
                        eprintln!("plugin load failed after successful build: {error:?}");
                    }
                }
            }
            Ok(Some(BuildOutput::Failure {
                generation,
                stdout,
                stderr,
            })) => {
                eprintln!("plugin build failed for gen {generation}");
                if !stdout.trim().is_empty() {
                    eprintln!("cargo stdout:\n{}", stdout.trim());
                }
                if !stderr.trim().is_empty() {
                    eprintln!("cargo stderr:\n{}", stderr.trim());
                }
            }
            Ok(None) => {}
            Err(error) => eprintln!("build system error: {error:?}"),
        }

        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            paused = !paused;
            scrub_back = 0;
            println!("{}", if paused { "paused" } else { "resumed" });
        }

        if paused && window.is_key_pressed(Key::Left, KeyRepeat::No) && !captures.is_empty() {
            scrub_back = (scrub_back + 1).min(captures.len() - 1);
        }

        if window.is_key_pressed(Key::Enter, KeyRepeat::No) {
            scrub_back = 0;
        }

        let step_frame = paused && window.is_key_pressed(Key::Right, KeyRepeat::No);
        let should_simulate = !paused || step_frame;

        if should_simulate {
            scrub_back = 0;
            let dt = if paused {
                1.0 / 60.0
            } else {
                now.duration_since(previous_frame)
                    .as_secs_f32()
                    .clamp(1.0 / 240.0, 1.0 / 15.0)
            };
            previous_frame = now;

            let (mouse_x, mouse_y) = window
                .get_mouse_pos(MouseMode::Discard)
                .unwrap_or((game_state.player_x, game_state.player_y));

            let input = Input {
                dt_seconds: dt,
                time_seconds: started_at.elapsed().as_secs_f64(),
                mouse_x,
                mouse_y,
                mouse_down: u32::from(window.get_mouse_down(MouseButton::Left)),
                key_bits: key_bits(&window),
                frame_index,
            };

            let mut draw_len = 0_usize;
            let mut debug_len = 0_usize;

            let mut context = FrameContext {
                input,
                state: &mut game_state as *mut GameState,
                user_data: std::ptr::null_mut(),
                draw_commands: draw_buffer.as_mut_ptr(),
                draw_len: &mut draw_len,
                draw_capacity: draw_buffer.len(),
                debug_events: debug_buffer.as_mut_ptr(),
                debug_len: &mut debug_len,
                debug_capacity: debug_buffer.len(),
            };

            if let Some(plugin) = &plugin {
                let status = plugin.update(&mut context);
                if status != 0 {
                    eprintln!("plugin returned status code {status}");
                }
            }

            let capture = FrameCapture {
                frame_index,
                input,
                state: game_state,
                draw_commands: draw_buffer.iter().take(draw_len).copied().collect(),
                debug_events: debug_buffer.iter().take(debug_len).copied().collect(),
            };

            if step_frame {
                print_step_debug(&capture, plugin.as_ref().map(|plugin| plugin.generation));
            }

            push_capture(&mut captures, capture);
            frame_index = frame_index.wrapping_add(1);
        }

        let capture_index = captures.len().checked_sub(1 + scrub_back);
        let display_capture = capture_index.and_then(|index| captures.get(index));
        let building = build_system.job.is_some();

        render_capture(&mut pixels, display_capture, paused, building);
        window
            .update_with_buffer(&pixels, WIDTH, HEIGHT)
            .context("failed to update window buffer")?;

        if last_title_update.elapsed() >= Duration::from_millis(120) {
            let title = capture_title(
                display_capture,
                plugin.as_ref(),
                &build_system.last_status,
                paused,
                scrub_back,
            );
            window.set_title(&title);
            last_title_update = Instant::now();
        }
    }

    Ok(())
}
