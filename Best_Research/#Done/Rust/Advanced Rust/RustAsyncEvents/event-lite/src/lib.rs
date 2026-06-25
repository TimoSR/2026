#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::sync::{Arc, Condvar, Mutex, RwLock, Weak};
use std::time::Duration;

pub use event_lite_macros::event;

#[macro_export]
macro_rules! event {
    () => {
        compile_error!("event! requires a payload type, for example: event! { UserRegistered }")
    };

    ($event_ty:ty $(,)?) => {
        $crate::Event::<$event_ty>::new()
    };
}

#[macro_export]
macro_rules! async_event {
    () => {
        compile_error!("async_event! requires a payload type, for example: async_event! { UserRegistered }")
    };

    ($event_ty:ty $(,)?) => {
        $crate::AsyncEvent::<$event_ty>::new()
    };
}

#[macro_export]
macro_rules! select_events {
    ($($event_source:expr => |$event:ident| $body:block),+ $(,)?) => {{
        loop {
            let __observed_generation = $crate::event_generation();

            $(
                if let Some(__payload) = $event_source.try_recv() {
                    let $event = __payload;
                    break $body;
                }
            )+

            $crate::wait_for_event_generation_change(__observed_generation);
        }
    }};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubscriptionId(u64);

impl SubscriptionId {
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

type FnHandler<T> = Arc<dyn Fn(&T) + Send + Sync + 'static>;
type FnMutHandler<T> = Arc<Mutex<Box<dyn FnMut(&T) + Send + 'static>>>;
type FnOnceHandler<T> = Box<dyn FnOnce(&T) + Send + 'static>;

struct EventInner<T> {
    next_id: u64,
    fn_handlers: HashMap<SubscriptionId, FnHandler<T>>,
    fn_mut_handlers: HashMap<SubscriptionId, FnMutHandler<T>>,
    fn_once_handlers: HashMap<SubscriptionId, FnOnceHandler<T>>,
}

impl<T> EventInner<T> {
    fn new() -> Self {
        Self {
            next_id: 1,
            fn_handlers: HashMap::new(),
            fn_mut_handlers: HashMap::new(),
            fn_once_handlers: HashMap::new(),
        }
    }

    fn next_subscription_id(&mut self) -> SubscriptionId {
        let id = SubscriptionId(self.next_id);

        self.next_id = self
            .next_id
            .checked_add(1)
            .expect("subscription id overflow");

        id
    }

    fn subscriber_count(&self) -> usize {
        self.fn_handlers.len() + self.fn_mut_handlers.len() + self.fn_once_handlers.len()
    }
}

#[derive(Clone)]
pub struct Event<T> {
    inner: Arc<RwLock<EventInner<T>>>,
}

impl<T> Event<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(EventInner::new())),
        }
    }

    pub fn subscribe<F>(&self, handler: F) -> SubscriptionId
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        let mut inner = self.inner.write().expect("event lock poisoned");

        let id = inner.next_subscription_id();
        inner.fn_handlers.insert(id, Arc::new(handler));

        id
    }

    pub fn subscribe_mut<F>(&self, handler: F) -> SubscriptionId
    where
        F: FnMut(&T) + Send + 'static,
    {
        let mut inner = self.inner.write().expect("event lock poisoned");

        let id = inner.next_subscription_id();

        inner
            .fn_mut_handlers
            .insert(id, Arc::new(Mutex::new(Box::new(handler))));

        id
    }

    pub fn subscribe_once<F>(&self, handler: F) -> SubscriptionId
    where
        F: FnOnce(&T) + Send + 'static,
    {
        let mut inner = self.inner.write().expect("event lock poisoned");

        let id = inner.next_subscription_id();
        inner.fn_once_handlers.insert(id, Box::new(handler));

        id
    }

    pub fn unsubscribe(&self, id: SubscriptionId) -> bool {
        let mut inner = self.inner.write().expect("event lock poisoned");

        let removed_fn = inner.fn_handlers.remove(&id).is_some();
        let removed_fn_mut = inner.fn_mut_handlers.remove(&id).is_some();
        let removed_fn_once = inner.fn_once_handlers.remove(&id).is_some();

        removed_fn || removed_fn_mut || removed_fn_once
    }

    pub fn emit(&self, event: &T) {
        let (fn_handlers, fn_mut_handlers, fn_once_handlers) = {
            let mut inner = self.inner.write().expect("event lock poisoned");

            let fn_handlers = inner.fn_handlers.values().cloned().collect::<Vec<_>>();

            let fn_mut_handlers = inner.fn_mut_handlers.values().cloned().collect::<Vec<_>>();

            let fn_once_handlers = inner
                .fn_once_handlers
                .drain()
                .map(|(_, handler)| handler)
                .collect::<Vec<_>>();

            (fn_handlers, fn_mut_handlers, fn_once_handlers)
        };

        for handler in fn_handlers {
            handler(event);
        }

        for handler in fn_mut_handlers {
            let mut handler = handler.lock().expect("FnMut handler lock poisoned");
            handler(event);
        }

        for handler in fn_once_handlers {
            handler(event);
        }
    }

    #[must_use]
    pub fn subscriber_count(&self) -> usize {
        let inner = self.inner.read().expect("event lock poisoned");
        inner.subscriber_count()
    }
}

impl<T> Default for Event<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Debug for Event<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Event")
            .field("subscriber_count", &self.subscriber_count())
            .finish()
    }
}

struct AsyncEventInner<T> {
    next_id: u64,
    readers: HashMap<SubscriptionId, Weak<AsyncSubscriber<T>>>,
}

impl<T> AsyncEventInner<T> {
    fn new() -> Self {
        Self {
            next_id: 1,
            readers: HashMap::new(),
        }
    }

    fn next_subscription_id(&mut self) -> SubscriptionId {
        let id = SubscriptionId(self.next_id);

        self.next_id = self
            .next_id
            .checked_add(1)
            .expect("subscription id overflow");

        id
    }
}

struct AsyncSubscriber<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

impl<T> AsyncSubscriber<T> {
    fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            available: Condvar::new(),
        }
    }

    fn push(&self, event: T) {
        let mut queue = self.queue.lock().expect("async event queue lock poisoned");
        queue.push_back(event);
        self.available.notify_one();
    }

    fn wait(&self) -> T {
        let mut queue = self.queue.lock().expect("async event queue lock poisoned");

        loop {
            if let Some(event) = queue.pop_front() {
                return event;
            }

            queue = self
                .available
                .wait(queue)
                .expect("async event queue lock poisoned");
        }
    }

    fn wait_timeout(&self, timeout: Duration) -> Option<T> {
        let queue = self.queue.lock().expect("async event queue lock poisoned");

        let (mut queue, timeout_result) = self
            .available
            .wait_timeout_while(queue, timeout, VecDeque::is_empty)
            .expect("async event queue lock poisoned");

        if timeout_result.timed_out() && queue.is_empty() {
            return None;
        }

        queue.pop_front()
    }

    fn try_recv(&self) -> Option<T> {
        let mut queue = self.queue.lock().expect("async event queue lock poisoned");
        queue.pop_front()
    }

    fn pending_count(&self) -> usize {
        let queue = self.queue.lock().expect("async event queue lock poisoned");
        queue.len()
    }
}

#[derive(Clone)]
pub struct AsyncEvent<T> {
    inner: Arc<Mutex<AsyncEventInner<T>>>,
    inbox: Arc<AsyncSubscriber<T>>,
}

impl<T> AsyncEvent<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(AsyncEventInner::new())),
            inbox: Arc::new(AsyncSubscriber::new()),
        }
    }

    pub fn emit(&self, event: &T) -> usize
    where
        T: Clone,
    {
        self.inbox.push(event.clone());
        let mut delivered = 1;

        {
            let mut inner = self.inner.lock().expect("async event lock poisoned");
            let mut dead_readers = Vec::new();

            for (id, reader) in &inner.readers {
                let Some(reader) = reader.upgrade() else {
                    dead_readers.push(*id);
                    continue;
                };

                reader.push(event.clone());
                delivered += 1;
            }

            for id in dead_readers {
                inner.readers.remove(&id);
            }
        }

        notify_event_generation_changed();
        delivered
    }

    pub fn wait(&self) -> T {
        self.inbox.wait()
    }

    pub fn wait_timeout(&self, timeout: Duration) -> Option<T> {
        self.inbox.wait_timeout(timeout)
    }

    pub fn try_recv(&self) -> Option<T> {
        self.inbox.try_recv()
    }

    #[must_use]
    pub fn pending_count(&self) -> usize {
        self.inbox.pending_count()
    }

    #[must_use]
    pub fn reader(&self) -> EventReader<T> {
        let subscriber = Arc::new(AsyncSubscriber::new());

        let mut inner = self.inner.lock().expect("async event lock poisoned");
        let id = inner.next_subscription_id();
        inner.readers.insert(id, Arc::downgrade(&subscriber));

        EventReader {
            id,
            event: Arc::downgrade(&self.inner),
            subscriber,
        }
    }

    #[must_use]
    pub fn subscribe(&self) -> EventReader<T> {
        self.reader()
    }

    #[must_use]
    pub fn reader_count(&self) -> usize {
        let mut inner = self.inner.lock().expect("async event lock poisoned");

        inner.readers.retain(|_, reader| reader.strong_count() > 0);
        inner.readers.len()
    }

    #[must_use]
    pub fn subscriber_count(&self) -> usize {
        self.reader_count()
    }
}

impl<T> Default for AsyncEvent<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Debug for AsyncEvent<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AsyncEvent")
            .field("pending_count", &self.pending_count())
            .field("reader_count", &self.reader_count())
            .finish()
    }
}

pub struct EventReader<T> {
    id: SubscriptionId,
    event: Weak<Mutex<AsyncEventInner<T>>>,
    subscriber: Arc<AsyncSubscriber<T>>,
}

impl<T> EventReader<T> {
    #[must_use]
    pub const fn id(&self) -> SubscriptionId {
        self.id
    }

    pub fn wait(&self) -> T {
        self.subscriber.wait()
    }

    pub fn wait_timeout(&self, timeout: Duration) -> Option<T> {
        self.subscriber.wait_timeout(timeout)
    }

    pub fn try_recv(&self) -> Option<T> {
        self.subscriber.try_recv()
    }

    #[must_use]
    pub fn pending_count(&self) -> usize {
        self.subscriber.pending_count()
    }
}

impl<T> Drop for EventReader<T> {
    fn drop(&mut self) {
        let Some(event) = self.event.upgrade() else {
            return;
        };

        let mut inner = event.lock().expect("async event lock poisoned");
        inner.readers.remove(&self.id);
    }
}

impl<T> fmt::Debug for EventReader<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EventReader")
            .field("id", &self.id)
            .field("pending_count", &self.pending_count())
            .finish()
    }
}

struct EventGenerationSignal {
    generation: Mutex<u64>,
    changed: Condvar,
}

impl EventGenerationSignal {
    fn new() -> Self {
        Self {
            generation: Mutex::new(0),
            changed: Condvar::new(),
        }
    }
}

static EVENT_GENERATION_SIGNAL: std::sync::OnceLock<EventGenerationSignal> =
    std::sync::OnceLock::new();

fn generation_signal() -> &'static EventGenerationSignal {
    EVENT_GENERATION_SIGNAL.get_or_init(EventGenerationSignal::new)
}

fn notify_event_generation_changed() {
    let signal = generation_signal();
    let mut generation = signal
        .generation
        .lock()
        .expect("event generation lock poisoned");

    *generation = generation.wrapping_add(1);
    signal.changed.notify_all();
}

#[doc(hidden)]
#[must_use]
pub fn event_generation() -> u64 {
    let signal = generation_signal();
    let generation = signal
        .generation
        .lock()
        .expect("event generation lock poisoned");

    *generation
}

#[doc(hidden)]
pub fn wait_for_event_generation_change(observed_generation: u64) {
    let signal = generation_signal();
    let mut generation = signal
        .generation
        .lock()
        .expect("event generation lock poisoned");

    while *generation == observed_generation {
        generation = signal
            .changed
            .wait(generation)
            .expect("event generation lock poisoned");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    #[derive(Debug, Clone)]
    struct TestEvent {
        value: usize,
    }

    #[derive(Debug, Clone)]
    struct OtherEvent {
        value: usize,
    }

    #[test]
    fn event_macro_creates_typed_event() {
        let event = crate::event! { TestEvent };

        assert_eq!(event.subscriber_count(), 0);
    }

    #[test]
    fn subscribe_and_emit_invokes_fn_handlers() {
        let event = Event::<TestEvent>::new();
        let observed = Arc::new(AtomicUsize::new(0));
        let observed_clone = Arc::clone(&observed);

        event.subscribe(move |payload| {
            observed_clone.store(payload.value, Ordering::SeqCst);
        });

        event.emit(&TestEvent { value: 42 });

        assert_eq!(observed.load(Ordering::SeqCst), 42);
    }

    #[test]
    fn fn_mut_handlers_can_hold_mutable_state() {
        let event = Event::<TestEvent>::new();
        let observed = Arc::new(AtomicUsize::new(0));
        let observed_clone = Arc::clone(&observed);

        event.subscribe_mut({
            let mut count = 0;

            move |_| {
                count += 1;
                observed_clone.store(count, Ordering::SeqCst);
            }
        });

        event.emit(&TestEvent { value: 1 });
        event.emit(&TestEvent { value: 2 });

        assert_eq!(observed.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn fn_once_handlers_are_called_only_once() {
        let event = Event::<TestEvent>::new();
        let observed = Arc::new(AtomicUsize::new(0));
        let observed_clone = Arc::clone(&observed);

        event.subscribe_once(move |_| {
            observed_clone.fetch_add(1, Ordering::SeqCst);
        });

        event.emit(&TestEvent { value: 1 });
        event.emit(&TestEvent { value: 2 });

        assert_eq!(observed.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn unsubscribe_removes_handler() {
        let event = Event::<TestEvent>::new();
        let observed = Arc::new(AtomicUsize::new(0));
        let observed_clone = Arc::clone(&observed);

        let id = event.subscribe(move |_| {
            observed_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert!(event.unsubscribe(id));
        event.emit(&TestEvent { value: 1 });

        assert_eq!(observed.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn event_can_be_emitted_from_multiple_threads() {
        let event = Event::<TestEvent>::new();
        let observed = Arc::new(AtomicUsize::new(0));
        let observed_clone = Arc::clone(&observed);

        event.subscribe(move |_| {
            observed_clone.fetch_add(1, Ordering::SeqCst);
        });

        let event_a = event.clone();
        let event_b = event.clone();

        let worker_a = thread::spawn(move || event_a.emit(&TestEvent { value: 1 }));
        let worker_b = thread::spawn(move || event_b.emit(&TestEvent { value: 2 }));

        worker_a.join().expect("worker A panicked");
        worker_b.join().expect("worker B panicked");

        assert_eq!(observed.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn async_event_waits_directly_for_payload() {
        let event = crate::async_event! { TestEvent };
        let publisher = event.clone();

        let worker = thread::spawn(move || {
            publisher.emit(&TestEvent { value: 99 });
        });

        let payload = event.wait();
        worker.join().expect("worker panicked");

        assert_eq!(payload.value, 99);
    }

    #[test]
    fn async_event_timeout_returns_none() {
        let event = crate::async_event! { TestEvent };

        assert!(event.wait_timeout(Duration::from_millis(1)).is_none());
    }

    #[test]
    fn async_event_can_create_additional_independent_readers() {
        let event = crate::async_event! { TestEvent };
        let reader = event.reader();

        event.emit(&TestEvent { value: 11 });

        assert_eq!(event.wait().value, 11);
        assert_eq!(reader.wait().value, 11);
    }

    #[test]
    fn select_events_reacts_to_first_available_event() {
        let test_event = crate::async_event! { TestEvent };
        let other_event = crate::async_event! { OtherEvent };

        other_event.emit(&OtherEvent { value: 7 });

        let observed = crate::select_events! {
            test_event => |event| {
                event.value
            },
            other_event => |event| {
                event.value
            },
        };

        assert_eq!(observed, 7);
    }
}
