use crate::{
    i18n,
    state::{BreakKind, DesktopSnapshot, PauzaSettings, PauzaState, ShortcutAction},
};
use std::{
    sync::{mpsc, Arc, Mutex, OnceLock},
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
const TRAY_MENU_REFRESH_DELAY_MS: u64 = 150;
static LAST_TRAY_REFRESH_KEY: OnceLock<Mutex<Option<TrayRefreshKey>>> = OnceLock::new();
type TrayMenuTextUpdateFn = Arc<dyn Fn(&str, &str) -> Result<(), String> + Send + Sync>;
static LAST_TRAY_MENU_TEXT_UPDATER: OnceLock<Mutex<Option<TrayMenuTextUpdateFn>>> =
    OnceLock::new();

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
    #[cfg_attr(target_os = "macos", allow(dead_code))]
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
    Waiting {
        kind: Option<BreakKind>,
        minute_bucket: u64,
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

    #[cfg(target_os = "macos")]
    request_notification_permission();

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
            // Set level 1000 and fix collection behavior BEFORE show() so the window
            // enters the display system at the correct level from the start.
            configure_break_window_native_behavior(&window, profile)?;
            window.show().map_err(app_error)?;
            let _ = present_break_window(&window);
            if focusable {
                let _ = window.set_focus();
            } else {
                let _ = activate_break_application(&window);
            }
        }

        Ok(())
    })
}

pub fn close_break_window<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    run_on_main_thread(app, |handle| {
        for (_, window) in break_windows(&handle) {
            let _ = set_break_window_fullscreen(&window, false);
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
    })?;
    sync_tray_text(app)
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

    sync_tray_text(app)
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
                let should_refresh = handle_tray_action(&handle, &action);
                if should_refresh {
                    // macOS native tray menus are sensitive to set_menu() during dismissal.
                    thread::sleep(Duration::from_millis(TRAY_MENU_REFRESH_DELAY_MS));
                    let _ = refresh_tray_if_needed(&handle);
                }
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

    let _ = sync_tray_text(app);

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

fn handle_tray_action<R: Runtime>(app: &AppHandle<R>, action: &str) -> bool {
    match action {
        "open" => {
            let _ = reveal_main_window(app);
            false
        }
        "skip-scheduled" => {
            if app
                .state::<PauzaState>()
                .skip_to_next_scheduled_break("tray")
            {
                let _ = close_break_window(app);
            }
            true
        }
        "skip-microbreak" => {
            if app.state::<PauzaState>().skip_to_microbreak("tray") {
                let _ = close_break_window(app);
            }
            true
        }
        "skip-long-break" => {
            if app.state::<PauzaState>().skip_to_long_break("tray") {
                let _ = close_break_window(app);
            }
            true
        }
        "pause-30" => {
            if app.state::<PauzaState>().pause_for_minutes(30, "tray") {
                let _ = close_break_window(app);
            }
            true
        }
        "pause-60" => {
            if app.state::<PauzaState>().pause_for_minutes(60, "tray") {
                let _ = close_break_window(app);
            }
            true
        }
        "pause-120" => {
            if app.state::<PauzaState>().pause_for_minutes(120, "tray") {
                let _ = close_break_window(app);
            }
            true
        }
        "pause-300" => {
            if app.state::<PauzaState>().pause_for_minutes(300, "tray") {
                let _ = close_break_window(app);
            }
            true
        }
        "pause-forever" => {
            if app.state::<PauzaState>().pause_for_minutes(0, "tray") {
                let _ = close_break_window(app);
            }
            true
        }
        "resume" => {
            if app.state::<PauzaState>().resume("tray") {
                let _ = close_break_window(app);
            }
            true
        }
        "focus-25" => {
            if app.state::<PauzaState>().start_focus_session(25, "tray") {
                let _ = close_break_window(app);
            }
            true
        }
        "focus-45" => {
            if app.state::<PauzaState>().start_focus_session(45, "tray") {
                let _ = close_break_window(app);
            }
            true
        }
        "focus-60" => {
            if app.state::<PauzaState>().start_focus_session(60, "tray") {
                let _ = close_break_window(app);
            }
            true
        }
        "reset" => {
            if app.state::<PauzaState>().reset_breaks("tray") {
                let _ = close_break_window(app);
            }
            true
        }
        "hide" => {
            let _ = hide_main_window(app);
            false
        }
        "toggle-autostart" => {
            toggle_autostart_from_tray(app);
            true
        }
        "quit" => {
            app.exit(0);
            false
        }
        _ => false,
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

fn sync_tray_text<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let Some(tray) = app.tray_by_id(TRAY_ID) else {
        return Ok(());
    };

    let snapshot = tray_snapshot(app);
    sync_tray_menu_text(&snapshot)?;
    apply_tray_title(&tray, &snapshot)?;
    apply_tray_tooltip(&tray, &snapshot)?;
    Ok(())
}

fn register_tray_menu_text_updater<R: Runtime + 'static>(
    status_item: MenuItem<R>,
    detail_item: MenuItem<R>,
) {
    let cache = LAST_TRAY_MENU_TEXT_UPDATER.get_or_init(|| Mutex::new(None));
    let mut updater = cache.lock().expect("tray menu text updater lock poisoned");
    *updater = Some(Arc::new(move |status: &str, detail: &str| {
        status_item.set_text(status).map_err(app_error)?;
        detail_item.set_text(detail).map_err(app_error)
    }));
}

fn sync_tray_menu_text(snapshot: &DesktopSnapshot) -> Result<(), String> {
    let cache = LAST_TRAY_MENU_TEXT_UPDATER.get_or_init(|| Mutex::new(None));
    // Clone the Arc under the lock (cheap), then release the lock BEFORE calling
    // set_text. This prevents a lock-inversion deadlock: set_text dispatches to
    // the main thread and blocks the calling thread; if the main thread is
    // simultaneously waiting to acquire this lock (in register_tray_menu_text_updater),
    // neither thread can proceed. Releasing the lock before the dispatch avoids this.
    let update_fn = cache.lock().expect("tray menu text updater lock poisoned").clone();
    if let Some(f) = update_fn {
        f(&snapshot.status, &snapshot.status_detail)?;
    }
    Ok(())
}

fn tray_snapshot<R: Runtime>(app: &AppHandle<R>) -> DesktopSnapshot {
    let state = app.state::<PauzaState>();
    let autostart_enabled = app.autolaunch().is_enabled().unwrap_or(false);
    state.snapshot(
        std::env::consts::OS.to_string(),
        app.package_info().version.to_string(),
        autostart_enabled,
    )
}

#[cfg(not(target_os = "windows"))]
fn apply_tray_title<R: Runtime>(
    tray: &tauri::tray::TrayIcon<R>,
    snapshot: &DesktopSnapshot,
) -> Result<(), String> {
    tray.set_title(tray_title(snapshot, current_time_ms()))
        .map_err(app_error)
}

#[cfg(target_os = "windows")]
fn apply_tray_title<R: Runtime>(
    _tray: &tauri::tray::TrayIcon<R>,
    _snapshot: &DesktopSnapshot,
) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn apply_tray_tooltip<R: Runtime>(
    tray: &tauri::tray::TrayIcon<R>,
    snapshot: &DesktopSnapshot,
) -> Result<(), String> {
    tray.set_tooltip(Some(tray_tooltip(snapshot)))
        .map_err(app_error)
}

#[cfg(target_os = "linux")]
fn apply_tray_tooltip<R: Runtime>(
    _tray: &tauri::tray::TrayIcon<R>,
    _snapshot: &DesktopSnapshot,
) -> Result<(), String> {
    Ok(())
}

fn tray_refresh_key<R: Runtime>(app: &AppHandle<R>) -> TrayRefreshKey {
    let snapshot = tray_snapshot(app);
    let settings = snapshot.settings.clone();
    let autostart_enabled = snapshot.autostart_enabled;

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

    if let Some(remaining) = snapshot.next_break_wait_remaining_ms {
        return TrayDetailMode::Waiting {
            kind: snapshot.next_break_kind,
            minute_bucket: duration_minute_bucket(remaining),
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

fn tray_title(snapshot: &DesktopSnapshot, now: u64) -> Option<String> {
    if let Some(remaining) = snapshot.next_break_wait_remaining_ms {
        return Some(format_tray_countdown(remaining));
    }

    if !snapshot.settings.show_time_to_break_in_tray {
        return None;
    }

    tray_countdown_ms(snapshot, now).map(format_tray_countdown)
}

fn tray_countdown_ms(snapshot: &DesktopSnapshot, now: u64) -> Option<u64> {
    if let Some(current) = &snapshot.current_break {
        if current.manual_awaiting {
            return Some(0);
        }

        return Some(current.ends_at_ms.saturating_sub(now));
    }

    if let Some(remaining) = snapshot
        .focus_until_ms
        .map(|until| until.saturating_sub(now))
        .filter(|remaining| *remaining > 0)
    {
        return Some(remaining);
    }

    if let Some(remaining) = snapshot
        .pause_until_ms
        .map(|until| until.saturating_sub(now))
        .filter(|remaining| *remaining > 0)
    {
        return Some(remaining);
    }

    if let Some(remaining) = snapshot.next_break_wait_remaining_ms {
        return Some(remaining);
    }

    if snapshot.app_exclusion_active || snapshot.dnd_active || natural_break_active(snapshot) {
        return None;
    }

    snapshot.next_break_in_ms.filter(|remaining| *remaining > 0)
}

fn format_tray_countdown(ms: u64) -> String {
    let total_seconds = ms.div_ceil(1_000);
    let hours = total_seconds / 3_600;
    let minutes = (total_seconds % 3_600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        return format!("{hours}:{minutes:02}:{seconds:02}");
    }

    format!("{minutes}:{seconds:02}")
}

fn tray_tooltip(snapshot: &DesktopSnapshot) -> String {
    if snapshot.status_detail.trim().is_empty() || snapshot.status == snapshot.status_detail {
        return snapshot.status.clone();
    }

    format!("{}\n{}", snapshot.status, snapshot.status_detail)
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

fn populate_tray_menu<R: Runtime + 'static, M: TrayMenuContainer<R>>(
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
    register_tray_menu_text_updater(status_item.clone(), detail_item.clone());
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
    if !profile.fullscreen {
        set_break_window_fullscreen(window, false)?;
    }
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
    // On macOS the window level is set natively in configure_break_window_native_behavior
    // to avoid a race: tao's set_always_on_top dispatches setLevel:3 asynchronously via GCD,
    // which would fire *after* our synchronous setLevel:1000 and silently reset it to 3.
    #[cfg(not(target_os = "macos"))]
    window
        .set_always_on_top(profile.always_on_top)
        .map_err(app_error)?;
    window
        .set_skip_taskbar(profile.skip_taskbar)
        .map_err(app_error)?;
    set_break_window_fullscreen(window, profile.fullscreen)?;
    window
        .set_visible_on_all_workspaces(true)
        .map_err(app_error)?;
    Ok(())
}

fn set_break_window_fullscreen<R: Runtime>(
    window: &WebviewWindow<R>,
    fullscreen: bool,
) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        if fullscreen {
            window.set_simple_fullscreen(true).map_err(app_error)?;
        } else {
            let _ = window.set_simple_fullscreen(false);
            window.set_fullscreen(false).map_err(app_error)?;
        }
        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    {
        window.set_fullscreen(fullscreen).map_err(app_error)
    }
}

// Pauza needs the break window to follow the user's active fullscreen Space instead of
// sitting on a different desktop or monitor. In practice we need the same trio of native
// collection behavior bits that previously fixed the fullscreen-space visibility bug:
//   - CanJoinAllSpaces
//   - MoveToActiveSpace
//   - FullScreenAuxiliary
// Removing the latter two regresses back to "the break started somewhere else".
#[cfg(target_os = "macos")]
const BREAK_WINDOW_COLLECTION_BEHAVIOR_BITS: usize = (1 << 0) | (1 << 1) | (1 << 8);
#[cfg(target_os = "macos")]
const BREAK_WINDOW_LEVEL_HIGHEST: isize = 1000;

#[cfg(target_os = "macos")]
fn break_window_level(profile: BreakWindowProfile) -> isize {
    let _ = profile;
    BREAK_WINDOW_LEVEL_HIGHEST
}

#[cfg(target_os = "macos")]
fn break_window_collection_behavior(current: usize) -> usize {
    current | BREAK_WINDOW_COLLECTION_BEHAVIOR_BITS
}

#[cfg(target_os = "macos")]
fn request_notification_permission() {
    use objc2::{class, exception, msg_send, runtime::AnyObject};
    use std::ffi::c_void;
    use std::panic::AssertUnwindSafe;

    // Ask macOS to register this app for prominent (banner/alert) notifications via
    // UNUserNotificationCenter. The deprecated NSUserNotificationCenter used by notify-rust
    // may default to "Silent" delivery on newer macOS unless the app has explicit
    // UNUserNotificationCenter authorization. A nil completion handler is valid — the system
    // simply won't call back with the result, but the authorization dialog is still shown.
    //
    // IMPORTANT: `currentNotificationCenter` crashes with NSInternalInconsistencyException if
    // the process has no bundle identifier (e.g. `tauri dev` runs a bare binary, not an .app).
    // The exception is thrown inside dispatch_once on a different thread, so exception::catch
    // cannot intercept it. Guard by checking bundleIdentifier first; skip in dev.
    let _ = exception::catch(AssertUnwindSafe(|| unsafe {
        let main_bundle: *mut AnyObject = msg_send![class!(NSBundle), mainBundle];
        if main_bundle.is_null() {
            return;
        }
        let bundle_id: *mut AnyObject = msg_send![main_bundle, bundleIdentifier];
        if bundle_id.is_null() {
            return; // not a .app bundle (e.g. tauri dev) — skip
        }
        let center: *mut AnyObject =
            msg_send![class!(UNUserNotificationCenter), currentNotificationCenter];
        if !center.is_null() {
            let options: usize = 1 | 2 | 4; // badge | sound | alert
            let _: () = msg_send![
                center,
                requestAuthorizationWithOptions: options,
                completionHandler: std::ptr::null::<c_void>()
            ];
        }
    }));
}


#[cfg(target_os = "macos")]
fn run_macos_native_break_window_patch<R: Runtime>(
    window: &WebviewWindow<R>,
    context: &'static str,
    patch: impl FnOnce(*mut objc2::runtime::AnyObject) -> Result<(), String>,
) -> Result<(), String> {
    use objc2::{exception, runtime::AnyObject};
    use std::panic::AssertUnwindSafe;

    match exception::catch(AssertUnwindSafe(|| {
        let ns_window = window.ns_window().map_err(app_error)? as *mut AnyObject;
        if ns_window.is_null() {
            return Ok(());
        }
        patch(ns_window)
    })) {
        Ok(result) => result,
        Err(Some(exception)) => {
            eprintln!(
                "pauza: skipped macOS break window native patch during {context}: {exception:?}"
            );
            Ok(())
        }
        Err(None) => {
            eprintln!(
                "pauza: skipped macOS break window native patch during {context}: nil Objective-C exception"
            );
            Ok(())
        }
    }
}

#[cfg(target_os = "macos")]
fn configure_break_window_native_behavior<R: Runtime>(
    window: &WebviewWindow<R>,
    profile: BreakWindowProfile,
) -> Result<(), String> {
    use objc2::{msg_send, runtime::AnyObject};
    run_macos_native_break_window_patch(
        window,
        "configure_break_window_native_behavior",
        move |ns_window: *mut AnyObject| {
            unsafe {
                let behavior: usize = msg_send![ns_window, collectionBehavior];
                let new_behavior = break_window_collection_behavior(behavior);
                let _: () = msg_send![ns_window, setCollectionBehavior: new_behavior];
                let _: () = msg_send![ns_window, setLevel: break_window_level(profile)];
            }

            Ok(())
        },
    )
}

#[cfg(not(target_os = "macos"))]
fn configure_break_window_native_behavior<R: Runtime>(
    _window: &WebviewWindow<R>,
    _profile: BreakWindowProfile,
) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "macos")]
fn present_break_window<R: Runtime>(window: &WebviewWindow<R>) -> Result<(), String> {
    use objc2::{msg_send, runtime::AnyObject};
    run_macos_native_break_window_patch(
        window,
        "present_break_window",
        |ns_window: *mut AnyObject| {
            unsafe {
                let _: () = msg_send![ns_window, orderFrontRegardless];
            }

            Ok(())
        },
    )
}

#[cfg(not(target_os = "macos"))]
fn present_break_window<R: Runtime>(_window: &WebviewWindow<R>) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "macos")]
fn activate_break_application<R: Runtime>(window: &WebviewWindow<R>) -> Result<(), String> {
    use objc2::{class, msg_send, runtime::AnyObject};
    run_macos_native_break_window_patch(
        window,
        "activate_break_application",
        |_ns_window: *mut AnyObject| {
            unsafe {
                let ns_app: *mut AnyObject =
                    msg_send![class!(NSApplication), sharedApplication];
                if !ns_app.is_null() {
                    // [NSApp activate] is the macOS 14+ replacement for the removed
                    // activateIgnoringOtherApps:. Fall back to the old API on older systems.
                    let sel = objc2::runtime::Sel::register(c"activate");
                    let responds: bool = msg_send![ns_app, respondsToSelector: sel];
                    if responds {
                        let _: () = msg_send![ns_app, activate];
                    } else {
                        let _: () = msg_send![ns_app, activateIgnoringOtherApps: true];
                    }
                }
            }

            Ok(())
        },
    )
}

#[cfg(not(target_os = "macos"))]
fn activate_break_application<R: Runtime>(_window: &WebviewWindow<R>) -> Result<(), String> {
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

    let profiles = selected_monitors
        .into_iter()
        .map(|monitor| {
            let profile = break_window_profile(&monitor, settings.fullscreen);
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

fn break_window_profile(monitor: &Monitor, fullscreen_mode: bool) -> BreakWindowProfile {
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

    let max_width = work_area.width as i32;
    let max_height = work_area.height as i32;
    let preferred_width_limit = clamp((work_area.width as f64 * 0.84).round() as i32, 1, max_width);
    let preferred_height_limit =
        clamp((work_area.height as f64 * 0.84).round() as i32, 1, max_height);
    let min_width_limit = 800.min(max_width);
    let min_height_limit = 450.min(max_height);
    let (preferred_width, preferred_height) =
        fit_aspect_ratio(preferred_width_limit, preferred_height_limit, 16.0 / 9.0);
    let (min_width, min_height) = fit_aspect_ratio(min_width_limit, min_height_limit, 16.0 / 9.0);
    let width = preferred_width.max(min_width);
    let height = preferred_height.max(min_height);
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

fn clamp(value: i32, min: i32, max: i32) -> i32 {
    value.clamp(min, max)
}

fn fit_aspect_ratio(width_limit: i32, height_limit: i32, aspect_ratio: f64) -> (u32, u32) {
    let width_from_height = (height_limit as f64 * aspect_ratio).round() as i32;

    if width_from_height <= width_limit {
        return (width_from_height.max(1) as u32, height_limit.max(1) as u32);
    }

    let height_from_width = (width_limit as f64 / aspect_ratio).round() as i32;
    (width_limit.max(1) as u32, height_from_width.max(1) as u32)
}

fn centered_position(bounds: Bounds, width: u32, height: u32) -> (i32, i32) {
    (
        bounds.x + ((bounds.width as i32 - width as i32) / 2),
        bounds.y + ((bounds.height as i32 - height as i32) / 2),
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

#[cfg(test)]
mod tests {
    use super::{
        format_tray_countdown, tray_countdown_ms, tray_detail_mode, tray_title,
        BreakWindowProfile, TrayDetailMode,
    };
    use crate::state::{BreakKind, CurrentBreakSnapshot, DesktopSnapshot, PauzaSettings};

    fn base_snapshot() -> DesktopSnapshot {
        DesktopSnapshot {
            product_name: "Pauza",
            runtime: "Tauri 2".into(),
            platform: "macos".into(),
            app_version: "0.1.0".into(),
            autostart_enabled: false,
            settings: PauzaSettings::default(),
            status: "Running".into(),
            status_detail: "Next break soon".into(),
            next_break_kind: Some(BreakKind::Microbreak),
            next_break_due_ms: Some(1_000),
            next_break_in_ms: Some(1_000),
            next_break_wait_remaining_ms: None,
            current_break: None,
            pause_until_ms: None,
            paused_indefinitely: false,
            focus_until_ms: None,
            idle_ms: 0,
            dnd_active: false,
            app_exclusion_active: false,
            app_exclusion_match: None,
            last_action: String::new(),
        }
    }

    #[test]
    fn format_tray_countdown_uses_live_seconds() {
        assert_eq!(format_tray_countdown(59_000), "0:59");
        assert_eq!(format_tray_countdown(3_661_000), "1:01:01");
    }

    #[test]
    fn tray_title_prefers_current_break_remaining() {
        let mut snapshot = base_snapshot();
        snapshot.next_break_in_ms = Some(120_000);
        snapshot.current_break = Some(CurrentBreakSnapshot {
            kind: BreakKind::Microbreak,
            title: "Microbreak".into(),
            detail: String::new(),
            started_at_ms: 1_000,
            ends_at_ms: 91_000,
            duration_ms: 90_000,
            strict_mode: false,
            manual_awaiting: false,
            can_postpone: true,
            can_skip: true,
            show_clock: false,
        });

        assert_eq!(tray_title(&snapshot, 1_000).as_deref(), Some("1:30"));
    }

    #[test]
    fn blocked_states_hide_schedule_countdown() {
        let mut snapshot = base_snapshot();
        snapshot.next_break_in_ms = Some(45_000);
        snapshot.dnd_active = true;

        assert_eq!(tray_countdown_ms(&snapshot, 0), None);
        assert_eq!(tray_title(&snapshot, 0), None);
    }

    #[test]
    fn waiting_countdown_is_visible_as_transient_feedback() {
        let mut snapshot = base_snapshot();
        snapshot.settings.show_time_to_break_in_tray = false;
        snapshot.next_break_due_ms = Some(0);
        snapshot.next_break_in_ms = None;
        snapshot.next_break_wait_remaining_ms = Some(89_001);

        assert_eq!(
            tray_detail_mode(&snapshot),
            TrayDetailMode::Waiting {
                kind: Some(BreakKind::Microbreak),
                minute_bucket: 2,
            }
        );
        assert_eq!(tray_countdown_ms(&snapshot, 0), Some(89_001));
        assert_eq!(tray_title(&snapshot, 0).as_deref(), Some("1:30"));
    }

    #[test]
    fn tray_title_respects_setting_toggle() {
        let mut snapshot = base_snapshot();
        snapshot.settings.show_time_to_break_in_tray = false;

        assert_eq!(tray_title(&snapshot, 0), None);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn macos_break_window_native_overlay_policy_uses_expected_levels() {
        let fullscreen = BreakWindowProfile {
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            decorations: false,
            focusable: true,
            always_on_top: true,
            fullscreen: true,
            skip_taskbar: true,
        };
        let windowed = BreakWindowProfile {
            fullscreen: false,
            ..fullscreen
        };

        assert_eq!(
            super::break_window_collection_behavior(0),
            super::BREAK_WINDOW_COLLECTION_BEHAVIOR_BITS
        );
        assert_eq!(
            super::BREAK_WINDOW_COLLECTION_BEHAVIOR_BITS,
            (1 << 0) | (1 << 1) | (1 << 8)
        );
        assert_eq!(super::break_window_level(fullscreen), super::BREAK_WINDOW_LEVEL_HIGHEST);
        assert_eq!(super::break_window_level(windowed), super::BREAK_WINDOW_LEVEL_HIGHEST);
    }
}
