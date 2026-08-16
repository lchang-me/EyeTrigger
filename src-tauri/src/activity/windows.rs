use windows::Win32::{
    System::SystemInformation::GetTickCount,
    UI::Input::KeyboardAndMouse::{
        GetLastInputInfo,
        LASTINPUTINFO,
    },
};

pub fn idle_seconds() -> f64 {
    unsafe {
        let mut info = LASTINPUTINFO {
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0,
        };

        if GetLastInputInfo(&mut info).as_bool() {
            let now = GetTickCount();

            let elapsed =
                now.wrapping_sub(info.dwTime);

            elapsed as f64 / 1000.0
        } else {
            0.0
        }
    }
}
