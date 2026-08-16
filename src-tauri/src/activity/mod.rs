#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
pub use macos::idle_seconds;

#[cfg(target_os = "windows")]
pub use windows::idle_seconds;
