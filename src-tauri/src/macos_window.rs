pub fn prepare_reminder_window(
    window: &tauri::WebviewWindow,
) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use objc2_app_kit::{
            NSStatusWindowLevel,
            NSWindow,
            NSWindowCollectionBehavior,
        };

        window
            .with_webview(|webview| {
                unsafe {
                    let ns_window: &NSWindow =
                        &*webview.ns_window().cast();

                    let mut behavior =
                        ns_window.collectionBehavior();

                    // ------------------------------------------------
                    // 普通 Desktop / Space
                    //
                    // 当 reminder 被激活时，
                    // 不切回它原来的 Desktop，
                    // 而是把 reminder 移到用户当前 Space。
                    // ------------------------------------------------

                    behavior.insert(
                        NSWindowCollectionBehavior::MoveToActiveSpace,
                    );

                    // ------------------------------------------------
                    // Fullscreen / Stage Manager
                    //
                    // 允许 EyeTrigger 这种 floating overlay
                    // 加入别的 App。
                    // ------------------------------------------------

                    behavior.insert(
                        NSWindowCollectionBehavior::CanJoinAllApplications,
                    );

                    behavior.remove(
                        NSWindowCollectionBehavior::FullScreenPrimary,
                    );

                    ns_window.setCollectionBehavior(
                        behavior,
                    );

                    // 提高窗口层级
                    ns_window.setLevel(
                        NSStatusWindowLevel,
                    );

                    // 即使 EyeTrigger 当前不是 active app
                    // 也先把窗口 order 到前面
                    ns_window.orderFrontRegardless();
                }
            })
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
