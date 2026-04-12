using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;

namespace FluentGraphics3D;

public enum RenderBackend
{
    DirectX12,
    Vulkan,
    Metal,
    OpenGL
}

public enum ShadowQuality
{
    Off,
    Low,
    Medium,
    High,
    Ultra
}

public enum AntiAliasingMode
{
    None,
    FXAA,
    TAA,
    MSAA
}

public enum DiagnosticsLevel
{
    Off,
    Overlay,
    Validation,
    Full
}

public enum RenderProfile
{
    Balanced,
    Performance,
    Cinematic,
    Development
}

public readonly record struct Resolution
{
    public Resolution(int width, int height)
    {
        if (width <= 0)
        {
            throw new ArgumentOutOfRangeException(nameof(width), width, "Width must be greater than zero.");
        }

        if (height <= 0)
        {
            throw new ArgumentOutOfRangeException(nameof(height), height, "Height must be greater than zero.");
        }

        Width = width;
        Height = height;
    }

    public int Width { get; }
    public int Height { get; }

    public static Resolution HD => new(1280, 720);
    public static Resolution FullHD => new(1920, 1080);
    public static Resolution QHD => new(2560, 1440);
    public static Resolution UHD4K => new(3840, 2160);

    public override string ToString()
    {
        return $"{Width}x{Height}";
    }
}

public readonly record struct AntiAliasing
{
    public AntiAliasing(AntiAliasingMode mode, int msaaSamples = 1)
    {
        RenderValidation.EnsureEnumDefined(mode, nameof(mode));

        if (mode == AntiAliasingMode.MSAA)
        {
            if (msaaSamples != 2 && msaaSamples != 4 && msaaSamples != 8 && msaaSamples != 16)
            {
                throw new ArgumentOutOfRangeException(
                    nameof(msaaSamples),
                    msaaSamples,
                    "MSAA samples must be one of: 2, 4, 8, or 16.");
            }
        }
        else
        {
            msaaSamples = 1;
        }

        Mode = mode;
        MsaaSamples = msaaSamples;
    }

    public AntiAliasingMode Mode { get; }
    public int MsaaSamples { get; }

    public static AntiAliasing None => new(AntiAliasingMode.None);
    public static AntiAliasing FXAA => new(AntiAliasingMode.FXAA);
    public static AntiAliasing TAA => new(AntiAliasingMode.TAA);

    public static AntiAliasing MSAA(int samples)
    {
        return new AntiAliasing(AntiAliasingMode.MSAA, samples);
    }
}

public readonly record struct RenderQuality
{
    public RenderQuality(ShadowQuality shadowQuality, AntiAliasing antiAliasing)
    {
        RenderValidation.EnsureEnumDefined(shadowQuality, nameof(shadowQuality));

        ShadowQuality = shadowQuality;
        AntiAliasing = antiAliasing;
    }

    public ShadowQuality ShadowQuality { get; }
    public AntiAliasing AntiAliasing { get; }

    public static RenderQuality Default => new(ShadowQuality.Medium, AntiAliasing.None);
    public static RenderQuality Balanced => new(ShadowQuality.Medium, AntiAliasing.TAA);
}

public readonly record struct RenderPerformance
{
    public RenderPerformance(int targetFramesPerSecond = 60, bool vSyncEnabled = false)
    {
        if (targetFramesPerSecond <= 0)
        {
            throw new ArgumentOutOfRangeException(
                nameof(targetFramesPerSecond),
                targetFramesPerSecond,
                "Target FPS must be greater than zero.");
        }

        TargetFramesPerSecond = targetFramesPerSecond;
        VSyncEnabled = vSyncEnabled;
    }

    public int TargetFramesPerSecond { get; }
    public bool VSyncEnabled { get; }

    public static RenderPerformance Default => new(60, false);
}

public sealed record RenderOptions(
    bool RenderingEnabled,
    RenderBackend Backend,
    int Width,
    int Height,
    bool VSyncEnabled,
    ShadowQuality ShadowQuality,
    AntiAliasingMode AntiAliasingMode,
    int MsaaSamples,
    int TargetFramesPerSecond,
    bool DebugOverlayEnabled,
    bool GpuValidationEnabled,
    IReadOnlyDictionary<string, string> Metadata);

public sealed record RenderProfileDescription(
    RenderProfile Profile,
    string IntendedUse,
    RenderOptions Options);

public sealed class RenderEngineConfiguration
{
    public RenderEngineConfiguration(
        RenderBackend renderBackend,
        Resolution resolution,
        ShadowQuality shadowQuality = ShadowQuality.Medium,
        AntiAliasing? antiAliasing = null,
        int targetFramesPerSecond = 60,
        bool vSyncEnabled = false,
        DiagnosticsLevel diagnosticsLevel = DiagnosticsLevel.Off,
        bool renderingEnabled = true,
        IReadOnlyDictionary<string, string>? metadata = null)
        : this(
            renderBackend: renderBackend,
            resolution: resolution,
            renderQuality: new RenderQuality(
                shadowQuality: shadowQuality,
                antiAliasing: antiAliasing ?? AntiAliasing.None),
            renderPerformance: new RenderPerformance(
                targetFramesPerSecond: targetFramesPerSecond,
                vSyncEnabled: vSyncEnabled),
            diagnosticsLevel: diagnosticsLevel,
            renderingEnabled: renderingEnabled,
            metadata: metadata)
    {
    }

    public RenderEngineConfiguration(
        RenderBackend renderBackend,
        Resolution resolution,
        RenderQuality? renderQuality = null,
        RenderPerformance? renderPerformance = null,
        DiagnosticsLevel diagnosticsLevel = DiagnosticsLevel.Off,
        bool renderingEnabled = true,
        IReadOnlyDictionary<string, string>? metadata = null)
    {
        RenderValidation.EnsureEnumDefined(renderBackend, nameof(renderBackend));
        RenderValidation.EnsureEnumDefined(diagnosticsLevel, nameof(diagnosticsLevel));
        RenderValidation.ValidateResolution(resolution.Width, resolution.Height);

        RenderQuality normalizedQuality = renderQuality ?? RenderQuality.Default;
        normalizedQuality = new RenderQuality(
            normalizedQuality.ShadowQuality,
            new AntiAliasing(
                normalizedQuality.AntiAliasing.Mode,
                normalizedQuality.AntiAliasing.MsaaSamples));

        RenderPerformance normalizedPerformance = renderPerformance ?? RenderPerformance.Default;
        normalizedPerformance = new RenderPerformance(
            normalizedPerformance.TargetFramesPerSecond,
            normalizedPerformance.VSyncEnabled);

        RenderingEnabled = renderingEnabled;
        Backend = renderBackend;
        Resolution = resolution;
        Quality = normalizedQuality;
        Performance = normalizedPerformance;
        Diagnostics = diagnosticsLevel;
        Metadata = new ReadOnlyDictionary<string, string>(
            metadata is null
                ? new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase)
                : new Dictionary<string, string>(metadata, StringComparer.OrdinalIgnoreCase));
    }

    public bool RenderingEnabled { get; }
    public RenderBackend Backend { get; }
    public Resolution Resolution { get; }
    public RenderQuality Quality { get; }
    public RenderPerformance Performance { get; }
    public DiagnosticsLevel Diagnostics { get; }
    public IReadOnlyDictionary<string, string> Metadata { get; }

    public int Width => Resolution.Width;
    public int Height => Resolution.Height;
    public bool VSyncEnabled => Performance.VSyncEnabled;
    public int TargetFramesPerSecond => Performance.TargetFramesPerSecond;
    public ShadowQuality ShadowQuality => Quality.ShadowQuality;
    public AntiAliasing AntiAliasing => Quality.AntiAliasing;
    public AntiAliasingMode AntiAliasingMode => AntiAliasing.Mode;
    public int MsaaSamples => AntiAliasing.MsaaSamples;
    public bool DebugOverlayEnabled =>
        Diagnostics == DiagnosticsLevel.Overlay || Diagnostics == DiagnosticsLevel.Full;
    public bool GpuValidationEnabled =>
        Diagnostics == DiagnosticsLevel.Validation || Diagnostics == DiagnosticsLevel.Full;

    public static RenderEngineConfiguration FromProfile(RenderProfile profile)
    {
        return FromOptions(RenderProfiles.GetOptions(profile));
    }

    internal static RenderEngineConfiguration FromOptions(RenderOptions options)
    {
        ArgumentNullException.ThrowIfNull(options);

        return new RenderEngineConfiguration(
            renderBackend: options.Backend,
            resolution: new Resolution(options.Width, options.Height),
            renderQuality: new RenderQuality(
                options.ShadowQuality,
                new AntiAliasing(options.AntiAliasingMode, options.MsaaSamples)),
            renderPerformance: new RenderPerformance(
                options.TargetFramesPerSecond,
                options.VSyncEnabled),
            diagnosticsLevel: ToDiagnosticsLevel(options.DebugOverlayEnabled, options.GpuValidationEnabled),
            renderingEnabled: options.RenderingEnabled,
            metadata: options.Metadata);
    }

    private static DiagnosticsLevel ToDiagnosticsLevel(bool debugOverlayEnabled, bool gpuValidationEnabled)
    {
        if (debugOverlayEnabled && gpuValidationEnabled)
        {
            return DiagnosticsLevel.Full;
        }

        if (debugOverlayEnabled)
        {
            return DiagnosticsLevel.Overlay;
        }

        if (gpuValidationEnabled)
        {
            return DiagnosticsLevel.Validation;
        }

        return DiagnosticsLevel.Off;
    }
}

public sealed class RenderEngine : IDisposable
{
    private bool _started;
    private bool _disposed;

    public RenderEngine(RenderOptions options)
    {
        Options = options ?? throw new ArgumentNullException(nameof(options));
    }

    public RenderOptions Options { get; }

    public void Start()
    {
        ThrowIfDisposed();

        if (_started)
        {
            throw new InvalidOperationException("The render engine has already been started.");
        }

        if (!Options.RenderingEnabled)
        {
            throw new InvalidOperationException("Rendering is disabled in the configured options.");
        }

        _started = true;

        Console.WriteLine(
            $"Renderer started. " +
            $"Backend={Options.Backend}, " +
            $"Resolution={Options.Width}x{Options.Height}, " +
            $"VSync={Options.VSyncEnabled}, " +
            $"Shadows={Options.ShadowQuality}, " +
            $"AA={Options.AntiAliasingMode}, " +
            $"MSAA={Options.MsaaSamples}, " +
            $"TargetFPS={Options.TargetFramesPerSecond}, " +
            $"DebugOverlay={Options.DebugOverlayEnabled}, " +
            $"GpuValidation={Options.GpuValidationEnabled}");
    }

    public void Stop()
    {
        ThrowIfDisposed();

        if (!_started)
        {
            return;
        }

        _started = false;
        Console.WriteLine("Renderer stopped.");
    }

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }

        Stop();
        _disposed = true;
    }

    private void ThrowIfDisposed()
    {
        if (_disposed)
        {
            throw new ObjectDisposedException(nameof(RenderEngine));
        }
    }
}

public static class Graphics3D
{
    public static IReadOnlyList<RenderProfileDescription> Profiles => RenderProfiles.Available;

    public static RenderEngine Default()
    {
        return Default(RenderProfile.Balanced);
    }

    public static RenderEngine Default(RenderProfile profile)
    {
        return new RenderEngine(RenderProfiles.GetOptions(profile));
    }

    public static RenderEngine Configure(RenderEngineConfiguration configuration)
    {
        ArgumentNullException.ThrowIfNull(configuration);
        RenderOptions options = RenderValidation.BuildOptions(configuration);
        return new RenderEngine(options);
    }
}

internal static class RenderProfiles
{
    private static readonly IReadOnlyList<RenderProfileDescription> _available = Build();

    public static IReadOnlyList<RenderProfileDescription> Available => _available;

    public static RenderOptions GetOptions(RenderProfile profile)
    {
        RenderValidation.EnsureEnumDefined(profile, nameof(profile));

        foreach (RenderProfileDescription description in _available)
        {
            if (description.Profile == profile)
            {
                return description.Options;
            }
        }

        throw new ArgumentOutOfRangeException(nameof(profile), profile, "Unknown render profile.");
    }

    private static IReadOnlyList<RenderProfileDescription> Build()
    {
        List<RenderProfileDescription> profiles = new()
        {
            new RenderProfileDescription(
                RenderProfile.Balanced,
                "General gameplay across typical desktop hardware.",
                CreateOptions(
                    backend: RenderBackend.Vulkan,
                    resolution: Resolution.FullHD,
                    quality: RenderQuality.Balanced,
                    performance: new RenderPerformance(targetFramesPerSecond: 60, vSyncEnabled: true),
                    diagnostics: DiagnosticsLevel.Off)),
            new RenderProfileDescription(
                RenderProfile.Performance,
                "High FPS for competitive gameplay.",
                CreateOptions(
                    backend: RenderBackend.DirectX12,
                    resolution: Resolution.FullHD,
                    quality: new RenderQuality(ShadowQuality.Low, AntiAliasing.FXAA),
                    performance: new RenderPerformance(targetFramesPerSecond: 144, vSyncEnabled: false),
                    diagnostics: DiagnosticsLevel.Off)),
            new RenderProfileDescription(
                RenderProfile.Cinematic,
                "Visual quality focused scenes and demos.",
                CreateOptions(
                    backend: RenderBackend.Vulkan,
                    resolution: Resolution.QHD,
                    quality: new RenderQuality(ShadowQuality.Ultra, AntiAliasing.TAA),
                    performance: new RenderPerformance(targetFramesPerSecond: 60, vSyncEnabled: true),
                    diagnostics: DiagnosticsLevel.Off)),
            new RenderProfileDescription(
                RenderProfile.Development,
                "Debugging and validation while building features.",
                CreateOptions(
                    backend: RenderBackend.OpenGL,
                    resolution: new Resolution(1600, 900),
                    quality: new RenderQuality(ShadowQuality.Off, AntiAliasing.None),
                    performance: new RenderPerformance(targetFramesPerSecond: 60, vSyncEnabled: false),
                    diagnostics: DiagnosticsLevel.Full))
        };

        return profiles.AsReadOnly();
    }

    private static RenderOptions CreateOptions(
        RenderBackend backend,
        Resolution resolution,
        RenderQuality quality,
        RenderPerformance performance,
        DiagnosticsLevel diagnostics)
    {
        RenderEngineConfiguration configuration = new(
            renderBackend: backend,
            resolution: resolution,
            renderQuality: quality,
            renderPerformance: performance,
            diagnosticsLevel: diagnostics,
            renderingEnabled: true);

        return RenderValidation.BuildOptions(configuration);
    }
}

internal static class RenderValidation
{
    public static RenderOptions BuildOptions(RenderEngineConfiguration configuration)
    {
        ArgumentNullException.ThrowIfNull(configuration);

        EnsureEnumDefined(configuration.Backend, nameof(configuration.Backend));
        EnsureEnumDefined(configuration.ShadowQuality, nameof(configuration.ShadowQuality));
        EnsureEnumDefined(configuration.AntiAliasing.Mode, nameof(configuration.AntiAliasing.Mode));
        EnsureEnumDefined(configuration.Diagnostics, nameof(configuration.Diagnostics));
        ValidateResolution(configuration.Resolution.Width, configuration.Resolution.Height);

        if (configuration.TargetFramesPerSecond <= 0)
        {
            throw new InvalidOperationException("Target FPS must be greater than zero.");
        }

        if (configuration.AntiAliasing.Mode == AntiAliasingMode.MSAA)
        {
            if (configuration.AntiAliasing.MsaaSamples != 2 &&
                configuration.AntiAliasing.MsaaSamples != 4 &&
                configuration.AntiAliasing.MsaaSamples != 8 &&
                configuration.AntiAliasing.MsaaSamples != 16)
            {
                throw new InvalidOperationException("MSAA samples must be one of: 2, 4, 8, or 16.");
            }
        }
        else if (configuration.AntiAliasing.MsaaSamples != 1)
        {
            throw new InvalidOperationException(
                "MSAA sample count can only be customized when AntiAliasingMode is MSAA.");
        }

        (bool debugOverlayEnabled, bool gpuValidationEnabled) = configuration.Diagnostics switch
        {
            DiagnosticsLevel.Off => (false, false),
            DiagnosticsLevel.Overlay => (true, false),
            DiagnosticsLevel.Validation => (false, true),
            DiagnosticsLevel.Full => (true, true),
            _ => throw new InvalidOperationException("Unknown diagnostics level.")
        };

        IReadOnlyDictionary<string, string> metadataSnapshot =
            new ReadOnlyDictionary<string, string>(
                new Dictionary<string, string>(configuration.Metadata, StringComparer.OrdinalIgnoreCase));

        return new RenderOptions(
            RenderingEnabled: configuration.RenderingEnabled,
            Backend: configuration.Backend,
            Width: configuration.Resolution.Width,
            Height: configuration.Resolution.Height,
            VSyncEnabled: configuration.VSyncEnabled,
            ShadowQuality: configuration.ShadowQuality,
            AntiAliasingMode: configuration.AntiAliasing.Mode,
            MsaaSamples: configuration.AntiAliasing.MsaaSamples,
            TargetFramesPerSecond: configuration.TargetFramesPerSecond,
            DebugOverlayEnabled: debugOverlayEnabled,
            GpuValidationEnabled: gpuValidationEnabled,
            Metadata: metadataSnapshot);
    }

    public static void ValidateResolution(int width, int height)
    {
        if (width <= 0)
        {
            throw new ArgumentOutOfRangeException(nameof(width), width, "Width must be greater than zero.");
        }

        if (height <= 0)
        {
            throw new ArgumentOutOfRangeException(nameof(height), height, "Height must be greater than zero.");
        }
    }

    public static void EnsureEnumDefined<TEnum>(TEnum value, string parameterName)
        where TEnum : struct, Enum
    {
        if (!Enum.IsDefined(typeof(TEnum), value))
        {
            throw new ArgumentOutOfRangeException(
                parameterName,
                value,
                $"Invalid value for {typeof(TEnum).Name}.");
        }
    }
}

public static class Demo
{
    public static void Main()
    {
        RenderEngine defaultEngine = Graphics3D.Default();
        try
        {
            defaultEngine.Start();
            defaultEngine.Stop();
        }
        finally
        {
            defaultEngine.Dispose();
        }

        RenderEngineConfiguration customConfiguration = new(
            renderBackend: RenderBackend.Vulkan,
            resolution: Resolution.QHD,
            shadowQuality: ShadowQuality.High,
            antiAliasing: AntiAliasing.MSAA(4),
            targetFramesPerSecond: 144,
            vSyncEnabled: true,
            diagnosticsLevel: DiagnosticsLevel.Full);

        RenderEngine customEngine = Graphics3D.Configure(customConfiguration);
        try
        {
            customEngine.Start();
            customEngine.Stop();
        }
        finally
        {
            customEngine.Dispose();
        }
    }
}
