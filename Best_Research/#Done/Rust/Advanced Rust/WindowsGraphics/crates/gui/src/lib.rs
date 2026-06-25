#![allow(clippy::needless_return)]
#![warn(missing_docs)]

//! A retained box-layout and immediate draw-data user-interface library.

mod immediate_mode;
mod layout;
pub mod ui;

pub use immediate_mode::ImmediateModeGui;
pub use layout::{
    ScreenRelativePosition,
    UserInterfaceLayout,
    UserInterfaceNode,
};

use layout::UserInterfaceRelativeRectangle;

/// Creates a retained UI layout from a screen-relative root placement and node tree.
pub fn new(
    position: ScreenRelativePosition,
    width: f32,
    height: f32,
    root: UserInterfaceNode,
) -> UserInterfaceLayout
{
    let mut root_box = root.into_box();
    let mut root_layout = root_box.layout();
    root_layout.relative_bounds = UserInterfaceRelativeRectangle::new(
        position[0],
        position[1],
        width,
        height,
    );
    root_box.set_layout(root_layout);

    let mut user_interface_layout = UserInterfaceLayout::new();
    user_interface_layout.add_box(root_box);

    return user_interface_layout;
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::layout::{
        UserInterfaceInsets,
        UserInterfaceLength,
        UserInterfaceRelativeRectangle,
    };

    #[test]
    fn new_places_a_nested_panel_layout()
    {
        let position = [0.1, 0.2];
        let layout = crate::new(
            position,
            0.3,
            0.4,
            ui::panel! {
                ui::text! { "Metrics" }
                ui::list! {
                    ui::button! { "Resume" }
                }
            },
        );

        assert_eq!(layout.boxes().len(), 1);
        assert_eq!(
            layout.boxes()[0].layout().relative_bounds,
            UserInterfaceRelativeRectangle::new(0.1, 0.2, 0.3, 0.4),
        );
        let panel = &layout.boxes()[0];

        assert_eq!(panel.children().len(), 2);
        assert_eq!(panel.children()[0].text(), Some("Metrics"));
        assert_eq!(panel.children()[1].children()[0].text(), Some("Resume"));
    }

    #[test]
    fn list_macro_expands_a_collection_into_button_children()
    {
        let labels = ["Save 1", "Save 2"];
        let list = ui::list! {
                for label in labels {
                    ui::button! { label }
                }
        };

        assert_eq!(list.children().len(), 2);
        assert_eq!(list.children()[0].text(), Some("Save 1"));
        assert_eq!(list.children()[1].text(), Some("Save 2"));
    }

    #[test]
    fn list_macro_expands_a_numeric_range_into_button_labels()
    {
        let list = ui::list! {
                for index in 0..3 {
                    ui::button! { index }
                }
        };

        assert_eq!(list.children().len(), 3);
        assert_eq!(list.children()[0].text(), Some("0"));
        assert_eq!(list.children()[2].text(), Some("2"));
    }

    #[test]
    fn panel_macro_applies_local_width_and_padding_options()
    {
        let layout = crate::new(
            [0.0, 0.0],
            1.0,
            1.0,
            ui::panel! {
                width: 320;
                padding: 24;

                ui::text! { "Settings" }
            },
        );

        let resolved_boxes = layout.resolve([1_000, 500]);
        let panel = &resolved_boxes[0];

        assert_eq!(panel.pixel_rectangle.width, 320.0);
        assert_eq!(panel.content_pixel_rectangle.left, 24.0);
        assert_eq!(panel.content_pixel_rectangle.top, 24.0);
        assert_eq!(panel.content_pixel_rectangle.width, 272.0);
    }

    #[test]
    fn button_macro_applies_local_options()
    {
        let button = ui::button! {
            width: 120;
            margin: 4;

            "Save"
        };
        let layout = button.layout();

        assert_eq!(button.text(), Some("Save"));
        assert_eq!(layout.width, UserInterfaceLength::Pixels(120.0));
        assert_eq!(layout.margin, UserInterfaceInsets::all(4.0));
    }
}
