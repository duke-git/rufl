use super::event::Event;
use super::subscriber::Subscriber;
use std::collections::HashMap;

/// # Event Bus
///
/// The event bus is a central hub for all events.
/// It is responsible for managing all subscribers and publishing events
/// related to the event bus.
///
/// ## Fields
///
/// * `events` - A vec of events grouped by their name that have been published to the event bus.
///
/// * `subscribers` - A vec of subscribers grouped by an event name.
///
/// ## Methods
///
/// * `publish` - Publishes an event to the event bus, and calls each listener's handle method
///
/// * `subscribe` - Subscribes a listener to the event bus.
///
/// * `subscribe_all` - Subscribes a vector of listeners to the event bus.
///
/// * `clear` - Clears all events from the event bus.
pub struct EventBus {
    events: HashMap<String, Vec<Box<Event>>>,
    subscribers: HashMap<String, Vec<Vec<Box<dyn Subscriber>>>>,
}

impl EventBus {
    /// # New
    ///
    /// Creates a new event bus.
    pub fn new() -> Self {
        EventBus {
            events: HashMap::new(),
            subscribers: HashMap::new(),
            // subs: HashMap::new(),
        }
    }

    /// # Register
    ///
    /// Registers an event with the event bus.
    pub fn register(&mut self, event_name: &str, event: Event) -> &mut Self {
        if self.events.contains_key(event_name) {
            self.events
                .get_mut(event_name)
                .unwrap()
                .push(Box::new(event));
        } else {
            self.events
                .insert(event_name.to_string(), vec![Box::new(event)]);
        }

        self
    }

    /// # Subscribe
    ///
    /// Subscribes a listener to the event bus.
    pub fn subscribe<T: Subscriber + 'static>(
        &mut self,
        event_name: &str,
        listener: T,
    ) -> &mut Self {
        if self.subscribers.contains_key(event_name) {
            self.subscribers
                .get_mut(event_name)
                .unwrap()
                .push(vec![Box::new(listener)]);
        } else {
            self.subscribers
                .insert(event_name.to_string(), vec![vec![Box::new(listener)]]);
        }

        self
    }

    /// # Subscribe all
    ///
    /// Subscribes a vector of listeners to the event bus.
    pub fn subscribe_all(
        &mut self,
        event_name: &str,
        listeners: Vec<Box<dyn Subscriber>>,
    ) -> &mut Self {
        if self.subscribers.contains_key(event_name) {
            self.subscribers
                .get_mut(event_name)
                .unwrap()
                .append(&mut vec![listeners]);
        } else {
            self.subscribers
                .insert(event_name.to_string(), vec![listeners]);
        }

        self
    }

    /// # Publish
    ///
    /// Publishes each event, and calls each listener's methods in order of on_before, on_event, on_after.
    pub fn publish(&mut self) -> Result<(), String> {
        for (event_name, mut events) in self.events.drain() {
            if self.subscribers.contains_key(&event_name) {
                for event in &mut events {
                    for listeners in self.subscribers.get_mut(&event_name).unwrap().iter_mut() {
                        for listener in listeners {
                            // handle before
                            match listener.handle_before(event) {
                                Err(event) => return Err(event),
                                _ => {}
                            }

                            // handle event
                            match listener.handle_event(event) {
                                Err(event) => return Err(event),
                                _ => {}
                            }
                            // handle after
                            match listener.handle_after(event) {
                                Err(event) => return Err(event),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// # Clear
    ///
    /// Clears all events from the event bus.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, EventBus, Subscriber};

    #[test]
    fn test_publish() {
        let mut event_bus = EventBus::new();

        let mut listeners: Vec<Box<dyn Subscriber>> = Vec::new();
        listeners.push(Box::new(TestSubscriber1::new()));
        listeners.push(Box::new(TestSubscriber2::new()));

        event_bus.subscribe_all("foo", listeners);

        event_bus.subscribe("foo", TestSubscriber3::new());

        let result = event_bus
            .register("foo", Event::new("hello".to_string()))
            .publish();

        assert_eq!(Ok(()), result);
    }

    struct TestSubscriber1 {}

    impl TestSubscriber1 {
        const NAME: &'static str = "TestSubscriber1";
        // const PRIORITY: usize = 1;

        pub fn new() -> Self {
            TestSubscriber1 {}
        }
    }

    impl Subscriber for TestSubscriber1 {
        fn handle_before(&mut self, _event: &mut Event) -> Result<(), String> {
            println!("TestSubscriber1 handle_before: ");
            Ok(())
        }

        fn handle_event(&mut self, event: &mut Event) -> Result<(), String> {
            match event.get_data::<String>() {
                Some(value) => {
                    println!(
                        "{} received string message: {}",
                        TestSubscriber1::NAME,
                        value
                    );
                    Ok(())
                }
                None => {
                    let message = format!("{} received unknown message", TestSubscriber1::NAME);
                    Err(message)
                }
            }
        }

        fn handle_after(&self, _event: &Event) -> Result<(), String> {
            println!("TestSubscriber1 handle_after: ");
            Ok(())
        }
    }

    struct TestSubscriber2 {}

    impl TestSubscriber2 {
        const NAME: &'static str = "TestSubscriber2";
        // const PRIORITY: usize = 1;

        pub fn new() -> Self {
            TestSubscriber2 {}
        }
    }

    impl Subscriber for TestSubscriber2 {
        fn handle_before(&mut self, _event: &mut Event) -> Result<(), String> {
            println!("TestSubscriber2 handle_before: ");
            Ok(())
        }

        fn handle_event(&mut self, event: &mut Event) -> Result<(), String> {
            match event.get_data::<String>() {
                Some(value) => {
                    println!(
                        "{} received string message: {}",
                        TestSubscriber2::NAME,
                        value
                    );
                    Ok(())
                }
                None => {
                    let message = format!("{} received unknown message", TestSubscriber2::NAME);
                    Err(message)
                }
            }
        }

        fn handle_after(&self, _event: &Event) -> Result<(), String> {
            println!("TestSubscriber2 handle_after: ");
            Ok(())
        }
    }

    struct TestSubscriber3 {}

    impl TestSubscriber3 {
        const NAME: &'static str = "TestSubscriber3";
        // const PRIORITY: usize = 1;

        pub fn new() -> Self {
            TestSubscriber3 {}
        }
    }

    impl Subscriber for TestSubscriber3 {
        fn handle_before(&mut self, _event: &mut Event) -> Result<(), String> {
            println!("TestSubscriber3 handle_before: ");
            Ok(())
        }

        fn handle_event(&mut self, event: &mut Event) -> Result<(), String> {
            match event.get_data::<String>() {
                Some(value) => {
                    println!(
                        "{} received string message: {}",
                        TestSubscriber3::NAME,
                        value
                    );
                    Ok(())
                }
                None => {
                    let message = format!("{} received unknown message", TestSubscriber3::NAME);
                    Err(message)
                }
            }
        }

        fn handle_after(&self, _event: &Event) -> Result<(), String> {
            println!("TestSubscriber3 handle_after: ");
            Ok(())
        }
    }
}
