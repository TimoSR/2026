#![allow(clippy::needless_return)]
#![warn(missing_docs)]

//! A retained box-layout and immediate draw-data user-interface library.

mod immediate_mode;
mod layout;

pub use immediate_mode::ImmediateModeGui;
pub use layout::{
    ResolvedUserInterfaceBox,
    ScreenRelativePosition,
    UserInterfaceBox,
    UserInterfaceBoxAppearance,
    UserInterfaceBoxLayout,
    UserInterfaceChildrenLayout,
    UserInterfaceColor,
    UserInterfaceLayer,
    UserInterfaceLayout,
    UserInterfacePixelRectangle,
    UserInterfaceRelativeRectangle,
};
