use std::any::Any;

/// # Event
///
/// An event is a struct that can
/// hold any data type. It is
/// then published to the event
/// bus. Once published, the event
/// is then passed to each subscriber
/// when the event bus runs.
///
/// ## Fields
///
/// * `data` - The data that is held by the event.
///
/// ## Methods
///
/// * `new` - Creates a new event.
///
/// * `get_data` - Returns the data held by the event.
///
/// * `set_data` - Updates the data held by the event.
pub struct Event {
    pub data: Box<dyn Any>,
}

impl Event {
    /// # New
    ///
    /// Creates a new event.
    pub fn new<T: 'static>(data: T) -> Self {
        Event {
            data: Box::new(data),
        }
    }

    /// # Get Data
    ///
    /// Returns the data held by the event.
    pub fn get_data<T: 'static>(&self) -> Option<&T> {
        self.data.downcast_ref::<T>()
    }

    /// # Set Data
    ///
    /// Updates the data held by the event.
    pub fn set_data<T: 'static>(&mut self, data: T) {
        self.data = Box::new(data);
    }
}
