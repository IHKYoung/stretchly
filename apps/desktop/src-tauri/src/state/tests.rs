use super::*;

fn settings() -> PauzaSettings {
    PauzaSettings::default()
}

#[test]
fn default_settings_use_the_balanced_hourly_rhythm() {
    let settings = settings();

    assert_eq!(settings.microbreak_interval_minutes, 20);
    assert_eq!(settings.microbreak_duration_seconds, 20);
    assert_eq!(settings.long_break_every, 3);
    assert_eq!(settings.long_break_duration_minutes, 5);
    assert_eq!(settings.microbreak_interval_minutes * settings.long_break_every, 60);
}

#[test]
fn schedules_pre_break_notification() {
    let mut runtime = RuntimeState {
        settings: settings(),
        ..Default::default()
    };

    runtime.schedule_next_slot(1_000);

    assert_eq!(runtime.next_break_kind, Some(BreakKind::Microbreak));
    assert_eq!(runtime.next_break_due_ms, Some(1_201_000));
    assert_eq!(runtime.next_notification_due_ms, Some(1_191_000));
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
fn active_break_is_not_closed_by_passive_blockers() {
    let state = PauzaState::default();
    {
        let mut runtime = state.runtime.lock().expect("state lock poisoned");
        runtime.settings = settings();
        runtime.schedule_specific_break(BreakKind::Microbreak, 1_100, 1_000);
    }

    let started = state.tick(1_100, 8_000, false, None);
    assert!(started.open_break_window);

    let blocked = state.tick(20_000, 6_000, true, None);
    assert!(!blocked.close_break_window);

    let runtime = state.runtime.lock().expect("state lock poisoned");
    assert!(runtime.current_break.is_some());
    assert!(runtime.delivery_block_started_ms.is_some());
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
    let actions = state.tick(9_100, 8_000, false, None);

    assert!(actions.open_break_window);

    let runtime = state.runtime.lock().expect("state lock poisoned");
    assert!(runtime.current_break.is_some());
    assert!(runtime.next_break_wait_started_ms.is_none());
}

#[test]
fn smart_mode_uses_fixed_microbreak_idle_threshold() {
    let state = PauzaState::default();
    {
        let mut runtime = state.runtime.lock().expect("state lock poisoned");
        runtime.settings = settings();
        runtime.schedule_specific_break(BreakKind::Microbreak, 1_100, 1_000);
    }

    let _ = state.tick(1_100, 0, false, None);
    let actions = state.tick(16_100, 3_000, false, None);

    assert!(!actions.open_break_window);

    let runtime = state.runtime.lock().expect("state lock poisoned");
    assert!(runtime.current_break.is_none());
    assert_eq!(runtime.next_break_wait_started_ms, Some(1_100));
}

#[test]
fn long_break_uses_fixed_idle_threshold() {
    let state = PauzaState::default();
    {
        let mut runtime = state.runtime.lock().expect("state lock poisoned");
        runtime.settings = settings();
        runtime.schedule_specific_break(BreakKind::LongBreak, 1_100, 1_000);
    }

    let _ = state.tick(1_100, 0, false, None);
    let still_waiting = state.tick(20_100, 6_000, false, None);
    assert!(!still_waiting.open_break_window);

    let actions = state.tick(35_100, 12_000, false, None);
    assert!(actions.open_break_window);

    let runtime = state.runtime.lock().expect("state lock poisoned");
    assert!(
        runtime
            .current_break
            .as_ref()
            .is_some_and(|current| current.kind == BreakKind::LongBreak)
    );
    assert!(runtime.next_break_wait_started_ms.is_none());
}

#[test]
fn pause_and_resume_shift_due_instead_of_resetting_schedule() {
    let mut runtime = RuntimeState {
        settings: settings(),
        ..Default::default()
    };
    runtime.schedule_specific_break(BreakKind::Microbreak, 61_000, 1_000);

    let was_blocked = runtime.delivery_blocking_reason(1_000).is_some();
    runtime.paused_until_ms = Some(31_000);
    runtime.sync_delivery_block_state(1_000, was_blocked);
    assert_eq!(runtime.delivery_block_started_ms, Some(1_000));

    let was_blocked = runtime.delivery_blocking_reason(11_000).is_some();
    runtime.paused_until_ms = None;
    runtime.paused_indefinitely = false;
    runtime.sync_delivery_block_state(11_000, was_blocked);

    assert_eq!(runtime.next_break_kind, Some(BreakKind::Microbreak));
    assert_eq!(runtime.next_break_due_ms, Some(71_000));
    assert!(runtime.delivery_block_started_ms.is_none());
}

#[test]
fn dnd_freezes_waiting_timer_without_reset() {
    let state = PauzaState::default();
    {
        let mut runtime = state.runtime.lock().expect("state lock poisoned");
        runtime.settings = settings();
        runtime.schedule_specific_break(BreakKind::Microbreak, 1_100, 1_000);
    }

    let _ = state.tick(1_100, 0, false, None);
    let blocked = state.tick(5_100, 0, true, None);
    assert!(!blocked.open_break_window);

    let resumed = state.tick(25_100, 0, false, None);
    assert!(!resumed.open_break_window);

    let runtime = state.runtime.lock().expect("state lock poisoned");
    assert_eq!(runtime.next_break_due_ms, Some(21_100));
    assert_eq!(runtime.next_break_wait_started_ms, Some(21_100));
    assert_eq!(runtime.cycle_index, 0);
}

#[test]
fn smart_mode_starts_break_after_max_wait_even_without_idle_gap() {
    let state = PauzaState::default();
    {
        let mut runtime = state.runtime.lock().expect("state lock poisoned");
        runtime.settings = settings();
        runtime.schedule_specific_break(BreakKind::Microbreak, 1_100, 1_000);
    }

    let _ = state.tick(1_100, 0, false, None);
    let actions = state.tick(91_100, 0, false, None);

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
fn long_idle_full_reset_replans_from_now() {
    let state = PauzaState::default();
    {
        let mut runtime = state.runtime.lock().expect("state lock poisoned");
        runtime.settings = settings();
        runtime.schedule_next_slot(1_000);
    }

    let away = state.tick(950_000, 300_000, false, None);
    assert!(!away.open_break_window);

    let _ = state.tick(951_000, 1_000, false, None);
    let runtime = state.runtime.lock().expect("state lock poisoned");
    assert_eq!(runtime.next_break_kind, Some(BreakKind::Microbreak));
    assert_eq!(
        runtime.next_break_due_ms,
        Some(951_000 + runtime.settings.microbreak_interval_ms())
    );
    assert_eq!(runtime.cycle_index, 1);
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
fn legacy_idle_opportunity_is_not_serialized_back_out() {
    let serialized = serde_json::to_value(settings()).expect("serialize settings");
    let object = serialized
        .as_object()
        .expect("settings serialize to object");

    assert!(!object.contains_key("idleOpportunitySeconds"));
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
    let runtime = state.runtime.lock().expect("state lock poisoned");
    let (status, status_detail) = runtime.status(1_100);

    assert_eq!(
        status,
        i18n::text("zh-CN", "runtime.break.status.waitingOpportunityTitle")
    );
    assert_eq!(
        status_detail,
        i18n::text1(
            "zh-CN",
            "runtime.break.status.waitingOpportunityDetail",
            "kind",
            i18n::text("zh-CN", "runtime.break.kind.microbreak")
        )
        .replace("{{duration}}", &i18n::duration("zh-CN", MICROBREAK_SMART_MAX_WAIT_MS))
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
