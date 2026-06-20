use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

use rust_memory_constructors::{Article, Heap, SharedLocal, SharedThread};

#[test]
fn owned_returns_article()
{
    let article: Article = Article::owned("Title");

    assert_eq!(article.title(), "Title");
}

#[test]
fn heap_returns_boxed_article()
{
    let article: Heap<Article> = Article::heap("Title");

    assert_eq!(article.title(), "Title");
}

#[test]
fn shared_local_returns_rc_article()
{
    let article: SharedLocal<Article> = Article::shared_local("Title");
    let cloned_reference: Rc<Article> = Rc::clone(&article);

    assert_eq!(article.title(), "Title");
    assert_eq!(cloned_reference.title(), "Title");
    assert_eq!(Rc::strong_count(&article), 2);
}

#[test]
fn shared_thread_returns_arc_article()
{
    let article: SharedThread<Article> = Article::shared_thread("Title");
    let cloned_reference: Arc<Article> = Arc::clone(&article);

    assert_eq!(article.title(), "Title");
    assert_eq!(cloned_reference.title(), "Title");
    assert_eq!(Arc::strong_count(&article), 2);
}

#[test]
fn clone_on_write_title_borrows_clean_title()
{
    let title = "Clean Title";
    let normalized_title = Article::clone_on_write_title(title);

    assert!(matches!(normalized_title, Cow::Borrowed("Clean Title")));
}

#[test]
fn clone_on_write_title_allocates_when_trim_required()
{
    let title = "  Dirty Title  ";
    let normalized_title = Article::clone_on_write_title(title);

    assert!(matches!(normalized_title, Cow::Owned(_)));
    assert_eq!(normalized_title, "Dirty Title");
}
