use tauri::{
    AppHandle,
    Manager,
    WebviewUrl,
    WebviewWindowBuilder,
};

pub fn show_dim(
    app: &AppHandle,
) -> Result<(), String> {
    // 如果强提醒窗口已经存在，
    // 直接显示并切到前台
    if let Some(window) =
        app.get_webview_window("dim")
    {
        window
            .show()
            .map_err(|e| e.to_string())?;

        window
            .set_focus()
            .map_err(|e| e.to_string())?;

        return Ok(());
    }

    // 创建 macOS 原生 fullscreen window
    //
    // 这里不要调用 make_reminder_visible_everywhere，
    // 因为 Level 3 的目的就是建立自己的 fullscreen Space。
    let window =
        WebviewWindowBuilder::new(
            app,
            "dim",
            WebviewUrl::App(
                "index.html?window=dim".into(),
            ),
        )
        .title("EyeTrigger")
        .fullscreen(true)
        .decorations(false)
        .resizable(false)
        .always_on_top(true)
        .build()
        .map_err(|e| e.to_string())?;

    window
        .set_focus()
        .map_err(|e| e.to_string())?;

    Ok(())
}


// ============================================================
// Tauri command wrapper
//
// 前端仍然可以：
// invoke("show_dim_window")
// ============================================================

#[tauri::command]
pub async fn show_dim_window(
    app: AppHandle,
) -> Result<(), String> {
    show_dim(&app)
}


// ============================================================
// Rust 后台直接调用的关闭函数
// ============================================================

pub fn close_dim(
    app: &AppHandle,
) -> Result<(), String> {
    if let Some(window) =
        app.get_webview_window("dim")
    {
        window
            .close()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}


// ============================================================
// Tauri command wrapper
// ============================================================

#[tauri::command]
pub fn close_dim_window(
    app: AppHandle,
) -> Result<(), String> {
    close_dim(&app)
}
