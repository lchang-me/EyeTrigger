use tauri::{
    menu::{
        Menu,
        MenuItem,
    },
    tray::TrayIconBuilder,
    Manager,
};


// ============================================================
// Windows dynamic Eye Load icon
// ============================================================

#[cfg(target_os = "windows")]
pub fn windows_load_icon(
    percent: u8,
) -> tauri::image::Image<'static> {
    use std::f64::consts::PI;

    const SIZE: u32 = 32;

    let mut rgba =
        vec![
            0u8;
            (SIZE * SIZE * 4) as usize
        ];

    let center =
        (SIZE as f64 - 1.0) / 2.0;

    let radius = 10.5;
    let stroke = 3.8;

    let percent =
        percent.min(100);

    let progress =
        percent as f64 / 100.0;


    // EyeTrigger load colors
    let active_color =
        if percent >= 86 {
            // Strong
            (235u8, 90u8, 90u8)
        } else if percent >= 74 {
            // Recovery
            (235u8, 150u8, 80u8)
        } else if percent >= 50 {
            // Gentle
            (230u8, 190u8, 90u8)
        } else {
            // Comfortable
            (90u8, 200u8, 145u8)
        };


    for y in 0..SIZE {
        for x in 0..SIZE {
            let dx =
                x as f64 - center;

            let dy =
                y as f64 - center;

            let distance =
                (dx * dx + dy * dy)
                    .sqrt();

            let ring_distance =
                (distance - radius)
                    .abs();


            // Not part of the ring
            if ring_distance
                > stroke / 2.0 + 1.0
            {
                continue;
            }


            // Simple anti-aliasing
            let alpha_factor =
                (
                    stroke / 2.0
                        + 1.0
                        - ring_distance
                )
                .clamp(
                    0.0,
                    1.0,
                );


            // Top = 0 degrees
            // Clockwise progress
            let mut angle =
                dy.atan2(dx)
                    + PI / 2.0;

            if angle < 0.0 {
                angle +=
                    2.0 * PI;
            }


            let active =
                angle
                    <= progress
                        * 2.0
                        * PI;


            let (
                r,
                g,
                b,
                base_alpha,
            ) =
                if active {
                    (
                        active_color.0,
                        active_color.1,
                        active_color.2,
                        255u8,
                    )
                } else {
                    (
                        150u8,
                        150u8,
                        150u8,
                        75u8,
                    )
                };


            let alpha =
                (
                    base_alpha as f64
                        * alpha_factor
                )
                .round()
                .clamp(
                    0.0,
                    255.0,
                )
                as u8;


            let index =
                ((y * SIZE + x) * 4)
                    as usize;

            rgba[index] = r;
            rgba[index + 1] = g;
            rgba[index + 2] = b;
            rgba[index + 3] = alpha;
        }
    }


    tauri::image::Image::new_owned(
        rgba,
        SIZE,
        SIZE,
    )
}


// ============================================================
// Windows tray updater
// ============================================================

#[cfg(target_os = "windows")]
pub fn update_windows_tray(
    tray: &tauri::tray::TrayIcon,
    percent: u8,
) {
    use std::sync::atomic::{
        AtomicU8,
        Ordering,
    };


    static LAST_ICON_PERCENT:
        AtomicU8 =
        AtomicU8::new(255);


    let percent =
        percent.min(100);


    // Update graphical icon in 5% steps.
    //
    // Tooltip still displays exact percentage.
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
                Some(icon),
            )
            .is_ok()
        {
            LAST_ICON_PERCENT.store(
                bucket,
                Ordering::Relaxed,
            );
        }
    }


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


// ============================================================
// Tray setup
// ============================================================

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
        .show_menu_on_left_click(
            false,
        );


    // ========================================================
    // macOS
    //
    // macOS supports tray title.
    // No extra image icon.
    // ========================================================

    #[cfg(target_os = "macos")]
    let builder =
        builder.title(
            "👁 0%",
        );


    // ========================================================
    // Windows
    //
    // Windows does NOT support tray title.
    // Use a dynamic graphical icon instead.
    // ========================================================

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
                                window.unminimize();

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
