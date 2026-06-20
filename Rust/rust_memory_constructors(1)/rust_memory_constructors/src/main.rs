//! Command-line example for the `rust_memory_constructors` crate.
//!
//! This binary demonstrates the public constructor pattern from the library
//! crate. The library is the reusable part; this binary only prints the
//! resulting values so the project can be run with `cargo run`.

use std::io::{self, Write};

use rust_memory_constructors::{Article, Heap, SharedLocal, SharedThread};

fn main() {
    let article_owned: Article = Article::owned("Owned title");
    let article_heap: Heap<Article> = Article::heap("Heap title");
    let article_shared_local: SharedLocal<Article> = Article::shared_local("Local shared title");
    let article_shared_thread: SharedThread<Article> = Article::shared_thread("Thread shared title");

    let borrowed_title = Article::clone_on_write_title("Already clean");
    let owned_title = Article::clone_on_write_title("  Needs trimming  ");

    println!(
        "owned: {}\nheap: {}\nshared_local: {}\nshared_thread: {}\nborrowed cow: {}\nowned cow: {}",
        article_owned.title(),
        article_heap.title(),
        article_shared_local.title(),
        article_shared_thread.title(),
        borrowed_title,
        owned_title,
    );
}
