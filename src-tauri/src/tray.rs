use tauri::{
    menu::{Menu, MenuItem},
    tray::{
        TrayIconBuilder,
    },
    Manager,
};

pub fn setup_tray(app: &tauri::App) -> tauri::Result<()> {
    let open = MenuItem::with_id(
        app,
        "open",
        "Open EyeTrigger",
        true,
        None::<&str>,
    )?;

    let take_break = MenuItem::with_id(
        app,
        "break",
        "Take a 20s Break",
        true,
        None::<&str>,
    )?;

    let quit = MenuItem::with_id(
        app,
        "quit",
        "Quit EyeTrigger",
        true,
        None::<&str>,
    )?;

    let menu = Menu::with_items(
        app,
        &[&open, &take_break, &quit],
    )?;

    TrayIconBuilder::with_id("eyetrigger-tray")
        .title("👁 0%")
        .tooltip("EyeTrigger")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "open" => {
                    if let Some(window) =
                        app.get_webview_window("main")
                    {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }

                "break" => {
                    // 下一步再把这里直接接到
                    // show_break_window
                    println!("Take break clicked");
                }

                "quit" => {
                    app.exit(0);
                }

                _ => {}
            }
        })
        .on_tray_icon_event(
            |tray, event| {
                use tauri::tray::{
                    MouseButton,
                    MouseButtonState,
                    TrayIconEvent,
                };
        
                if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
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
                            window.unminimize();
        
                        let _ =
                            window.show();
        
                        let _ =
                            window.set_focus();
                    }
                }
            },
        )
        .build(app)?;

    Ok(())
}
