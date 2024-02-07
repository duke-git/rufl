use super::event::Event;

pub trait Subscriber {
    /// Get subscriber id property
    fn id(&self) -> String {
        "".to_string()
    }

    /// Get subscriber priority property, the lowwer value is higher priority.
    fn priority(&self) -> usize {
        0
    }

    /// Called before the on_event is run by the event bus
    fn handle_before(&mut self, _event: &mut Event) -> Result<(), String> {
        Ok(())
    }

    /// Called when the event bus is run.
    fn handle_event(&mut self, _event: &mut Event) -> Result<(), String> {
        Ok(())
    }

    /// Called after the handle_event is run by the event bus
    fn handle_after(&self, _event: &Event) -> Result<(), String> {
        Ok(())
    }
}
