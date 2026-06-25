#![forbid(unsafe_code)]

use std::{thread, time::Duration};

use event_lite::{async_event, event, select_events};

#[event]
#[derive(Debug, Clone)]
pub struct UserRegistered
{
    pub user_id: u64,
    pub email: String,
}

#[event]
#[derive(Debug, Clone)]
pub struct UserEmailChanged
{
    pub user_id: u64,
    pub old_email: String,
    pub new_email: String,
}

fn main()
{
    synchronous_example();
    threaded_synchronous_example();
    waitable_event_example();
    multiple_waitable_events_example();
    additional_reader_example();
}

fn synchronous_example()
{
    let user_registered = event! { UserRegistered };

    user_registered.subscribe(|event: &UserRegistered| {
        println!("Fn subscriber: user_id={}, email={}", event.user_id, event.email);
    });

    user_registered.subscribe_mut({
        let mut count = 0;

        move |event: &UserRegistered| {
            count += 1;

            println!("FnMut subscriber #{count}: user_id={}, email={}", event.user_id, event.email);
        }
    });

    user_registered.subscribe_once(|event: &UserRegistered| {
        println!("FnOnce subscriber: first user only: user_id={}, email={}", event.user_id, event.email);
    });

    let removable = user_registered.subscribe(|event: &UserRegistered| {
        println!("removable subscriber: {}", event.email);
    });

    user_registered.emit(&UserRegistered::new(42, "timothy@example.com".to_string()));

    user_registered.emit(&UserRegistered::new(43, "second@example.com".to_string()));

    let removed = user_registered.unsubscribe(removable);
    println!("removed subscriber: {removed}");

    user_registered.emit(&UserRegistered::new(44, "third@example.com".to_string()));
}

fn threaded_synchronous_example()
{
    let user_registered = event! { UserRegistered };

    user_registered.subscribe(|event| {
        println!("thread-safe Fn subscriber: {}", event.email);
    });

    user_registered.subscribe_mut({
        let mut count = 0;

        move |event| {
            count += 1;
            println!("thread-safe FnMut subscriber #{count}: {}", event.email);
        }
    });

    let thread_a_event = user_registered.clone();
    let thread_b_event = user_registered.clone();

    let thread_a = thread::spawn(move || {
        thread_a_event.emit(&UserRegistered::new(100, "thread-a@example.com".to_string()));
    });

    let thread_b = thread::spawn(move || {
        thread_b_event.emit(&UserRegistered::new(200, "thread-b@example.com".to_string()));
    });

    thread_a.join().expect("thread A panicked");
    thread_b.join().expect("thread B panicked");
}

fn waitable_event_example()
{
    let user_registered = async_event! { UserRegistered };
    let publisher = user_registered.clone();

    let worker = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        publisher.emit(&UserRegistered::new(300, "waited@example.com".to_string()));
    });

    let event = user_registered.wait();
    println!("waited for event: user_id={}, email={}", event.user_id, event.email);

    worker.join().expect("waitable worker panicked");
}

fn multiple_waitable_events_example()
{
    let user_registered = async_event! { UserRegistered };
    let user_email_changed = async_event! { UserEmailChanged };

    let email_changed_publisher = user_email_changed.clone();

    let worker = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        email_changed_publisher.emit(&UserEmailChanged::new(42, "old@example.com".to_string(), "new@example.com".to_string()));
    });

    let message = select_events! {
        user_registered => |event| {
            format!("selected registered event: {}", event.email)
        },
        user_email_changed => |event| {
            format!("selected email-changed event: {} -> {}", event.old_email, event.new_email)
        },
    };

    println!("{message}");

    worker.join().expect("select worker panicked");
}

fn additional_reader_example()
{
    let user_registered = async_event! { UserRegistered };
    let audit_reader = user_registered.reader();

    user_registered.emit(&UserRegistered::new(500, "fanout@example.com".to_string()));

    let direct = user_registered.wait();
    let audit = audit_reader.wait();

    println!("direct event: {}", direct.email);
    println!("audit reader event: {}", audit.email);
}
