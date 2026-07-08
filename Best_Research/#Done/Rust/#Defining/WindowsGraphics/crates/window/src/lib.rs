#![allow(clippy::needless_return)]
#![warn(missing_docs)]

//! Win32 application-window support.

mod window;

pub use window::{
    create_window,
    enable_per_monitor_dpi_awareness,
    ApplicationWindow,
    WindowEvents,
};
