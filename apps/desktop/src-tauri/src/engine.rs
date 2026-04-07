use crate::platform::PlatformMonitor;
use crate::shell;
use crate::state::{now_ms, PauzaState};
use std::time::Duration;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_notification::NotificationExt;

pub fn spawn<R: Runtime>(app: AppHandle<R>) {
    std::thread::spawn(move || {
        let mut monitor = PlatformMonitor::default();

        loop {
            let settings = app.state::<PauzaState>().settings();
            let signals = monitor.probe(&settings);
            let actions = app.state::<PauzaState>().tick(
                now_ms(),
                signals.idle_ms,
                signals.dnd_active,
                signals.app_exclusion_match,
            );

            if actions.close_break_window {
                let _ = shell::close_break_window(&app);
            }

            if let (Some(title), Some(body)) =
                (actions.notify_title.as_ref(), actions.notify_body.as_ref())
            {
                let _ = app.notification().builder().title(title).body(body).show();
            }

            if actions.open_break_window {
                let _ = shell::show_break_window(&app);
            }

            let _ = shell::refresh_tray_if_needed(&app);

            std::thread::sleep(Duration::from_secs(1));
        }
    });
}
