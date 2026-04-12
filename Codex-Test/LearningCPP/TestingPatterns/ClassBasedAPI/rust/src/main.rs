use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RenderBackend {
    DirectX12,
    Vulkan,
    Metal,
    OpenGL,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ShadowQuality {
    Off,
    Low,
    Medium,
    High,
    Ultra,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AntiAliasingMode {
    None,
    Fxaa,
    Taa,
    Msaa,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DiagnosticsLevel {
    Off,
    Overlay,
    Validation,
    Full,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RenderProfile {
    Balanced,
    Performance,
    Cinematic,
    Development,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Resolution {
    width: i32,
    height: i32,
}

impl Resolution {
    fn full_hd() -> Self {
        Self {
            width: 1920,
            height: 1080,
        }
    }

    fn qhd() -> Self {
        Self {
            width: 2560,
            height: 1440,
        }
    }

    fn uhd_4k() -> Self {
        Self {
            width: 3840,
            height: 2160,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AntiAliasing {
    mode: AntiAliasingMode,
    msaa_samples: i32,
}

impl AntiAliasing {
    fn none() -> Self {
        Self {
            mode: AntiAliasingMode::None,
            msaa_samples: 1,
        }
    }

    fn fxaa() -> Self {
        Self {
            mode: AntiAliasingMode::Fxaa,
            msaa_samples: 1,
        }
    }

    fn taa() -> Self {
        Self {
            mode: AntiAliasingMode::Taa,
            msaa_samples: 1,
        }
    }

    fn msaa(samples: i32) -> Self {
        Self {
            mode: AntiAliasingMode::Msaa,
            msaa_samples: samples,
        }
    }
}

#[derive(Clone, Debug)]
struct RenderOptions {
    rendering_enabled: bool,
    backend: RenderBackend,
    width: i32,
    height: i32,
    v_sync_enabled: bool,
    shadow_quality: ShadowQuality,
    anti_aliasing_mode: AntiAliasingMode,
    msaa_samples: i32,
    target_frames_per_second: i32,
    debug_overlay_enabled: bool,
    gpu_validation_enabled: bool,
    metadata: BTreeMap<String, String>,
}

#[derive(Clone, Debug)]
struct RenderProfileDescription {
    profile: RenderProfile,
    intended_use: &'static str,
    options: RenderOptions,
}

#[derive(Clone, Debug)]
struct RenderEngineConfiguration {
    render_backend: RenderBackend,
    resolution: Resolution,
    shadow_quality: ShadowQuality,
    anti_aliasing: AntiAliasing,
    target_frames_per_second: i32,
    v_sync_enabled: bool,
    diagnostics_level: DiagnosticsLevel,
    rendering_enabled: bool,
    metadata: BTreeMap<String, String>,
}

impl Default for RenderEngineConfiguration {
    fn default() -> Self {
        Self {
            render_backend: RenderBackend::Vulkan,
            resolution: Resolution::full_hd(),
            shadow_quality: ShadowQuality::Medium,
            anti_aliasing: AntiAliasing::none(),
            target_frames_per_second: 60,
            v_sync_enabled: false,
            diagnostics_level: DiagnosticsLevel::Off,
            rendering_enabled: true,
            metadata: BTreeMap::new(),
        }
    }
}

impl RenderEngineConfiguration {
    fn new(
        render_backend: RenderBackend,
        resolution: Resolution,
        shadow_quality: ShadowQuality,
        anti_aliasing: AntiAliasing,
        target_frames_per_second: i32,
        v_sync_enabled: bool,
        diagnostics_level: DiagnosticsLevel,
        rendering_enabled: bool,
    ) -> Self {
        Self {
            render_backend,
            resolution,
            shadow_quality,
            anti_aliasing,
            target_frames_per_second,
            v_sync_enabled,
            diagnostics_level,
            rendering_enabled,
            metadata: BTreeMap::new(),
        }
    }

    fn from_profile(profile: RenderProfile) -> Self {
        profile_configuration(profile)
    }
}

#[derive(Debug)]
struct RenderEngine {
    options: RenderOptions,
    started: bool,
    disposed: bool,
}

impl RenderEngine {
    fn new(options: RenderOptions) -> Self {
        Self {
            options,
            started: false,
            disposed: false,
        }
    }

    fn start(&mut self) -> Result<(), String> {
        if self.disposed {
            return Err("The render engine is disposed.".to_string());
        }

        if self.started {
            return Err("The render engine has already been started.".to_string());
        }

        if !self.options.rendering_enabled {
            return Err("Rendering is disabled in the configured options.".to_string());
        }

        self.started = true;
        println!(
            "Renderer started. Backend={}, Resolution={}x{}, VSync={}, Shadows={}, AA={}, MSAA={}, TargetFPS={}, DebugOverlay={}, GpuValidation={}",
            self.options.backend,
            self.options.width,
            self.options.height,
            self.options.v_sync_enabled,
            self.options.shadow_quality,
            self.options.anti_aliasing_mode,
            self.options.msaa_samples,
            self.options.target_frames_per_second,
            self.options.debug_overlay_enabled,
            self.options.gpu_validation_enabled
        );
        Ok(())
    }

    fn stop(&mut self) {
        if self.disposed || !self.started {
            return;
        }

        self.started = false;
        println!("Renderer stopped.");
    }

    fn dispose(&mut self) {
        if self.disposed {
            return;
        }

        self.stop();
        self.disposed = true;
    }
}

struct Graphics3D;

impl Graphics3D {
    fn profiles() -> Vec<RenderProfileDescription> {
        [
            RenderProfile::Balanced,
            RenderProfile::Performance,
            RenderProfile::Cinematic,
            RenderProfile::Development,
        ]
        .into_iter()
        .map(profile_description)
        .collect()
    }

    fn default() -> RenderEngine {
        Self::default_profile(RenderProfile::Balanced)
    }

    fn default_profile(profile: RenderProfile) -> RenderEngine {
        RenderEngine::new(profile_description(profile).options)
    }

    fn configure(configuration: RenderEngineConfiguration) -> Result<RenderEngine, String> {
        let options = build_options(&configuration)?;
        Ok(RenderEngine::new(options))
    }
}

fn profile_description(profile: RenderProfile) -> RenderProfileDescription {
    let (intended_use, configuration) = match profile {
        RenderProfile::Balanced => (
            "General gameplay across typical desktop hardware.",
            RenderEngineConfiguration {
                render_backend: RenderBackend::Vulkan,
                resolution: Resolution::full_hd(),
                shadow_quality: ShadowQuality::Medium,
                anti_aliasing: AntiAliasing::taa(),
                target_frames_per_second: 60,
                v_sync_enabled: true,
                diagnostics_level: DiagnosticsLevel::Off,
                ..RenderEngineConfiguration::default()
            },
        ),
        RenderProfile::Performance => (
            "High FPS for competitive gameplay.",
            RenderEngineConfiguration {
                render_backend: RenderBackend::DirectX12,
                resolution: Resolution::full_hd(),
                shadow_quality: ShadowQuality::Low,
                anti_aliasing: AntiAliasing::fxaa(),
                target_frames_per_second: 144,
                v_sync_enabled: false,
                diagnostics_level: DiagnosticsLevel::Off,
                ..RenderEngineConfiguration::default()
            },
        ),
        RenderProfile::Cinematic => (
            "Visual quality focused scenes and demos.",
            RenderEngineConfiguration {
                render_backend: RenderBackend::Vulkan,
                resolution: Resolution::qhd(),
                shadow_quality: ShadowQuality::Ultra,
                anti_aliasing: AntiAliasing::taa(),
                target_frames_per_second: 60,
                v_sync_enabled: true,
                diagnostics_level: DiagnosticsLevel::Off,
                ..RenderEngineConfiguration::default()
            },
        ),
        RenderProfile::Development => (
            "Debugging and validation while building features.",
            RenderEngineConfiguration {
                render_backend: RenderBackend::OpenGL,
                resolution: Resolution {
                    width: 1600,
                    height: 900,
                },
                shadow_quality: ShadowQuality::Off,
                anti_aliasing: AntiAliasing::none(),
                target_frames_per_second: 60,
                v_sync_enabled: false,
                diagnostics_level: DiagnosticsLevel::Full,
                ..RenderEngineConfiguration::default()
            },
        ),
    };

    RenderProfileDescription {
        profile,
        intended_use,
        options: build_options(&configuration).expect("Internal profile configuration must be valid."),
    }
}

fn profile_configuration(profile: RenderProfile) -> RenderEngineConfiguration {
    let options = profile_description(profile).options;
    RenderEngineConfiguration {
        render_backend: options.backend,
        resolution: Resolution {
            width: options.width,
            height: options.height,
        },
        shadow_quality: options.shadow_quality,
        anti_aliasing: AntiAliasing {
            mode: options.anti_aliasing_mode,
            msaa_samples: options.msaa_samples,
        },
        target_frames_per_second: options.target_frames_per_second,
        v_sync_enabled: options.v_sync_enabled,
        diagnostics_level: if options.debug_overlay_enabled && options.gpu_validation_enabled {
            DiagnosticsLevel::Full
        } else if options.debug_overlay_enabled {
            DiagnosticsLevel::Overlay
        } else if options.gpu_validation_enabled {
            DiagnosticsLevel::Validation
        } else {
            DiagnosticsLevel::Off
        },
        rendering_enabled: options.rendering_enabled,
        metadata: options.metadata,
    }
}

fn build_options(configuration: &RenderEngineConfiguration) -> Result<RenderOptions, String> {
    validate_resolution(configuration.resolution)?;
    validate_anti_aliasing(configuration.anti_aliasing)?;

    if configuration.target_frames_per_second <= 0 {
        return Err("target_frames_per_second must be greater than zero.".to_string());
    }

    let (debug_overlay_enabled, gpu_validation_enabled) = match configuration.diagnostics_level {
        DiagnosticsLevel::Off => (false, false),
        DiagnosticsLevel::Overlay => (true, false),
        DiagnosticsLevel::Validation => (false, true),
        DiagnosticsLevel::Full => (true, true),
    };

    Ok(RenderOptions {
        rendering_enabled: configuration.rendering_enabled,
        backend: configuration.render_backend,
        width: configuration.resolution.width,
        height: configuration.resolution.height,
        v_sync_enabled: configuration.v_sync_enabled,
        shadow_quality: configuration.shadow_quality,
        anti_aliasing_mode: configuration.anti_aliasing.mode,
        msaa_samples: configuration.anti_aliasing.msaa_samples,
        target_frames_per_second: configuration.target_frames_per_second,
        debug_overlay_enabled,
        gpu_validation_enabled,
        metadata: configuration.metadata.clone(),
    })
}

fn validate_resolution(resolution: Resolution) -> Result<(), String> {
    if resolution.width <= 0 {
        return Err("resolution.width must be greater than zero.".to_string());
    }

    if resolution.height <= 0 {
        return Err("resolution.height must be greater than zero.".to_string());
    }

    Ok(())
}

fn validate_anti_aliasing(anti_aliasing: AntiAliasing) -> Result<(), String> {
    if anti_aliasing.mode == AntiAliasingMode::Msaa {
        match anti_aliasing.msaa_samples {
            2 | 4 | 8 | 16 => return Ok(()),
            _ => return Err("MSAA samples must be one of: 2, 4, 8, or 16.".to_string()),
        }
    }

    if anti_aliasing.msaa_samples != 1 {
        return Err("MSAA sample count can only be customized when mode is MSAA.".to_string());
    }

    Ok(())
}

impl Display for RenderBackend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RenderBackend::DirectX12 => "DirectX12",
            RenderBackend::Vulkan => "Vulkan",
            RenderBackend::Metal => "Metal",
            RenderBackend::OpenGL => "OpenGL",
        })
    }
}

impl Display for ShadowQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ShadowQuality::Off => "Off",
            ShadowQuality::Low => "Low",
            ShadowQuality::Medium => "Medium",
            ShadowQuality::High => "High",
            ShadowQuality::Ultra => "Ultra",
        })
    }
}

impl Display for AntiAliasingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            AntiAliasingMode::None => "None",
            AntiAliasingMode::Fxaa => "FXAA",
            AntiAliasingMode::Taa => "TAA",
            AntiAliasingMode::Msaa => "MSAA",
        })
    }
}

impl Display for RenderProfile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RenderProfile::Balanced => "Balanced",
            RenderProfile::Performance => "Performance",
            RenderProfile::Cinematic => "Cinematic",
            RenderProfile::Development => "Development",
        })
    }
}

fn run() -> Result<(), String> {
    for profile in Graphics3D::profiles() {
        println!("{}: {}", profile.profile, profile.intended_use);
    }

    let mut default_engine = Graphics3D::default();
    default_engine.start()?;
    default_engine.stop();
    default_engine.dispose();

    let mut custom_config_designated = RenderEngineConfiguration {
        render_backend: RenderBackend::DirectX12,
        resolution: Resolution::uhd_4k(),
        shadow_quality: ShadowQuality::Ultra,
        anti_aliasing: AntiAliasing::fxaa(),
        target_frames_per_second: 144,
        v_sync_enabled: true,
        diagnostics_level: DiagnosticsLevel::Full,
        ..RenderEngineConfiguration::default()
    };

    let mut custom2 = RenderEngineConfiguration{
        render_backend: RenderBackend::DirectX12,
        resolution: Resolution::uhd_4k()
    };
    
    custom_config_designated
        .metadata
        .insert("environment".to_string(), "production".to_string());

    let mut custom_engine_designated = Graphics3D::configure(custom_config_designated)?;
    custom_engine_designated.start()?;
    custom_engine_designated.stop();
    custom_engine_designated.dispose();

    let mut custom_config_ctor = RenderEngineConfiguration::new(
        RenderBackend::Vulkan,
        Resolution::qhd(),
        ShadowQuality::High,
        AntiAliasing::msaa(4),
        144,
        true,
        DiagnosticsLevel::Full,
        true,
    );
    custom_config_ctor
        .metadata
        .insert("scene".to_string(), "hangar".to_string());

    let mut custom_engine_ctor = Graphics3D::configure(custom_config_ctor)?;
    custom_engine_ctor.start()?;
    custom_engine_ctor.stop();
    custom_engine_ctor.dispose();

    let cloned_profile_config = RenderEngineConfiguration::from_profile(RenderProfile::Cinematic);
    let mut cloned_profile_engine = Graphics3D::configure(cloned_profile_config)?;
    cloned_profile_engine.start()?;
    cloned_profile_engine.stop();
    cloned_profile_engine.dispose();

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
