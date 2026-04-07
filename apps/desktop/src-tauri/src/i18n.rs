use serde_json::Value;
use std::sync::OnceLock;

static EN_BUNDLE: OnceLock<Value> = OnceLock::new();
static ZH_BUNDLE: OnceLock<Value> = OnceLock::new();

pub fn normalize_language(language: &str) -> &'static str {
    match language {
        "en" => "en",
        "zh" | "zh-CN" | "zh_CN" | "zh-Hans" | "zh-Hans-CN" => "zh-CN",
        _ => "zh-CN",
    }
}

pub fn text(language: &str, key: &str) -> String {
    template(language, key)
}

pub fn text1(language: &str, key: &str, name: &str, value: impl AsRef<str>) -> String {
    template(language, key).replace(&placeholder(name), value.as_ref())
}

pub fn text2(
    language: &str,
    key: &str,
    first_name: &str,
    first_value: impl AsRef<str>,
    second_name: &str,
    second_value: impl AsRef<str>,
) -> String {
    text1(
        language,
        key,
        first_name,
        first_value.as_ref(),
    )
    .replace(&placeholder(second_name), second_value.as_ref())
}

pub fn duration(language: &str, ms: u64) -> String {
    let total_seconds = (ms / 1000).max(1);
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    if minutes == 0 {
        return text1(language, "duration.seconds", "value", seconds.to_string());
    }

    if seconds == 0 {
        return text1(language, "duration.minutes", "value", minutes.to_string());
    }

    let seconds_value = if normalize_language(language) == "zh-CN" {
        format!("{seconds:02}")
    } else {
        format!("{seconds}s")
    };

    text2(
        language,
        "duration.minutesSeconds",
        "minutes",
        minutes.to_string(),
        "seconds",
        seconds_value,
    )
}

fn placeholder(name: &str) -> String {
    format!("{{{{{name}}}}}")
}

fn template(language: &str, key: &str) -> String {
    lookup(bundle(language), key).unwrap_or_else(|| key.to_string())
}

fn bundle(language: &str) -> &'static Value {
    match normalize_language(language) {
        "en" => EN_BUNDLE.get_or_init(|| {
            serde_json::from_str(include_str!("../../src/locales/en.json"))
                .expect("valid English locale bundle")
        }),
        _ => ZH_BUNDLE.get_or_init(|| {
            serde_json::from_str(include_str!("../../src/locales/zh-CN.json"))
                .expect("valid Chinese locale bundle")
        }),
    }
}

fn lookup(bundle: &Value, key: &str) -> Option<String> {
    let mut current = bundle;

    for segment in key.split('.') {
        current = current.get(segment)?;
    }

    current.as_str().map(ToOwned::to_owned)
}
