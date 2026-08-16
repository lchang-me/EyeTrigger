use tauri::{
    menu::{
        Menu,
        MenuItem,
    },
    tray::TrayIconBuilder,
    Manager,
};

pub fn setup_tray(
    app: &tauri::App,
) -> tauri::Result<()> {
    let open =
        MenuItem::with_id(
            app,
            "open",
            "Open EyeTrigger",
            true,
            None::<&str>,
        )?;

    let quit =
        MenuItem::with_id(
            app,
            "quit",
            "Quit EyeTrigger",
            true,
            None::<&str>,
        )?;

    let menu =
        Menu::with_items(
            app,
            &[
                &open,
                &quit,
            ],
        )?;


    let builder =
        TrayIconBuilder::with_id(
            "eyetrigger-tray",
        )
        .tooltip("EyeTrigger")
        .menu(&menu)
        .show_menu_on_left_click(false);


    // ============================================
    // macOS
    //
    // macOS 支持 tray title，
    // 所以直接显示 👁 0%
    // 不需要额外图标。
    // ============================================

    #[cfg(target_os = "macos")]
    let builder =
        builder.title("👁 0%");


    // ============================================
    // Windows
    //
    // Windows 不支持 tray title。
    // 必须提供真正的 icon。
    // ============================================

    #[cfg(target_os = "windows")]
    let builder =
        builder.icon(
            windows_load_icon(0),
        );


    builder
        .on_tray_icon_event(
            |tray, event| {
                use tauri::tray::{
                    MouseButton,
                    MouseButtonState,
                    TrayIconEvent,
                };

                if let
                    TrayIconEvent::Click {
                        button:
                            MouseButton::Left,

                        button_state:
                            MouseButtonState::Up,

                        ..
                    } = event
                {
                    let app =
                        tray.app_handle();

                    if let Some(window) =
                        app.get_webview_window(
                            "main",
                        )
                    {
                        let _ =
                            window.show();

                        let _ =
                            window.unminimize();

                        let _ =
                            window.set_focus();
                    }
                }
            },
        )

        .on_menu_event(
            |app, event| {
                match event
                    .id()
                    .as_ref()
                {
                    "open" => {
                        if let Some(window) =
                            app.get_webview_window(
                                "main",
                            )
                        {
                            let _ =
                                window.show();

                            let _ =
                                window.set_focus();
                        }
                    }

                    "quit" => {
                        app.exit(0);
                    }

                    _ => {}
                }
            },
        )

        .build(app)?;

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn update_windows_tray(
    tray: &tauri::tray::TrayIcon,
    percent: u8,
) {
    use std::sync::atomic::{
        AtomicU8,
        Ordering,
    };

    // 255 表示还没有绘制过
    static LAST_ICON_PERCENT:
        AtomicU8 =
        AtomicU8::new(255);

    let percent =
        percent.min(100);

    // Icon 每5%更新一次，
    // 避免每秒重新创建系统托盘资源。
    let bucket =
        if percent >= 98 {
            100
        } else {
            (percent / 5) * 5
        };

    let previous =
        LAST_ICON_PERCENT.load(
            Ordering::Relaxed,
        );

    if previous != bucket {
        let icon =
            windows_load_icon(
                bucket,
            );

        if tray
            .set_icon(
                Some(icon)
            )
            .is_ok()
        {
            LAST_ICON_PERCENT.store(
                bucket,
                Ordering::Relaxed,
            );
        }
    }

    // Tooltip 保持精确百分比
    let _ =
        tray.set_tooltip(
            Some(
                format!(
                    "EyeTrigger — Eye Load {}%",
                    percent
                ),
            ),
        );
}
