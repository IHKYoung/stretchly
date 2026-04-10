use crate::i18n;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const SETTINGS_FILE: &str = "settings.json";
const BREAK_ACTION_DELAY_MS: u64 = 100;
const BREAK_POSTPONE_WINDOW_MS: u64 = 10_000;
const DEFAULT_IDLE_OPPORTUNITY_SECONDS: u64 = 6;
const MICROBREAK_FINAL_WAIT_CAP_MS: u64 = 45_000;
const LONG_BREAK_FINAL_WAIT_CAP_MS: u64 = 90_000;

#[derive(Debug, Clone, Copy)]
struct SmartWaitStage {
    wait_until_ms: u64,
    idle_required_ms: u64,
}

const MICROBREAK_SMART_WAIT_STAGES: [SmartWaitStage; 3] = [
    SmartWaitStage {
        wait_until_ms: 15_000,
        idle_required_ms: 6_000,
    },
    SmartWaitStage {
        wait_until_ms: 30_000,
        idle_required_ms: 3_000,
    },
    SmartWaitStage {
        wait_until_ms: MICROBREAK_FINAL_WAIT_CAP_MS,
        idle_required_ms: 1_000,
    },
];

const LONG_BREAK_SMART_WAIT_STAGES: [SmartWaitStage; 3] = [
    SmartWaitStage {
        wait_until_ms: 30_000,
        idle_required_ms: 8_000,
    },
    SmartWaitStage {
        wait_until_ms: 60_000,
        idle_required_ms: 4_000,
    },
    SmartWaitStage {
        wait_until_ms: LONG_BREAK_FINAL_WAIT_CAP_MS,
        idle_required_ms: 1_000,
    },
];

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum BreakKind {
    #[default]
    Microbreak,
    LongBreak,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum AppExclusionRule {
    #[default]
    Pause,
    Resume,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum ReminderMode {
    #[default]
    Smart,
    Forced,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub enum BreakBackdrop {
    #[default]
    Paper,
    Dawn,
    Forest,
    Night,
    Custom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum BreakSound {
    #[default]
    Silence,
    CrystalGlass,
    WindChime,
    TicToc,
    Reverie,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PauzaSettings {
    pub language: String,
    pub microbreak_enabled: bool,
    pub microbreak_interval_minutes: u64,
    pub microbreak_duration_seconds: u64,
    pub microbreak_notification_enabled: bool,
    pub microbreak_notification_seconds: u64,
    pub microbreak_allow_postpone: bool,
    pub microbreak_postpone_minutes: u64,
    pub microbreak_postpones_limit: u64,
    pub reminder_mode: ReminderMode,
    pub idle_opportunity_seconds: u64,
    pub microbreak_manual_finish: bool,
    pub long_break_enabled: bool,
    pub long_break_every: u64,
    pub long_break_duration_minutes: u64,
    pub long_break_notification_enabled: bool,
    pub long_break_notification_seconds: u64,
    pub long_break_allow_postpone: bool,
    pub long_break_postpone_minutes: u64,
    pub long_break_postpones_limit: u64,
    pub long_break_manual_finish: bool,
    pub natural_breaks: bool,
    pub natural_break_reset_minutes: u64,
    pub monitor_dnd: bool,
    pub app_exclusions_enabled: bool,
    pub app_exclusion_rule: AppExclusionRule,
    pub app_exclusion_commands: String,
    pub fullscreen: bool,
    pub break_backdrop: BreakBackdrop,
    pub break_custom_backdrop_label: Option<String>,
    pub break_custom_backdrop_data_url: Option<String>,
    pub break_ideas_enabled: bool,
    pub show_breaks_on_all_screens: bool,
    pub target_screen: String,
    pub current_time_in_breaks: bool,
    pub show_time_to_break_in_tray: bool,
    pub microbreak_start_sound: BreakSound,
    pub microbreak_end_sound: BreakSound,
    pub long_break_start_sound: BreakSound,
    pub long_break_end_sound: BreakSound,
    pub break_sound_volume: u64,
    pub show_tray_menu_in_strict_mode: bool,
    pub reveal_settings_shortcut: String,
    pub focus_45_shortcut: String,
    pub pause_toggle_shortcut: String,
    pub pause_30_shortcut: String,
    pub pause_60_shortcut: String,
    pub pause_120_shortcut: String,
    pub pause_300_shortcut: String,
    pub skip_next_scheduled_shortcut: String,
    pub skip_next_microbreak_shortcut: String,
    pub skip_next_long_break_shortcut: String,
    pub reset_breaks_shortcut: String,
}

impl Default for PauzaSettings {
    fn default() -> Self {
        Self {
            language: "zh-CN".into(),
            microbreak_enabled: true,
            microbreak_interval_minutes: 10,
            microbreak_duration_seconds: 20,
            microbreak_notification_enabled: true,
            microbreak_notification_seconds: 10,
            microbreak_allow_postpone: true,
            microbreak_postpone_minutes: 2,
            microbreak_postpones_limit: 1,
            reminder_mode: ReminderMode::Smart,
            idle_opportunity_seconds: DEFAULT_IDLE_OPPORTUNITY_SECONDS,
            microbreak_manual_finish: false,
            long_break_enabled: true,
            long_break_every: 3,
            long_break_duration_minutes: 5,
            long_break_notification_enabled: true,
            long_break_notification_seconds: 30,
            long_break_allow_postpone: true,
            long_break_postpone_minutes: 5,
            long_break_postpones_limit: 1,
            long_break_manual_finish: false,
            natural_breaks: true,
            natural_break_reset_minutes: 5,
            monitor_dnd: true,
            app_exclusions_enabled: false,
            app_exclusion_rule: AppExclusionRule::Pause,
            app_exclusion_commands: String::new(),
            fullscreen: false,
            break_backdrop: BreakBackdrop::Paper,
            break_custom_backdrop_label: None,
            break_custom_backdrop_data_url: None,
            break_ideas_enabled: true,
            show_breaks_on_all_screens: true,
            target_screen: "primary".into(),
            current_time_in_breaks: false,
            show_time_to_break_in_tray: true,
            microbreak_start_sound: BreakSound::Silence,
            microbreak_end_sound: BreakSound::CrystalGlass,
            long_break_start_sound: BreakSound::Silence,
            long_break_end_sound: BreakSound::CrystalGlass,
            break_sound_volume: 100,
            show_tray_menu_in_strict_mode: false,
            reveal_settings_shortcut: "CmdOrCtrl+Shift+P".into(),
            focus_45_shortcut: "CmdOrCtrl+Shift+F".into(),
            pause_toggle_shortcut: String::new(),
            pause_30_shortcut: String::new(),
            pause_60_shortcut: String::new(),
            pause_120_shortcut: String::new(),
            pause_300_shortcut: String::new(),
            skip_next_scheduled_shortcut: String::new(),
            skip_next_microbreak_shortcut: String::new(),
            skip_next_long_break_shortcut: String::new(),
            reset_breaks_shortcut: String::new(),
        }
    }
}

impl PauzaSettings {
    pub fn sanitized(mut self) -> Self {
        self.language = i18n::normalize_language(&self.language).into();
        self.microbreak_interval_minutes = self.microbreak_interval_minutes.clamp(1, 180);
        self.microbreak_duration_seconds = self.microbreak_duration_seconds.clamp(10, 300);
        self.microbreak_notification_seconds = self.microbreak_notification_seconds.clamp(5, 300);
        self.microbreak_postpone_minutes = self.microbreak_postpone_minutes.clamp(1, 30);
        self.microbreak_postpones_limit = self.microbreak_postpones_limit.clamp(0, 5);
        self.idle_opportunity_seconds = self.idle_opportunity_seconds.clamp(3, 120);
        self.long_break_every = self.long_break_every.clamp(1, 12);
        self.long_break_duration_minutes = self.long_break_duration_minutes.clamp(1, 60);
        self.long_break_notification_seconds = self.long_break_notification_seconds.clamp(5, 600);
        self.long_break_postpone_minutes = self.long_break_postpone_minutes.clamp(1, 60);
        self.long_break_postpones_limit = self.long_break_postpones_limit.clamp(0, 5);
        self.natural_break_reset_minutes = self.natural_break_reset_minutes.clamp(1, 60);
        self.break_sound_volume = self.break_sound_volume.clamp(0, 100);
        self.break_custom_backdrop_label =
            sanitize_optional_string(self.break_custom_backdrop_label.take());
        self.break_custom_backdrop_data_url =
            sanitize_optional_string(self.break_custom_backdrop_data_url.take());
        if self.break_backdrop == BreakBackdrop::Custom
            && self.break_custom_backdrop_data_url.is_none()
        {
            self.break_backdrop = BreakBackdrop::Paper;
        }
        self.target_screen = normalize_target_screen(&self.target_screen);
        self.reveal_settings_shortcut = normalize_shortcut(
            &self.reveal_settings_shortcut,
            "CmdOrCtrl+Shift+P",
        );
        self.focus_45_shortcut =
            normalize_shortcut(&self.focus_45_shortcut, "CmdOrCtrl+Shift+F");
        self.pause_toggle_shortcut = normalize_shortcut(&self.pause_toggle_shortcut, "");
        self.pause_30_shortcut = normalize_shortcut(&self.pause_30_shortcut, "");
        self.pause_60_shortcut = normalize_shortcut(&self.pause_60_shortcut, "");
        self.pause_120_shortcut = normalize_shortcut(&self.pause_120_shortcut, "");
        self.pause_300_shortcut = normalize_shortcut(&self.pause_300_shortcut, "");
        self.skip_next_scheduled_shortcut =
            normalize_shortcut(&self.skip_next_scheduled_shortcut, "");
        self.skip_next_microbreak_shortcut =
            normalize_shortcut(&self.skip_next_microbreak_shortcut, "");
        self.skip_next_long_break_shortcut =
            normalize_shortcut(&self.skip_next_long_break_shortcut, "");
        self.reset_breaks_shortcut = normalize_shortcut(&self.reset_breaks_shortcut, "");
        self
    }

    pub fn microbreak_interval_ms(&self) -> u64 {
        self.microbreak_interval_minutes * 60_000
    }

    pub fn duration_ms(&self, kind: BreakKind) -> u64 {
        match kind {
            BreakKind::Microbreak => self.microbreak_duration_seconds * 1_000,
            BreakKind::LongBreak => self.long_break_duration_minutes * 60_000,
        }
    }

    pub fn notification_enabled(&self, kind: BreakKind) -> bool {
        match kind {
            BreakKind::Microbreak => self.microbreak_notification_enabled,
            BreakKind::LongBreak => self.long_break_notification_enabled,
        }
    }

    pub fn notification_ms(&self, kind: BreakKind) -> u64 {
        match kind {
            BreakKind::Microbreak => self.microbreak_notification_seconds * 1_000,
            BreakKind::LongBreak => self.long_break_notification_seconds * 1_000,
        }
    }

    pub fn allow_postpone(&self, kind: BreakKind) -> bool {
        match kind {
            BreakKind::Microbreak => self.microbreak_allow_postpone,
            BreakKind::LongBreak => self.long_break_allow_postpone,
        }
    }

    pub fn postpone_ms(&self, kind: BreakKind) -> u64 {
        match kind {
            BreakKind::Microbreak => self.microbreak_postpone_minutes * 60_000,
            BreakKind::LongBreak => self.long_break_postpone_minutes * 60_000,
        }
    }

    pub fn postpone_limit(&self, kind: BreakKind) -> u64 {
        match kind {
            BreakKind::Microbreak => self.microbreak_postpones_limit,
            BreakKind::LongBreak => self.long_break_postpones_limit,
        }
    }

    pub fn strict_mode(&self, _kind: BreakKind) -> bool {
        self.reminder_mode == ReminderMode::Forced
    }

    pub fn manual_finish(&self, kind: BreakKind) -> bool {
        match kind {
            BreakKind::Microbreak => self.microbreak_manual_finish,
            BreakKind::LongBreak => self.long_break_manual_finish,
        }
    }

    pub fn natural_break_reset_ms(&self) -> u64 {
        self.natural_break_reset_minutes * 60_000
    }

    pub fn exclusion_commands(&self) -> Vec<String> {
        self.app_exclusion_commands
            .split(|ch| [',', '\n', ';'].contains(&ch))
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .map(ToOwned::to_owned)
            .collect()
    }

    pub fn shortcut_bindings(&self) -> Vec<ShortcutBinding> {
        let bindings = [
            ShortcutBinding::new(
                "reveal-settings",
                &self.reveal_settings_shortcut,
                ShortcutAction::RevealSettings,
            ),
            ShortcutBinding::new(
                "focus-45",
                &self.focus_45_shortcut,
                ShortcutAction::Focus45,
            ),
            ShortcutBinding::new(
                "pause-toggle",
                &self.pause_toggle_shortcut,
                ShortcutAction::PauseToggle,
            ),
            ShortcutBinding::new("pause-30", &self.pause_30_shortcut, ShortcutAction::Pause30),
            ShortcutBinding::new("pause-60", &self.pause_60_shortcut, ShortcutAction::Pause60),
            ShortcutBinding::new("pause-120", &self.pause_120_shortcut, ShortcutAction::Pause120),
            ShortcutBinding::new("pause-300", &self.pause_300_shortcut, ShortcutAction::Pause300),
            ShortcutBinding::new(
                "skip-next",
                &self.skip_next_scheduled_shortcut,
                ShortcutAction::SkipNextScheduled,
            ),
            ShortcutBinding::new(
                "skip-microbreak",
                &self.skip_next_microbreak_shortcut,
                ShortcutAction::SkipNextMicrobreak,
            ),
            ShortcutBinding::new(
                "skip-long-break",
                &self.skip_next_long_break_shortcut,
                ShortcutAction::SkipNextLongBreak,
            ),
            ShortcutBinding::new(
                "reset-breaks",
                &self.reset_breaks_shortcut,
                ShortcutAction::ResetBreaks,
            ),
        ];

        bindings
            .into_iter()
            .filter(|binding| !binding.shortcut.is_empty())
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutAction {
    RevealSettings,
    Focus45,
    PauseToggle,
    Pause30,
    Pause60,
    Pause120,
    Pause300,
    SkipNextScheduled,
    SkipNextMicrobreak,
    SkipNextLongBreak,
    ResetBreaks,
}

#[derive(Debug, Clone)]
pub struct ShortcutBinding {
    pub shortcut: String,
    pub action: ShortcutAction,
}

impl ShortcutBinding {
    fn new(_id: &'static str, shortcut: &str, action: ShortcutAction) -> Self {
        Self {
            shortcut: shortcut.to_string(),
            action,
        }
    }
}

#[derive(Debug, Clone)]
struct CurrentBreak {
    kind: BreakKind,
    title: String,
    detail: String,
    started_at_ms: u64,
    ends_at_ms: u64,
    duration_ms: u64,
    strict_mode: bool,
    manual_finish: bool,
    manual_awaiting: bool,
    postpones_used: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentBreakSnapshot {
    pub kind: BreakKind,
    pub title: String,
    pub detail: String,
    pub started_at_ms: u64,
    pub ends_at_ms: u64,
    pub duration_ms: u64,
    pub strict_mode: bool,
    pub manual_awaiting: bool,
    pub can_postpone: bool,
    pub can_skip: bool,
    pub show_clock: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DesktopSnapshot {
    pub product_name: &'static str,
    pub runtime: String,
    pub platform: String,
    pub app_version: String,
    pub autostart_enabled: bool,
    pub settings: PauzaSettings,
    pub status: String,
    pub status_detail: String,
    pub next_break_kind: Option<BreakKind>,
    pub next_break_due_ms: Option<u64>,
    pub next_break_in_ms: Option<u64>,
    pub current_break: Option<CurrentBreakSnapshot>,
    pub pause_until_ms: Option<u64>,
    pub paused_indefinitely: bool,
    pub focus_until_ms: Option<u64>,
    pub idle_ms: u64,
    pub dnd_active: bool,
    pub app_exclusion_active: bool,
    pub app_exclusion_match: Option<String>,
    pub last_action: String,
}

#[derive(Debug, Default)]
pub struct EngineActions {
    pub open_break_window: bool,
    pub close_break_window: bool,
    pub notify_title: Option<String>,
    pub notify_body: Option<String>,
}

#[derive(Default)]
struct RuntimeState {
    settings: PauzaSettings,
    config_path: Option<PathBuf>,
    cycle_index: u64,
    next_break_due_ms: Option<u64>,
    next_break_kind: Option<BreakKind>,
    next_notification_due_ms: Option<u64>,
    next_break_wait_started_ms: Option<u64>,
    pending_break_postpones: u64,
    current_break: Option<CurrentBreak>,
    paused_until_ms: Option<u64>,
    paused_indefinitely: bool,
    focus_until_ms: Option<u64>,
    idle_ms: u64,
    dnd_active: bool,
    app_exclusion_match: Option<String>,
    last_action: String,
}

#[derive(Clone, Default)]
pub struct PauzaState {
    runtime: Arc<Mutex<RuntimeState>>,
    autostart: Arc<Mutex<Option<bool>>>,
}

impl PauzaState {
    /// Returns the cached autostart state, calling `init` once if not yet initialized.
    pub fn autostart_enabled_or_init<F>(&self, init: F) -> Result<bool, String>
    where
        F: FnOnce() -> Result<bool, String>,
    {
        let mut guard = self.autostart.lock().expect("autostart lock poisoned");
        match *guard {
            Some(v) => Ok(v),
            None => {
                let v = init()?;
                *guard = Some(v);
                Ok(v)
            }
        }
    }

    /// Updates the cached autostart state after enable/disable.
    pub fn set_autostart_enabled(&self, enabled: bool) {
        *self.autostart.lock().expect("autostart lock poisoned") = Some(enabled);
    }
}

impl PauzaState {
    pub fn initialize(&self, config_dir: PathBuf) -> Result<(), String> {
        fs::create_dir_all(&config_dir).map_err(|error| error.to_string())?;
        let config_path = config_dir.join(SETTINGS_FILE);
        let settings = load_settings(&config_path)?;

        let mut runtime = self.runtime.lock().expect("state lock poisoned");
        runtime.config_path = Some(config_path);
        runtime.settings = settings;
        runtime.last_action = i18n::text(&runtime.settings.language, "runtime.actions.ready");
        runtime.reset_schedule(now_ms());
        Ok(())
    }

    pub fn settings(&self) -> PauzaSettings {
        self.runtime
            .lock()
            .expect("state lock poisoned")
            .settings
            .clone()
    }

    pub fn set_last_action<S: Into<String>>(&self, action: S) {
        self.runtime
            .lock()
            .expect("state lock poisoned")
            .last_action = action.into();
    }

    pub fn snapshot(
        &self,
        platform: String,
        app_version: String,
        autostart_enabled: bool,
    ) -> DesktopSnapshot {
        let now = now_ms();
        let mut runtime = self.runtime.lock().expect("state lock poisoned");
        runtime.clear_expired_manual_states(now);

        let next_break_in_ms = runtime
            .next_break_due_ms
            .map(|due| due.saturating_sub(now))
            .filter(|remaining| *remaining > 0);

        let current_break = runtime
            .current_break
            .as_ref()
            .map(|current| current.snapshot(&runtime.settings, now));

        let (status, status_detail) = runtime.status(now);

        DesktopSnapshot {
            product_name: "Pauza",
            runtime: "Tauri 2".into(),
            platform,
            app_version,
            autostart_enabled,
            settings: runtime.settings.clone(),
            status,
            status_detail,
            next_break_kind: runtime.next_break_kind,
            next_break_due_ms: runtime.next_break_due_ms,
            next_break_in_ms,
            current_break,
            pause_until_ms: runtime.paused_until_ms,
            paused_indefinitely: runtime.paused_indefinitely,
            focus_until_ms: runtime.focus_until_ms,
            idle_ms: runtime.idle_ms,
            dnd_active: runtime.dnd_active,
            app_exclusion_active: runtime.app_exclusion_blocks(),
            app_exclusion_match: runtime.app_exclusion_match.clone(),
            last_action: runtime.last_action.clone(),
        }
    }

    pub fn update_settings(&self, settings: PauzaSettings) -> Result<(), String> {
        let now = now_ms();
        let mut runtime = self.runtime.lock().expect("state lock poisoned");
        let has_active_break_flow = runtime.current_break.is_some()
            || runtime.next_break_due_ms.is_some()
            || runtime.next_break_wait_started_ms.is_some();
        runtime.settings = settings.sanitized();
        save_settings(&runtime)?;
        runtime.last_action =
            i18n::text(&runtime.settings.language, "runtime.actions.settingsUpdated");
        if !has_active_break_flow && runtime.blocking_reason(now).is_none() {
            runtime.reset_schedule(now);
        }
        Ok(())
    }

    pub fn pause_for_minutes(&self, minutes: u64, source: &str) -> bool {
        let now = now_ms();
        let mut runtime = self.runtime.lock().expect("state lock poisoned");
        if runtime.current_break_is_forced() {
            runtime.last_action = i18n::text(
                &runtime.settings.language,
                "runtime.actions.forcedBreakLocked",
            );
            return false;
        }
        runtime.paused_indefinitely = minutes == 0;
        runtime.paused_until_ms = if minutes == 0 {
            None
        } else {
            Some(now + minutes * 60_000)
        };
        runtime.focus_until_ms = None;
        runtime.clear_waiting_schedule();

        let language = runtime.settings.language.clone();
        let translated_source = source_translation(&language, source);
        runtime.last_action = if minutes == 0 {
            i18n::text1(
                &language,
                "runtime.actions.pauseIndefinitelyVia",
                "source",
                translated_source,
            )
        } else {
            i18n::text2(
                &language,
                "runtime.actions.pauseVia",
                "source",
                translated_source,
                "minutes",
                minutes.to_string(),
            )
        };

        runtime.current_break.take().is_some()
    }

    pub fn resume(&self, source: &str) -> bool {
        let now = now_ms();
        let mut runtime = self.runtime.lock().expect("state lock poisoned");
        runtime.paused_until_ms = None;
        runtime.paused_indefinitely = false;
        runtime.focus_until_ms = None;
        let language = runtime.settings.language.clone();
        let translated_source = source_translation(&language, source);
        runtime.last_action =
            i18n::text1(&language, "runtime.actions.resumeVia", "source", translated_source);
        runtime.reset_schedule(now);
        runtime.current_break.take().is_some()
    }

    pub fn start_focus_session(&self, minutes: u64, source: &str) -> bool {
        let now = now_ms();
        let mut runtime = self.runtime.lock().expect("state lock poisoned");
        if runtime.current_break_is_forced() {
            runtime.last_action = i18n::text(
                &runtime.settings.language,
                "runtime.actions.forcedBreakLocked",
            );
            return false;
        }
        runtime.focus_until_ms = Some(now + minutes * 60_000);
        runtime.paused_until_ms = None;
        runtime.paused_indefinitely = false;
        runtime.clear_waiting_schedule();
        let language = runtime.settings.language.clone();
        let translated_source = source_translation(&language, source);
        runtime.last_action = i18n::text2(
            &language,
            "runtime.actions.focusVia",
            "source",
            translated_source,
            "minutes",
            minutes.to_string(),
        );
        runtime.current_break.take().is_some()
    }

    pub fn clear_focus_session(&self, source: &str) -> bool {
        let now = now_ms();
        let mut runtime = self.runtime.lock().expect("state lock poisoned");
        if runtime.current_break_is_forced() {
            runtime.last_action = i18n::text(
                &runtime.settings.language,
                "runtime.actions.forcedBreakLocked",
            );
            return false;
        }
        runtime.focus_until_ms = None;
        runtime.paused_until_ms = None;
        runtime.paused_indefinitely = false;
        let language = runtime.settings.language.clone();
        let translated_source = source_translation(&language, source);
        runtime.last_action =
            i18n::text1(&language, "runtime.actions.focusClearedVia", "source", translated_source);
        runtime.reset_schedule(now);
        runtime.current_break.take().is_some()
    }

    pub fn finish_current_break(&self, source: &str) -> bool {
        let now = now_ms();
        let mut runtime = self.runtime.lock().expect("state lock poisoned");
        if let Some(current) = runtime.current_break.take() {
            if !current.manual_awaiting {
                runtime.current_break = Some(current);
                return false;
            }
            let language = runtime.settings.language.clone();
            let translated_source = source_translation(&language, source);
            runtime.last_action = i18n::text2(
                &language,
                "runtime.actions.completedVia",
                "source",
                translated_source,
                "title",
                current.title.clone(),
            );
            runtime.schedule_next_slot(now);
            return true;
        }
        false
    }

    pub fn skip_current_break(&self, source: &str) -> bool {
        let now = now_ms();
        let mut runtime = self.runtime.lock().expect("state lock poisoned");
        if let Some(current) = runtime.current_break.take() {
            if current.strict_mode || !current.can_skip(&runtime.settings, now) {
                if current.strict_mode {
                    runtime.last_action = i18n::text(
                        &runtime.settings.language,
                        "runtime.actions.forcedBreakLocked",
                    );
                }
                runtime.current_break = Some(current);
                return false;
            }
            let language = runtime.settings.language.clone();
            let translated_source = source_translation(&language, source);
            runtime.last_action = i18n::text2(
                &language,
                "runtime.actions.skippedVia",
                "source",
                translated_source,
                "title",
                current.title.clone(),
            );
            runtime.schedule_next_slot(now);
            return true;
        }
        false
    }

    pub fn postpone_current_break(&self, source: &str) -> Result<bool, String> {
        let now = now_ms();
        let mut runtime = self.runtime.lock().expect("state lock poisoned");
        if let Some(current) = runtime.current_break.take() {
            if !current.can_postpone(&runtime.settings, now) {
                runtime.current_break = Some(current);
                return Err(i18n::text(
                    &runtime.settings.language,
                    "runtime.actions.postponeDisabled",
                ));
            }

            runtime.pending_break_postpones = current.postpones_used + 1;
            let postpone_ms = runtime.settings.postpone_ms(current.kind);
            runtime.schedule_specific_break(current.kind, now + postpone_ms, now);

            let language = runtime.settings.language.clone();
            let translated_source = source_translation(&language, source);
            runtime.last_action = i18n::text2(
                &language,
                "runtime.actions.postponedVia",
                "source",
                translated_source,
                "title",
                current.title.clone(),
            );
            return Ok(true);
        }

        Ok(false)
    }

    pub fn skip_to_next_scheduled_break(&self, source: &str) -> bool {
        let now = now_ms();
        let mut runtime = self.runtime.lock().expect("state lock poisoned");
        if runtime.current_break_is_forced() {
            runtime.last_action = i18n::text(
                &runtime.settings.language,
                "runtime.actions.forcedBreakLocked",
            );
            return false;
        }
        let has_visible_break = runtime.current_break.take().is_some();
        if runtime.next_break_kind.is_none() {
            runtime.schedule_next_slot(now);
        }
        if let Some(kind) = runtime.next_break_kind {
            runtime.schedule_specific_break(kind, now + BREAK_ACTION_DELAY_MS, now);
            let language = runtime.settings.language.clone();
            let translated_source = source_translation(&language, source);
            runtime.last_action = i18n::text2(
                &language,
                "runtime.actions.skipToScheduledVia",
                "source",
                translated_source,
                "title",
                break_kind_label(&language, kind),
            );
        }
        has_visible_break
    }

    pub fn skip_to_microbreak(&self, source: &str) -> bool {
        let now = now_ms();
        let mut runtime = self.runtime.lock().expect("state lock poisoned");
        if runtime.current_break_is_forced() {
            runtime.last_action = i18n::text(
                &runtime.settings.language,
                "runtime.actions.forcedBreakLocked",
            );
            return false;
        }
        let had_break = runtime.current_break.take().is_some();
        if runtime.settings.microbreak_enabled {
            if runtime.settings.long_break_enabled && runtime.next_break_kind == Some(BreakKind::LongBreak)
            {
                runtime.cycle_index = 1;
            }
            runtime.pending_break_postpones = 0;
            runtime.schedule_specific_break(
                BreakKind::Microbreak,
                now + BREAK_ACTION_DELAY_MS,
                now,
            );
            let language = runtime.settings.language.clone();
            let translated_source = source_translation(&language, source);
            runtime.last_action = i18n::text2(
                &language,
                "runtime.actions.skipToSpecificVia",
                "source",
                translated_source,
                "title",
                break_kind_label(&language, BreakKind::Microbreak),
            );
        }
        had_break
    }

    pub fn skip_to_long_break(&self, source: &str) -> bool {
        let now = now_ms();
        let mut runtime = self.runtime.lock().expect("state lock poisoned");
        if runtime.current_break_is_forced() {
            runtime.last_action = i18n::text(
                &runtime.settings.language,
                "runtime.actions.forcedBreakLocked",
            );
            return false;
        }
        let had_break = runtime.current_break.take().is_some();
        if runtime.settings.long_break_enabled {
            if runtime.settings.microbreak_enabled {
                runtime.cycle_index = runtime.settings.long_break_every;
            }
            runtime.pending_break_postpones = 0;
            runtime.schedule_specific_break(BreakKind::LongBreak, now + BREAK_ACTION_DELAY_MS, now);
            let language = runtime.settings.language.clone();
            let translated_source = source_translation(&language, source);
            runtime.last_action = i18n::text2(
                &language,
                "runtime.actions.skipToSpecificVia",
                "source",
                translated_source,
                "title",
                break_kind_label(&language, BreakKind::LongBreak),
            );
        }
        had_break
    }

    pub fn reset_breaks(&self, source: &str) -> bool {
        let now = now_ms();
        let mut runtime = self.runtime.lock().expect("state lock poisoned");
        if runtime.current_break_is_forced() {
            runtime.last_action = i18n::text(
                &runtime.settings.language,
                "runtime.actions.forcedBreakLocked",
            );
            return false;
        }
        let had_break = runtime.current_break.take().is_some();
        let language = runtime.settings.language.clone();
        let translated_source = source_translation(&language, source);
        runtime.last_action =
            i18n::text1(&language, "runtime.actions.resetVia", "source", translated_source);
        runtime.reset_schedule(now);
        had_break
    }

    pub fn toggle_pause(&self, source: &str) -> bool {
        let blocking_break = self
            .runtime
            .lock()
            .expect("state lock poisoned")
            .paused_indefinitely
            || self
                .runtime
                .lock()
                .expect("state lock poisoned")
                .paused_until_ms
                .is_some();

        if blocking_break {
            self.resume(source)
        } else {
            self.pause_for_minutes(0, source)
        }
    }

    pub fn current_break_is_strict(&self) -> bool {
        self.runtime
            .lock()
            .expect("state lock poisoned")
            .current_break
            .as_ref()
            .is_some_and(|current| current.strict_mode)
    }

    pub fn tick(
        &self,
        now: u64,
        idle_ms: u64,
        dnd_active: bool,
        app_exclusion_match: Option<String>,
    ) -> EngineActions {
        let mut runtime = self.runtime.lock().expect("state lock poisoned");
        let previous_blocking = runtime.blocking_reason(now);
        let previous_natural_break = runtime.natural_break_blocks();
        let previous_dnd = runtime.dnd_active;
        let previous_app_block = runtime.app_exclusion_blocks();
        let previous_idle_ms = runtime.idle_ms;

        runtime.clear_expired_manual_states(now);
        runtime.idle_ms = idle_ms;
        runtime.dnd_active = dnd_active;
        runtime.app_exclusion_match = app_exclusion_match;

        let current_blocking = runtime.blocking_reason(now);
        let mut actions = EngineActions::default();
        let language = runtime.settings.language.clone();

        if !previous_dnd && runtime.dnd_active {
            runtime.clear_waiting_schedule();
            runtime.last_action = i18n::text(&language, "runtime.actions.dndStarted");
        } else if previous_dnd && !runtime.dnd_active {
            runtime.last_action = i18n::text(&language, "runtime.actions.dndEnded");
        }

        if !previous_natural_break && runtime.natural_break_blocks() {
            runtime.clear_waiting_schedule();
            runtime.last_action = i18n::text(&language, "runtime.actions.naturalBreakDetected");
        } else if previous_natural_break
            && !runtime.natural_break_blocks()
            && previous_idle_ms >= runtime.settings.natural_break_reset_ms()
        {
            runtime.last_action = i18n::text(&language, "runtime.actions.naturalBreakFinished");
        }

        if !previous_app_block && runtime.app_exclusion_blocks() {
            runtime.clear_waiting_schedule();
            runtime.last_action = runtime.app_exclusion_started_message();
        } else if previous_app_block && !runtime.app_exclusion_blocks() {
            runtime.last_action = i18n::text(&language, "runtime.actions.appExclusionCleared");
        }

        if current_blocking.is_some() {
            if runtime.current_break.take().is_some() {
                actions.close_break_window = true;
            }
            runtime.clear_waiting_schedule();
            return actions;
        }

        if previous_blocking.is_some() && current_blocking.is_none() && runtime.current_break.is_none()
        {
            runtime.reset_schedule(now);
        }

        if runtime.current_break.is_none() && runtime.next_break_due_ms.is_none() {
            runtime.reset_schedule(now);
        }

        if let Some(current) = runtime.current_break.as_mut() {
            if !current.manual_awaiting && current.ends_at_ms <= now {
                if current.manual_finish {
                    current.manual_awaiting = true;
                    current.ends_at_ms = now;
                    runtime.last_action = i18n::text1(
                        &language,
                        "runtime.actions.manualFinishWaiting",
                        "title",
                        current.title.clone(),
                    );
                } else if let Some(finished) = runtime.current_break.take() {
                    runtime.last_action = i18n::text1(
                        &language,
                        "runtime.actions.breakFinished",
                        "title",
                        finished.title,
                    );
                    runtime.schedule_next_slot(now);
                    actions.close_break_window = true;
                }
            }
            return actions;
        }

        if runtime
            .next_notification_due_ms
            .is_some_and(|due| due <= now && runtime.next_break_kind.is_some())
        {
            if let Some(kind) = runtime.next_break_kind {
                let title = i18n::text(&language, "runtime.notifications.title");
                let body = pre_break_notification_body(&runtime.settings, &language, kind);
                actions.notify_title = Some(title);
                actions.notify_body = Some(body);
                runtime.next_notification_due_ms = None;
                runtime.last_action = i18n::text1(
                    &language,
                    "runtime.actions.notificationSent",
                    "title",
                    break_kind_label(&language, kind),
                );
            }
            return actions;
        }

        if runtime
            .next_break_due_ms
            .is_some_and(|due| due <= now && runtime.next_break_kind.is_some())
        {
            if let Some(kind) = runtime.next_break_kind {
                if runtime.should_wait_for_opportunity(kind, now) {
                    if runtime.next_break_wait_started_ms.is_none() {
                        runtime.next_break_wait_started_ms = Some(now);
                        runtime.last_action = i18n::text1(
                            &language,
                            "runtime.actions.waitingForOpportunity",
                            "title",
                            break_kind_label(&language, kind),
                        );
                    }
                    return actions;
                }
            }

            if let Some(current_break) = runtime.start_current_break(now) {
                actions.notify_title = Some(current_break.title.clone());
                actions.notify_body = Some(current_break.detail.clone());
                actions.open_break_window = true;
                runtime.last_action = i18n::text1(
                    &language,
                    "runtime.actions.breakStarted",
                    "title",
                    current_break.title,
                );
            }
        }

        actions
    }
}

impl RuntimeState {
    fn clear_expired_manual_states(&mut self, now: u64) {
        if self.paused_until_ms.is_some_and(|until| until <= now) {
            self.paused_until_ms = None;
            self.paused_indefinitely = false;
            self.last_action = i18n::text(&self.settings.language, "runtime.actions.pauseEnded");
        }

        if self.focus_until_ms.is_some_and(|until| until <= now) {
            self.focus_until_ms = None;
            self.last_action = i18n::text(&self.settings.language, "runtime.actions.focusFinished");
        }
    }

    fn reset_schedule(&mut self, now: u64) {
        self.current_break = None;
        self.cycle_index = 0;
        self.pending_break_postpones = 0;
        self.clear_waiting_schedule();
        self.schedule_next_slot(now);
    }

    fn clear_waiting_schedule(&mut self) {
        self.cycle_index = 0;
        self.pending_break_postpones = 0;
        self.next_break_due_ms = None;
        self.next_break_kind = None;
        self.next_notification_due_ms = None;
        self.next_break_wait_started_ms = None;
    }

    fn schedule_next_slot(&mut self, now: u64) {
        self.pending_break_postpones = 0;
        let base_interval = self.settings.microbreak_interval_ms();

        match (self.settings.microbreak_enabled, self.settings.long_break_enabled) {
            (false, false) => {
                self.next_break_due_ms = None;
                self.next_break_kind = None;
                self.next_notification_due_ms = None;
            }
            (true, false) => {
                self.cycle_index += 1;
                self.schedule_specific_break(BreakKind::Microbreak, now + base_interval, now);
            }
            (false, true) => {
                self.cycle_index += 1;
                self.schedule_specific_break(
                    BreakKind::LongBreak,
                    now + (base_interval * self.settings.long_break_every),
                    now,
                );
            }
            (true, true) => {
                self.cycle_index += 1;
                let kind = if self.cycle_index % self.settings.long_break_every == 0 {
                    BreakKind::LongBreak
                } else {
                    BreakKind::Microbreak
                };
                self.schedule_specific_break(kind, now + base_interval, now);
            }
        }
    }

    fn schedule_specific_break(&mut self, kind: BreakKind, due_ms: u64, scheduled_from_ms: u64) {
        self.next_break_kind = Some(kind);
        self.next_break_due_ms = Some(due_ms);
        self.next_notification_due_ms =
            notification_due_at(&self.settings, kind, due_ms, scheduled_from_ms);
        self.next_break_wait_started_ms = None;
    }

    fn start_current_break(&mut self, now: u64) -> Option<CurrentBreakSnapshot> {
        let kind = self.next_break_kind?;
        let duration_ms = self.settings.duration_ms(kind);
        let language = self.settings.language.clone();
        let (title, detail) = match kind {
            BreakKind::Microbreak => (
                i18n::text(&language, "runtime.break.microbreak.title"),
                i18n::text(&language, "runtime.break.microbreak.detail"),
            ),
            BreakKind::LongBreak => (
                i18n::text(&language, "runtime.break.long.title"),
                i18n::text(&language, "runtime.break.long.detail"),
            ),
        };

        let current = CurrentBreak {
            kind,
            title: title.clone(),
            detail: detail.clone(),
            started_at_ms: now,
            ends_at_ms: now + duration_ms,
            duration_ms,
            strict_mode: self.settings.strict_mode(kind),
            manual_finish: self.settings.manual_finish(kind),
            manual_awaiting: false,
            postpones_used: self.pending_break_postpones,
        };

        self.next_break_due_ms = None;
        self.next_break_kind = None;
        self.next_notification_due_ms = None;
        self.next_break_wait_started_ms = None;
        self.current_break = Some(current.clone());

        Some(current.snapshot(&self.settings, now))
    }

    fn pending_due_kind(&self, now: u64) -> Option<BreakKind> {
        let kind = self.next_break_kind?;
        let due = self.next_break_due_ms?;
        (due <= now).then_some(kind)
    }

    fn should_wait_for_opportunity(&self, kind: BreakKind, now: u64) -> bool {
        if self.settings.reminder_mode != ReminderMode::Smart {
            return false;
        }

        if self.pending_due_kind(now).is_none() {
            return false;
        }

        let Some(idle_required_ms) = self.current_smart_wait_idle_requirement_ms(kind, now) else {
            return false;
        };

        self.idle_ms < idle_required_ms
    }

    fn waiting_for_opportunity_kind(&self, now: u64) -> Option<BreakKind> {
        let kind = self.pending_due_kind(now)?;
        if self.should_wait_for_opportunity(kind, now) {
            return Some(kind);
        }
        if self.next_break_wait_started_ms.is_some() {
            return Some(kind);
        }
        None
    }

    fn heads_up_kind(&self, now: u64) -> Option<BreakKind> {
        let kind = self.next_break_kind?;
        let due = self.next_break_due_ms?;
        if due <= now || !self.settings.notification_enabled(kind) {
            return None;
        }

        let lead_ms = self.settings.notification_ms(kind);
        if lead_ms == 0 {
            return None;
        }

        (due.saturating_sub(now) <= lead_ms).then_some(kind)
    }

    fn smart_wait_elapsed_ms(&self, now: u64) -> u64 {
        let started_at = self.next_break_wait_started_ms.unwrap_or(now);
        now.saturating_sub(started_at)
    }

    fn current_smart_wait_idle_requirement_ms(&self, kind: BreakKind, now: u64) -> Option<u64> {
        let elapsed_ms = self.smart_wait_elapsed_ms(now);
        smart_wait_stages(kind)
            .iter()
            .find(|stage| elapsed_ms < stage.wait_until_ms)
            .map(|stage| stage.idle_required_ms)
    }

    fn blocking_reason(&self, now: u64) -> Option<&'static str> {
        if self.paused_indefinitely {
            return Some("paused");
        }

        if self.paused_until_ms.is_some_and(|until| until > now) {
            return Some("paused");
        }

        if self.focus_until_ms.is_some_and(|until| until > now) {
            return Some("focus");
        }

        if self.app_exclusion_blocks() {
            return Some("app-exclusion");
        }

        if self.dnd_active {
            return Some("dnd");
        }

        if self.natural_break_blocks() {
            return Some("natural-break");
        }

        None
    }

    fn natural_break_blocks(&self) -> bool {
        self.settings.natural_breaks && self.idle_ms >= self.settings.natural_break_reset_ms()
    }

    fn app_exclusion_blocks(&self) -> bool {
        if !self.settings.app_exclusions_enabled || self.settings.exclusion_commands().is_empty() {
            return false;
        }

        match self.settings.app_exclusion_rule {
            AppExclusionRule::Pause => self.app_exclusion_match.is_some(),
            AppExclusionRule::Resume => self.app_exclusion_match.is_none(),
        }
    }

    fn app_exclusion_started_message(&self) -> String {
        match self.settings.app_exclusion_rule {
            AppExclusionRule::Pause => match self.app_exclusion_match.as_deref() {
                Some(found) => i18n::text1(
                    &self.settings.language,
                    "runtime.appExclusion.pauseMatched",
                    "match",
                    found,
                ),
                None => i18n::text(&self.settings.language, "runtime.appExclusion.pauseGeneric"),
            },
            AppExclusionRule::Resume => {
                i18n::text(&self.settings.language, "runtime.appExclusion.resumeGeneric")
            }
        }
    }

    fn current_break_is_forced(&self) -> bool {
        self.current_break
            .as_ref()
            .is_some_and(|current| current.strict_mode)
    }

    fn status(&self, now: u64) -> (String, String) {
        let language = self.settings.language.as_str();

        if let Some(current) = &self.current_break {
            if current.manual_awaiting {
                return (
                    i18n::text(language, "runtime.break.status.manualFinishTitle"),
                    i18n::text(language, "runtime.break.status.manualFinishDetail"),
                );
            }

            return (
                current.title.clone(),
                i18n::text1(
                    language,
                    "runtime.break.status.endsIn",
                    "duration",
                    i18n::duration(language, current.ends_at_ms.saturating_sub(now)),
                ),
            );
        }

        if self.focus_until_ms.is_some_and(|until| until > now) {
            return (
                i18n::text(language, "runtime.break.status.focusTitle"),
                i18n::text1(
                    language,
                    "runtime.break.status.focusDetail",
                    "duration",
                    i18n::duration(language, self.focus_until_ms.unwrap_or(now).saturating_sub(now)),
                ),
            );
        }

        if self.paused_indefinitely {
            return (
                i18n::text(language, "runtime.break.status.pausedTitle"),
                i18n::text(language, "runtime.break.status.pausedForeverDetail"),
            );
        }

        if self.paused_until_ms.is_some_and(|until| until > now) {
            return (
                i18n::text(language, "runtime.break.status.pausedTitle"),
                i18n::text1(
                    language,
                    "runtime.break.status.pausedDetail",
                    "duration",
                    i18n::duration(language, self.paused_until_ms.unwrap_or(now).saturating_sub(now)),
                ),
            );
        }

        if self.app_exclusion_blocks() {
            return (
                i18n::text(language, "runtime.break.status.appExclusionTitle"),
                self.app_exclusion_started_message(),
            );
        }

        if self.dnd_active {
            return (
                i18n::text(language, "runtime.break.status.dndTitle"),
                i18n::text(language, "runtime.break.status.dndDetail"),
            );
        }

        if self.natural_break_blocks() {
            return (
                i18n::text(language, "runtime.break.status.naturalTitle"),
                i18n::text(language, "runtime.break.status.naturalDetail"),
            );
        }

        if let Some(kind) = self.heads_up_kind(now) {
            return (
                i18n::text(language, "runtime.break.status.headsUpTitle"),
                pre_break_heads_up_detail(
                    &self.settings,
                    language,
                    kind,
                    self.next_break_due_ms.unwrap_or(now).saturating_sub(now),
                ),
            );
        }

        if let Some(kind) = self.waiting_for_opportunity_kind(now) {
            return (
                i18n::text(language, "runtime.break.status.waitingOpportunityTitle"),
                i18n::text1(
                    language,
                    "runtime.break.status.waitingOpportunityDetail",
                    "kind",
                    break_kind_label(language, kind),
                ),
            );
        }

        if let Some(remaining) = self.next_break_due_ms.map(|due| due.saturating_sub(now)) {
            let kind = break_kind_label(language, self.next_break_kind.unwrap_or(BreakKind::Microbreak));
            return (
                i18n::text(language, "runtime.break.status.runningTitle"),
                i18n::text2(
                    language,
                    "runtime.break.status.runningDetail",
                    "kind",
                    kind,
                    "duration",
                    i18n::duration(language, remaining),
                ),
            );
        }

        (
            i18n::text(language, "runtime.break.status.idleTitle"),
            i18n::text(language, "runtime.break.status.idleDetail"),
        )
    }
}

impl CurrentBreak {
    fn snapshot(&self, settings: &PauzaSettings, now: u64) -> CurrentBreakSnapshot {
        CurrentBreakSnapshot {
            kind: self.kind,
            title: self.title.clone(),
            detail: self.detail.clone(),
            started_at_ms: self.started_at_ms,
            ends_at_ms: self.ends_at_ms,
            duration_ms: self.duration_ms,
            strict_mode: self.strict_mode,
            manual_awaiting: self.manual_awaiting,
            can_postpone: self.can_postpone(settings, now),
            can_skip: self.can_skip(settings, now),
            show_clock: settings.current_time_in_breaks,
        }
    }

    fn can_postpone(&self, settings: &PauzaSettings, now: u64) -> bool {
        if self.manual_awaiting || self.strict_mode || !settings.allow_postpone(self.kind) {
            return false;
        }

        let limit = settings.postpone_limit(self.kind);
        if limit == 0 || self.postpones_used >= limit {
            return false;
        }

        now.saturating_sub(self.started_at_ms) <= BREAK_POSTPONE_WINDOW_MS
    }

    fn can_skip(&self, settings: &PauzaSettings, now: u64) -> bool {
        if self.manual_awaiting || self.strict_mode {
            return false;
        }

        !self.can_postpone(settings, now)
    }
}

fn load_settings(path: &PathBuf) -> Result<PauzaSettings, String> {
    if !path.exists() {
        let settings = PauzaSettings::default();
        let contents = serde_json::to_string_pretty(&settings).map_err(|error| error.to_string())?;
        fs::write(path, contents).map_err(|error| error.to_string())?;
        return Ok(settings);
    }

    let contents = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut raw: serde_json::Value =
        serde_json::from_str(&contents).map_err(|error| error.to_string())?;
    migrate_legacy_settings(&mut raw);
    let parsed: PauzaSettings =
        serde_json::from_value(raw).map_err(|error| error.to_string())?;
    Ok(parsed.sanitized())
}

fn migrate_legacy_settings(value: &mut serde_json::Value) {
    let Some(object) = value.as_object_mut() else {
        return;
    };

    if !object.contains_key("reminderMode") {
        let legacy_strict = object
            .get("microbreakStrictMode")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
            || object
                .get("longBreakStrictMode")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);

        object.insert(
            "reminderMode".into(),
            serde_json::Value::String(if legacy_strict { "forced" } else { "smart" }.into()),
        );
    }

    if !object.contains_key("idleOpportunitySeconds") {
        let opportunity = object
            .get("microbreakIdleOpportunitySeconds")
            .and_then(serde_json::Value::as_u64)
            .or_else(|| {
                object
                    .get("longBreakIdleOpportunitySeconds")
                    .and_then(serde_json::Value::as_u64)
            });

        if let Some(seconds) = opportunity {
            object.insert("idleOpportunitySeconds".into(), serde_json::Value::from(seconds));
        }
    }

    if !object.contains_key("breakIdeasEnabled") {
        let enabled = object
            .get("ideas")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(true);
        object.insert("breakIdeasEnabled".into(), serde_json::Value::from(enabled));
    }

    if !object.contains_key("microbreakStartSound") {
        if let Some(sound) = object.get("miniBreakStartSound").cloned() {
            object.insert("microbreakStartSound".into(), sound);
        }
    }

    if !object.contains_key("longBreakStartSound") {
        if let Some(sound) = object.get("longBreakStartSound").cloned() {
            object.insert("longBreakStartSound".into(), sound);
        }
    }

    if !object.contains_key("microbreakEndSound") {
        if let Some(sound) = object.get("miniBreakAudio").cloned() {
            object.insert("microbreakEndSound".into(), sound);
        }
    }

    if !object.contains_key("longBreakEndSound") {
        if let Some(sound) = object.get("longBreakAudio").cloned() {
            object.insert("longBreakEndSound".into(), sound);
        }
    }

    if !object.contains_key("breakSoundVolume") {
        let volume = object
            .get("volume")
            .and_then(serde_json::Value::as_f64)
            .map(|value| (value * 100.0).round().clamp(0.0, 100.0) as u64);

        if let Some(volume) = volume {
            object.insert("breakSoundVolume".into(), serde_json::Value::from(volume));
        }
    }
}

fn save_settings(runtime: &RuntimeState) -> Result<(), String> {
    let path = runtime
        .config_path
        .as_ref()
        .ok_or_else(|| "settings path is not initialized".to_string())?;
    let contents =
        serde_json::to_string_pretty(&runtime.settings).map_err(|error| error.to_string())?;
    fs::write(path, contents).map_err(|error| error.to_string())
}

fn source_translation(language: &str, source: &str) -> String {
    match source {
        "tray" => i18n::text(language, "runtime.sources.tray"),
        "settings" => i18n::text(language, "runtime.sources.settings"),
        "break-window" => i18n::text(language, "runtime.sources.breakWindow"),
        "shortcut" => i18n::text(language, "runtime.sources.shortcut"),
        "window-close" => i18n::text(language, "runtime.sources.windowClose"),
        _ => source.to_string(),
    }
}

fn break_kind_label(language: &str, kind: BreakKind) -> String {
    match kind {
        BreakKind::Microbreak => i18n::text(language, "runtime.break.kind.microbreak"),
        BreakKind::LongBreak => i18n::text(language, "runtime.break.kind.long"),
    }
}

fn notification_key(kind: BreakKind) -> &'static str {
    match kind {
        BreakKind::Microbreak => "runtime.notifications.microbreakSoon",
        BreakKind::LongBreak => "runtime.notifications.longBreakSoon",
    }
}

fn adaptive_notification_key(kind: BreakKind) -> &'static str {
    match kind {
        BreakKind::Microbreak => "runtime.notifications.microbreakAdaptiveSoon",
        BreakKind::LongBreak => "runtime.notifications.longBreakAdaptiveSoon",
    }
}

fn pre_break_notification_body(settings: &PauzaSettings, language: &str, kind: BreakKind) -> String {
    let title = break_kind_label(language, kind);
    if settings.reminder_mode == ReminderMode::Smart {
        i18n::text1(language, adaptive_notification_key(kind), "title", title)
    } else {
        i18n::text2(
            language,
            notification_key(kind),
            "title",
            title,
            "duration",
            i18n::duration(language, settings.notification_ms(kind)),
        )
    }
}

fn pre_break_heads_up_detail(
    settings: &PauzaSettings,
    language: &str,
    kind: BreakKind,
    remaining_ms: u64,
) -> String {
    let title = break_kind_label(language, kind);
    let duration = i18n::duration(language, remaining_ms);
    if settings.reminder_mode == ReminderMode::Smart {
        i18n::text2(
            language,
            "runtime.break.status.headsUpAdaptiveDetail",
            "kind",
            title,
            "duration",
            duration,
        )
    } else {
        i18n::text2(
            language,
            "runtime.break.status.headsUpDetail",
            "kind",
            title,
            "duration",
            duration,
        )
    }
}

fn notification_due_at(
    settings: &PauzaSettings,
    kind: BreakKind,
    due_ms: u64,
    scheduled_from_ms: u64,
) -> Option<u64> {
    if !settings.notification_enabled(kind) {
        return None;
    }

    let lead_ms = settings.notification_ms(kind);
    if lead_ms == 0 || due_ms <= scheduled_from_ms + lead_ms {
        return None;
    }

    due_ms.checked_sub(lead_ms)
}

fn normalize_target_screen(value: &str) -> String {
    let lowered = value.trim().to_ascii_lowercase();
    match lowered.as_str() {
        "cursor" => "cursor".into(),
        _ => "primary".into(),
    }
}

fn normalize_shortcut(value: &str, fallback: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return fallback.to_string();
    }
    trimmed.to_string()
}

fn sanitize_optional_string(value: Option<String>) -> Option<String> {
    value.and_then(|raw| {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_millis() as u64
}

fn smart_wait_stages(kind: BreakKind) -> &'static [SmartWaitStage] {
    match kind {
        BreakKind::Microbreak => &MICROBREAK_SMART_WAIT_STAGES,
        BreakKind::LongBreak => &LONG_BREAK_SMART_WAIT_STAGES,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> PauzaSettings {
        PauzaSettings::default()
    }

    #[test]
    fn schedules_pre_break_notification() {
        let mut runtime = RuntimeState {
            settings: settings(),
            ..Default::default()
        };

        runtime.schedule_next_slot(1_000);

        assert_eq!(runtime.next_break_kind, Some(BreakKind::Microbreak));
        assert_eq!(runtime.next_break_due_ms, Some(601_000));
        assert_eq!(runtime.next_notification_due_ms, Some(591_000));
    }

    #[test]
    fn enters_manual_finish_instead_of_closing() {
        let state = PauzaState::default();
        {
            let mut runtime = state.runtime.lock().expect("state lock poisoned");
            let mut settings = settings();
            settings.long_break_manual_finish = true;
            runtime.settings = settings;
            runtime.schedule_specific_break(BreakKind::LongBreak, 1_100, 1_000);
        }

        let actions = state.tick(1_100, 20_000, false, None);
        assert!(actions.open_break_window);

        let actions = state.tick(301_101, 0, false, None);
        assert!(!actions.close_break_window);
        let runtime = state.runtime.lock().expect("state lock poisoned");
        assert!(runtime.current_break.as_ref().is_some_and(|current| current.manual_awaiting));
    }

    #[test]
    fn postpone_limit_blocks_second_postpone() {
        let settings = settings();
        let current = CurrentBreak {
            kind: BreakKind::Microbreak,
            title: "Microbreak".into(),
            detail: "detail".into(),
            started_at_ms: 1_000,
            ends_at_ms: 21_000,
            duration_ms: 20_000,
            strict_mode: false,
            manual_finish: false,
            manual_awaiting: false,
            postpones_used: 1,
        };

        assert!(!current.can_postpone(&settings, 5_000));
    }

    #[test]
    fn postpone_is_only_available_in_first_ten_seconds() {
        let settings = settings();
        let current = CurrentBreak {
            kind: BreakKind::Microbreak,
            title: "Microbreak".into(),
            detail: "detail".into(),
            started_at_ms: 1_000,
            ends_at_ms: 21_000,
            duration_ms: 20_000,
            strict_mode: false,
            manual_finish: false,
            manual_awaiting: false,
            postpones_used: 0,
        };

        assert!(current.can_postpone(&settings, 11_000));
        assert!(!current.can_postpone(&settings, 11_001));
    }

    #[test]
    fn finish_current_break_requires_manual_awaiting() {
        let state = PauzaState::default();
        {
            let mut runtime = state.runtime.lock().expect("state lock poisoned");
            runtime.current_break = Some(CurrentBreak {
                kind: BreakKind::Microbreak,
                title: "Microbreak".into(),
                detail: "detail".into(),
                started_at_ms: 1_000,
                ends_at_ms: 21_000,
                duration_ms: 20_000,
                strict_mode: false,
                manual_finish: false,
                manual_awaiting: false,
                postpones_used: 0,
            });
        }

        assert!(!state.finish_current_break("break-window"));
        let runtime = state.runtime.lock().expect("state lock poisoned");
        assert!(runtime.current_break.is_some());
    }

    #[test]
    fn due_break_waits_for_idle_opportunity() {
        let state = PauzaState::default();
        {
            let mut runtime = state.runtime.lock().expect("state lock poisoned");
            runtime.settings = settings();
            runtime.schedule_specific_break(BreakKind::Microbreak, 1_100, 1_000);
        }

        let actions = state.tick(1_100, 0, false, None);

        assert!(!actions.open_break_window);
        assert!(actions.notify_title.is_none());

        let runtime = state.runtime.lock().expect("state lock poisoned");
        assert!(runtime.current_break.is_none());
        assert_eq!(runtime.next_break_kind, Some(BreakKind::Microbreak));
        assert_eq!(runtime.next_break_wait_started_ms, Some(1_100));
    }

    #[test]
    fn pending_break_starts_when_idle_opportunity_appears() {
        let state = PauzaState::default();
        {
            let mut runtime = state.runtime.lock().expect("state lock poisoned");
            runtime.settings = settings();
            runtime.schedule_specific_break(BreakKind::Microbreak, 1_100, 1_000);
        }

        let _ = state.tick(1_100, 0, false, None);
        let actions = state.tick(7_100, 6_000, false, None);

        assert!(actions.open_break_window);

        let runtime = state.runtime.lock().expect("state lock poisoned");
        assert!(runtime.current_break.is_some());
        assert!(runtime.next_break_wait_started_ms.is_none());
    }

    #[test]
    fn microbreak_wait_threshold_relaxes_after_first_stage() {
        let state = PauzaState::default();
        {
            let mut runtime = state.runtime.lock().expect("state lock poisoned");
            runtime.settings = settings();
            runtime.schedule_specific_break(BreakKind::Microbreak, 1_100, 1_000);
        }

        let _ = state.tick(1_100, 0, false, None);
        let actions = state.tick(16_100, 3_000, false, None);

        assert!(actions.open_break_window);

        let runtime = state.runtime.lock().expect("state lock poisoned");
        assert!(runtime.current_break.is_some());
        assert!(runtime.next_break_wait_started_ms.is_none());
    }

    #[test]
    fn long_break_uses_wider_idle_window_before_relaxing() {
        let state = PauzaState::default();
        {
            let mut runtime = state.runtime.lock().expect("state lock poisoned");
            runtime.settings = settings();
            runtime.schedule_specific_break(BreakKind::LongBreak, 1_100, 1_000);
        }

        let _ = state.tick(1_100, 0, false, None);
        let still_waiting = state.tick(20_100, 6_000, false, None);
        assert!(!still_waiting.open_break_window);

        let actions = state.tick(35_100, 4_000, false, None);
        assert!(actions.open_break_window);

        let runtime = state.runtime.lock().expect("state lock poisoned");
        assert!(runtime.current_break.as_ref().is_some_and(|current| current.kind == BreakKind::LongBreak));
        assert!(runtime.next_break_wait_started_ms.is_none());
    }

    #[test]
    fn smart_mode_starts_break_after_final_stage_even_without_idle_gap() {
        let state = PauzaState::default();
        {
            let mut runtime = state.runtime.lock().expect("state lock poisoned");
            runtime.settings = settings();
            runtime.schedule_specific_break(BreakKind::Microbreak, 1_100, 1_000);
        }

        let _ = state.tick(1_100, 0, false, None);
        let actions = state.tick(46_100, 0, false, None);

        assert!(actions.open_break_window);

        let runtime = state.runtime.lock().expect("state lock poisoned");
        assert!(runtime.current_break.is_some());
        assert!(runtime.next_break_wait_started_ms.is_none());
    }

    #[test]
    fn forced_mode_starts_break_immediately() {
        let state = PauzaState::default();
        {
            let mut runtime = state.runtime.lock().expect("state lock poisoned");
            let mut next_settings = settings();
            next_settings.reminder_mode = ReminderMode::Forced;
            runtime.settings = next_settings;
            runtime.schedule_specific_break(BreakKind::Microbreak, 1_100, 1_000);
        }

        let actions = state.tick(1_100, 0, false, None);

        assert!(actions.open_break_window);

        let runtime = state.runtime.lock().expect("state lock poisoned");
        assert!(runtime.current_break.as_ref().is_some_and(|current| current.strict_mode));
        assert!(runtime.next_break_wait_started_ms.is_none());
    }

    #[test]
    fn forced_mode_disables_postpone_and_skip() {
        let mut next_settings = settings();
        next_settings.reminder_mode = ReminderMode::Forced;

        let current = CurrentBreak {
            kind: BreakKind::Microbreak,
            title: "Microbreak".into(),
            detail: "detail".into(),
            started_at_ms: 1_000,
            ends_at_ms: 21_000,
            duration_ms: 20_000,
            strict_mode: true,
            manual_finish: false,
            manual_awaiting: false,
            postpones_used: 0,
        };

        assert!(!current.can_postpone(&next_settings, 5_000));
        assert!(!current.can_skip(&next_settings, 5_000));
    }

    #[test]
    fn legacy_strict_settings_migrate_to_forced_mode() {
        let unique = now_ms();
        let temp_path = std::env::temp_dir().join(format!("pauza-settings-{unique}.json"));
        let legacy = r#"{
  "language": "zh-CN",
  "microbreakStrictMode": true,
  "naturalBreaks": true
}"#;

        std::fs::write(&temp_path, legacy).expect("write legacy settings");
        let loaded = load_settings(&temp_path).expect("load migrated settings");

        assert_eq!(loaded.reminder_mode, ReminderMode::Forced);

        let _ = std::fs::remove_file(temp_path);
    }

    #[test]
    fn snapshot_status_reflects_waiting_for_opportunity() {
        let state = PauzaState::default();
        {
            let mut runtime = state.runtime.lock().expect("state lock poisoned");
            runtime.settings = settings();
            runtime.schedule_specific_break(BreakKind::Microbreak, 1_100, 1_000);
        }

        let _ = state.tick(1_100, 0, false, None);
        let snapshot = state.snapshot("test".into(), "0.0.0".into(), false);

        assert_eq!(
            snapshot.status,
            i18n::text("zh-CN", "runtime.break.status.waitingOpportunityTitle")
        );
        assert_eq!(
            snapshot.status_detail,
            i18n::text1(
                "zh-CN",
                "runtime.break.status.waitingOpportunityDetail",
                "kind",
                i18n::text("zh-CN", "runtime.break.kind.microbreak")
            )
        );
    }

    #[test]
    fn snapshot_status_reflects_pre_break_heads_up() {
        let state = PauzaState::default();
        let scheduled_from = now_ms();
        let due_ms = scheduled_from + 9_000;
        {
            let mut runtime = state.runtime.lock().expect("state lock poisoned");
            runtime.settings = settings();
            runtime.schedule_specific_break(BreakKind::Microbreak, due_ms, scheduled_from);
        }

        let snapshot = state.snapshot("test".into(), "0.0.0".into(), false);
        let remaining = snapshot.next_break_in_ms.expect("remaining time available");

        assert_eq!(
            snapshot.status,
            i18n::text("zh-CN", "runtime.break.status.headsUpTitle")
        );
        assert_eq!(
            snapshot.status_detail,
            i18n::text2(
                "zh-CN",
                "runtime.break.status.headsUpAdaptiveDetail",
                "kind",
                i18n::text("zh-CN", "runtime.break.kind.microbreak"),
                "duration",
                i18n::duration("zh-CN", remaining)
            )
        );
    }

    #[test]
    fn update_settings_keeps_current_break_running() {
        let state = PauzaState::default();
        let temp_path = std::env::temp_dir().join(format!("pauza-settings-current-{}.json", now_ms()));

        {
            let mut runtime = state.runtime.lock().expect("state lock poisoned");
            runtime.config_path = Some(temp_path.clone());
            runtime.settings = settings();
            runtime.schedule_specific_break(BreakKind::Microbreak, 1_100, 1_000);
        }

        let _ = state.tick(1_100, 12_000, false, None);

        let mut updated = settings();
        updated.microbreak_duration_seconds = 90;
        state.update_settings(updated).expect("update settings");

        let runtime = state.runtime.lock().expect("state lock poisoned");
        let current = runtime.current_break.as_ref().expect("current break preserved");
        assert_eq!(current.kind, BreakKind::Microbreak);
        assert_eq!(current.duration_ms, 20_000);
        assert_eq!(current.ends_at_ms, 21_100);
        assert!(runtime.next_break_due_ms.is_none());

        let _ = std::fs::remove_file(temp_path);
    }

    #[test]
    fn update_settings_keeps_already_scheduled_next_break() {
        let state = PauzaState::default();
        let temp_path = std::env::temp_dir().join(format!("pauza-settings-next-{}.json", now_ms()));

        {
            let mut runtime = state.runtime.lock().expect("state lock poisoned");
            runtime.config_path = Some(temp_path.clone());
            runtime.settings = settings();
            runtime.cycle_index = 1;
            runtime.schedule_specific_break(BreakKind::LongBreak, 61_000, 1_000);
        }

        let mut updated = settings();
        updated.long_break_duration_minutes = 15;
        updated.microbreak_interval_minutes = 30;
        state.update_settings(updated).expect("update settings");

        let runtime = state.runtime.lock().expect("state lock poisoned");
        assert_eq!(runtime.next_break_kind, Some(BreakKind::LongBreak));
        assert_eq!(runtime.next_break_due_ms, Some(61_000));
        assert_eq!(runtime.cycle_index, 1);

        let _ = std::fs::remove_file(temp_path);
    }
}
