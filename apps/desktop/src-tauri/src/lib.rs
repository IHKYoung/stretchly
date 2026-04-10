mod commands;
mod engine;
mod i18n;
mod platform;
mod shell;
mod state;

use state::PauzaState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(PauzaState::default())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let config_dir = app.path().app_config_dir()?;
            app.state::<PauzaState>()
                .initialize(config_dir)
                .map_err(std::io::Error::other)?;

            #[cfg(desktop)]
            {
                use tauri_plugin_autostart::MacosLauncher;

                app.handle().plugin(tauri_plugin_autostart::init(
                    MacosLauncher::AppleScript,
                    None::<Vec<&str>>,
                ))?;
                shell::setup_desktop_shell(&app.handle())?;
                engine::spawn(app.handle().clone());
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::bootstrap,
            commands::get_snapshot,
            commands::show_main_window,
            commands::hide_main_window,
            commands::update_settings,
            commands::pause_breaks,
            commands::resume_breaks,
            commands::start_focus_session,
            commands::clear_focus_session,
            commands::finish_current_break,
            commands::skip_current_break,
            commands::postpone_current_break,
            commands::skip_to_next_scheduled_break,
            commands::skip_to_next_microbreak,
            commands::skip_to_next_long_break,
            commands::reset_breaks,
            commands::toggle_autostart,
            commands::list_running_app_names
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
