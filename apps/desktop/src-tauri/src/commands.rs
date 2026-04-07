use crate::{i18n, shell};
use crate::state::{DesktopSnapshot, PauzaSettings, PauzaState};
use tauri::{AppHandle, Runtime, State};
use tauri_plugin_autostart::ManagerExt;

fn app_error<E: std::fmt::Display>(error: E) -> String {
    error.to_string()
}

fn build_snapshot<R: Runtime>(
    app: &AppHandle<R>,
    state: &PauzaState,
) -> Result<DesktopSnapshot, String> {
    let autostart_enabled = app.autolaunch().is_enabled().map_err(app_error)?;
    Ok(state.snapshot(
        std::env::consts::OS.to_string(),
        app.package_info().version.to_string(),
        autostart_enabled,
    ))
}

#[tauri::command]
pub fn bootstrap<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, PauzaState>,
) -> Result<DesktopSnapshot, String> {
    build_snapshot(&app, &state)
}

#[tauri::command]
pub fn get_snapshot<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, PauzaState>,
) -> Result<DesktopSnapshot, String> {
    build_snapshot(&app, &state)
}

#[tauri::command]
pub fn show_main_window<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    shell::reveal_main_window(&app)
}

#[tauri::command]
pub fn hide_main_window<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    shell::hide_main_window(&app)
}

#[tauri::command]
pub fn update_settings<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, PauzaState>,
    settings: PauzaSettings,
) -> Result<DesktopSnapshot, String> {
    state.update_settings(settings)?;
    shell::refresh_shortcuts(&app)?;
    shell::refresh_tray(&app)?;
    let _ = shell::close_break_window(&app);
    build_snapshot(&app, &state)
}

#[tauri::command]
pub fn pause_breaks<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, PauzaState>,
    minutes: u64,
) -> Result<DesktopSnapshot, String> {
    if state.pause_for_minutes(minutes.min(480), "settings") {
        let _ = shell::close_break_window(&app);
    }
    let _ = shell::refresh_tray(&app);
    build_snapshot(&app, &state)
}

#[tauri::command]
pub fn resume_breaks<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, PauzaState>,
) -> Result<DesktopSnapshot, String> {
    if state.resume("settings") {
        let _ = shell::close_break_window(&app);
    }
    let _ = shell::refresh_tray(&app);
    build_snapshot(&app, &state)
}

#[tauri::command]
pub fn start_focus_session<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, PauzaState>,
    minutes: u64,
) -> Result<DesktopSnapshot, String> {
    if state.start_focus_session(minutes.clamp(5, 180), "settings") {
        let _ = shell::close_break_window(&app);
    }
    let _ = shell::refresh_tray(&app);
    build_snapshot(&app, &state)
}

#[tauri::command]
pub fn clear_focus_session<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, PauzaState>,
) -> Result<DesktopSnapshot, String> {
    if state.clear_focus_session("settings") {
        let _ = shell::close_break_window(&app);
    }
    let _ = shell::refresh_tray(&app);
    build_snapshot(&app, &state)
}

#[tauri::command]
pub fn finish_current_break<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, PauzaState>,
) -> Result<DesktopSnapshot, String> {
    if state.finish_current_break("break-window") {
        shell::close_break_window_deferred(app.clone());
    }
    let _ = shell::refresh_tray(&app);
    build_snapshot(&app, &state)
}

#[tauri::command]
pub fn skip_current_break<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, PauzaState>,
) -> Result<DesktopSnapshot, String> {
    if state.skip_current_break("break-window") {
        shell::close_break_window_deferred(app.clone());
    }
    let _ = shell::refresh_tray(&app);
    build_snapshot(&app, &state)
}

#[tauri::command]
pub fn postpone_current_break<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, PauzaState>,
) -> Result<DesktopSnapshot, String> {
    if state.postpone_current_break("break-window")? {
        shell::close_break_window_deferred(app.clone());
    }
    let _ = shell::refresh_tray(&app);
    build_snapshot(&app, &state)
}

#[tauri::command]
pub fn skip_to_next_scheduled_break<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, PauzaState>,
) -> Result<DesktopSnapshot, String> {
    if state.skip_to_next_scheduled_break("settings") {
        let _ = shell::close_break_window(&app);
    }
    let _ = shell::refresh_tray(&app);
    build_snapshot(&app, &state)
}

#[tauri::command]
pub fn skip_to_next_microbreak<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, PauzaState>,
) -> Result<DesktopSnapshot, String> {
    if state.skip_to_microbreak("settings") {
        let _ = shell::close_break_window(&app);
    }
    let _ = shell::refresh_tray(&app);
    build_snapshot(&app, &state)
}

#[tauri::command]
pub fn skip_to_next_long_break<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, PauzaState>,
) -> Result<DesktopSnapshot, String> {
    if state.skip_to_long_break("settings") {
        let _ = shell::close_break_window(&app);
    }
    let _ = shell::refresh_tray(&app);
    build_snapshot(&app, &state)
}

#[tauri::command]
pub fn reset_breaks<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, PauzaState>,
) -> Result<DesktopSnapshot, String> {
    if state.reset_breaks("settings") {
        let _ = shell::close_break_window(&app);
    }
    let _ = shell::refresh_tray(&app);
    build_snapshot(&app, &state)
}

#[tauri::command]
pub fn toggle_autostart<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, PauzaState>,
) -> Result<DesktopSnapshot, String> {
    let autostart_manager = app.autolaunch();
    let enabled = autostart_manager.is_enabled().map_err(app_error)?;
    let settings = state.settings();

    if enabled {
        autostart_manager.disable().map_err(app_error)?;
        state.set_last_action(i18n::text(
            &settings.language,
            "runtime.actions.autostartDisabled",
        ));
    } else {
        autostart_manager.enable().map_err(app_error)?;
        state.set_last_action(i18n::text(
            &settings.language,
            "runtime.actions.autostartEnabled",
        ));
    }

    let _ = shell::refresh_tray(&app);
    build_snapshot(&app, &state)
}
