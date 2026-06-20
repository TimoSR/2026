# Rust Memory Constructors

Conclusion: this project shows that Rust can support readable constructor names such as `Article::owned`, `Article::heap`, `Article::shared_local`, and `Article::shared_thread` without changing Rust's ownership model.

## What this project demonstrates

```rust
use rust_memory_constructors::{Article, Heap, SharedLocal, SharedThread};

fn main()
{
    let article_owned: Article = Article::owned("Owned title");
    let article_heap: Heap<Article> = Article::heap("Heap title");
    let article_shared_local: SharedLocal<Article> = Article::shared_local("Local shared title");
    let article_shared_thread: SharedThread<Article> = Article::shared_thread("Thread shared title");

    println!("{}", article_owned.title());
    println!("{}", article_heap.title());
    println!("{}", article_shared_local.title());
    println!("{}", article_shared_thread.title());
}
```

## Type aliases

```rust
pub type Owned<T> = T;
pub type Heap<T> = Box<T>;
pub type SharedLocal<T> = std::rc::Rc<T>;
pub type SharedThread<T> = std::sync::Arc<T>;
pub type CloneOnWrite<'a, T> = std::borrow::Cow<'a, T>;
```

## Constructor mapping

| Constructor | Return type | Meaning |
|---|---:|---|
| `Article::owned(...)` | `Article` | Direct unique ownership |
| `Article::heap(...)` | `Box<Article>` | Unique heap ownership |
| `Article::shared_local(...)` | `Rc<Article>` | Shared ownership in one thread |
| `Article::shared_thread(...)` | `Arc<Article>` | Shared ownership across threads |
| `Article::clone_on_write_title(...)` | `Cow<'a, str>` | Borrow first, allocate only when needed |

## Commands

```bash
cargo run
cargo test
cargo clippy --all-targets --all-features
```

## Transpiler relevance

For a Rust-plus transpiler, the user-facing syntax could stay simple:

```rust
let article = Article::owned("Title");
let article = Article::heap("Title");
let article = Article::shared_local("Title");
let article = Article::shared_thread("Title");
```

The generated Rust should simply target standard Rust containers:

```rust
Article::owned("Title")
Box::new(Article::owned("Title"))
Rc::new(Article::owned("Title"))
Arc::new(Article::owned("Title"))
```

This keeps the language sugar shallow and avoids inventing a second memory model.
