#[cfg(target_os = "linux")]
use {
    super::wayland_idle::{get_idle_time as get_wayland_idle_time, initialize},
    std::env,
    std::path::Path,
    uzers::get_current_uid,
};

use std::time::Duration;

pub fn get_idle_time() -> Duration {
    #[cfg(target_os = "linux")]
    {
        if is_wayland() {
            initialize();
            get_wayland_idle_time()
        } else if is_x11() {
            user_idle::UserIdle::get_time()
                .map(|t| t.duration())
                .unwrap_or_default()
        } else {
            Duration::ZERO
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        user_idle::UserIdle::get_time()
            .map(|t| t.duration())
            .unwrap_or_default()
    }
}

// Keep your existing is_wayland() and is_x11() implementations

#[cfg(target_os = "linux")]
fn is_wayland() -> bool {
    env::var("XDG_SESSION_TYPE")
        .map(|v| v == "wayland")
        .unwrap_or_else(|_| {
            env::var("WAYLAND_DISPLAY").is_ok()
                && Path::new(&format!("/run/user/{}/wayland-0", get_current_uid())).exists()
        })
}

#[cfg(target_os = "linux")]
fn is_x11() -> bool {
    x11rb::connect(None).is_ok()
}
