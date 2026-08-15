const GENTLE_START: f64 = 0.50;

// 每完成一次小休息，
// 下一次小提醒提高8个百分点。
//
// 50% -> 58% -> 66%
const GENTLE_STEP: f64 = 0.08;

const BREAK_LOAD: f64 = 0.74;

const STRONG_LOAD: f64 = 0.86;

// 足够长的自然休息后也认为进入新周期
const RESET_LOAD: f64 = 0.30;


pub struct TriggerEngine {
    current_level: u8,

    // 下一次 Gentle Reminder
    next_gentle_load: f64,

    // 是否已经欠着一个“大休息”
    //
    // Level 2 出现以后为 true。
    // 只有真正完成大休息以后才变回 false。
    break_pending: bool,
}


impl TriggerEngine {
    pub fn new() -> Self {
        Self {
            current_level: 0,

            next_gentle_load:
                GENTLE_START,

            break_pending: false,
        }
    }


    pub fn update(
        &mut self,
        risk: f64,
    ) -> u8 {
        // ====================================================
        // 非常充分的自然恢复
        // ====================================================

        if risk < RESET_LOAD {
            self.current_level = 0;

            self.next_gentle_load =
                GENTLE_START;

            self.break_pending = false;

            return 0;
        }


        // ====================================================
        // 已经欠着 Level 2 大休息
        //
        // 此时不再给小提醒。
        //
        // 如果继续工作到86%，
        // 直接进入强制休息。
        // ====================================================

        if self.break_pending {
            if risk >= STRONG_LOAD
                && self.current_level != 3
            {
                self.current_level = 3;

                println!(
                    "EyeTrigger: STRONG reminder at {:.1}%",
                    risk * 100.0
                );
            }

            return self.current_level;
        }


        // ====================================================
        // Level 2
        //
        // 一旦达到74%，
        // 大休息就成为 pending。
        // ====================================================

        if risk >= BREAK_LOAD {
            self.break_pending = true;

            self.current_level = 2;

            println!(
                "EyeTrigger: BREAK reminder at {:.1}%",
                risk * 100.0
            );

            return 2;
        }


        // ====================================================
        // Level 1
        //
        // 一个大休息周期中可以出现多次。
        //
        // 50% -> 58% -> 66%
        // ====================================================

        if self.current_level == 0
            && self.next_gentle_load
                < BREAK_LOAD
            && risk
                >= self.next_gentle_load
        {
            self.current_level = 1;

            println!(
                "EyeTrigger: GENTLE reminder at {:.1}%",
                risk * 100.0
            );

            return 1;
        }


        self.current_level
    }


    // ========================================================
    // 完成一次20秒小休息
    // ========================================================

    pub fn acknowledge_gentle(
        &mut self,
    ) {
        if self.current_level != 1 {
            return;
        }

        self.current_level = 0;

        // 下一次 Gentle 的阈值提高
        //
        // 50 -> 58 -> 66 -> 74
        self.next_gentle_load +=
            GENTLE_STEP;
    }


    // ========================================================
    // 用户点 Later
    //
    // 关闭 Level 2 窗口，
    // 但“大休息债务”仍然存在。
    // ========================================================

    pub fn defer_break(
        &mut self,
    ) {
        if self.break_pending {
            self.current_level = 0;
        }
    }


    // ========================================================
    // 真正完成3分钟大休息
    //
    // 开始新的 reminder cycle。
    // ========================================================

    pub fn complete_break(
        &mut self,
    ) {
        self.current_level = 0;

        self.break_pending = false;

        self.next_gentle_load =
            GENTLE_START;
    }
}
