#![allow(clippy::needless_return)]
#![warn(missing_docs)]

//! A retained box-layout and immediate draw-data user-interface library.

mod immediate_mode;
mod layout;
pub mod ui;

pub use immediate_mode::ImmediateModeGui;
pub use layout::{
    ResolvedUserInterfaceBox,
    ScreenRelativePosition,
    UserInterfaceBox,
    UserInterfaceBoxAppearance,
    UserInterfaceBoxLayout,
    UserInterfaceChildrenLayout,
    UserInterfaceColor,
    UserInterfaceInsets,
    UserInterfaceLayer,
    UserInterfaceLayout,
    UserInterfaceLength,
    UserInterfacePixelRectangle,
    UserInterfaceRelativeRectangle,
};

/// Creates a retained UI layout from a screen-relative root placement and node tree.
pub fn new(
    position: ScreenRelativePosition,
    width: f32,
    height: f32,
    mut root: ui::Node,
) -> UserInterfaceLayout
{
    let mut root_layout = root.layout();
    root_layout.relative_bounds = UserInterfaceRelativeRectangle::new(
        position[0],
        position[1],
        width,
        height,
    );
    root.set_layout(root_layout);

    let mut user_interface_layout = UserInterfaceLayout::new();
    user_interface_layout.add_box(root);

    return user_interface_layout;
}

/// Constructs a panel node from nested UI nodes.
#[macro_export]
macro_rules! panel
{
    ($($children:tt)*) => {{
        let mut panel = $crate::ui::panel();

        $crate::__ui_add_nodes!(panel; $($children)*);

        panel
    }};
}

/// Constructs a vertical list node from nested UI nodes.
#[macro_export]
macro_rules! list
{
    ($($children:tt)*) => {{
        let mut list = $crate::ui::list();

        $crate::__ui_add_nodes!(list; $($children)*);

        list
    }};
}

/// Constructs a horizontal row node from nested UI nodes.
#[macro_export]
macro_rules! row
{
    ($($children:tt)*) => {{
        let mut row = $crate::ui::row();

        $crate::__ui_add_nodes!(row; $($children)*);

        row
    }};
}

/// Constructs a vertical column node from nested UI nodes.
#[macro_export]
macro_rules! column
{
    ($($children:tt)*) => {{
        let mut column = $crate::ui::column();

        $crate::__ui_add_nodes!(column; $($children)*);

        column
    }};
}

/// Constructs a text node.
#[macro_export]
macro_rules! text
{
    ($text:expr) => {
        $crate::ui::text($text)
    };
}

/// Constructs a button node.
#[macro_export]
macro_rules! button
{
    ($text:expr) => {
        $crate::ui::button($text)
    };
}

/// Internal implementation detail for the element macros.
#[doc(hidden)]
#[macro_export]
macro_rules! __ui_add_nodes
{
    ($parent:ident;) => {};
    ($parent:ident; for $item:ident in $start:literal .. $end:literal { $($children:tt)* } $($remaining:tt)*) => {{
        for $item in $start..$end
        {
            $crate::__ui_add_nodes!($parent; $($children)*);
        }

        $crate::__ui_add_nodes!($parent; $($remaining)*);
    }};
    ($parent:ident; for $item:ident in $items:ident { $($children:tt)* } $($remaining:tt)*) => {{
        for $item in $items
        {
            $crate::__ui_add_nodes!($parent; $($children)*);
        }

        $crate::__ui_add_nodes!($parent; $($remaining)*);
    }};
    ($parent:ident; $namespace:ident :: panel ! { $($children:tt)* } $($remaining:tt)*) => {{
        $parent.add_child($crate::panel! { $($children)* });

        $crate::__ui_add_nodes!($parent; $($remaining)*);
    }};
    ($parent:ident; $namespace:ident :: list ! { $($children:tt)* } $($remaining:tt)*) => {{
        $parent.add_child($crate::list! { $($children)* });

        $crate::__ui_add_nodes!($parent; $($remaining)*);
    }};
    ($parent:ident; $namespace:ident :: row ! { $($children:tt)* } $($remaining:tt)*) => {{
        $parent.add_child($crate::row! { $($children)* });

        $crate::__ui_add_nodes!($parent; $($remaining)*);
    }};
    ($parent:ident; $namespace:ident :: column ! { $($children:tt)* } $($remaining:tt)*) => {{
        $parent.add_child($crate::column! { $($children)* });

        $crate::__ui_add_nodes!($parent; $($remaining)*);
    }};
    ($parent:ident; $namespace:ident :: text ! { $text:expr } $($remaining:tt)*) => {{
        $parent.add_child($crate::text! { $text });

        $crate::__ui_add_nodes!($parent; $($remaining)*);
    }};
    ($parent:ident; $namespace:ident :: button ! { $text:expr } $($remaining:tt)*) => {{
        $parent.add_child($crate::button! { $text });

        $crate::__ui_add_nodes!($parent; $($remaining)*);
    }};
    ($parent:ident; $child:expr; $($remaining:tt)*) => {{
        $parent.add_child($child);

        $crate::__ui_add_nodes!($parent; $($remaining)*);
    }};
    ($parent:ident; $child:expr) => {
        $parent.add_child($child);
    };
}

#[cfg(test)]
mod tests
{
    use super::*;

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
    fn normal_ui_functions_apply_panel_width_and_padding()
    {
        let layout = crate::new(
            [0.0, 0.0],
            1.0,
            1.0,
            ui::padding(
                24,
                ui::width(
                    320,
                    ui::panel! {
                        ui::text! { "Settings" }
                    },
                ),
            ),
        );

        let resolved_boxes = layout.resolve([1_000, 500]);
        let panel = &resolved_boxes[0];

        assert_eq!(panel.pixel_rectangle.width, 320.0);
        assert_eq!(panel.content_pixel_rectangle.left, 24.0);
        assert_eq!(panel.content_pixel_rectangle.top, 24.0);
        assert_eq!(panel.content_pixel_rectangle.width, 272.0);
    }

}
