use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LanguageConfig {
    code: String,
    fallback: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LocaleRegistry {
    default_language: String,
    languages: Vec<LanguageConfig>,
    bundles: HashMap<String, Value>,
}

static REGISTRY: OnceLock<LocaleRegistry> = OnceLock::new();

pub fn normalize_language(language: &str) -> &'static str {
    let Some(default) = default_config() else {
        return "zh-CN";
    };

    config_for_code(canonical_language(language))
        .unwrap_or(default)
        .code
        .as_str()
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
    text1(language, key, first_name, first_value.as_ref())
        .replace(&placeholder(second_name), second_value.as_ref())
}

pub fn duration(language: &str, ms: u64) -> String {
    let normalized_language = normalize_language(language);
    let total_seconds = (ms / 1000).max(1);
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    if minutes == 0 {
        return text1(normalized_language, "duration.seconds", "value", seconds.to_string());
    }

    if seconds == 0 {
        return text1(normalized_language, "duration.minutes", "value", minutes.to_string());
    }

    let seconds_value = if normalized_language == "zh-CN" {
        format!("{seconds:02}")
    } else {
        format!("{seconds}s")
    };

    text2(
        normalized_language,
        "duration.minutesSeconds",
        "minutes",
        minutes.to_string(),
        "seconds",
        seconds_value,
    )
}

fn registry() -> &'static LocaleRegistry {
    REGISTRY.get_or_init(|| {
        serde_json::from_str(include_str!("../../src/locales/registry.generated.json"))
            .expect("valid locale registry")
    })
}

fn config_for_code(code: &str) -> Option<&'static LanguageConfig> {
    registry().languages.iter().find(|config| config.code == code)
}

fn default_config() -> Option<&'static LanguageConfig> {
    config_for_code(&registry().default_language)
        .or_else(|| registry().languages.first())
}

fn canonical_language(language: &str) -> &str {
    match language {
        "zh" | "zh_CN" | "zh-Hans" | "zh-Hans-CN" => "zh-CN",
        _ => language,
    }
}

fn bundle_chain(language: &str) -> Vec<&'static Value> {
    let registry = registry();
    let Some(default) = default_config() else {
        return Vec::new();
    };

    let mut chain = Vec::new();
    let mut seen = HashSet::new();
    let mut current = Some(config_for_code(canonical_language(language)).unwrap_or(default));

    while let Some(config) = current {
        if !seen.insert(config.code.as_str()) {
            break;
        }

        if let Some(bundle) = registry.bundles.get(&config.code) {
            chain.push(bundle);
        }

        current = if config.fallback.is_empty() {
            None
        } else {
            config_for_code(&config.fallback)
        };
    }

    let normalized = normalize_language(language);
    if seen.insert(normalized) {
        if let Some(bundle) = registry.bundles.get(normalized) {
            chain.push(bundle);
        }
    }

    chain
}

fn placeholder(name: &str) -> String {
    format!("{{{{{name}}}}}")
}

fn template(language: &str, key: &str) -> String {
    for bundle in bundle_chain(language) {
        if let Some(template) = lookup(bundle, key) {
            return template;
        }
    }

    key.to_string()
}

fn lookup(bundle: &Value, key: &str) -> Option<String> {
    let mut current = bundle;

    for segment in key.split('.') {
        current = current.get(segment)?;
    }

    current.as_str().map(ToOwned::to_owned)
}
