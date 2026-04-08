use crate::{
    i18n,
    state::{BreakKind, BreakPromptStyle, DesktopSnapshot, PauzaSettings, PauzaState, ShortcutAction},
};
use std::{
    sync::{mpsc, Mutex, OnceLock},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tauri::{
    menu::{IsMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Monitor, PhysicalPosition, PhysicalSize, Position, Runtime, Size,
    WebviewUrl, WebviewWindow,
};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

const MAIN_WINDOW_LABEL: &str = "main";
const BREAK_WINDOW_PREFIX: &str = "break";
const TRAY_ID: &str = "pauza-tray";
const BREAK_WINDOW_DESTROY_DELAY_MS: u64 = 75;
static LAST_TRAY_REFRESH_KEY: OnceLock<Mutex<Option<TrayRefreshKey>>> = OnceLock::new();

trait TrayMenuContainer<R: Runtime> {
    fn append_item(&self, item: &dyn IsMenuItem<R>) -> tauri::Result<()>;
}

impl<R: Runtime> TrayMenuContainer<R> for Menu<R> {
    fn append_item(&self, item: &dyn IsMenuItem<R>) -> tauri::Result<()> {
        self.append(item)
    }
}

impl<R: Runtime> TrayMenuContainer<R> for Submenu<R> {
    fn append_item(&self, item: &dyn IsMenuItem<R>) -> tauri::Result<()> {
        self.append(item)
    }
}

#[derive(Debug, Clone, Copy)]
struct Bounds {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

#[derive(Debug, Clone, Copy)]
struct BreakWindowProfile {
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    decorations: bool,
    focusable: bool,
    always_on_top: bool,
    fullscreen: bool,
    skip_taskbar: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TrayRefreshKey {
    status: String,
    detail_mode: TrayDetailMode,
    strict_locked: bool,
    microbreak_enabled: bool,
    long_break_enabled: bool,
    autostart_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TrayDetailMode {
    Static(String),
    CurrentBreak {
        kind: BreakKind,
        strict_mode: bool,
        manual_awaiting: bool,
        minute_bucket: u64,
    },
    Focus {
        minute_bucket: u64,
    },
    Paused {
        indefinite: bool,
        minute_bucket: Option<u64>,
    },
    Running {
        kind: Option<BreakKind>,
        minute_bucket: u64,
    },
}

pub fn setup_desktop_shell<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    app.plugin(tauri_plugin_global_shortcut::Builder::new().build())?;
    build_tray(app)?;
    refresh_shortcuts(app).map_err(std::io::Error::other)?;

    if let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) {
        attach_main_window_behavior(window);
    }

    Ok(())
}

fn app_error<E: std::fmt::Display>(error: E) -> String {
    error.to_string()
}

fn run_on_main_thread<R, T, F>(app: &AppHandle<R>, task: F) -> Result<T, String>
where
    R: Runtime,
    T: Send + 'static,
    F: FnOnce(AppHandle<R>) -> Result<T, String> + Send + 'static,
{
    let (sender, receiver) = mpsc::channel();
    let handle = app.clone();

    app.run_on_main_thread(move || {
        let _ = sender.send(task(handle));
    })
    .map_err(app_error)?;

    receiver
        .recv()
        .map_err(|_| "main thread task did not return a result".to_string())?
}

pub fn reveal_main_window<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    run_on_main_thread(app, |handle| {
        let window = handle
            .get_webview_window(MAIN_WINDOW_LABEL)
            .ok_or_else(|| "main window is not available".to_string())?;
        let _ = window.unminimize();
        window.show().map_err(app_error)?;
        window.set_focus().map_err(app_error)?;
        Ok(())
    })
}

pub fn hide_main_window<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    run_on_main_thread(app, |handle| {
        let window = handle
            .get_webview_window(MAIN_WINDOW_LABEL)
            .ok_or_else(|| "main window is not available".to_string())?;
        window.hide().map_err(app_error)
    })
}

pub fn show_break_window<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    run_on_main_thread(app, |handle| {
        let settings = handle.state::<PauzaState>().settings();
        let language = settings.language.clone();
        let title = i18n::text(&language, "runtime.window.breakTitle");
        let strict_break = handle.state::<PauzaState>().current_break_is_strict();
        let targets = break_window_targets(&handle, &settings)?;

        for (index, (_monitor, profile)) in targets.into_iter().enumerate() {
            let label = break_label(index);
            let focusable = profile.focusable || strict_break;

            let window = if let Some(window) = handle.get_webview_window(&label) {
                configure_break_window(&window, profile, focusable)?;
                window
            } else {
                let window = tauri::WebviewWindowBuilder::new(
                    &handle,
                    label.clone(),
                    WebviewUrl::App("index.html?window=break".into()),
                )
                .title(title.clone())
                .visible(false)
                .resizable(false)
                .maximizable(false)
                .minimizable(false)
                .build()
                .map_err(app_error)?;
                configure_break_window(&window, profile, focusable)?;
                attach_break_window_behavior(window.clone());
                window
            };

            let _ = window.unminimize();
            window.set_title(&title).map_err(app_error)?;
            window.show().map_err(app_error)?;
            if focusable {
                let _ = window.set_focus();
            }
        }

        Ok(())
    })
}

pub fn close_break_window<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    run_on_main_thread(app, |handle| {
        for (_, window) in break_windows(&handle) {
            if window.is_fullscreen().unwrap_or(false) {
                let _ = window.set_fullscreen(false);
            }
            let _ = window.hide();
            let _ = window.destroy();
        }
        Ok(())
    })
}

pub fn close_break_window_deferred<R: Runtime + 'static>(app: AppHandle<R>) {
    thread::spawn(move || {
        // Let the invoke response finish before tearing down the active break webview.
        thread::sleep(Duration::from_millis(BREAK_WINDOW_DESTROY_DELAY_MS));
        let _ = close_break_window(&app);
    });
}

pub fn refresh_tray<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    sync_tray_refresh_key(app)?;
    run_on_main_thread(app, |handle| {
        let menu = create_tray_menu(&handle).map_err(app_error)?;

        if let Some(tray) = handle.tray_by_id(TRAY_ID) {
            tray.set_menu(Some(menu)).map_err(app_error)?;
            return Ok(());
        }

        build_tray(&handle).map_err(app_error)
    })
}

pub fn refresh_tray_if_needed<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let next_key = tray_refresh_key(app);
    let cache = LAST_TRAY_REFRESH_KEY.get_or_init(|| Mutex::new(None));
    let should_refresh = {
        let mut last_key = cache.lock().expect("tray refresh lock poisoned");
        if last_key.as_ref() == Some(&next_key) {
            false
        } else {
            *last_key = Some(next_key);
            true
        }
    };

    if should_refresh {
        run_on_main_thread(app, |handle| {
            let menu = create_tray_menu(&handle).map_err(app_error)?;

            if let Some(tray) = handle.tray_by_id(TRAY_ID) {
                tray.set_menu(Some(menu)).map_err(app_error)?;
                return Ok(());
            }

            build_tray(&handle).map_err(app_error)
        })?;
    }

    Ok(())
}

pub fn refresh_shortcuts<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let manager = app.global_shortcut();
    manager.unregister_all().map_err(app_error)?;

    for binding in app.state::<PauzaState>().settings().shortcut_bindings() {
        let action = binding.action;
        let shortcut_text = binding.shortcut.clone();
        manager
            .on_shortcut(shortcut_text.as_str(), move |app, _shortcut, event| {
                if event.state() != ShortcutState::Pressed {
                    return;
                }
                let handle = app.clone();
                thread::spawn(move || {
                    handle_shortcut_action(&handle, action);
                });
            })
            .map_err(app_error)?;
    }

    Ok(())
}

fn build_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let menu = create_tray_menu(app)?;

    let tray_icon = tauri::include_image!("icons/tray-iconTemplate@2x.png");
    let mut builder = TrayIconBuilder::with_id(TRAY_ID)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .icon(tray_icon)
        .on_menu_event(|app, event| {
            let handle = app.clone();
            let action = event.id.as_ref().to_string();
            thread::spawn(move || {
                handle_tray_action(&handle, &action);
                let _ = refresh_tray(&handle);
            });
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let handle = tray.app_handle().clone();
                thread::spawn(move || {
                    let _ = reveal_main_window(&handle);
                });
            }
        });

    #[cfg(target_os = "macos")]
    {
        builder = builder.icon_as_template(true);
    }

    let tray = builder.build(app)?;

    #[cfg(target_os = "macos")]
    {
        if !is_running_from_app_bundle() {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
                patch_macos_dock_icon();
            }));
        }
        let _ = patch_macos_tray_icon(&tray);
    }

    Ok(())
}

/// Keep unbundled dev runs from showing the oversized fallback icon.
#[cfg(target_os = "macos")]
unsafe fn patch_macos_dock_icon() {
    use objc2::{class, exception, msg_send};
    use objc2::runtime::AnyObject;
    use std::ffi::c_void;

    let icon_bytes: &[u8] = include_bytes!("../icons/icon.icns");

    let result = exception::catch(|| {
        let data: *mut AnyObject = msg_send![class!(NSData),
            dataWithBytes: icon_bytes.as_ptr() as *const c_void,
            length: icon_bytes.len()
        ];
        if data.is_null() {
            return;
        }
        let image: *mut AnyObject = msg_send![class!(NSImage), alloc];
        let image: *mut AnyObject = msg_send![image, initWithData: data];
        if image.is_null() {
            return;
        }
        let app: *mut AnyObject = msg_send![class!(NSApplication), sharedApplication];
        if !app.is_null() {
            let _: () = msg_send![app, setApplicationIconImage: image];
        }
    });
    let _ = result;
}

#[cfg(target_os = "macos")]
fn is_running_from_app_bundle() -> bool {
    std::env::current_exe()
        .ok()
        .map(|path| {
            path.ancestors().any(|ancestor| {
                ancestor
                    .extension()
                    .and_then(|ext| ext.to_str())
                    == Some("app")
            })
        })
        .unwrap_or(false)
}

#[cfg(target_os = "macos")]
fn patch_macos_tray_icon<R: Runtime>(tray: &tauri::tray::TrayIcon<R>) -> tauri::Result<()> {
    use objc2::msg_send;
    use objc2::runtime::AnyObject;

    tray.with_inner_tray_icon(|inner| unsafe {
        let Some(status_item) = inner.ns_status_item() else {
            return;
        };
        let status_item_obj: &AnyObject = AsRef::<AnyObject>::as_ref(&status_item);
        let button: *mut AnyObject = msg_send![status_item_obj, button];
        if button.is_null() {
            return;
        }
        let image = build_retina_template_tray_image(
            include_bytes!("../icons/tray-iconTemplate.png"),
            include_bytes!("../icons/tray-iconTemplate@2x.png"),
        );
        if image.is_null() {
            return;
        }
        let _: () = msg_send![button, setImage: image];
    })
}

#[cfg(target_os = "macos")]
unsafe fn build_retina_template_tray_image(
    base_bytes: &[u8],
    retina_bytes: &[u8],
) -> *mut objc2::runtime::AnyObject {
    use objc2::{class, exception, msg_send};
    use objc2::runtime::AnyObject;
    use objc2_foundation::NSSize;
    use std::ffi::c_void;

    let result = exception::catch(|| {
        let image: *mut AnyObject = msg_send![class!(NSImage), alloc];
        let image: *mut AnyObject = msg_send![image, initWithSize: NSSize::new(18.0, 18.0)];
        if image.is_null() {
            return std::ptr::null_mut();
        }
        for bytes in [base_bytes, retina_bytes] {
            let data: *mut AnyObject = msg_send![class!(NSData),
                dataWithBytes: bytes.as_ptr() as *const c_void,
                length: bytes.len()
            ];
            if data.is_null() {
                continue;
            }
            let rep: *mut AnyObject = msg_send![class!(NSBitmapImageRep), imageRepWithData: data];
            if rep.is_null() {
                continue;
            }
            let _: () = msg_send![rep, setSize: NSSize::new(18.0, 18.0)];
            let _: () = msg_send![image, addRepresentation: rep];
        }
        let _: () = msg_send![image, setTemplate: true];
        image
    });
    match result {
        Ok(image) => image,
        Err(_) => std::ptr::null_mut(),
    }
}

fn handle_tray_action<R: Runtime>(app: &AppHandle<R>, action: &str) {
    match action {
        "open" => {
            let _ = reveal_main_window(app);
        }
        "skip-scheduled" => {
            if app
                .state::<PauzaState>()
                .skip_to_next_scheduled_break("tray")
            {
                let _ = close_break_window(app);
            }
        }
        "skip-microbreak" => {
            if app.state::<PauzaState>().skip_to_microbreak("tray") {
                let _ = close_break_window(app);
            }
        }
        "skip-long-break" => {
            if app.state::<PauzaState>().skip_to_long_break("tray") {
                let _ = close_break_window(app);
            }
        }
        "pause-30" => {
            if app.state::<PauzaState>().pause_for_minutes(30, "tray") {
                let _ = close_break_window(app);
            }
        }
        "pause-60" => {
            if app.state::<PauzaState>().pause_for_minutes(60, "tray") {
                let _ = close_break_window(app);
            }
        }
        "pause-120" => {
            if app.state::<PauzaState>().pause_for_minutes(120, "tray") {
                let _ = close_break_window(app);
            }
        }
        "pause-300" => {
            if app.state::<PauzaState>().pause_for_minutes(300, "tray") {
                let _ = close_break_window(app);
            }
        }
        "pause-forever" => {
            if app.state::<PauzaState>().pause_for_minutes(0, "tray") {
                let _ = close_break_window(app);
            }
        }
        "resume" => {
            if app.state::<PauzaState>().resume("tray") {
                let _ = close_break_window(app);
            }
        }
        "focus-25" => {
            if app.state::<PauzaState>().start_focus_session(25, "tray") {
                let _ = close_break_window(app);
            }
        }
        "focus-45" => {
            if app.state::<PauzaState>().start_focus_session(45, "tray") {
                let _ = close_break_window(app);
            }
        }
        "focus-60" => {
            if app.state::<PauzaState>().start_focus_session(60, "tray") {
                let _ = close_break_window(app);
            }
        }
        "reset" => {
            if app.state::<PauzaState>().reset_breaks("tray") {
                let _ = close_break_window(app);
            }
        }
        "hide" => {
            let _ = hide_main_window(app);
        }
        "toggle-autostart" => toggle_autostart_from_tray(app),
        "quit" => {
            app.exit(0);
        }
        _ => {}
    }
}

fn handle_shortcut_action<R: Runtime>(app: &AppHandle<R>, action: ShortcutAction) {
    match action {
        ShortcutAction::RevealSettings => {
            let _ = reveal_main_window(app);
        }
        ShortcutAction::Focus45 => {
            if app.state::<PauzaState>().start_focus_session(45, "shortcut") {
                let _ = close_break_window(app);
            }
        }
        ShortcutAction::PauseToggle => {
            if app.state::<PauzaState>().toggle_pause("shortcut") {
                let _ = close_break_window(app);
            }
        }
        ShortcutAction::Pause30 => {
            if app.state::<PauzaState>().pause_for_minutes(30, "shortcut") {
                let _ = close_break_window(app);
            }
        }
        ShortcutAction::Pause60 => {
            if app.state::<PauzaState>().pause_for_minutes(60, "shortcut") {
                let _ = close_break_window(app);
            }
        }
        ShortcutAction::Pause120 => {
            if app.state::<PauzaState>().pause_for_minutes(120, "shortcut") {
                let _ = close_break_window(app);
            }
        }
        ShortcutAction::Pause300 => {
            if app.state::<PauzaState>().pause_for_minutes(300, "shortcut") {
                let _ = close_break_window(app);
            }
        }
        ShortcutAction::SkipNextScheduled => {
            if app
                .state::<PauzaState>()
                .skip_to_next_scheduled_break("shortcut")
            {
                let _ = close_break_window(app);
            }
        }
        ShortcutAction::SkipNextMicrobreak => {
            if app.state::<PauzaState>().skip_to_microbreak("shortcut") {
                let _ = close_break_window(app);
            }
        }
        ShortcutAction::SkipNextLongBreak => {
            if app.state::<PauzaState>().skip_to_long_break("shortcut") {
                let _ = close_break_window(app);
            }
        }
        ShortcutAction::ResetBreaks => {
            if app.state::<PauzaState>().reset_breaks("shortcut") {
                let _ = close_break_window(app);
            }
        }
    }

    let _ = refresh_tray(app);
}

fn sync_tray_refresh_key<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let cache = LAST_TRAY_REFRESH_KEY.get_or_init(|| Mutex::new(None));
    let mut last_key = cache.lock().expect("tray refresh lock poisoned");
    *last_key = Some(tray_refresh_key(app));
    Ok(())
}

fn tray_refresh_key<R: Runtime>(app: &AppHandle<R>) -> TrayRefreshKey {
    let state = app.state::<PauzaState>();
    let settings = state.settings();
    let autostart_enabled = app.autolaunch().is_enabled().unwrap_or(false);
    let snapshot = state.snapshot(
        std::env::consts::OS.to_string(),
        app.package_info().version.to_string(),
        autostart_enabled,
    );

    TrayRefreshKey {
        status: snapshot.status.clone(),
        detail_mode: tray_detail_mode(&snapshot),
        strict_locked: snapshot
            .current_break
            .as_ref()
            .is_some_and(|current| current.strict_mode)
            && !settings.show_tray_menu_in_strict_mode,
        microbreak_enabled: settings.microbreak_enabled,
        long_break_enabled: settings.long_break_enabled,
        autostart_enabled,
    }
}

fn tray_detail_mode(snapshot: &DesktopSnapshot) -> TrayDetailMode {
    let now = current_time_ms();

    if let Some(current) = &snapshot.current_break {
        return TrayDetailMode::CurrentBreak {
            kind: current.kind,
            strict_mode: current.strict_mode,
            manual_awaiting: current.manual_awaiting,
            minute_bucket: duration_minute_bucket(current.ends_at_ms.saturating_sub(now)),
        };
    }

    if let Some(remaining) = snapshot
        .focus_until_ms
        .map(|until| until.saturating_sub(now))
        .filter(|remaining| *remaining > 0)
    {
        return TrayDetailMode::Focus {
            minute_bucket: duration_minute_bucket(remaining),
        };
    }

    if snapshot.paused_indefinitely {
        return TrayDetailMode::Paused {
            indefinite: true,
            minute_bucket: None,
        };
    }

    if let Some(remaining) = snapshot
        .pause_until_ms
        .map(|until| until.saturating_sub(now))
        .filter(|remaining| *remaining > 0)
    {
        return TrayDetailMode::Paused {
            indefinite: false,
            minute_bucket: Some(duration_minute_bucket(remaining)),
        };
    }

    if snapshot.app_exclusion_active
        || snapshot.dnd_active
        || natural_break_active(snapshot)
        || snapshot.next_break_in_ms.is_none()
    {
        return TrayDetailMode::Static(snapshot.status_detail.clone());
    }

    TrayDetailMode::Running {
        kind: snapshot.next_break_kind,
        minute_bucket: duration_minute_bucket(snapshot.next_break_in_ms.unwrap_or_default()),
    }
}

fn natural_break_active(snapshot: &DesktopSnapshot) -> bool {
    snapshot.settings.natural_breaks
        && snapshot.idle_ms >= snapshot.settings.natural_break_reset_minutes * 60_000
}

fn duration_minute_bucket(ms: u64) -> u64 {
    ms.max(1).div_ceil(60_000)
}

fn current_time_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn toggle_autostart_from_tray<R: Runtime>(app: &AppHandle<R>) {
    let manager = app.autolaunch();
    match manager.is_enabled() {
        Ok(true) => {
            let _ = manager.disable();
            let settings = app.state::<PauzaState>().settings();
            app.state::<PauzaState>().set_last_action(i18n::text(
                &settings.language,
                "runtime.actions.autostartDisabledFromTray",
            ));
        }
        Ok(false) => {
            let _ = manager.enable();
            let settings = app.state::<PauzaState>().settings();
            app.state::<PauzaState>().set_last_action(i18n::text(
                &settings.language,
                "runtime.actions.autostartEnabledFromTray",
            ));
        }
        Err(error) => {
            let settings = app.state::<PauzaState>().settings();
            app.state::<PauzaState>().set_last_action(i18n::text1(
                &settings.language,
                "runtime.actions.autostartToggleFailed",
                "error",
                error.to_string(),
            ));
        }
    }
}

fn attach_main_window_behavior<R: Runtime>(window: WebviewWindow<R>) {
    let handle = window.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            let _ = handle.hide();
        }
    });
}

fn attach_break_window_behavior<R: Runtime>(window: WebviewWindow<R>) {
    let handle = window.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            let state = handle.app_handle().state::<PauzaState>();
            if state.current_break_is_strict() {
                return;
            }
            if state.skip_current_break("window-close") {
                let _ = close_break_window(&handle.app_handle());
            } else {
                let _ = handle.hide();
            }
        }
    });
}

#[cfg(target_os = "macos")]
fn create_tray_menu<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Submenu<R>> {
    let state = app.state::<PauzaState>();
    let settings = state.settings();
    let language = settings.language.clone();
    let autostart_enabled = app.autolaunch().is_enabled().unwrap_or(false);
    let snapshot = state.snapshot(
        std::env::consts::OS.to_string(),
        app.package_info().version.to_string(),
        autostart_enabled,
    );
    let menu = Submenu::new(app, snapshot.product_name, true)?;

    populate_tray_menu(
        &menu,
        app,
        &settings,
        &language,
        autostart_enabled,
        &snapshot,
    )?;

    Ok(menu)
}

#[cfg(not(target_os = "macos"))]
fn create_tray_menu<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let state = app.state::<PauzaState>();
    let settings = state.settings();
    let language = settings.language.clone();
    let autostart_enabled = app.autolaunch().is_enabled().unwrap_or(false);
    let snapshot = state.snapshot(
        std::env::consts::OS.to_string(),
        app.package_info().version.to_string(),
        autostart_enabled,
    );
    let menu = Menu::new(app)?;

    populate_tray_menu(
        &menu,
        app,
        &settings,
        &language,
        autostart_enabled,
        &snapshot,
    )?;

    Ok(menu)
}

fn populate_tray_menu<R: Runtime, M: TrayMenuContainer<R>>(
    menu: &M,
    app: &AppHandle<R>,
    settings: &PauzaSettings,
    language: &str,
    autostart_enabled: bool,
    snapshot: &DesktopSnapshot,
) -> tauri::Result<()> {
    let status_item = MenuItem::with_id(app, "status", &snapshot.status, false, None::<&str>)?;
    let detail_item = MenuItem::with_id(
        app,
        "status-detail",
        &snapshot.status_detail,
        false,
        None::<&str>,
    )?;
    menu.append_item(&status_item)?;
    menu.append_item(&detail_item)?;
    menu.append_item(&PredefinedMenuItem::separator(app)?)?;

    let strict_locked = snapshot
        .current_break
        .as_ref()
        .is_some_and(|current| current.strict_mode)
        && !settings.show_tray_menu_in_strict_mode;

    if !strict_locked {
        let open_item = MenuItem::with_id(
            app,
            "open",
            i18n::text(language, "runtime.tray.open"),
            true,
            None::<&str>,
        )?;
        menu.append_item(&open_item)?;

        if settings.microbreak_enabled || settings.long_break_enabled {
            let skip_menu = Submenu::new(app, i18n::text(language, "runtime.tray.skipSubmenu"), true)?;
            let skip_scheduled_item = MenuItem::with_id(
                app,
                "skip-scheduled",
                i18n::text(language, "runtime.tray.skipScheduled"),
                true,
                None::<&str>,
            )?;
            skip_menu.append(&skip_scheduled_item)?;
            if settings.microbreak_enabled {
                let skip_microbreak_item = MenuItem::with_id(
                    app,
                    "skip-microbreak",
                    i18n::text(language, "runtime.tray.skipMicrobreak"),
                    true,
                    None::<&str>,
                )?;
                skip_menu.append(&skip_microbreak_item)?;
            }
            if settings.long_break_enabled {
                let skip_long_break_item = MenuItem::with_id(
                    app,
                    "skip-long-break",
                    i18n::text(language, "runtime.tray.skipLongBreak"),
                    true,
                    None::<&str>,
                )?;
                skip_menu.append(&skip_long_break_item)?;
            }
            menu.append_item(&skip_menu)?;
        }

        let focus_25_item = MenuItem::with_id(
            app,
            "focus-25",
            i18n::text(language, "runtime.tray.focus25"),
            true,
            None::<&str>,
        )?;
        let focus_45_item = MenuItem::with_id(
            app,
            "focus-45",
            i18n::text(language, "runtime.tray.focus45"),
            true,
            None::<&str>,
        )?;
        let focus_60_item = MenuItem::with_id(
            app,
            "focus-60",
            i18n::text(language, "runtime.tray.focus60"),
            true,
            None::<&str>,
        )?;
        let focus_menu = Submenu::with_items(
            app,
            i18n::text(language, "runtime.tray.focusSubmenu"),
            true,
            &[&focus_25_item, &focus_45_item, &focus_60_item],
        )?;
        menu.append_item(&focus_menu)?;

        let pause_30_item = MenuItem::with_id(
            app,
            "pause-30",
            i18n::text(language, "runtime.tray.pause30"),
            true,
            None::<&str>,
        )?;
        let pause_60_item = MenuItem::with_id(
            app,
            "pause-60",
            i18n::text(language, "runtime.tray.pause60"),
            true,
            None::<&str>,
        )?;
        let pause_120_item = MenuItem::with_id(
            app,
            "pause-120",
            i18n::text(language, "runtime.tray.pause120"),
            true,
            None::<&str>,
        )?;
        let pause_300_item = MenuItem::with_id(
            app,
            "pause-300",
            i18n::text(language, "runtime.tray.pause300"),
            true,
            None::<&str>,
        )?;
        let pause_forever_item = MenuItem::with_id(
            app,
            "pause-forever",
            i18n::text(language, "runtime.tray.pauseForever"),
            true,
            None::<&str>,
        )?;
        let pause_menu = Submenu::with_items(
            app,
            i18n::text(language, "runtime.tray.pauseSubmenu"),
            true,
            &[
                &pause_30_item,
                &pause_60_item,
                &pause_120_item,
                &pause_300_item,
                &pause_forever_item,
            ],
        )?;
        menu.append_item(&pause_menu)?;

        let resume_item = MenuItem::with_id(
            app,
            "resume",
            i18n::text(language, "runtime.tray.resume"),
            true,
            None::<&str>,
        )?;
        let reset_item = MenuItem::with_id(
            app,
            "reset",
            i18n::text(language, "runtime.tray.reset"),
            true,
            None::<&str>,
        )?;
        let hide_item = MenuItem::with_id(
            app,
            "hide",
            i18n::text(language, "runtime.tray.hide"),
            true,
            None::<&str>,
        )?;
        let toggle_autostart_item = MenuItem::with_id(
            app,
            "toggle-autostart",
            if autostart_enabled {
                i18n::text(language, "runtime.tray.disableAutostart")
            } else {
                i18n::text(language, "runtime.tray.enableAutostart")
            },
            true,
            None::<&str>,
        )?;

        menu.append_item(&resume_item)?;
        menu.append_item(&reset_item)?;
        menu.append_item(&PredefinedMenuItem::separator(app)?)?;
        menu.append_item(&hide_item)?;
        menu.append_item(&toggle_autostart_item)?;
    }

    menu.append_item(&PredefinedMenuItem::separator(app)?)?;
    let quit_item = MenuItem::with_id(
        app,
        "quit",
        i18n::text(language, "runtime.tray.quit"),
        true,
        None::<&str>,
    )?;
    menu.append_item(&quit_item)?;

    Ok(())
}

fn configure_break_window<R: Runtime>(
    window: &WebviewWindow<R>,
    profile: BreakWindowProfile,
    focusable: bool,
) -> Result<(), String> {
    window
        .set_size(Size::Physical(PhysicalSize::new(profile.width, profile.height)))
        .map_err(app_error)?;
    window
        .set_position(Position::Physical(PhysicalPosition::new(profile.x, profile.y)))
        .map_err(app_error)?;
    window
        .set_decorations(profile.decorations)
        .map_err(app_error)?;
    window.set_focusable(focusable).map_err(app_error)?;
    window
        .set_always_on_top(profile.always_on_top)
        .map_err(app_error)?;
    window
        .set_skip_taskbar(profile.skip_taskbar)
        .map_err(app_error)?;
    window.set_fullscreen(profile.fullscreen).map_err(app_error)?;
    window
        .set_visible_on_all_workspaces(true)
        .map_err(app_error)?;
    Ok(())
}

fn break_window_targets<R: Runtime>(
    app: &AppHandle<R>,
    settings: &PauzaSettings,
) -> Result<Vec<(Monitor, BreakWindowProfile)>, String> {
    let monitors = app.available_monitors().map_err(app_error)?;
    if monitors.is_empty() {
        return Err("no monitors available".to_string());
    }

    let selected_monitors = if settings.show_breaks_on_all_screens {
        monitors
    } else {
        vec![selected_monitor(app, settings)?.unwrap_or_else(|| fallback_monitor(app))]
    };

    let kind = active_break_kind(app);
    let profiles = selected_monitors
        .into_iter()
        .map(|monitor| {
            let profile =
                break_window_profile(kind, &monitor, settings.break_prompt_style, settings.fullscreen);
            (monitor, profile)
        })
        .collect();

    Ok(profiles)
}

fn selected_monitor<R: Runtime>(
    app: &AppHandle<R>,
    settings: &PauzaSettings,
) -> Result<Option<Monitor>, String> {
    match settings.target_screen.as_str() {
        "cursor" => {
            let position = app.cursor_position().map_err(app_error)?;
            app.monitor_from_point(position.x, position.y)
                .map_err(app_error)
        }
        _ => app.primary_monitor().map_err(app_error),
    }
}

fn fallback_monitor<R: Runtime>(app: &AppHandle<R>) -> Monitor {
    app.primary_monitor()
        .ok()
        .flatten()
        .unwrap_or_else(|| {
            app.available_monitors()
                .ok()
                .and_then(|mut monitors| monitors.pop())
                .expect("at least one monitor exists")
        })
}

fn active_break_kind<R: Runtime>(app: &AppHandle<R>) -> BreakKind {
    let autostart_enabled = app.autolaunch().is_enabled().unwrap_or(false);
    app.state::<PauzaState>()
        .snapshot(
            std::env::consts::OS.to_string(),
            app.package_info().version.to_string(),
            autostart_enabled,
        )
        .current_break
        .map(|current| current.kind)
        .unwrap_or(BreakKind::Microbreak)
}

fn break_window_profile(
    kind: BreakKind,
    monitor: &Monitor,
    style: BreakPromptStyle,
    fullscreen_mode: bool,
) -> BreakWindowProfile {
    let work_area = Bounds {
        x: monitor.work_area().position.x,
        y: monitor.work_area().position.y,
        width: monitor.work_area().size.width,
        height: monitor.work_area().size.height,
    };
    let full_bounds = Bounds {
        x: monitor.position().x,
        y: monitor.position().y,
        width: monitor.size().width,
        height: monitor.size().height,
    };
    let is_microbreak = kind == BreakKind::Microbreak;

    if fullscreen_mode {
        return BreakWindowProfile {
            width: full_bounds.width,
            height: full_bounds.height,
            x: full_bounds.x,
            y: full_bounds.y,
            decorations: false,
            focusable: true,
            always_on_top: true,
            fullscreen: true,
            skip_taskbar: true,
        };
    }

    match style {
        BreakPromptStyle::Gentle => {
            let width = if is_microbreak {
                clamp((work_area.width as f64 * 0.36) as i32, 420, 560) as u32
            } else {
                clamp((work_area.width as f64 * 0.58) as i32, 680, 940) as u32
            };
            let height = if is_microbreak {
                clamp((work_area.height as f64 * 0.34) as i32, 320, 420) as u32
            } else {
                clamp((work_area.height as f64 * 0.62) as i32, 520, 720) as u32
            };
            let (x, y) = if is_microbreak {
                bottom_right_position(work_area, width, height, 28)
            } else {
                centered_position(work_area, width, height)
            };

            BreakWindowProfile {
                width,
                height,
                x,
                y,
                decorations: false,
                focusable: false,
                always_on_top: true,
                fullscreen: false,
                skip_taskbar: true,
            }
        }
        BreakPromptStyle::Balanced => {
            let width = if is_microbreak {
                clamp((work_area.width as f64 * 0.58) as i32, 620, 860) as u32
            } else {
                clamp((work_area.width as f64 * 0.72) as i32, 840, 1120) as u32
            };
            let height = if is_microbreak {
                clamp((work_area.height as f64 * 0.42) as i32, 360, 520) as u32
            } else {
                clamp((work_area.height as f64 * 0.72) as i32, 620, 860) as u32
            };
            let (x, y) = centered_position(work_area, width, height);

            BreakWindowProfile {
                width,
                height,
                x,
                y,
                decorations: false,
                focusable: false,
                always_on_top: true,
                fullscreen: false,
                skip_taskbar: true,
            }
        }
        BreakPromptStyle::Immersive => {
            let width = if is_microbreak {
                clamp((work_area.width as f64 * 0.76) as i32, 760, 1080) as u32
            } else {
                clamp(
                    (work_area.width as f64 * 0.88) as i32,
                    920,
                    work_area.width as i32,
                ) as u32
            };
            let height = if is_microbreak {
                clamp((work_area.height as f64 * 0.62) as i32, 460, 760) as u32
            } else {
                clamp(
                    (work_area.height as f64 * 0.88) as i32,
                    680,
                    work_area.height as i32,
                ) as u32
            };
            let (x, y) = centered_position(work_area, width, height);

            BreakWindowProfile {
                width,
                height,
                x,
                y,
                decorations: false,
                focusable: true,
                always_on_top: true,
                fullscreen: false,
                skip_taskbar: true,
            }
        }
    }
}

fn clamp(value: i32, min: i32, max: i32) -> i32 {
    value.clamp(min, max)
}

fn centered_position(bounds: Bounds, width: u32, height: u32) -> (i32, i32) {
    (
        bounds.x + ((bounds.width as i32 - width as i32) / 2),
        bounds.y + ((bounds.height as i32 - height as i32) / 2),
    )
}

fn bottom_right_position(bounds: Bounds, width: u32, height: u32, offset: i32) -> (i32, i32) {
    (
        bounds.x + bounds.width as i32 - width as i32 - offset,
        bounds.y + bounds.height as i32 - height as i32 - offset,
    )
}

fn break_label(index: usize) -> String {
    format!("{BREAK_WINDOW_PREFIX}-{index}")
}

fn break_windows<R: Runtime>(app: &AppHandle<R>) -> Vec<(String, WebviewWindow<R>)> {
    app.webview_windows()
        .into_iter()
        .filter(|(label, _)| label.starts_with(BREAK_WINDOW_PREFIX))
        .collect()
}
