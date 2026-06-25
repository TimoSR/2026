pub struct Event<T> {
    subscribers: Vec<Box<dyn Fn(&T)>>,
}

impl<T> Event<T> {
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    pub fn subscribe<F>(&mut self, subscriber: F)
    where
        F: Fn(&T) + 'static,
    {
        self.subscribers.push(Box::new(subscriber));
    }

    pub fn emit(&self, event: &T) {
        for subscriber in &self.subscribers {
            subscriber(event);
        }
    }
}

impl<T> Default for Event<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct UserRegistered {
    pub user_id: u64,
    pub email: String,
}

fn main() {
    let mut user_registered = Event::<UserRegistered>::new();

    user_registered.subscribe(|event| {
        println!("subscriber 1: user registered: {:?}", event);
    });

    user_registered.subscribe(|event| {
        println!("subscriber 2: welcome email sent to {}", event.email);
    });

    let event = UserRegistered {
        user_id: 42,
        email: "timothy@example.com".to_string(),
    };

    user_registered.emit(&event);
}