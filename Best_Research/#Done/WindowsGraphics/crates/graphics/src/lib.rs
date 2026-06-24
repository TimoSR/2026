#![allow(clippy::needless_return)]
#![warn(missing_docs)]

//! Direct3D 11 rendering contracts and implementation.

mod contract;
mod direct3d;
mod gpu_timing;
mod temporal_antialiasing;
mod user_interface_renderer;

pub use contract::{GraphicsObject, GraphicsVertex};
pub use contract::{
    GraphicsUserInterface,
    GraphicsUserInterfaceShader,
    GraphicsUserInterfaceVertex,
};
pub use direct3d::{
    create_direct3d_graphics,
    Direct3DGraphics,
    GraphicsMemoryMetrics,
    GraphicsPerformanceMetrics,
};
