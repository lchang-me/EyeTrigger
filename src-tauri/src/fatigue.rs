const WORK_TAU: f64 = 1800.0;

// 普通短暂停顿恢复比较慢
const SHORT_REST_TAU: f64 = 1800.0;

// 真正离开电脑后恢复较快
const REST_TAU: f64 = 600.0;

const ACTIVE_THRESHOLD: f64 = 60.0;
const BREAK_THRESHOLD: f64 = 180.0;

// Level 1：用户确认已经远眺约20秒
const GENTLE_BREAK_CREDIT: f64 = 0.06;

// Level 2：完成正式 Guided Break
const GUIDED_BREAK_CREDIT: f64 = 0.22;

pub struct FatigueEngine {
    risk: f64,
}

impl FatigueEngine {
    pub fn new() -> Self {
        Self {
            risk: 0.0,
        }
    }

    pub fn update(
        &mut self,
        idle_seconds: f64,
        break_active: bool,
        dt: f64,
    ) -> f64 {
        if break_active {
            // EyeTrigger 主动休息模式
            self.risk *= (-dt / REST_TAU).exp();
        } else if idle_seconds < ACTIVE_THRESHOLD {
            // 正在使用电脑
            //
            // dR/dt = (1-R) / WORK_TAU
            self.risk =
                1.0
                - (1.0 - self.risk)
                    * (-dt / WORK_TAU).exp();
        } else if idle_seconds < BREAK_THRESHOLD {
            // 短暂停顿
            self.risk *=
                (-dt / SHORT_REST_TAU).exp();
        } else {
            // 真正离开电脑
            self.risk *=
                (-dt / REST_TAU).exp();
        }

        self.risk =
            self.risk.clamp(0.0, 1.0);

        self.risk
    }

    // Level 1：
    // 用户看远约20秒后主动点击确认
    pub fn complete_gentle_break(&mut self) {
        self.risk =
            (self.risk - GENTLE_BREAK_CREDIT)
                .max(0.0);
    }

    // Level 2：
    // 完成正式 Guided Break
    pub fn complete_short_break(&mut self) {
        self.risk =
            (self.risk - GUIDED_BREAK_CREDIT)
                .max(0.0);
    }

    // Mac sleep / suspend 时补算休息
    pub fn apply_rest(&mut self, seconds: f64) {
        self.risk *=
            (-seconds / REST_TAU).exp();

        self.risk =
            self.risk.clamp(0.0, 1.0);
    }

    pub fn risk(&self) -> f64 {
        self.risk
    }
}
