use std::fs;
use std::path::PathBuf;

use super::{PauzaSettings, RuntimeState};

pub(super) fn load_settings(path: &PathBuf) -> Result<PauzaSettings, String> {
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

pub(super) fn save_settings(runtime: &RuntimeState) -> Result<(), String> {
    let path = runtime
        .config_path
        .as_ref()
        .ok_or_else(|| "settings path is not initialized".to_string())?;
    let contents =
        serde_json::to_string_pretty(&runtime.settings).map_err(|error| error.to_string())?;
    fs::write(path, contents).map_err(|error| error.to_string())
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
