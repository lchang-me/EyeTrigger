use tauri::{
    AppHandle,
    Manager,
    WebviewUrl,
    WebviewWindowBuilder,
};

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;


// ============================================================
// Level 3
// ============================================================

pub fn show_dim(
    app: &AppHandle,
) -> Result<(), String> {
    // --------------------------------------------------------
    // macOS:
    //
    // EyeTrigger 平时是 Accessory menu-bar app。
    // Level 3 需要真正进入 native fullscreen Space，
    // 所以这里临时切回 Regular。
    // --------------------------------------------------------

    #[cfg(target_os = "macos")]
    {
        app.set_activation_policy(
            ActivationPolicy::Regular,
        )
        .map_err(|e| e.to_string())?;
    }


    // --------------------------------------------------------
    // 如果窗口已经存在
    // --------------------------------------------------------

    if let Some(window) =
        app.get_webview_window("dim")
    {
        window
            .show()
            .map_err(|e| e.to_string())?;

        window
            .set_focus()
            .map_err(|e| e.to_string())?;

        // 关键：
        // 明确请求 native fullscreen
        window
            .set_fullscreen(true)
            .map_err(|e| e.to_string())?;

        println!(
            "EyeTrigger: strong window entered fullscreen"
        );

        return Ok(());
    }


    // --------------------------------------------------------
    // 先创建普通窗口
    //
    // 不要在 Builder 里直接 .fullscreen(true)
    // --------------------------------------------------------

    let window =
        WebviewWindowBuilder::new(
            app,
            "dim",
            WebviewUrl::App(
                "index.html?window=dim".into(),
            ),
        )
        .title("EyeTrigger")

        // 先普通创建
        .fullscreen(false)

        // fullscreen transition 时保持正常 window capability
        .resizable(true)

        .decorations(false)
        .always_on_top(true)
        .focused(true)
        .build()
        .map_err(|e| e.to_string())?;


    // --------------------------------------------------------
    // 先成为真正的前台窗口
    // --------------------------------------------------------

    window
        .show()
        .map_err(|e| e.to_string())?;

    window
        .set_focus()
        .map_err(|e| e.to_string())?;


    // --------------------------------------------------------
    // 再请求 macOS native fullscreen
    //
    // 这个才应该产生新的 fullscreen Space。
    // --------------------------------------------------------

    window
        .set_fullscreen(true)
        .map_err(|e| e.to_string())?;


    println!(
        "EyeTrigger: strong window created and fullscreen requested"
    );

    Ok(())
}


// ============================================================
// Frontend command
// ============================================================

#[tauri::command]
pub async fn show_dim_window(
    app: AppHandle,
) -> Result<(), String> {
    show_dim(&app)
}


// ============================================================
// Close Level 3
// ============================================================

pub fn close_dim(
    app: &AppHandle,
) -> Result<(), String> {
    if let Some(window) =
        app.get_webview_window("dim")
    {
        // 先退出 fullscreen
        let _ =
            window.set_fullscreen(false);

        window
            .close()
            .map_err(|e| e.to_string())?;
    }


    // --------------------------------------------------------
    // Level 3结束后恢复 menu-bar utility 模式
    // --------------------------------------------------------

    #[cfg(target_os = "macos")]
    {
        app.set_activation_policy(
            ActivationPolicy::Accessory,
        )
        .map_err(|e| e.to_string())?;

        app.set_dock_visibility(false)
            .map_err(|e| e.to_string())?;
    }


    println!(
        "EyeTrigger: strong window closed"
    );

    Ok(())
}


#[tauri::command]
pub fn close_dim_window(
    app: AppHandle,
) -> Result<(), String> {
    close_dim(&app)
}
