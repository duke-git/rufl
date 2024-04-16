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
/// * `subscribe_all` - Subscribes a vec of listeners to the event bus.
///
/// * `unsubscribe` - unubscribes a listener to the event bus.
///
/// * `clear` - Clears all events from the event bus.
///
/// # Examples
///
/// ```
/// use rufl::eventbus::{Event, EventBus, Subscriber};
///
///
/// let mut event_bus = EventBus::new();
/// event_bus.subscribe("foo", Box::new(MySubscriber1::new()))
///          .subscribe("foo", Box::new(MySubscriber2::new()));
///
/// let result = event_bus
///            .register("foo", Event::new("hello".to_string()))
///            .publish();
///     
/// //MySubscriber2 will run first because its priority is higher than MySubscriber1
/// assert_eq!(Ok(()), result);
///
/// #[derive(Clone, Copy)]
/// struct MySubscriber1 {}
///
/// impl MySubscriber1 {
///     const NAME: &'static str = "MySubscriber1";
///
///     pub fn new() -> Self {
///         MySubscriber1 {}
///     }
/// }
///
/// impl Subscriber for MySubscriber1 {
///     fn id(&self) -> String {
///         "subscriber1".to_string()
///     }
///
///     fn priority(&self) -> usize {
///         2
///     }
///
///     fn handle_before(&mut self, _event: &mut Event) -> Result<(), String> {
///        println!("MySubscriber1 handle_before: ");
///        Ok(())
///     }
///
///     fn handle_event(&mut self, event: &mut Event) -> Result<(), String> {
///        match event.get_data::<String>() {
///            Some(value) => {
///                println!("{} received event message: {}", MySubscriber1::NAME, value);
///                Ok(())
///            }
///            None => {
///                let message = format!("{} received error message", MySubscriber1::NAME);
///                    Err(message)
///                }
///            }
///        }
///
///    fn handle_after(&self, _event: &Event) -> Result<(), String> {
///       println!("MySubscriber1 handle_after: ");
///       Ok(())
///    }
/// }
/// #[derive(Clone, Copy)]
/// struct MySubscriber2 {}
///
/// impl MySubscriber2 {
///   const NAME: &'static str = "MySubscriber2";
///
///   pub fn new() -> Self {
///       MySubscriber2 {}
///   }
/// }
///
/// impl Subscriber for MySubscriber2 {
///     fn id(&self) -> String {
///         "subscriber2".to_string()
///     }
///
///     fn priority(&self) -> usize {
///         1
///     }
///
///     fn handle_before(&mut self, _event: &mut Event) -> Result<(), String> {
///        println!("MySubscriber2 handle_before: ");
///            Ok(())
///        }
///
///     fn handle_event(&mut self, event: &mut Event) -> Result<(), String> {
///        match event.get_data::<String>() {
///            Some(value) => {
///                println!("{} received event message: {}", MySubscriber2::NAME, value);
///                Ok(())
///            }
///            None => {
///                let message = format!("{} received error message", MySubscriber2::NAME);
///                    Err(message)
///                }
///            }
///        }
///
///        fn handle_after(&self, _event: &Event) -> Result<(), String> {
///            println!("MySubscriber2 handle_after: ");
///            Ok(())
///        }
///    }
/// ```
pub struct EventBus {
    events: HashMap<String, Vec<Box<Event>>>,
    subscribers: HashMap<String, Vec<Box<dyn Subscriber>>>,
}

impl EventBus {
    /// # New
    ///
    /// Creates a new event bus.
    pub fn new() -> Self {
        EventBus {
            events: HashMap::new(),
            subscribers: HashMap::new(),
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
    pub fn subscribe(&mut self, event_name: &str, listener: Box<dyn Subscriber>) -> &mut Self {
        if self.subscribers.contains_key(event_name) {
            self.subscribers.get_mut(event_name).unwrap().push(listener);
        } else {
            self.subscribers
                .insert(event_name.to_string(), vec![listener]);
        }

        self.sort_subscribers_by_priority(event_name);

        self
    }

    /// # Subscribe all
    ///
    /// Subscribes a vec of listeners to the event bus.
    pub fn subscribe_all(
        &mut self,
        event_name: &str,
        listeners: Vec<Box<dyn Subscriber>>,
    ) -> &mut Self {
        if self.subscribers.contains_key(event_name) {
            for listener in listeners {
                self.subscribers.get_mut(event_name).unwrap().push(listener);
            }
        } else {
            self.subscribers.insert(event_name.to_string(), listeners);
        }

        self.sort_subscribers_by_priority(event_name);

        self
    }

    /// # Unsubscribe
    ///
    /// Unsubscribes a listener to the event bus.
    pub fn unsubscribe<T: Subscriber + 'static>(
        &mut self,
        event_name: &str,
        listener: T,
    ) -> &mut Self {
        if self.subscribers.contains_key(event_name) {
            let index = self
                .subscribers
                .get(event_name)
                .unwrap()
                .iter()
                .position(|item| *item.id() == listener.id())
                .unwrap();
            self.subscribers.get_mut(event_name).unwrap().remove(index);

            println!("123");
        }
        self
    }

    /// # Publish
    ///
    /// Publishes each event, and calls each listener by priority value. (lower priority value is high order)
    /// in listener's methods the order is: handle_before -> handle_event -> handle_after.
    pub fn publish(&mut self) -> Result<(), String> {
        for (event_name, mut events) in self.events.drain() {
            if self.subscribers.contains_key(&event_name) {
                for event in &mut events {
                    for listener in self.subscribers.get_mut(&event_name).unwrap().iter_mut() {
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

        Ok(())
    }

    /// # Clear
    ///
    /// Clears all events from the event bus.
    pub fn clear(&mut self) {
        self.events.clear();
    }

    fn sort_subscribers_by_priority(&mut self, event_name: &str) {
        if self.subscribers.contains_key(event_name) {
            self.subscribers
                .get_mut(event_name)
                .unwrap()
                .sort_by(|a, b| a.priority().cmp(&b.priority()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, EventBus, Subscriber};

    #[test]
    fn test_publish() {
        let mut event_bus = EventBus::new();

        let listener1 = TestSubscriber1::new();
        let listener2 = TestSubscriber2::new();
        let listeners: Vec<Box<dyn Subscriber>> = vec![Box::new(listener1), Box::new(listener2)];

        event_bus.subscribe_all("foo", listeners);

        let listener3 = TestSubscriber3::new();
        event_bus.subscribe("foo", Box::new(listener3));

        let result = event_bus
            .register("foo", Event::new("hello".to_string()))
            .publish();

        assert_eq!(Ok(()), result);

        event_bus.unsubscribe("foo", listener3);

        let result = event_bus
            .register("foo", Event::new("hello".to_string()))
            .publish();

        assert_eq!(Ok(()), result);
    }

    #[derive(Clone, Copy)]
    struct TestSubscriber1 {}

    impl TestSubscriber1 {
        const NAME: &'static str = "TestSubscriber1";

        pub fn new() -> Self {
            TestSubscriber1 {}
        }
    }

    impl Subscriber for TestSubscriber1 {
        fn id(&self) -> String {
            "subscriber1".to_string()
        }

        fn priority(&self) -> usize {
            2
        }

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

    #[derive(Clone, Copy)]
    struct TestSubscriber2 {}

    impl TestSubscriber2 {
        const NAME: &'static str = "TestSubscriber2";

        pub fn new() -> Self {
            TestSubscriber2 {}
        }
    }

    impl Subscriber for TestSubscriber2 {
        fn id(&self) -> String {
            "subscriber2".to_string()
        }

        fn priority(&self) -> usize {
            1
        }

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

    #[derive(Clone, Copy)]
    struct TestSubscriber3 {}

    impl TestSubscriber3 {
        const NAME: &'static str = "TestSubscriber3";

        pub fn new() -> Self {
            TestSubscriber3 {}
        }
    }

    impl Subscriber for TestSubscriber3 {
        fn id(&self) -> String {
            "subscriber3".to_string()
        }

        fn priority(&self) -> usize {
            3
        }

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
