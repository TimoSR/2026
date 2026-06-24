use diagnostics::performance_metrics::PerformanceSample;
use graphics::{
    GraphicsPerformanceMetrics,
    GraphicsUserInterface,
    GraphicsUserInterfaceShader,
    GraphicsUserInterfaceVertex,
};

// data structures
/// A minimal immediate-mode UI that emits text panels as coloured triangles.
pub struct ImmediateModeGui
{
    viewport_width: f32,
    viewport_height: f32,
    vertices: Vec<GraphicsUserInterfaceVertex>,
}
// data structures

// domain constants
const IMMEDIATE_MODE_SHADER_SOURCE: &[u8] = include_bytes!("../shaders/immediate_mode_gui.hlsl");
const IMMEDIATE_MODE_SHADER_IDENTIFIER: &str = "immediate_mode_gui_v1";
const PANEL_PADDING: f32 = 12.0;
const GLYPH_SCALE: f32 = 3.0;
const GLYPH_WIDTH: f32 = 5.0;
const GLYPH_HEIGHT: f32 = 7.0;
const CHARACTER_ADVANCE: f32 = 18.0;
const LINE_ADVANCE: f32 = 27.0;
const PANEL_BACKGROUND_COLOR: [f32; 4] = [0.02, 0.03, 0.05, 1.0];
const TEXT_COLOR: [f32; 4] = [0.85, 0.95, 1.0, 1.0];
// domain constants

impl ImmediateModeGui
{
    /// Creates an immediate-mode UI for a viewport of the supplied pixel dimensions.
    ///
    /// A zero dimension is treated as one pixel to keep coordinate conversion valid.
    pub fn new(viewport_width: u32, viewport_height: u32) -> Self
    {
        return Self {
            viewport_width: viewport_width.max(1) as f32,
            viewport_height: viewport_height.max(1) as f32,
            vertices: Vec::with_capacity(16_384),
        };
    }

    /// Clears the previous frame so the caller can issue the next frame's UI commands.
    pub fn begin_frame(&mut self)
    {
        self.vertices.clear();
    }

    /// Adds a dark text panel at the supplied pixel position.
    pub fn add_text_panel(&mut self, panel_position: [f32; 2], text: &str)
    {
        let panel_size = text_panel_size(text);
        let mut character_left = panel_position[0] + PANEL_PADDING;
        let mut character_top = panel_position[1] + PANEL_PADDING;

        self.add_rectangle(
            panel_position[0],
            panel_position[1],
            panel_size[0],
            panel_size[1],
            PANEL_BACKGROUND_COLOR,
        );

        for character in text.chars()
        {
            if character == '\n'
            {
                character_left = panel_position[0] + PANEL_PADDING;
                character_top += LINE_ADVANCE;
                continue;
            }

            self.add_glyph(
                character.to_ascii_uppercase(),
                character_left,
                character_top,
            );
            character_left += CHARACTER_ADVANCE;
        }
    }

    /// Adds the built-in performance-metrics panel at the supplied pixel position.
    pub fn add_performance_metrics_panel(
        &mut self,
        panel_position: [f32; 2],
        performance_sample: &PerformanceSample,
        graphics_performance_metrics: &GraphicsPerformanceMetrics,
    )
    {
        let metrics_text = performance_metrics_text(performance_sample, graphics_performance_metrics);
        self.add_text_panel(panel_position, &metrics_text);
    }

    fn add_glyph(&mut self, character: char, glyph_left: f32, glyph_top: f32)
    {
        let glyph_rows = glyph_rows(character);
        let mut row_index = 0;

        while row_index < GLYPH_HEIGHT as usize
        {
            let mut column_index = 0;

            while column_index < GLYPH_WIDTH as usize
            {
                let pixel_mask = 1 << (GLYPH_WIDTH as usize - 1 - column_index);

                if glyph_rows[row_index] & pixel_mask != 0
                {
                    self.add_rectangle(
                        glyph_left + column_index as f32 * GLYPH_SCALE,
                        glyph_top + row_index as f32 * GLYPH_SCALE,
                        GLYPH_SCALE,
                        GLYPH_SCALE,
                        TEXT_COLOR,
                    );
                }

                column_index += 1;
            }

            row_index += 1;
        }
    }

    fn add_rectangle(
        &mut self,
        left: f32,
        top: f32,
        width: f32,
        height: f32,
        color: [f32; 4],
    )
    {
        let left = left / self.viewport_width * 2.0 - 1.0;
        let right = left + width / self.viewport_width * 2.0;
        let top = 1.0 - top / self.viewport_height * 2.0;
        let bottom = top - height / self.viewport_height * 2.0;
        let top_left = GraphicsUserInterfaceVertex {
            position: [left, top],
            color,
        };
        let top_right = GraphicsUserInterfaceVertex {
            position: [right, top],
            color,
        };
        let bottom_left = GraphicsUserInterfaceVertex {
            position: [left, bottom],
            color,
        };
        let bottom_right = GraphicsUserInterfaceVertex {
            position: [right, bottom],
            color,
        };

        self.vertices.push(top_left);
        self.vertices.push(top_right);
        self.vertices.push(bottom_right);
        self.vertices.push(top_left);
        self.vertices.push(bottom_right);
        self.vertices.push(bottom_left);
    }
}

impl GraphicsUserInterface for ImmediateModeGui
{
    fn shader(&self) -> GraphicsUserInterfaceShader
    {
        return GraphicsUserInterfaceShader {
            source: IMMEDIATE_MODE_SHADER_SOURCE,
            identifier: IMMEDIATE_MODE_SHADER_IDENTIFIER,
        };
    }

    fn vertices(&self) -> &[GraphicsUserInterfaceVertex]
    {
        return &self.vertices;
    }
}

fn text_panel_size(text: &str) -> [f32; 2]
{
    let mut character_count_in_line = 0;
    let mut maximum_character_count = 0;
    let mut line_count = 1;

    for character in text.chars()
    {
        if character == '\n'
        {
            if character_count_in_line > maximum_character_count
            {
                maximum_character_count = character_count_in_line;
            }

            character_count_in_line = 0;
            line_count += 1;
            continue;
        }

        character_count_in_line += 1;
    }

    if character_count_in_line > maximum_character_count
    {
        maximum_character_count = character_count_in_line;
    }

    return [
        PANEL_PADDING * 2.0 + maximum_character_count as f32 * CHARACTER_ADVANCE,
        PANEL_PADDING * 2.0
            + (line_count - 1) as f32 * LINE_ADVANCE
            + GLYPH_HEIGHT * GLYPH_SCALE,
    ];
}

fn performance_metrics_text(
    performance_sample: &PerformanceSample,
    graphics_performance_metrics: &GraphicsPerformanceMetrics,
) -> String
{
    let antialiasing_text = format!(
        "MSAA: {} | TAA: {}",
        if graphics_performance_metrics.is_multisample_antialiasing_enabled { "On" } else { "Off" },
        if graphics_performance_metrics.is_temporal_antialiasing_enabled { "On" } else { "Off" },
    );
    let graphics_memory_text = match &graphics_performance_metrics.graphics_memory
    {
        Some(graphics_memory) => format!(
            "GPU memory: {:.0} / {:.0} MiB",
            graphics_memory.used_bytes as f64 / 1_048_576.0,
            graphics_memory.budget_bytes as f64 / 1_048_576.0,
        ),
        None => String::from("GPU memory: unavailable"),
    };
    let gpu_usage_text = match graphics_performance_metrics.gpu_frame_time_in_milliseconds
    {
        Some(gpu_frame_time_in_milliseconds) => format!(
            "GPU: {:.2} MS {:.0}% FRAME",
            gpu_frame_time_in_milliseconds,
            gpu_frame_time_in_milliseconds / performance_sample.frame_time_in_milliseconds * 100.0,
        ),
        None => String::from("GPU: collecting timing"),
    };

    return format!(
        "FPS: {:.1}\nFrame time: {:.2} ms\nProcess CPU: {:.1}%\n{}\n{}\n{}\nObjects: {}\nPress Tab to hide metrics",
        performance_sample.frames_per_second,
        performance_sample.frame_time_in_milliseconds,
        performance_sample.process_cpu_usage_percentage,
        gpu_usage_text,
        graphics_memory_text,
        antialiasing_text,
        graphics_performance_metrics.loaded_object_count,
    );
}

fn glyph_rows(character: char) -> [u8; 7]
{
    match character
    {
        'A' => [14, 17, 17, 31, 17, 17, 17],
        'B' => [30, 17, 17, 30, 17, 17, 30],
        'C' => [14, 17, 16, 16, 16, 17, 14],
        'D' => [30, 17, 17, 17, 17, 17, 30],
        'E' => [31, 16, 16, 30, 16, 16, 31],
        'F' => [31, 16, 16, 30, 16, 16, 16],
        'G' => [14, 17, 16, 23, 17, 17, 14],
        'H' => [17, 17, 17, 31, 17, 17, 17],
        'I' => [31, 4, 4, 4, 4, 4, 31],
        'J' => [7, 2, 2, 2, 2, 18, 12],
        'K' => [17, 18, 20, 24, 20, 18, 17],
        'L' => [16, 16, 16, 16, 16, 16, 31],
        'M' => [17, 27, 21, 21, 17, 17, 17],
        'N' => [17, 25, 21, 19, 17, 17, 17],
        'O' => [14, 17, 17, 17, 17, 17, 14],
        'P' => [30, 17, 17, 30, 16, 16, 16],
        'Q' => [14, 17, 17, 17, 21, 18, 13],
        'R' => [30, 17, 17, 30, 20, 18, 17],
        'S' => [15, 16, 16, 14, 1, 1, 30],
        'T' => [31, 4, 4, 4, 4, 4, 4],
        'U' => [17, 17, 17, 17, 17, 17, 14],
        'V' => [17, 17, 17, 17, 17, 10, 4],
        'W' => [17, 17, 17, 21, 21, 21, 10],
        'X' => [17, 17, 10, 4, 10, 17, 17],
        'Y' => [17, 17, 10, 4, 4, 4, 4],
        'Z' => [31, 1, 2, 4, 8, 16, 31],
        '0' => [14, 17, 19, 21, 25, 17, 14],
        '1' => [4, 12, 4, 4, 4, 4, 14],
        '2' => [14, 17, 1, 2, 4, 8, 31],
        '3' => [30, 1, 1, 14, 1, 1, 30],
        '4' => [2, 6, 10, 18, 31, 2, 2],
        '5' => [31, 16, 16, 30, 1, 1, 30],
        '6' => [14, 16, 16, 30, 17, 17, 14],
        '7' => [31, 1, 2, 4, 8, 8, 8],
        '8' => [14, 17, 17, 14, 17, 17, 14],
        '9' => [14, 17, 17, 15, 1, 1, 14],
        '.' => [0, 0, 0, 0, 0, 12, 12],
        ':' => [0, 12, 12, 0, 12, 12, 0],
        '/' => [1, 2, 2, 4, 8, 8, 16],
        '%' => [17, 2, 4, 8, 16, 17, 0],
        '-' => [0, 0, 0, 31, 0, 0, 0],
        _ => [0, 0, 0, 0, 0, 0, 0],
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn text_panel_encloses_the_longest_line()
    {
        let panel_size = text_panel_size("AB\nC");

        assert_eq!(panel_size[0], PANEL_PADDING * 2.0 + CHARACTER_ADVANCE * 2.0);
        assert_eq!(panel_size[1], PANEL_PADDING * 2.0 + LINE_ADVANCE + GLYPH_HEIGHT * GLYPH_SCALE);
    }

    #[test]
    fn begin_frame_clears_previous_draw_data()
    {
        let mut gui = ImmediateModeGui::new(1920, 1080);
        gui.add_text_panel([16.0, 16.0], "METRICS");

        assert!(!gui.vertices().is_empty());

        gui.begin_frame();

        assert!(gui.vertices().is_empty());
    }

    #[test]
    fn performance_metrics_panel_formats_graphics_and_process_data()
    {
        let performance_sample = PerformanceSample {
            frames_per_second: 120.0,
            frame_time_in_milliseconds: 8.0,
            process_cpu_usage_percentage: 15.0,
        };
        let graphics_performance_metrics = GraphicsPerformanceMetrics {
            gpu_frame_time_in_milliseconds: Some(4.0),
            graphics_memory: Some(graphics::GraphicsMemoryMetrics {
                used_bytes: 128 * 1_048_576,
                budget_bytes: 1024 * 1_048_576,
            }),
            is_multisample_antialiasing_enabled: true,
            is_temporal_antialiasing_enabled: false,
            loaded_object_count: 4,
        };
        let metrics_text = performance_metrics_text(
            &performance_sample,
            &graphics_performance_metrics,
        );

        assert!(metrics_text.contains("FPS: 120.0"));
        assert!(metrics_text.contains("GPU: 4.00 MS 50% FRAME"));
        assert!(metrics_text.contains("GPU memory: 128 / 1024 MiB"));
        assert!(metrics_text.contains("MSAA: On | TAA: Off"));
    }
}
