use tauri::{
    AppHandle,
    Manager,
    WebviewUrl,
    WebviewWindowBuilder,
};

pub fn show_break(
    app: &AppHandle,
) -> Result<(), String> {
    // --------------------------------------------------------
    // Existing window
    // --------------------------------------------------------

    if let Some(window) =
        app.get_webview_window("break")
    {
        crate::macos_window::
            prepare_reminder_window(
                &window,
            )?;

        window
            .show()
            .map_err(|e| e.to_string())?;

        window
            .set_focus()
            .map_err(|e| e.to_string())?;

        return Ok(());
    }


    // --------------------------------------------------------
    // Create hidden
    // --------------------------------------------------------

    let window =
        WebviewWindowBuilder::new(
            app,
            "break",
            WebviewUrl::App(
                "index.html?window=break".into(),
            ),
        )
        .title("EyeTrigger")
        .inner_size(420.0, 280.0)
        .resizable(false)
        .decorations(false)
        .always_on_top(true)
        .center()

        // 先隐藏创建
        .visible(false)

        .build()
        .map_err(|e| e.to_string())?;


    // --------------------------------------------------------
    // Configure Space behavior before showing
    // --------------------------------------------------------

    crate::macos_window::
        prepare_reminder_window(
            &window,
        )?;


    // --------------------------------------------------------
    // Show in current active Space
    // --------------------------------------------------------

    window
        .show()
        .map_err(|e| e.to_string())?;

    window
        .set_focus()
        .map_err(|e| e.to_string())?;


    Ok(())
}


#[tauri::command]
pub async fn show_break_window(
    app: AppHandle,
) -> Result<(), String> {
    show_break(&app)
}


pub fn close_break(
    app: &AppHandle,
) -> Result<(), String> {
    if let Some(window) =
        app.get_webview_window("break")
    {
        window
            .close()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}


#[tauri::command]
pub fn close_break_window(
    app: AppHandle,
) -> Result<(), String> {
    close_break(&app)
}
