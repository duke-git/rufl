//! eventbus implements a simple event bus lib. based on [Simple-Event-Bus](!https://github.com/dimitri-br/Simple-Event-Bus), add some features.
//!

mod event;
mod event_bus;
mod subscriber;

pub use event::Event;
pub use event_bus::EventBus;
pub use subscriber::Subscriber;
