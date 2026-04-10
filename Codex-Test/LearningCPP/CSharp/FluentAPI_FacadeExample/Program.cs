using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Threading;

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
            throw new InvalidOperationException(
                "Rendering is disabled in the configured options. Call EnableRendering() or remove DisableRendering().");
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
    public static RenderEngineConfigurator UseDirectX12()
    {
        return UseBackend(RenderBackend.DirectX12);
    }

    public static RenderEngineConfigurator UseVulkan()
    {
        return UseBackend(RenderBackend.Vulkan);
    }

    public static RenderEngineConfigurator UseMetal()
    {
        return UseBackend(RenderBackend.Metal);
    }

    public static RenderEngineConfigurator UseOpenGL()
    {
        return UseBackend(RenderBackend.OpenGL);
    }

    public static RenderEngineConfigurator UseBackend(RenderBackend backend)
    {
        RenderEngineBuilderCore.EnsureEnumDefined(backend, nameof(backend));

        return new RenderEngineConfigurator(
            new RenderEngineBlueprint
            {
                Backend = backend
            });
    }
}

public sealed class RenderEngineConfigurator
{
    private readonly RenderEngineBlueprint _blueprint;
    private readonly Lazy<RenderOptions> _materializedOptions;
    private readonly Lazy<RenderEngine> _materializedEngine;

    internal RenderEngineConfigurator(RenderEngineBlueprint blueprint)
    {
        _blueprint = blueprint ?? throw new ArgumentNullException(nameof(blueprint));

        _materializedOptions = new Lazy<RenderOptions>(
            () => RenderEngineBuilderCore.BuildOptions(_blueprint),
            LazyThreadSafetyMode.ExecutionAndPublication);

        _materializedEngine = new Lazy<RenderEngine>(
            () => new RenderEngine(_materializedOptions.Value),
            LazyThreadSafetyMode.ExecutionAndPublication);
    }

    public RenderEngineConfigurator WithResolution(int width, int height)
    {
        if (width <= 0)
        {
            throw new ArgumentOutOfRangeException(nameof(width), width, "Width must be greater than zero.");
        }

        if (height <= 0)
        {
            throw new ArgumentOutOfRangeException(nameof(height), height, "Height must be greater than zero.");
        }

        return New(_blueprint with
        {
            Width = width,
            Height = height
        });
    }

    public RenderEngineConfigurator EnableRendering()
    {
        return New(_blueprint with
        {
            RenderingEnabled = true
        });
    }

    public RenderEngineConfigurator DisableRendering()
    {
        return New(_blueprint with
        {
            RenderingEnabled = false
        });
    }

    public RenderEngineConfigurator EnableVSync()
    {
        return New(_blueprint with
        {
            VSyncEnabled = true
        });
    }

    public RenderEngineConfigurator DisableVSync()
    {
        return New(_blueprint with
        {
            VSyncEnabled = false
        });
    }

    public RenderEngineConfigurator WithShadows(ShadowQuality quality)
    {
        RenderEngineBuilderCore.EnsureEnumDefined(quality, nameof(quality));

        return New(_blueprint with
        {
            ShadowQuality = quality
        });
    }

    public RenderEngineConfigurator WithAntiAliasing(AntiAliasingMode mode, int msaaSamples = 1)
    {
        RenderEngineBuilderCore.EnsureEnumDefined(mode, nameof(mode));

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

        return New(_blueprint with
        {
            AntiAliasingMode = mode,
            MsaaSamples = msaaSamples
        });
    }

    public RenderEngineConfigurator TargetFramesPerSecond(int framesPerSecond)
    {
        if (framesPerSecond <= 0)
        {
            throw new ArgumentOutOfRangeException(
                nameof(framesPerSecond),
                framesPerSecond,
                "Target FPS must be greater than zero.");
        }

        return New(_blueprint with
        {
            TargetFramesPerSecond = framesPerSecond
        });
    }

    public RenderEngineConfigurator EnableDebugOverlay()
    {
        return New(_blueprint with
        {
            DebugOverlayEnabled = true
        });
    }

    public RenderEngineConfigurator DisableDebugOverlay()
    {
        return New(_blueprint with
        {
            DebugOverlayEnabled = false
        });
    }

    public RenderEngineConfigurator EnableGpuValidation()
    {
        return New(_blueprint with
        {
            GpuValidationEnabled = true
        });
    }

    public RenderEngineConfigurator DisableGpuValidation()
    {
        return New(_blueprint with
        {
            GpuValidationEnabled = false
        });
    }

    public RenderEngineConfigurator Tag(string key, string value)
    {
        if (string.IsNullOrWhiteSpace(key))
        {
            throw new ArgumentException("Metadata key cannot be null, empty, or whitespace.", nameof(key));
        }

        ArgumentNullException.ThrowIfNull(value);

        Dictionary<string, string> metadata = new(_blueprint.Metadata, StringComparer.OrdinalIgnoreCase)
        {
            [key] = value
        };

        return New(_blueprint with
        {
            Metadata = metadata
        });
    }

    public RenderOptions ToOptions()
    {
        return _materializedOptions.Value;
    }

    public RenderEngine ToRenderEngine()
    {
        return _materializedEngine.Value;
    }

    public RenderEngine Build()
    {
        return ToRenderEngine();
    }

    public static implicit operator RenderEngine(RenderEngineConfigurator configurator)
    {
        ArgumentNullException.ThrowIfNull(configurator);
        return configurator.ToRenderEngine();
    }

    private static RenderEngineConfigurator New(RenderEngineBlueprint blueprint)
    {
        return new RenderEngineConfigurator(blueprint);
    }
}

internal sealed record RenderEngineBlueprint
{
    public RenderBackend? Backend { get; init; }
    public int? Width { get; init; }
    public int? Height { get; init; }

    public bool RenderingEnabled { get; init; } = false;
    public bool VSyncEnabled { get; init; } = false;
    public ShadowQuality ShadowQuality { get; init; } = ShadowQuality.Medium;
    public AntiAliasingMode AntiAliasingMode { get; init; } = AntiAliasingMode.None;
    public int MsaaSamples { get; init; } = 1;
    public int TargetFramesPerSecond { get; init; } = 60;
    public bool DebugOverlayEnabled { get; init; } = false;
    public bool GpuValidationEnabled { get; init; } = false;

    public Dictionary<string, string> Metadata { get; init; } =
        new(StringComparer.OrdinalIgnoreCase);
}

internal static class RenderEngineBuilderCore
{
    public static RenderOptions BuildOptions(RenderEngineBlueprint blueprint)
    {
        ArgumentNullException.ThrowIfNull(blueprint);

        if (blueprint.Backend is null)
        {
            throw new InvalidOperationException("A render backend is required.");
        }

        if (blueprint.Width is null || blueprint.Height is null)
        {
            throw new InvalidOperationException(
                "A resolution is required. Call WithResolution(width, height) before materializing the engine.");
        }

        EnsureEnumDefined(blueprint.Backend.Value, nameof(blueprint.Backend));
        EnsureEnumDefined(blueprint.ShadowQuality, nameof(blueprint.ShadowQuality));
        EnsureEnumDefined(blueprint.AntiAliasingMode, nameof(blueprint.AntiAliasingMode));

        if (blueprint.Width.Value <= 0)
        {
            throw new InvalidOperationException("Width must be greater than zero.");
        }

        if (blueprint.Height.Value <= 0)
        {
            throw new InvalidOperationException("Height must be greater than zero.");
        }

        if (blueprint.TargetFramesPerSecond <= 0)
        {
            throw new InvalidOperationException("Target FPS must be greater than zero.");
        }

        if (blueprint.AntiAliasingMode == AntiAliasingMode.MSAA)
        {
            if (blueprint.MsaaSamples != 2 &&
                blueprint.MsaaSamples != 4 &&
                blueprint.MsaaSamples != 8 &&
                blueprint.MsaaSamples != 16)
            {
                throw new InvalidOperationException(
                    "MSAA requires samples to be one of: 2, 4, 8, or 16.");
            }
        }
        else if (blueprint.MsaaSamples != 1)
        {
            throw new InvalidOperationException(
                "MSAA sample count can only be customized when AntiAliasingMode is MSAA.");
        }

        IReadOnlyDictionary<string, string> metadataSnapshot =
            new ReadOnlyDictionary<string, string>(
                new Dictionary<string, string>(blueprint.Metadata, StringComparer.OrdinalIgnoreCase));

        return new RenderOptions(
            RenderingEnabled: blueprint.RenderingEnabled,
            Backend: blueprint.Backend.Value,
            Width: blueprint.Width.Value,
            Height: blueprint.Height.Value,
            VSyncEnabled: blueprint.VSyncEnabled,
            ShadowQuality: blueprint.ShadowQuality,
            AntiAliasingMode: blueprint.AntiAliasingMode,
            MsaaSamples: blueprint.MsaaSamples,
            TargetFramesPerSecond: blueprint.TargetFramesPerSecond,
            DebugOverlayEnabled: blueprint.DebugOverlayEnabled,
            GpuValidationEnabled: blueprint.GpuValidationEnabled,
            Metadata: metadataSnapshot);
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
        RenderEngine renderer = Graphics3D
            .UseVulkan()
            .WithResolution(2560, 1440)
            .EnableRendering()
            .EnableVSync()
            .WithShadows(ShadowQuality.High)
            .WithAntiAliasing(AntiAliasingMode.MSAA, 4)
            .TargetFramesPerSecond(144)
            .EnableGpuValidation()
            .EnableDebugOverlay()
            .Tag("environment", "development")
            .Tag("scene", "hangar");

        renderer.Start();

        Console.WriteLine();
        Console.WriteLine("Materialized options snapshot:");

        renderer.Stop();
        renderer.Dispose();
    }
}
