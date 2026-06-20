//! Constructor naming pattern for Rust ownership containers.
//!
//! This crate demonstrates a naming model that can be used directly in Rust or
//! generated from a Rust-plus transpiler:
//!
//! - `owned` -> `T`
//! - `heap` -> `Box<T>`
//! - `shared_local` -> `Rc<T>`
//! - `shared_thread` -> `Arc<T>`
//! - `clone_on_write` -> `Cow<'a, T>`

pub mod article;
pub mod memory;

pub use article::Article;
pub use memory::{CloneOnWrite, Heap, Owned, SharedLocal, SharedThread};
