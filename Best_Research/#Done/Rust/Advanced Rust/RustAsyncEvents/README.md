# minimal-events

Minimal Rust event API with:

- `#[event]` for typed event payload structs
- `let event = event! { PayloadType };` for synchronous callback events
- `let event = async_event! { PayloadType };` for std-only waitable events
- direct waiting on the event value itself: `event.wait()`
- `Fn`, `FnMut`, and `FnOnce` subscribers for synchronous events
- thread-safe emit/subscribe/unsubscribe
- no Tokio, no async runtime, no external event system

## Synchronous callback events

```rust
use event_lite::event;

#[event]
#[derive(Debug, Clone)]
pub struct UserRegistered {
    pub user_id: u64,
    pub email: String,
}

fn main() {
    let user_registered = event! { UserRegistered };

    user_registered.subscribe(|event| {
        println!("{}", event.email);
    });

    user_registered.emit(&UserRegistered::new(
        42,
        "timothy@example.com".to_string(),
    ));
}
```

## Synchronous API

```rust
let event = event! { UserRegistered };

let id = event.subscribe(|event: &UserRegistered| {});
let id = event.subscribe_mut(|event: &UserRegistered| {});
let id = event.subscribe_once(|event: &UserRegistered| {});

let removed = event.unsubscribe(id);
event.emit(&payload);
let count = event.subscriber_count();
```

`event! { UserRegistered }` expands conceptually to:

```rust
Event::<UserRegistered>::new()
```

There is no global event registry. The payload type is known at compile time, and subscribers are registered on the event value.

## Callable semantics

| Method | Bound | Use case |
|---|---|---|
| `subscribe` | `Fn(&T) + Send + Sync + 'static` | reusable immutable handlers |
| `subscribe_mut` | `FnMut(&T) + Send + 'static` | reusable mutable handlers, internally mutex-protected |
| `subscribe_once` | `FnOnce(&T) + Send + 'static` | one-shot handlers drained on first emit |

## Std-only waitable events

`async_event!` creates a waitable event mailbox. The event value is already the receiver. No separate subscription is needed for the common case.

It does not use Tokio. It uses `std::sync::Mutex`, `std::sync::Condvar`, and `VecDeque`.

```rust
use event_lite::{async_event, event};

#[event]
#[derive(Debug, Clone)]
pub struct UserRegistered {
    pub user_id: u64,
    pub email: String,
}

fn main() {
    let user_registered = async_event! { UserRegistered };

    user_registered.emit(&UserRegistered::new(
        42,
        "timothy@example.com".to_string(),
    ));

    let event = user_registered.wait();
    println!("{}", event.email);
}
```

## Waitable API

```rust
let event = async_event! { UserRegistered };

let delivered = event.emit(&payload);
let payload = event.wait();
let maybe_payload = event.try_recv();
let maybe_payload = event.wait_timeout(std::time::Duration::from_secs(1));
let pending = event.pending_count();
```

`AsyncEvent<T>::emit(&T)` requires `T: Clone`, because the event stores a queued copy.

## Reacting to multiple waitable events

Use `select_events!` to block until one of several event values receives a payload:

```rust
use event_lite::{async_event, event, select_events};

#[event]
#[derive(Debug, Clone)]
pub struct UserRegistered {
    pub user_id: u64,
    pub email: String,
}

#[event]
#[derive(Debug, Clone)]
pub struct UserEmailChanged {
    pub user_id: u64,
    pub old_email: String,
    pub new_email: String,
}

fn main() {
    let user_registered = async_event! { UserRegistered };
    let user_email_changed = async_event! { UserEmailChanged };

    user_email_changed.emit(&UserEmailChanged::new(
        42,
        "old@example.com".to_string(),
        "new@example.com".to_string(),
    ));

    let message = select_events! {
        user_registered => |event| {
            format!("registered: {}", event.email)
        },
        user_email_changed => |event| {
            format!("email changed: {} -> {}", event.old_email, event.new_email)
        },
    };

    println!("{message}");
}
```

The branches must return the same type, like Rust `match` arms.

## Additional independent readers

The default waitable event is a single mailbox. Clones share that same mailbox.

If you need fan-out, create an extra reader explicitly:

```rust
let user_registered = async_event! { UserRegistered };
let audit_reader = user_registered.reader();

user_registered.emit(&UserRegistered::new(
    42,
    "timothy@example.com".to_string(),
));

let normal = user_registered.wait();
let audit = audit_reader.wait();
```

`reader()` is the advanced case. The default case stays minimal.

## Design constraints

This project deliberately avoids an async runtime. That means:

- `Event<T>` is synchronous callback dispatch.
- `AsyncEvent<T>` is a blocking waitable event mailbox.
- `AsyncEvent<T>::wait()` blocks the current OS thread.
- `select_events!` waits using a shared generation counter plus `Condvar`.
- There is no `async fn`, no `Future`, and no executor.

If true `async/await` is required later, the project would need its own executor or a `Future` implementation. That is a separate design from this minimal waitable-event layer.

## Run

```bash
cargo run -p app
```

## Test

```bash
cargo test
```
