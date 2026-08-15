use objc2_core_graphics::{
    CGEventSource,
    CGEventSourceStateID,
    CGEventType,
};

pub fn idle_seconds() -> f64 {
    CGEventSource::seconds_since_last_event_type(
        CGEventSourceStateID::HIDSystemState,
        CGEventType(u32::MAX),
    )
}
