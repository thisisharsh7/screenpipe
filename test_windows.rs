#[cfg(target_os = "windows")]
pub fn start_sleep_monitor() {
    use windows::Win32::System::StationsAndDesktops::{
        CloseDesktop, OpenInputDesktop, DESKTOP_ACCESS_FLAGS, DESKTOP_CONTROL_FLAGS,
    };

    info!("Starting Windows screen-lock monitor (OpenInputDesktop polling)");
    // ...
}
