use std::time::{Duration, SystemTime};

const SUSPEND_GAP: Duration = Duration::from_secs(5);

pub struct LifecycleClock {
    last_wall: SystemTime,
}

pub struct LifecycleTick {
    pub dt: f64,
    pub suspend_seconds: Option<f64>,
}

impl LifecycleClock {
    pub fn new() -> Self {
        Self {
            last_wall: SystemTime::now(),
        }
    }

    pub fn tick(&mut self) -> LifecycleTick {
        let now = SystemTime::now();

        let elapsed = now
            .duration_since(self.last_wall)
            .unwrap_or(Duration::ZERO);

        self.last_wall = now;

        if elapsed >= SUSPEND_GAP {
            LifecycleTick {
                dt: 0.0,
                suspend_seconds: Some(elapsed.as_secs_f64()),
            }
        } else {
            LifecycleTick {
                dt: elapsed.as_secs_f64(),
                suspend_seconds: None,
            }
        }
    }
}
