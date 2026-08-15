const ACTIVE_THRESHOLD: f64 = 60.0;
const BREAK_THRESHOLD: f64 = 180.0;

pub struct SessionTracker {
    session_seconds: f64,
}

impl SessionTracker {
    pub fn new() -> Self {
        Self {
            session_seconds: 0.0,
        }
    }

    pub fn update(
        &mut self,
        idle_seconds: f64,
        dt: f64,
    ) -> f64 {
        if idle_seconds < ACTIVE_THRESHOLD {
            self.session_seconds += dt;
        } else if idle_seconds >= BREAK_THRESHOLD {
            self.session_seconds = 0.0;
        }

        self.session_seconds
    }

    pub fn reset(&mut self) {
        self.session_seconds = 0.0;
    }

}
