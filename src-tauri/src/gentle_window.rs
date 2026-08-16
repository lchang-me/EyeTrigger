use tauri::{
    AppHandle,
    WebviewUrl,
};

#[cfg(target_os = "macos")]
use tauri::Manager;

#[cfg(target_os = "macos")]
use tauri_nspanel::{
    tauri_panel,
    CollectionBehavior,
    ManagerExt,
    PanelBuilder,
    PanelLevel,
    StyleMask,
};

#[cfg(target_os = "macos")]
use objc2_app_kit::NSWindowCollectionBehavior;

// ============================================================
// macOS native NSPanel
// ============================================================

#[cfg(target_os = "macos")]
tauri_panel! {
    panel!(GentlePanel {
        config: {
            // 不抢当前应用的键盘焦点
            can_become_key_window: false,

            // 真正的 floating NSPanel
            is_floating_panel: true
        }
    })
}


// ============================================================
// Show Level 1
// ============================================================

#[cfg(target_os = "macos")]
pub fn show_gentle(
    app: &AppHandle,
) -> Result<(), String> {
    let handle = app.clone();

    app.run_on_main_thread(move || {
        // 已经创建过：
        // 直接复用同一个 NSPanel
        if let Ok(panel) =
            handle.get_webview_panel("gentle")
        {
            panel.show();
            panel.order_front_regardless();

            println!(
                "EyeTrigger: gentle NSPanel shown"
            );

            return;
        }


        // --------------------------------------------
        // Spaces / Fullscreen behavior
        // --------------------------------------------

        let raw_behavior =
            NSWindowCollectionBehavior::CanJoinAllSpaces
            | NSWindowCollectionBehavior::CanJoinAllApplications
            | NSWindowCollectionBehavior::FullScreenAuxiliary
            | NSWindowCollectionBehavior::Stationary
            | NSWindowCollectionBehavior::IgnoresCycle;
        
        let behavior =
            CollectionBehavior::from_raw(
                raw_behavior,
            );
        // --------------------------------------------
        // Non-activating borderless panel
        // --------------------------------------------

        let style =
            StyleMask::empty()
                .borderless()
                .nonactivating_panel();


        // --------------------------------------------
        // Create NSPanel
        // --------------------------------------------

        let result =
            PanelBuilder::<_, GentlePanel>::new(
                &handle,
                "gentle",
            )
            .url(
                WebviewUrl::App(
                    "index.html?window=gentle"
                        .into(),
                ),
            )
            .title("EyeTrigger")

            // 比普通窗口高
            .level(PanelLevel::ScreenSaver)

            .floating(true)

            // EyeTrigger 不是 active app 时
            // Panel 仍然保持显示
            .hides_on_deactivate(false)

            // 创建过程中不激活整个 EyeTrigger
            .no_activate(true)

            .released_when_closed(false)

            .style_mask(style)

            .collection_behavior(
                behavior,
            )

            .with_window(|window| {
                window
                    .inner_size(
                        420.0,
                        76.0,
                    )
                    .resizable(false)
                    .decorations(false)
                    .always_on_top(true)
                    .skip_taskbar(true)

                    // 先隐藏创建，
                    // 转成 NSPanel 后再 show
                    .visible(false)

                    .center()
            })

            .build();


        match result {
            Ok(panel) => {
                panel.show();
                panel.order_front_regardless();

                println!(
                    "EyeTrigger: gentle NSPanel created"
                );
            }

            Err(error) => {
                eprintln!(
                    "EyeTrigger: failed to create gentle NSPanel: {}",
                    error
                );
            }
        }
    })
    .map_err(|e| e.to_string())
}


// ============================================================
// Non-macOS fallback
// ============================================================

#[cfg(not(target_os = "macos"))]
pub fn show_gentle(
    app: &AppHandle,
) -> Result<(), String> {
    use tauri::{
        Manager,
        WebviewWindowBuilder,
    };

    if let Some(window) =
        app.get_webview_window("gentle")
    {
        window
            .show()
            .map_err(|e| e.to_string())?;

        return Ok(());
    }

    WebviewWindowBuilder::new(
        app,
        "gentle",
        WebviewUrl::App(
            "index.html?window=gentle".into(),
        ),
    )
    .title("EyeTrigger")
    .inner_size(420.0, 76.0)
    .resizable(false)
    .decorations(false)
    .always_on_top(true)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}


// ============================================================
// Command wrapper
// ============================================================

#[tauri::command]
pub async fn show_gentle_window(
    app: AppHandle,
) -> Result<(), String> {
    show_gentle(&app)
}


// ============================================================
// Hide Level 1
// ============================================================

#[cfg(target_os = "macos")]
pub fn close_gentle(
    app: &AppHandle,
) -> Result<(), String> {
    let handle = app.clone();

    app.run_on_main_thread(move || {
        if let Ok(panel) =
            handle.get_webview_panel("gentle")
        {
            // 不销毁，只隐藏。
            // 下一次直接复用。
            panel.hide();

            println!(
                "EyeTrigger: gentle NSPanel hidden"
            );
        }
    })
    .map_err(|e| e.to_string())
}


#[cfg(not(target_os = "macos"))]
pub fn close_gentle(
    app: &AppHandle,
) -> Result<(), String> {
    use tauri::Manager;

    if let Some(window) =
        app.get_webview_window("gentle")
    {
        window
            .hide()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}


#[tauri::command]
pub fn close_gentle_window(
    app: AppHandle,
) -> Result<(), String> {
    close_gentle(&app)
}
