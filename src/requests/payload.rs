use std::time::Duration;

pub trait Payload {
    type Output;

    fn name(&self) -> String;

    fn timeout_hint(&self) -> Option<Duration> {
        None
    }
}
