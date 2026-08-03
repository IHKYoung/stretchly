mod persistence;
mod settings;
#[cfg(test)]
mod tests;

use crate::i18n;
use persistence::{load_settings, save_settings};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[allow(unused_imports)]
pub use settings::{
    AppExclusionRule, BreakBackdrop, BreakKind, BreakSound, PauzaSettings, ReminderMode,
    ShortcutAction, ShortcutBinding,
};

const SETTINGS_FILE: &str = "settings.json";
const BREAK_ACTION_DELAY_MS: u64 = 100;
const BREAK_POSTPONE_WINDOW_MS: u64 = 10_000;
const LEGACY_IDLE_OPPORTUNITY_SECONDS: u64 = 6;
const MICROBREAK_SMART_IDLE_THRESHOLD_MS: u64 = 8_000;
const LONG_BREAK_SMART_IDLE_THRESHOLD_MS: u64 = 12_000;
const MICROBREAK_SMART_MAX_WAIT_MS: u64 = 90_000;
const LONG_BREAK_SMART_MAX_WAIT_MS: u64 = 180_000;

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
    pub next_break_wait_remaining_ms: Option<u64>,
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
    delivery_block_started_ms: Option<u64>,
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

        let next_break_wait_remaining_ms = if runtime.blocking_reason(now).is_none() {
            runtime
                .waiting_for_opportunity_kind(now)
                .and_then(|kind| runtime.current_smart_wait_remaining_ms(kind, now))
        } else {
            None
        };

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
            next_break_wait_remaining_ms,
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
        let was_blocked = runtime.delivery_blocking_reason(now).is_some();
        runtime.paused_indefinitely = minutes == 0;
        runtime.paused_until_ms = if minutes == 0 {
            None
        } else {
            Some(now + minutes * 60_000)
        };
        runtime.focus_until_ms = None;
        runtime.sync_delivery_block_state(now, was_blocked);

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
        let was_blocked = runtime.delivery_blocking_reason(now).is_some();
        runtime.paused_until_ms = None;
        runtime.paused_indefinitely = false;
        runtime.focus_until_ms = None;
        runtime.sync_delivery_block_state(now, was_blocked);
        let language = runtime.settings.language.clone();
        let translated_source = source_translation(&language, source);
        runtime.last_action =
            i18n::text1(&language, "runtime.actions.resumeVia", "source", translated_source);
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
        let was_blocked = runtime.delivery_blocking_reason(now).is_some();
        runtime.focus_until_ms = Some(now + minutes * 60_000);
        runtime.paused_until_ms = None;
        runtime.paused_indefinitely = false;
        runtime.sync_delivery_block_state(now, was_blocked);
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
        let was_blocked = runtime.delivery_blocking_reason(now).is_some();
        runtime.focus_until_ms = None;
        runtime.paused_until_ms = None;
        runtime.paused_indefinitely = false;
        runtime.sync_delivery_block_state(now, was_blocked);
        let language = runtime.settings.language.clone();
        let translated_source = source_translation(&language, source);
        runtime.last_action =
            i18n::text1(&language, "runtime.actions.focusClearedVia", "source", translated_source);
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
        let previous_delivery_block = runtime.delivery_blocking_reason(now).is_some();
        let previous_natural_break = runtime.natural_break_blocks();
        let previous_dnd = runtime.dnd_active;
        let previous_app_block = runtime.app_exclusion_blocks();
        runtime.clear_expired_manual_states(now);
        runtime.idle_ms = idle_ms;
        runtime.dnd_active = dnd_active;
        runtime.app_exclusion_match = app_exclusion_match;

        let current_delivery_block = runtime.delivery_blocking_reason(now).is_some();
        let mut actions = EngineActions::default();
        let language = runtime.settings.language.clone();

        if !previous_dnd && runtime.dnd_active {
            runtime.enter_delivery_block(now);
            runtime.last_action = i18n::text(&language, "runtime.actions.dndStarted");
        } else if previous_dnd && !runtime.dnd_active {
            runtime.sync_delivery_block_state(now, previous_delivery_block);
            runtime.last_action = i18n::text(&language, "runtime.actions.dndEnded");
        }

        if !previous_natural_break && runtime.natural_break_blocks() {
            runtime.next_break_wait_started_ms = None;
            runtime.last_action = i18n::text(&language, "runtime.actions.naturalBreakDetected");
        }

        if !previous_app_block && runtime.app_exclusion_blocks() {
            runtime.enter_delivery_block(now);
            runtime.last_action = runtime.app_exclusion_started_message();
        } else if previous_app_block && !runtime.app_exclusion_blocks() {
            runtime.sync_delivery_block_state(now, previous_delivery_block);
            runtime.last_action = i18n::text(&language, "runtime.actions.appExclusionCleared");
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

        if !current_delivery_block && previous_natural_break && !runtime.natural_break_blocks() {
            runtime.reset_schedule(now);
            runtime.last_action = i18n::text(&language, "runtime.actions.naturalBreakFinished");
        }

        if current_delivery_block {
            return actions;
        }

        if runtime.natural_break_blocks() {
            return actions;
        }

        if runtime.current_break.is_none() && runtime.next_break_due_ms.is_none() {
            runtime.reset_schedule(now);
        }

        if runtime.maybe_dispatch_due_notification(now, &language, &mut actions) {
            return actions;
        }

        if runtime.maybe_start_due_break(now, &language, &mut actions) {
            return actions;
        }

        actions
    }
}

impl RuntimeState {
    fn clear_expired_manual_states(&mut self, now: u64) {
        if self.paused_until_ms.is_some_and(|until| until <= now) {
            self.paused_until_ms = None;
            self.paused_indefinitely = false;
            self.sync_delivery_block_state(now, true);
            self.last_action = i18n::text(&self.settings.language, "runtime.actions.pauseEnded");
        }

        if self.focus_until_ms.is_some_and(|until| until <= now) {
            self.focus_until_ms = None;
            self.sync_delivery_block_state(now, true);
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

    fn shift_pending_schedule_by(&mut self, delta_ms: u64) {
        if delta_ms == 0 {
            return;
        }

        self.next_break_due_ms = self
            .next_break_due_ms
            .map(|due_ms| due_ms.saturating_add(delta_ms));
        self.next_notification_due_ms = self
            .next_notification_due_ms
            .map(|due_ms| due_ms.saturating_add(delta_ms));
        self.next_break_wait_started_ms = self
            .next_break_wait_started_ms
            .map(|started_ms| started_ms.saturating_add(delta_ms));
    }

    fn enter_delivery_block(&mut self, now: u64) {
        if self.delivery_block_started_ms.is_none() {
            self.delivery_block_started_ms = Some(now);
        }
    }

    fn release_delivery_block(&mut self, now: u64) {
        if let Some(started_ms) = self.delivery_block_started_ms.take() {
            self.shift_pending_schedule_by(now.saturating_sub(started_ms));
        }
    }

    fn sync_delivery_block_state(&mut self, now: u64, was_blocked: bool) {
        let is_blocked = self.delivery_blocking_reason(now).is_some();
        match (was_blocked, is_blocked) {
            (false, true) => self.enter_delivery_block(now),
            (true, false) => self.release_delivery_block(now),
            _ => {}
        }
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

    fn pending_notification_kind(&self, now: u64) -> Option<BreakKind> {
        let kind = self.next_break_kind?;
        let due = self.next_notification_due_ms?;
        (due <= now).then_some(kind)
    }

    fn maybe_dispatch_due_notification(
        &mut self,
        now: u64,
        language: &str,
        actions: &mut EngineActions,
    ) -> bool {
        let Some(kind) = self.pending_notification_kind(now) else {
            return false;
        };

        let title = i18n::text(language, "runtime.notifications.title");
        let body = pre_break_notification_body(&self.settings, language, kind);
        actions.notify_title = Some(title);
        actions.notify_body = Some(body);
        self.next_notification_due_ms = None;
        self.last_action = i18n::text1(
            language,
            "runtime.actions.notificationSent",
            "title",
            break_kind_label(language, kind),
        );
        true
    }

    fn maybe_start_due_break(
        &mut self,
        now: u64,
        language: &str,
        actions: &mut EngineActions,
    ) -> bool {
        let Some(kind) = self.pending_due_kind(now) else {
            return false;
        };

        if self.should_wait_for_opportunity(kind, now) {
            if self.next_break_wait_started_ms.is_none() {
                self.next_break_wait_started_ms = Some(now);
                self.last_action = i18n::text1(
                    language,
                    "runtime.actions.waitingForOpportunity",
                    "title",
                    break_kind_label(language, kind),
                );
            }
            return true;
        }

        if let Some(current_break) = self.start_current_break(now) {
            actions.notify_title = Some(current_break.title.clone());
            actions.notify_body = Some(current_break.detail.clone());
            actions.open_break_window = true;
            self.last_action = i18n::text1(
                language,
                "runtime.actions.breakStarted",
                "title",
                current_break.title,
            );
        }

        true
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

        if self.smart_wait_elapsed_ms(now) >= smart_wait_max_wait_ms(kind) {
            return false;
        }

        self.idle_ms < smart_wait_idle_threshold_ms(kind)
    }

    fn waiting_for_opportunity_kind(&self, now: u64) -> Option<BreakKind> {
        let kind = self.pending_due_kind(now)?;
        self.should_wait_for_opportunity(kind, now).then_some(kind)
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

    fn current_smart_wait_remaining_ms(&self, kind: BreakKind, now: u64) -> Option<u64> {
        let elapsed_ms = self.smart_wait_elapsed_ms(now);
        let max_wait_ms = smart_wait_max_wait_ms(kind);
        if elapsed_ms < max_wait_ms {
            Some(max_wait_ms - elapsed_ms)
        } else {
            None
        }
    }

    fn delivery_blocking_reason(&self, now: u64) -> Option<&'static str> {
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

        None
    }

    fn blocking_reason(&self, now: u64) -> Option<&'static str> {
        if let Some(reason) = self.delivery_blocking_reason(now) {
            return Some(reason);
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
                )
                .replace(
                    "{{duration}}",
                    &i18n::duration(
                        language,
                        self.current_smart_wait_remaining_ms(kind, now).unwrap_or(0),
                    ),
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

pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_millis() as u64
}

fn smart_wait_idle_threshold_ms(kind: BreakKind) -> u64 {
    match kind {
        BreakKind::Microbreak => MICROBREAK_SMART_IDLE_THRESHOLD_MS,
        BreakKind::LongBreak => LONG_BREAK_SMART_IDLE_THRESHOLD_MS,
    }
}

fn smart_wait_max_wait_ms(kind: BreakKind) -> u64 {
    match kind {
        BreakKind::Microbreak => MICROBREAK_SMART_MAX_WAIT_MS,
        BreakKind::LongBreak => LONG_BREAK_SMART_MAX_WAIT_MS,
    }
}
