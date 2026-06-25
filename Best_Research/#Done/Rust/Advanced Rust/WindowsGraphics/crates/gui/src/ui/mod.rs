//! Public UI construction macro namespace.

#[doc(hidden)]
pub mod __private;

pub use crate::{
    __gui_button as button,
    __gui_column as column,
    __gui_list as list,
    __gui_panel as panel,
    __gui_row as row,
    __gui_text as text,
};

/// Constructs a panel node from nested UI nodes.
#[doc(hidden)]
#[macro_export]
macro_rules! __gui_panel
{
    ($($children:tt)*) => {{
        let mut panel = $crate::ui::__private::panel();

        $crate::__gui_configure_node!(panel; $($children)*);

        panel
    }};
}

/// Constructs a vertical list node from nested UI nodes.
#[doc(hidden)]
#[macro_export]
macro_rules! __gui_list
{
    ($($children:tt)*) => {{
        let mut list = $crate::ui::__private::list();

        $crate::__gui_configure_node!(list; $($children)*);

        list
    }};
}

/// Constructs a horizontal row node from nested UI nodes.
#[doc(hidden)]
#[macro_export]
macro_rules! __gui_row
{
    ($($children:tt)*) => {{
        let mut row = $crate::ui::__private::row();

        $crate::__gui_configure_node!(row; $($children)*);

        row
    }};
}

/// Constructs a vertical column node from nested UI nodes.
#[doc(hidden)]
#[macro_export]
macro_rules! __gui_column
{
    ($($children:tt)*) => {{
        let mut column = $crate::ui::__private::column();

        $crate::__gui_configure_node!(column; $($children)*);

        column
    }};
}

/// Constructs a text node.
#[doc(hidden)]
#[macro_export]
macro_rules! __gui_text
{
    ($text:expr) => {
        $crate::ui::__private::text($text)
    };
    ($($tokens:tt)*) => {{
        let mut text = $crate::ui::__private::text("");

        $crate::__gui_configure_leaf!(text; $($tokens)*);

        text
    }};
}

/// Constructs a button node.
#[doc(hidden)]
#[macro_export]
macro_rules! __gui_button
{
    ($text:expr) => {
        $crate::ui::__private::button($text)
    };
    ($($tokens:tt)*) => {{
        let mut button = $crate::ui::__private::button("");

        $crate::__gui_configure_leaf!(button; $($tokens)*);

        button
    }};
}

/// Internal implementation detail for the element macros.
#[doc(hidden)]
#[macro_export]
macro_rules! __gui_configure_node
{
    ($parent:ident;) => {};
    ($parent:ident; width: $width:expr; $($remaining:tt)*) => {{
        $crate::ui::__private::set_width(&mut $parent, $width as f32);

        $crate::__gui_configure_node!($parent; $($remaining)*);
    }};
    ($parent:ident; height: $height:expr; $($remaining:tt)*) => {{
        $crate::ui::__private::set_height(&mut $parent, $height as f32);

        $crate::__gui_configure_node!($parent; $($remaining)*);
    }};
    ($parent:ident; margin: $margin:expr; $($remaining:tt)*) => {{
        $crate::ui::__private::set_margin(&mut $parent, $margin as f32);

        $crate::__gui_configure_node!($parent; $($remaining)*);
    }};
    ($parent:ident; padding: $padding:expr; $($remaining:tt)*) => {{
        $crate::ui::__private::set_padding(&mut $parent, $padding as f32);

        $crate::__gui_configure_node!($parent; $($remaining)*);
    }};
    ($parent:ident; weight: $layout_weight:expr; $($remaining:tt)*) => {{
        $crate::ui::__private::set_layout_weight(&mut $parent, $layout_weight as f32);

        $crate::__gui_configure_node!($parent; $($remaining)*);
    }};
    ($parent:ident; for $child_name:ident in $start:literal .. $end:literal { $($children:tt)* } $($remaining:tt)*) => {{
        for $child_name in $start..$end
        {
            $crate::__gui_configure_node!($parent; $($children)*);
        }

        $crate::__gui_configure_node!($parent; $($remaining)*);
    }};
    ($parent:ident; for $child_name:ident in $children_source:ident { $($children:tt)* } $($remaining:tt)*) => {{
        for $child_name in $children_source
        {
            $crate::__gui_configure_node!($parent; $($children)*);
        }

        $crate::__gui_configure_node!($parent; $($remaining)*);
    }};
    ($parent:ident; $namespace:ident :: panel ! { $($children:tt)* } $($remaining:tt)*) => {{
        $crate::ui::__private::add_child(&mut $parent, $crate::__gui_panel! { $($children)* });

        $crate::__gui_configure_node!($parent; $($remaining)*);
    }};
    ($parent:ident; $namespace:ident :: list ! { $($children:tt)* } $($remaining:tt)*) => {{
        $crate::ui::__private::add_child(&mut $parent, $crate::__gui_list! { $($children)* });

        $crate::__gui_configure_node!($parent; $($remaining)*);
    }};
    ($parent:ident; $namespace:ident :: row ! { $($children:tt)* } $($remaining:tt)*) => {{
        $crate::ui::__private::add_child(&mut $parent, $crate::__gui_row! { $($children)* });

        $crate::__gui_configure_node!($parent; $($remaining)*);
    }};
    ($parent:ident; $namespace:ident :: column ! { $($children:tt)* } $($remaining:tt)*) => {{
        $crate::ui::__private::add_child(&mut $parent, $crate::__gui_column! { $($children)* });

        $crate::__gui_configure_node!($parent; $($remaining)*);
    }};
    ($parent:ident; $namespace:ident :: text ! { $($tokens:tt)* } $($remaining:tt)*) => {{
        $crate::ui::__private::add_child(&mut $parent, $crate::__gui_text! { $($tokens)* });

        $crate::__gui_configure_node!($parent; $($remaining)*);
    }};
    ($parent:ident; $namespace:ident :: button ! { $($tokens:tt)* } $($remaining:tt)*) => {{
        $crate::ui::__private::add_child(&mut $parent, $crate::__gui_button! { $($tokens)* });

        $crate::__gui_configure_node!($parent; $($remaining)*);
    }};
    ($parent:ident; $child:expr; $($remaining:tt)*) => {{
        $crate::ui::__private::add_child(&mut $parent, $child);

        $crate::__gui_configure_node!($parent; $($remaining)*);
    }};
    ($parent:ident; $child:expr) => {
        $crate::ui::__private::add_child(&mut $parent, $child);
    };
}

/// Internal implementation detail for text and button macros.
#[doc(hidden)]
#[macro_export]
macro_rules! __gui_configure_leaf
{
    ($leaf:ident;) => {};
    ($leaf:ident; $text:expr) => {
        $crate::ui::__private::set_text(&mut $leaf, $text);
    };
    ($leaf:ident; width: $width:expr; $($remaining:tt)*) => {{
        $crate::ui::__private::set_width(&mut $leaf, $width as f32);

        $crate::__gui_configure_leaf!($leaf; $($remaining)*);
    }};
    ($leaf:ident; height: $height:expr; $($remaining:tt)*) => {{
        $crate::ui::__private::set_height(&mut $leaf, $height as f32);

        $crate::__gui_configure_leaf!($leaf; $($remaining)*);
    }};
    ($leaf:ident; margin: $margin:expr; $($remaining:tt)*) => {{
        $crate::ui::__private::set_margin(&mut $leaf, $margin as f32);

        $crate::__gui_configure_leaf!($leaf; $($remaining)*);
    }};
    ($leaf:ident; padding: $padding:expr; $($remaining:tt)*) => {{
        $crate::ui::__private::set_padding(&mut $leaf, $padding as f32);

        $crate::__gui_configure_leaf!($leaf; $($remaining)*);
    }};
    ($leaf:ident; weight: $layout_weight:expr; $($remaining:tt)*) => {{
        $crate::ui::__private::set_layout_weight(&mut $leaf, $layout_weight as f32);

        $crate::__gui_configure_leaf!($leaf; $($remaining)*);
    }};
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn ui_namespace_exports_macro_names()
    {
        let panel = panel! {
            text! { "Settings" }
        };

        assert_eq!(panel.children().len(), 1);
        assert_eq!(panel.children()[0].text(), Some("Settings"));
    }
}
