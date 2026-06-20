//! Example domain type using explicit ownership constructors.

use std::borrow::Cow;

use crate::memory::{CloneOnWrite, Heap, Owned, SharedLocal, SharedThread};

/// A small example entity used to demonstrate ownership constructor naming.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Article
{
    title: String,
}

impl Article
{
    /// Creates a directly owned article.
    #[must_use]
    pub fn owned(title: impl Into<String>) -> Owned<Self>
    {
        return Self
        {
            title: title.into(),
        };
    }

    /// Creates a uniquely owned heap-allocated article.
    #[must_use]
    pub fn heap(title: impl Into<String>) -> Heap<Self>
    {
        return Box::new(Self::owned(title));
    }

    /// Creates a single-thread shared article.
    #[must_use]
    pub fn shared_local(title: impl Into<String>) -> SharedLocal<Self>
    {
        return SharedLocal::new(Self::owned(title));
    }

    /// Creates a thread-safe shared article.
    #[must_use]
    pub fn shared_thread(title: impl Into<String>) -> SharedThread<Self>
    {
        return SharedThread::new(Self::owned(title));
    }

    /// Creates a clone-on-write title value.
    ///
    /// This returns a title value rather than `Article` because `Cow` is most
    /// naturally used for borrowed-or-owned fields such as `str` and `[T]`.
    #[must_use]
    pub fn clone_on_write_title<'a>(title: &'a str) -> CloneOnWrite<'a, str>
    {
        if title.trim() == title
        {
            return Cow::Borrowed(title);
        }

        return Cow::Owned(title.trim().to_string());
    }

    /// Returns the article title.
    #[must_use]
    pub fn title(&self) -> &str
    {
        return &self.title;
    }

    /// Replaces the article title.
    pub fn rename(&mut self, title: impl Into<String>)
    {
        self.title = title.into();
    }
}
