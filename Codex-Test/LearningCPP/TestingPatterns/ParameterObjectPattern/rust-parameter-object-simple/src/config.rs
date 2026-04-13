#[derive(Debug, Clone, Copy)]
pub struct AppConfig {
    pub display: DisplayConfig,
    pub diagnostics: DiagnosticsConfig,
    pub render: RenderConfig,
    pub audio: AudioConfig,
    pub window: WindowConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            display: DisplayConfig::default(),
            diagnostics: DiagnosticsConfig::default(),
            render: RenderConfig::default(),
            audio: AudioConfig::default(),
            window: WindowConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DisplayConfig {
    pub resolution: Resolution,
    pub refresh_rate: Framerate,
    pub v_sync: VSync,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            resolution: Resolution::full_hd(),
            refresh_rate: Framerate::from_fps(60),
            v_sync: VSync::Enabled,
        }
    }
}

impl DisplayConfig {
    pub fn summary(&self) -> String {
        format!(
            "{}x{} @ {}Hz, v_sync={}",
            self.resolution.width,
            self.resolution.height,
            self.refresh_rate.fps,
            self.v_sync.as_str()
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DiagnosticsConfig {
    pub level: DiagnosticsLevel,
}

impl Default for DiagnosticsConfig {
    fn default() -> Self {
        Self {
            level: DiagnosticsLevel::Basic,
        }
    }
}

impl DiagnosticsConfig {
    pub fn summary(&self) -> &'static str {
        self.level.as_str()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RenderConfig {
    pub render_backend: RenderBackend,
    pub shadow_quality: ShadowQuality,
    pub anti_aliasing: AntiAliasing,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            render_backend: RenderBackend::Vulkan,
            shadow_quality: ShadowQuality::Medium,
            anti_aliasing: AntiAliasing::None,
        }
    }
}

impl RenderConfig {
    pub fn summary(&self) -> String {
        format!(
            "backend={}, shadows={}, aa={}",
            self.render_backend.as_str(),
            self.shadow_quality.as_str(),
            self.anti_aliasing.as_str()
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AudioConfig {
    pub audio_backend: AudioBackend,
    pub master_volume: Volume,
    pub music_volume: Volume,
    pub sfx_volume: Volume,
    pub spatial_audio: SpatialAudio,
    pub output_sample_rate: SampleRate,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            audio_backend: AudioBackend::XAudio2,
            master_volume: Volume::from_percent(100),
            music_volume: Volume::from_percent(80),
            sfx_volume: Volume::from_percent(90),
            spatial_audio: SpatialAudio::Disabled,
            output_sample_rate: SampleRate::from_hz(48_000),
        }
    }
}

impl AudioConfig {
    pub fn summary(&self) -> String {
        format!(
            "backend={}, master={}%, music={}%, sfx={}%, spatial={}, sample_rate={}Hz",
            self.audio_backend.as_str(),
            self.master_volume.percent,
            self.music_volume.percent,
            self.sfx_volume.percent,
            self.spatial_audio.as_str(),
            self.output_sample_rate.hz
        )
    }
}

pub type Text = &'static str;

#[derive(Debug, Clone, Copy)]
pub struct WindowConfig {
    pub title: Text,
    pub window_mode: WindowMode,
    pub cursor_mode: CursorMode,
    pub resizable: Resizable,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Parameter Object Pattern Demo",
            window_mode: WindowMode::Windowed,
            cursor_mode: CursorMode::Visible,
            resizable: Resizable::Enabled,
        }
    }
}

impl WindowConfig {
    pub fn summary(&self) -> String {
        format!(
            "title='{}', mode={}, cursor={}, resizable={}",
            self.title.to_string(),
            self.window_mode.as_str(),
            self.cursor_mode.as_str(),
            self.resizable.as_str()
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub fn full_hd() -> Self {
        Self {
            width: 1920,
            height: 1080,
        }
    }

    pub fn qhd() -> Self {
        Self {
            width: 2560,
            height: 1440,
        }
    }

    pub fn uhd4k() -> Self {
        Self {
            width: 3840,
            height: 2160,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Framerate {
    pub fps: u16,
}

impl Framerate {
    pub fn from_fps(fps: u16) -> Self {
        Self { fps }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Volume {
    pub percent: u8,
}

impl Volume {
    pub fn from_percent(percent: u8) -> Self {
        Self { percent }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SampleRate {
    pub hz: u32,
}

impl SampleRate {
    pub fn from_hz(hz: u32) -> Self {
        Self { hz }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RenderBackend {
    Vulkan,
    DirectX12,
}

impl RenderBackend {
    fn as_str(self) -> &'static str {
        match self {
            Self::Vulkan => "Vulkan",
            Self::DirectX12 => "DirectX12",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ShadowQuality {
    Medium,
    High,
}

impl ShadowQuality {
    fn as_str(self) -> &'static str {
        match self {
            Self::Medium => "Medium",
            Self::High => "High",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AntiAliasing {
    None,
    Msaa4,
}

impl AntiAliasing {
    fn as_str(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Msaa4 => "MSAAx4",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DiagnosticsLevel {
    Basic,
    Full,
}

impl DiagnosticsLevel {
    fn as_str(self) -> &'static str {
        match self {
            Self::Basic => "Basic",
            Self::Full => "Full",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VSync {
    Disabled,
    Enabled,
}

impl VSync {
    fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "Off",
            Self::Enabled => "On",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AudioBackend {
    XAudio2,
    Wasapi,
}

impl AudioBackend {
    fn as_str(self) -> &'static str {
        match self {
            Self::XAudio2 => "XAudio2",
            Self::Wasapi => "WASAPI",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SpatialAudio {
    Disabled,
    Enabled,
}

impl SpatialAudio {
    fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "Off",
            Self::Enabled => "On",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WindowMode {
    Windowed,
    Borderless,
}

impl WindowMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Windowed => "Windowed",
            Self::Borderless => "Borderless",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Resizable {
    Disabled,
    Enabled,
}

impl Resizable {
    fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "No",
            Self::Enabled => "Yes",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CursorMode {
    Visible,
    Captured,
}

impl CursorMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Visible => "Visible",
            Self::Captured => "Captured",
        }
    }
}

pub fn validate_config(config: AppConfig) -> Result<(), String> {
    if config.display.resolution.width == 0 || config.display.resolution.height == 0 {
        return Err("resolution must be greater than 0".to_string());
    }

    if config.display.refresh_rate.fps == 0 {
        return Err("refresh_rate must be greater than 0".to_string());
    }

    if config.audio.master_volume.percent > 100
        || config.audio.music_volume.percent > 100
        || config.audio.sfx_volume.percent > 100
    {
        return Err("volume percent must be in the range 0..=100".to_string());
    }

    if config.audio.output_sample_rate.hz < 8_000 {
        return Err("sample rate must be at least 8000 Hz".to_string());
    }

    if config.window.title.trim().is_empty() {
        return Err("window title cannot be empty".to_string());
    }

    Ok(())
}
