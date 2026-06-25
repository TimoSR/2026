//! Internal macro support for the public `gui::ui` namespace.

use std::fmt::Display;

use crate::{
    layout::{
        UserInterfaceBox,
        UserInterfaceBoxLayout,
    },
    UserInterfaceNode,
};

/// Creates a panel node for the `ui::panel!` macro.
#[doc(hidden)]
pub fn panel() -> UserInterfaceNode
{
    return UserInterfaceNode::from_box(
        UserInterfaceBox::new_panel(UserInterfaceBoxLayout::default()),
    );
}

/// Creates a text node for the `ui::text!` macro.
#[doc(hidden)]
pub fn text(content: impl Display) -> UserInterfaceNode
{
    return UserInterfaceNode::from_box(UserInterfaceBox::new_text(
        UserInterfaceBoxLayout::default(),
        content.to_string(),
    ));
}

/// Creates a button node for the `ui::button!` macro.
#[doc(hidden)]
pub fn button(label: impl Display) -> UserInterfaceNode
{
    return UserInterfaceNode::from_box(UserInterfaceBox::new_button_with_text(
        UserInterfaceBoxLayout::default(),
        label.to_string(),
    ));
}

/// Creates a list node for the `ui::list!` macro.
#[doc(hidden)]
pub fn list() -> UserInterfaceNode
{
    return UserInterfaceNode::from_box(
        UserInterfaceBox::new_list(UserInterfaceBoxLayout::default()),
    );
}

/// Creates a row node for the `ui::row!` macro.
#[doc(hidden)]
pub fn row() -> UserInterfaceNode
{
    return UserInterfaceNode::from_box(
        UserInterfaceBox::new_row(UserInterfaceBoxLayout::default()),
    );
}

/// Creates a column node for the `ui::column!` macro.
#[doc(hidden)]
pub fn column() -> UserInterfaceNode
{
    return UserInterfaceNode::from_box(
        UserInterfaceBox::new_container(UserInterfaceBoxLayout::default()),
    );
}

/// Adds a child node for the construction macros.
#[doc(hidden)]
pub fn add_child(parent: &mut UserInterfaceNode, child: UserInterfaceNode)
{
    parent.add_child(child);
}

/// Applies an explicit width for the construction macros.
#[doc(hidden)]
pub fn set_width(node: &mut UserInterfaceNode, width: f32)
{
    node.set_width(width);
}

/// Applies an explicit height for the construction macros.
#[doc(hidden)]
pub fn set_height(node: &mut UserInterfaceNode, height: f32)
{
    node.set_height(height);
}

/// Applies outer spacing for the construction macros.
#[doc(hidden)]
pub fn set_margin(node: &mut UserInterfaceNode, margin: f32)
{
    node.set_margin(margin);
}

/// Applies inner spacing for the construction macros.
#[doc(hidden)]
pub fn set_padding(node: &mut UserInterfaceNode, padding: f32)
{
    node.set_padding(padding);
}

/// Applies a glued-layout weight for the construction macros.
#[doc(hidden)]
pub fn set_layout_weight(node: &mut UserInterfaceNode, layout_weight: f32)
{
    node.set_layout_weight(layout_weight);
}

/// Applies text for the construction macros.
#[doc(hidden)]
pub fn set_text(node: &mut UserInterfaceNode, text: impl Into<String>)
{
    node.set_text(text);
}
