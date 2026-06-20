//! Readable aliases for common Rust ownership containers.
//!
//! These aliases do not change Rust semantics. They only give the memory and
//! sharing intent a domain-specific name that is easier to expose from a
//! transpiler or higher-level API.

use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

/// Direct unique ownership.
///
/// This is intentionally an identity alias. It exists so API documentation can
/// talk about `Owned<T>` alongside `Heap<T>`, `SharedLocal<T>`, and
/// `SharedThread<T>`.
pub type Owned<T> = T;

/// Unique ownership through heap indirection.
pub type Heap<T> = Box<T>;

/// Shared ownership for single-threaded code.
///
/// `Rc<T>` is cheaper than `Arc<T>`, but it is not `Send` or `Sync`.
pub type SharedLocal<T> = Rc<T>;

/// Shared ownership that can cross thread boundaries when `T` supports it.
///
/// `Arc<T>` uses atomic reference counting.
pub type SharedThread<T> = Arc<T>;

/// Borrowed-or-owned data.
///
/// `Cow<'a, T>` borrows data until mutation or ownership is required.
pub type CloneOnWrite<'a, T> = Cow<'a, T>;
