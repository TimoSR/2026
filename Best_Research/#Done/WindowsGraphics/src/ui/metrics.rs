use diagnostics::performance_metrics::PerformanceSample;
use graphics::GraphicsPerformanceMetrics;
use gui::{ImmediateModeGui, ScreenRelativePosition};

// public types
/// The demo's performance-metrics panel.
pub struct PerformanceMetricsPanel
{
    text: String,
}
// public types

// domain constants
const METRICS_PANEL_SCREEN_POSITION: ScreenRelativePosition = [
    16.0 / 1_920.0,
    16.0 / 1_080.0,
];
const BYTES_PER_MEBIBYTE: f64 = 1_048_576.0;
const MINIMUM_FRAME_TIME_IN_MILLISECONDS: f32 = 0.001;
// domain constants

impl PerformanceMetricsPanel
{
    /// Creates a panel from the latest process and renderer performance data.
    pub fn new(
        performance_sample: &PerformanceSample,
        graphics_performance_metrics: &GraphicsPerformanceMetrics,
    ) -> Self
    {
        let gpu_usage_text = gpu_usage_text(performance_sample, graphics_performance_metrics);
        let graphics_memory_text = graphics_memory_text(graphics_performance_metrics);
        let antialiasing_text = antialiasing_text(graphics_performance_metrics);
        let text = format!(
            "FPS: {:.1}\nFrame time: {:.2} ms\nProcess CPU: {:.1}%\n{}\n{}\n{}\nObjects: {}\nPress Tab to hide metrics",
            performance_sample.frames_per_second,
            performance_sample.frame_time_in_milliseconds,
            performance_sample.process_cpu_usage_percentage,
            gpu_usage_text,
            graphics_memory_text,
            antialiasing_text,
            graphics_performance_metrics.loaded_object_count,
        );

        return Self { text };
    }

    /// Emits this panel into the current immediate-mode UI frame.
    pub fn draw(&self, user_interface: &mut ImmediateModeGui)
    {
        user_interface.add_text_panel(METRICS_PANEL_SCREEN_POSITION, &self.text);
    }
}

fn gpu_usage_text(
    performance_sample: &PerformanceSample,
    graphics_performance_metrics: &GraphicsPerformanceMetrics,
) -> String
{
    let gpu_frame_time_in_milliseconds = match graphics_performance_metrics.gpu_frame_time_in_milliseconds
    {
        Some(gpu_frame_time_in_milliseconds) => gpu_frame_time_in_milliseconds,
        None => return String::from("GPU: collecting timing"),
    };
    let frame_time_in_milliseconds = performance_sample
        .frame_time_in_milliseconds
        .max(MINIMUM_FRAME_TIME_IN_MILLISECONDS);
    let gpu_frame_percentage = gpu_frame_time_in_milliseconds / frame_time_in_milliseconds * 100.0;

    return format!(
        "GPU: {:.2} MS {:.0}% FRAME",
        gpu_frame_time_in_milliseconds,
        gpu_frame_percentage,
    );
}

fn graphics_memory_text(graphics_performance_metrics: &GraphicsPerformanceMetrics) -> String
{
    let graphics_memory = match &graphics_performance_metrics.graphics_memory
    {
        Some(graphics_memory) => graphics_memory,
        None => return String::from("GPU memory: unavailable"),
    };
    let used_memory_in_mebibytes = graphics_memory.used_bytes as f64 / BYTES_PER_MEBIBYTE;
    let budget_memory_in_mebibytes = graphics_memory.budget_bytes as f64 / BYTES_PER_MEBIBYTE;

    return format!(
        "GPU memory: {:.0} / {:.0} MiB",
        used_memory_in_mebibytes,
        budget_memory_in_mebibytes,
    );
}

fn antialiasing_text(graphics_performance_metrics: &GraphicsPerformanceMetrics) -> String
{
    let multisample_antialiasing_text = if graphics_performance_metrics.is_multisample_antialiasing_enabled
    {
        "On"
    }
    else
    {
        "Off"
    };
    let temporal_antialiasing_text = if graphics_performance_metrics.is_temporal_antialiasing_enabled
    {
        "On"
    }
    else
    {
        "Off"
    };

    return format!(
        "MSAA: {} | TAA: {}",
        multisample_antialiasing_text,
        temporal_antialiasing_text,
    );
}

#[cfg(test)]
mod tests
{
    use super::*;
    use graphics::{GraphicsMemoryMetrics, GraphicsUserInterface};

    #[test]
    fn performance_metrics_panel_formats_available_graphics_and_process_data()
    {
        let performance_sample = PerformanceSample {
            frames_per_second: 120.0,
            frame_time_in_milliseconds: 8.0,
            process_cpu_usage_percentage: 15.0,
        };
        let graphics_performance_metrics = GraphicsPerformanceMetrics {
            gpu_frame_time_in_milliseconds: Some(4.0),
            graphics_memory: Some(GraphicsMemoryMetrics {
                used_bytes: 128 * 1_048_576,
                budget_bytes: 1024 * 1_048_576,
            }),
            is_multisample_antialiasing_enabled: true,
            is_temporal_antialiasing_enabled: false,
            loaded_object_count: 4,
        };
        let performance_metrics_panel = PerformanceMetricsPanel::new(
            &performance_sample,
            &graphics_performance_metrics,
        );

        assert!(performance_metrics_panel.text.contains("FPS: 120.0"));
        assert!(performance_metrics_panel.text.contains("GPU: 4.00 MS 50% FRAME"));
        assert!(performance_metrics_panel.text.contains("GPU memory: 128 / 1024 MiB"));
        assert!(performance_metrics_panel.text.contains("MSAA: On | TAA: Off"));
    }

    #[test]
    fn performance_metrics_panel_reports_unavailable_graphics_data()
    {
        let performance_sample = PerformanceSample {
            frames_per_second: 0.0,
            frame_time_in_milliseconds: 0.0,
            process_cpu_usage_percentage: 0.0,
        };
        let graphics_performance_metrics = GraphicsPerformanceMetrics {
            gpu_frame_time_in_milliseconds: None,
            graphics_memory: None,
            is_multisample_antialiasing_enabled: false,
            is_temporal_antialiasing_enabled: false,
            loaded_object_count: 0,
        };
        let performance_metrics_panel = PerformanceMetricsPanel::new(
            &performance_sample,
            &graphics_performance_metrics,
        );

        assert!(performance_metrics_panel.text.contains("GPU: collecting timing"));
        assert!(performance_metrics_panel.text.contains("GPU memory: unavailable"));
        assert!(performance_metrics_panel.text.contains("MSAA: Off | TAA: Off"));
    }

    #[test]
    fn performance_metrics_panel_emits_draw_data_at_its_screen_relative_position()
    {
        let performance_sample = PerformanceSample {
            frames_per_second: 120.0,
            frame_time_in_milliseconds: 8.0,
            process_cpu_usage_percentage: 15.0,
        };
        let graphics_performance_metrics = GraphicsPerformanceMetrics {
            gpu_frame_time_in_milliseconds: None,
            graphics_memory: None,
            is_multisample_antialiasing_enabled: false,
            is_temporal_antialiasing_enabled: false,
            loaded_object_count: 0,
        };
        let performance_metrics_panel = PerformanceMetricsPanel::new(
            &performance_sample,
            &graphics_performance_metrics,
        );
        let mut user_interface = ImmediateModeGui::new(1_920, 1_080);

        performance_metrics_panel.draw(&mut user_interface);

        let vertices = user_interface.vertices();

        assert!(!vertices.is_empty());
        assert_eq!(vertices[0].position, [-0.983_333_35, 0.970_370_35]);
    }
}
