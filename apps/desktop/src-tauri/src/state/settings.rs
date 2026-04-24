use crate::i18n;
use serde::{Deserialize, Serialize};

use super::LEGACY_IDLE_OPPORTUNITY_SECONDS;

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
    #[serde(
        default = "legacy_idle_opportunity_seconds_default",
        skip_serializing
    )]
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
            idle_opportunity_seconds: legacy_idle_opportunity_seconds_default(),
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

fn legacy_idle_opportunity_seconds_default() -> u64 {
    LEGACY_IDLE_OPPORTUNITY_SECONDS
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
