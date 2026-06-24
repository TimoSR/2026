//! Discoverable public factories for constructing generic UI nodes.

use std::fmt::Display;

use crate::{
    UserInterfaceBox,
    UserInterfaceBoxLayout,
    UserInterfaceInsets,
    UserInterfaceLength,
};

pub use crate::{
    button,
    column,
    list,
    panel,
    row,
    text,
};

/// A generic UI node that can be nested inside a panel, row, column, or list.
pub type Node = UserInterfaceBox;

/// Pixel insets used for margins and padding.
pub type Insets = UserInterfaceInsets;

/// A dimension override for a UI node.
pub type Length = UserInterfaceLength;

/// Creates a visible panel with default padding and vertical child flow.
pub fn panel() -> Node
{
    return UserInterfaceBox::new_panel(UserInterfaceBoxLayout::default());
}

/// Creates a text node without a background.
pub fn text(content: impl Display) -> Node
{
    return UserInterfaceBox::new_text(
        UserInterfaceBoxLayout::default(),
        content.to_string(),
    );
}

/// Creates a visible button with text.
pub fn button(label: impl Display) -> Node
{
    return UserInterfaceBox::new_button_with_text(
        UserInterfaceBoxLayout::default(),
        label.to_string(),
    );
}

/// Creates a visible list with vertical child flow.
pub fn list() -> Node
{
    return UserInterfaceBox::new_list(UserInterfaceBoxLayout::default());
}

/// Creates an invisible row with horizontal child flow.
pub fn row() -> Node
{
    return UserInterfaceBox::new_row(UserInterfaceBoxLayout::default());
}

/// Creates an invisible column with vertical child flow.
pub fn column() -> Node
{
    return UserInterfaceBox::new_container(UserInterfaceBoxLayout::default());
}

/// Applies an explicit width to a node.
pub fn width(length: impl Into<Length>, mut node: Node) -> Node
{
    let mut layout = node.layout();
    layout.width = length.into();
    node.set_layout(layout);

    return node;
}

/// Applies an explicit height to a node.
pub fn height(length: impl Into<Length>, mut node: Node) -> Node
{
    let mut layout = node.layout();
    layout.height = length.into();
    node.set_layout(layout);

    return node;
}

/// Applies outer spacing to a node.
pub fn margin(insets: impl Into<Insets>, mut node: Node) -> Node
{
    let mut layout = node.layout();
    layout.margin = insets.into();
    node.set_layout(layout);

    return node;
}

/// Applies inner spacing to a node.
pub fn padding(insets: impl Into<Insets>, mut node: Node) -> Node
{
    let mut layout = node.layout();
    layout.padding = insets.into();
    node.set_layout(layout);

    return node;
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn width_margin_and_padding_update_node_layout()
    {
        let node = padding(
            12.0,
            margin(8.0, width(320.0, panel())),
        );
        let layout = node.layout();

        assert_eq!(layout.width, Length::Pixels(320.0));
        assert_eq!(layout.margin, Insets::all(8.0));
        assert_eq!(layout.padding, Insets::all(12.0));
    }
}
