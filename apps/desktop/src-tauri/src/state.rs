use crate::i18n;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const SETTINGS_FILE: &str = "settings.json";
const BREAK_ACTION_WINDOW_PERCENT: f64 = 30.0;
const BREAK_ACTION_DELAY_MS: u64 = 100;

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
pub enum BreakPromptStyle {
    #[default]
    Gentle,
    Balanced,
    Immersive,
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
    pub microbreak_strict_mode: bool,
    pub microbreak_manual_finish: bool,
    pub long_break_enabled: bool,
    pub long_break_every: u64,
    pub long_break_duration_minutes: u64,
    pub long_break_notification_enabled: bool,
    pub long_break_notification_seconds: u64,
    pub long_break_allow_postpone: bool,
    pub long_break_postpone_minutes: u64,
    pub long_break_postpones_limit: u64,
    pub long_break_strict_mode: bool,
    pub long_break_manual_finish: bool,
    pub natural_breaks: bool,
    pub natural_break_reset_minutes: u64,
    pub monitor_dnd: bool,
    pub app_exclusions_enabled: bool,
    pub app_exclusion_rule: AppExclusionRule,
    pub app_exclusion_commands: String,
    pub break_prompt_style: BreakPromptStyle,
    pub fullscreen: bool,
    pub show_breaks_on_all_screens: bool,
    pub target_screen: String,
    pub current_time_in_breaks: bool,
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
            microbreak_strict_mode: false,
            microbreak_manual_finish: false,
            long_break_enabled: true,
            long_break_every: 3,
            long_break_duration_minutes: 5,
            long_break_notification_enabled: true,
            long_break_notification_seconds: 30,
            long_break_allow_postpone: true,
            long_break_postpone_minutes: 5,
            long_break_postpones_limit: 1,
            long_break_strict_mode: false,
            long_break_manual_finish: false,
            natural_breaks: true,
            natural_break_reset_minutes: 5,
            monitor_dnd: true,
            app_exclusions_enabled: false,
            app_exclusion_rule: AppExclusionRule::Pause,
            app_exclusion_commands: String::new(),
            break_prompt_style: BreakPromptStyle::Gentle,
            fullscreen: false,
            show_breaks_on_all_screens: true,
            target_screen: "primary".into(),
            current_time_in_breaks: false,
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
        self.long_break_every = self.long_break_every.clamp(1, 12);
        self.long_break_duration_minutes = self.long_break_duration_minutes.clamp(1, 60);
        self.long_break_notification_seconds = self.long_break_notification_seconds.clamp(5, 600);
        self.long_break_postpone_minutes = self.long_break_postpone_minutes.clamp(1, 60);
        self.long_break_postpones_limit = self.long_break_postpones_limit.clamp(0, 5);
        self.natural_break_reset_minutes = self.natural_break_reset_minutes.clamp(1, 60);
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

    pub fn strict_mode(&self, kind: BreakKind) -> bool {
        match kind {
            BreakKind::Microbreak => self.microbreak_strict_mode,
            BreakKind::LongBreak => self.long_break_strict_mode,
        }
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
        runtime.settings = settings.sanitized();
        save_settings(&runtime)?;
        runtime.last_action =
            i18n::text(&runtime.settings.language, "runtime.actions.settingsUpdated");
        runtime.reset_schedule(now);
        Ok(())
    }

    pub fn pause_for_minutes(&self, minutes: u64, source: &str) -> bool {
        let now = now_ms();
        let mut runtime = self.runtime.lock().expect("state lock poisoned");
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
                let body = i18n::text2(
                    &language,
                    notification_key(kind),
                    "title",
                    break_kind_label(&language, kind),
                    "duration",
                    i18n::duration(
                        &language,
                        runtime.settings.notification_ms(kind),
                    ),
                );
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
        self.current_break = Some(current.clone());

        Some(current.snapshot(&self.settings, now))
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
        if self.manual_awaiting || !settings.allow_postpone(self.kind) {
            return false;
        }

        let limit = settings.postpone_limit(self.kind);
        if limit == 0 || self.postpones_used >= limit {
            return false;
        }

        self.elapsed_percent(now) <= BREAK_ACTION_WINDOW_PERCENT
    }

    fn can_skip(&self, settings: &PauzaSettings, now: u64) -> bool {
        if self.manual_awaiting || self.strict_mode {
            return false;
        }

        !self.can_postpone(settings, now)
    }

    fn elapsed_percent(&self, now: u64) -> f64 {
        let elapsed = now.saturating_sub(self.started_at_ms).min(self.duration_ms);
        if self.duration_ms == 0 {
            return 100.0;
        }
        (elapsed as f64 / self.duration_ms as f64) * 100.0
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
    let parsed: PauzaSettings = serde_json::from_str(&contents).map_err(|error| error.to_string())?;
    Ok(parsed.sanitized())
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

pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_millis() as u64
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

        let actions = state.tick(1_100, 0, false, None);
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
}
