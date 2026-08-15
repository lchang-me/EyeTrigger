mod activity;
mod break_window;
mod dim_window;
mod fatigue;
mod gentle_window;
mod lifecycle;
mod macos_window;
mod session;
mod tray;
mod trigger;

use break_window::{
    close_break_window,
    show_break_window,
};

use dim_window::{
    close_dim_window,
    show_dim_window,
};

use fatigue::FatigueEngine;

use gentle_window::{
    close_gentle_window,
    show_gentle_window,
};

use lifecycle::LifecycleClock;
use session::SessionTracker;
use trigger::TriggerEngine;

use serde::Serialize;
use std::sync::Mutex;
use tauri::{
    Manager,
    State,
};


// ============================================================
// Application state
// ============================================================

struct AppState {
    session: Mutex<SessionTracker>,

    fatigue: Mutex<FatigueEngine>,

    trigger: Mutex<TriggerEngine>,

    break_active: Mutex<bool>,

    status: Mutex<EyeStatus>,

    lifecycle: Mutex<LifecycleClock>,
}


// ============================================================
// Status returned to frontend
// ============================================================

#[derive(Serialize, Clone)]
struct EyeStatus {
    idle_seconds: f64,

    session_seconds: f64,

    fatigue: f64,

    reminder_level: u8,

    active: bool,
}


// ============================================================
// Reminder dispatcher
//
// 现在提醒窗口完全由 Rust 后台负责。
// React 不再负责 show_gentle / show_break / show_dim。
// ============================================================

fn dispatch_reminder(
    app: &tauri::AppHandle,
    previous_level: u8,
    current_level: u8,
) {
    // 只处理“升级”
    //
    // 0 -> 1
    // 0/1 -> 2
    // 0/1/2 -> 3
    if current_level <= previous_level {
        return;
    }

    match current_level {
        // ----------------------------------------------------
        // Level 1
        // ----------------------------------------------------
        1 => {
            println!(
                "EyeTrigger: Level 1 reminder"
            );

            if let Err(error) =
                gentle_window::show_gentle(app)
            {
                eprintln!(
                    "EyeTrigger: failed to show gentle window: {}",
                    error
                );
            }
        }

        // ----------------------------------------------------
        // Level 2
        // ----------------------------------------------------
        2 => {
            println!(
                "EyeTrigger: Level 2 reminder"
            );

            // Level 2 覆盖 Level 1
            if let Err(error) =
                gentle_window::close_gentle(app)
            {
                eprintln!(
                    "EyeTrigger: failed to close gentle window: {}",
                    error
                );
            }

            if let Err(error) =
                break_window::show_break(app)
            {
                eprintln!(
                    "EyeTrigger: failed to show break window: {}",
                    error
                );
            }
        }

        // ----------------------------------------------------
        // Level 3
        // ----------------------------------------------------
        3 => {
            println!(
                "EyeTrigger: Level 3 reminder"
            );

            // Level 3 覆盖所有低等级提醒
            if let Err(error) =
                gentle_window::close_gentle(app)
            {
                eprintln!(
                    "EyeTrigger: failed to close gentle window: {}",
                    error
                );
            }

            if let Err(error) =
                break_window::close_break(app)
            {
                eprintln!(
                    "EyeTrigger: failed to close break window: {}",
                    error
                );
            }

            if let Err(error) =
                dim_window::show_dim(app)
            {
                eprintln!(
                    "EyeTrigger: failed to show dim window: {}",
                    error
                );
            }
        }

        _ => {}
    }
}


// ============================================================
// Background monitor
//
// 这是 EyeTrigger 真正的核心循环。
// 即使主窗口被隐藏、位于别的 Desktop，
// Eye Load 和 Reminder 仍然继续运行。
// ============================================================

fn start_background_monitor(
    app: tauri::AppHandle,
) {
    std::thread::spawn(move || {
        loop {
            let state =
                app.state::<AppState>();


            // =================================================
            // 1. Lifecycle
            // =================================================

            let tick = {
                let mut lifecycle =
                    state
                        .lifecycle
                        .lock()
                        .unwrap();

                lifecycle.tick()
            };


            // =================================================
            // 2. Sleep / suspend recovery
            // =================================================

            if let Some(rest_seconds) =
                tick.suspend_seconds
            {
                {
                    let mut engine =
                        state
                            .fatigue
                            .lock()
                            .unwrap();

                    engine.apply_rest(
                        rest_seconds,
                    );
                }

                // 如果真正离开超过3分钟，
                // session重新开始。
                if rest_seconds >= 180.0 {
                    let mut session =
                        state
                            .session
                            .lock()
                            .unwrap();

                    session.reset();
                }
            }


            // =================================================
            // 3. Read user activity
            // =================================================

            let idle =
                activity::idle_seconds();


            // =================================================
            // 4. Is guided break active?
            // =================================================

            let break_active = {
                *state
                    .break_active
                    .lock()
                    .unwrap()
            };


            // =================================================
            // 5. Session
            // =================================================

            let session_seconds = {
                let mut session =
                    state
                        .session
                        .lock()
                        .unwrap();

                session.update(
                    idle,
                    tick.dt,
                )
            };


            // =================================================
            // 6. Eye Load
            // =================================================

            {
                let mut engine =
                    state
                        .fatigue
                        .lock()
                        .unwrap();

                engine.update(
                    idle,
                    break_active,
                    tick.dt,
                );
            }


            let fatigue = {
                let engine =
                    state
                        .fatigue
                        .lock()
                        .unwrap();

                engine.risk()
            };


            // =================================================
            // 7. Previous reminder level
            //
            // 保存旧状态，用来检测：
            //
            // 0 -> 1
            // 0 -> 2
            // 1 -> 2
            // 2 -> 3
            // =================================================

            let previous_level = {
                let status =
                    state
                        .status
                        .lock()
                        .unwrap();

                status.reminder_level
            };


            // =================================================
            // 8. Trigger Engine
            // =================================================

            let reminder_level = {
                let mut trigger =
                    state
                        .trigger
                        .lock()
                        .unwrap();

                trigger.update(
                    fatigue,
                )
            };


            // =================================================
            // 9. Rust directly shows reminder windows
            //
            // 这是这次最关键的修改。
            // =================================================

            dispatch_reminder(
                &app,
                previous_level,
                reminder_level,
            );


            // =================================================
            // 10. Update shared status
            // =================================================

            {
                let mut status =
                    state
                        .status
                        .lock()
                        .unwrap();

                *status = EyeStatus {
                    idle_seconds: idle,

                    session_seconds,

                    fatigue,

                    reminder_level,

                    active: idle < 60.0,
                };
            }


            // =================================================
            // 11. Update tray Eye Load
            // =================================================

            if let Some(tray) =
                app.tray_by_id(
                    "eyetrigger-tray",
                )
            {
                let percent =
                    (fatigue * 100.0)
                        .round()
                        as u8;

                let _ =
                    tray.set_title(
                        Some(
                            format!(
                                "👁 {}%",
                                percent
                            ),
                        ),
                    );
            }


            // =================================================
            // 12. 1 Hz background loop
            // =================================================

            std::thread::sleep(
                std::time::Duration::from_secs(
                    1,
                ),
            );
        }
    });
}


// ============================================================
// Frontend status
// ============================================================

#[tauri::command]
fn get_eye_status(
    state: State<AppState>,
) -> EyeStatus {
    state
        .status
        .lock()
        .unwrap()
        .clone()
}


// ============================================================
// Level 1
//
// 用户已经看远约20秒，然后点击小提醒。
// ============================================================
#[tauri::command]
fn complete_gentle_break(
    state: State<AppState>,
) {
    {
        let mut engine =
            state
                .fatigue
                .lock()
                .unwrap();

        engine.complete_gentle_break();
    }

    {
        let mut trigger =
            state
                .trigger
                .lock()
                .unwrap();

        trigger.acknowledge_gentle();
    }

    // 非常重要：
    //
    // 立即告诉 background dispatcher：
    // 当前提醒已经回到0。
    {
        let mut status =
            state
                .status
                .lock()
                .unwrap();

        status.reminder_level = 0;
    }
}


// ============================================================
// Level 2 guided break starts
// ============================================================

#[tauri::command]
fn start_break(
    state: State<AppState>,
) {
    let mut active =
        state
            .break_active
            .lock()
            .unwrap();

    *active = true;
}


// ============================================================
// User cancels Level 2 break
// ============================================================

#[tauri::command]
fn cancel_break(
    state: State<AppState>,
) {
    // 没有在进行真正的 Break
    {
        let mut active =
            state
                .break_active
                .lock()
                .unwrap();

        *active = false;
    }

    // Later：
    //
    // 大休息仍然 pending。
    // 后面到86%必须升级 Level 3。
    {
        let mut trigger =
            state
                .trigger
                .lock()
                .unwrap();

        trigger.defer_break();
    }

    {
        let mut status =
            state
                .status
                .lock()
                .unwrap();

        status.reminder_level = 0;
    }
}


// ============================================================
// Level 2 guided break completed
// ============================================================
#[tauri::command]
fn complete_break(
    state: State<AppState>,
) {
    {
        let mut active =
            state
                .break_active
                .lock()
                .unwrap();

        *active = false;
    }


    // 3分钟休息期间已经持续衰减，
    // 完成时再给一次正式 Break credit。
    {
        let mut engine =
            state
                .fatigue
                .lock()
                .unwrap();

        engine.complete_short_break();
    }


    // 一个大休息周期完成。
    //
    // 下一轮重新从 Level 1 开始。
    {
        let mut trigger =
            state
                .trigger
                .lock()
                .unwrap();

        trigger.complete_break();
    }


    {
        let mut status =
            state
                .status
                .lock()
                .unwrap();

        status.reminder_level = 0;
    }
}


// ============================================================
// Run
// ============================================================

#[cfg_attr(
    mobile,
    tauri::mobile_entry_point
)]
pub fn run() {
    tauri::Builder::default()

        // ----------------------------------------------------
        // Global state
        // ----------------------------------------------------
        .plugin(tauri_nspanel::init())
        .manage(
            AppState {
                session:
                    Mutex::new(
                        SessionTracker::new(),
                    ),

                fatigue:
                    Mutex::new(
                        FatigueEngine::new(),
                    ),

                trigger:
                    Mutex::new(
                        TriggerEngine::new(),
                    ),

                break_active:
                    Mutex::new(false),

                lifecycle:
                    Mutex::new(
                        LifecycleClock::new(),
                    ),

                status:
                    Mutex::new(
                        EyeStatus {
                            idle_seconds: 0.0,

                            session_seconds: 0.0,

                            fatigue: 0.0,

                            reminder_level: 0,

                            active: true,
                        },
                    ),
            },
        )


        // ----------------------------------------------------
        // Setup
        // ----------------------------------------------------
        .on_window_event(
            |window, event| {
                if window.label() != "main" {
                    return;
                }
        
                if let tauri::WindowEvent::CloseRequested {
                    api,
                    ..
                } = event
                {
                    // 阻止真正关闭
                    api.prevent_close();
        
                    // 只是隐藏主窗口
                    let _ = window.hide();
                }
            },
        )
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                app.handle()
                    .set_activation_policy(
                        tauri::ActivationPolicy::Accessory,
                    )?;
        
                app.handle()
                    .set_dock_visibility(false)?;
            }
        
            tray::setup_tray(app)?;
        
            start_background_monitor(
                app.handle().clone(),
            );
        
            Ok(())
        })


        // ----------------------------------------------------
        // Commands available to frontend
        // ----------------------------------------------------

        .invoke_handler(
            tauri::generate_handler![
                get_eye_status,

                // Level 1
                show_gentle_window,
                close_gentle_window,
                complete_gentle_break,

                // Level 2
                show_break_window,
                close_break_window,
                start_break,
                cancel_break,
                complete_break,
                complete_strong_break,

                // Level 3
                show_dim_window,
                close_dim_window,
            ],
        )


        // ----------------------------------------------------
        // Run
        // ----------------------------------------------------

        .run(
            tauri::generate_context!(),
        )
        .expect(
            "error while running tauri application",
        );
}

#[tauri::command]
fn complete_strong_break(
    state: State<AppState>,
) {
    {
        let mut active =
            state
                .break_active
                .lock()
                .unwrap();

        *active = false;
    }

    {
        let mut engine =
            state
                .fatigue
                .lock()
                .unwrap();

        engine.complete_short_break();
    }

    {
        let mut trigger =
            state
                .trigger
                .lock()
                .unwrap();

        trigger.complete_break();
    }

    {
        let mut status =
            state
                .status
                .lock()
                .unwrap();

        status.reminder_level = 0;
    }
}
