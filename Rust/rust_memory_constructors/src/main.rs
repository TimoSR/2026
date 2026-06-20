use rust_memory_constructors::{Article, Heap, SharedLocal, SharedThread};

fn main()
{
    let article_owned: Article = Article::owned("Owned title");
    let article_heap: Heap<Article> = Article::heap("Heap title");
    let article_shared_local: SharedLocal<Article> = Article::shared_local("Local shared title");
    let article_shared_thread: SharedThread<Article> = Article::shared_thread("Thread shared title");

    let borrowed_title = Article::clone_on_write_title("Already clean");
    let owned_title = Article::clone_on_write_title("  Needs trimming  ");

    println!("owned: {}", article_owned.title());
    println!("heap: {}", article_heap.title());
    println!("shared_local: {}", article_shared_local.title());
    println!("shared_thread: {}", article_shared_thread.title());
    println!("borrowed cow: {borrowed_title}");
    println!("owned cow: {owned_title}");
}
