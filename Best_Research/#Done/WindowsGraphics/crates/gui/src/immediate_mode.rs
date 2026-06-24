use graphics::{
    GraphicsUserInterface,
    GraphicsUserInterfaceShader,
    GraphicsUserInterfaceVertex,
};
use crate::{ScreenRelativePosition, UserInterfaceLayout};

// data structures
/// A minimal immediate-mode renderer that emits user-interface triangles.
pub struct ImmediateModeGui
{
    viewport_width: f32,
    viewport_height: f32,
    vertices: Vec<GraphicsUserInterfaceVertex>,
}
// data structures

// semantic type aliases
type UserInterfaceColor = [f32; 4];
type GlyphRows = [u8; 7];
type PixelPosition = [f32; 2];
// semantic type aliases

// domain constants
const IMMEDIATE_MODE_SHADER_SOURCE: &[u8] = include_bytes!("../shaders/immediate_mode_gui.hlsl");
const IMMEDIATE_MODE_SHADER_IDENTIFIER: &str = "immediate_mode_gui_v1";
const PANEL_PADDING: f32 = 12.0;
const GLYPH_SCALE: f32 = 3.0;
const GLYPH_WIDTH: f32 = 5.0;
const GLYPH_HEIGHT: f32 = 7.0;
const CHARACTER_ADVANCE: f32 = 18.0;
const LINE_ADVANCE: f32 = 27.0;
const PANEL_BACKGROUND_COLOR: UserInterfaceColor = [0.02, 0.03, 0.05, 1.0];
const TEXT_COLOR: UserInterfaceColor = [0.85, 0.95, 1.0, 1.0];
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

    /// Resolves and draws all visible boxes from a retained screen layout.
    ///
    /// Box positions are screen-relative in the layout. This method supplies
    /// the current viewport dimensions only when converting them to draw data.
    pub fn add_layout(&mut self, user_interface_layout: &UserInterfaceLayout)
    {
        let viewport_pixel_size = [self.viewport_width as u32, self.viewport_height as u32];
        let resolved_boxes = user_interface_layout.resolve(viewport_pixel_size);

        for resolved_box in resolved_boxes
        {
            let rectangle = resolved_box.pixel_rectangle;
            self.add_rectangle(
                rectangle.left,
                rectangle.top,
                rectangle.width,
                rectangle.height,
                resolved_box.background_color,
            );
        }
    }

    /// Adds a dark text panel at the supplied screen-relative position.
    ///
    /// A value of `[0.0, 0.0]` places the panel at the top-left viewport corner.
    /// Pixel coordinates are calculated only while the panel is rendered.
    pub fn add_text_panel(&mut self, screen_position: ScreenRelativePosition, text: &str)
    {
        let panel_position = self.pixel_position_from_screen_position(screen_position);
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

    fn pixel_position_from_screen_position(
        &self,
        screen_position: ScreenRelativePosition,
    ) -> PixelPosition
    {
        return [
            screen_position[0] * self.viewport_width,
            screen_position[1] * self.viewport_height,
        ];
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
        color: UserInterfaceColor,
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

fn text_panel_size(text: &str) -> PixelPosition
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

fn glyph_rows(character: char) -> GlyphRows
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
    use crate::{UserInterfaceBox, UserInterfaceBoxLayout};

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
        gui.add_text_panel([0.0, 0.0], "METRICS");

        assert!(!gui.vertices().is_empty());

        gui.begin_frame();

        assert!(gui.vertices().is_empty());
    }

    #[test]
    fn retained_layout_emits_one_rectangle_per_visible_box()
    {
        let mut layout = UserInterfaceLayout::new();
        layout.add_box(UserInterfaceBox::new(UserInterfaceBoxLayout::default()));
        let mut gui = ImmediateModeGui::new(1_920, 1_080);

        gui.add_layout(&layout);

        assert_eq!(gui.vertices().len(), 6);
    }

    #[test]
    fn text_panel_position_resolves_from_screen_relative_units()
    {
        let mut gui = ImmediateModeGui::new(200, 100);

        gui.add_text_panel([0.25, 0.5], "A");

        let vertices = gui.vertices();

        assert_eq!(vertices[0].position, [-0.5, 0.0]);
    }

}
