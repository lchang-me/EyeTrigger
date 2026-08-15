#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::idle_seconds;

#[cfg(not(target_os = "macos"))]
pub fn idle_seconds() -> f64 {
    0.0
}
