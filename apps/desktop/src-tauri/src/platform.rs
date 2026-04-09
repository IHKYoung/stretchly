use crate::state::PauzaSettings;
use std::ffi::OsString;
use std::process::Command;
use sysinfo::{ProcessesToUpdate, System};

#[derive(Debug, Default)]
pub struct PlatformSignals {
    pub idle_ms: u64,
    pub dnd_active: bool,
    pub app_exclusion_match: Option<String>,
}

pub struct PlatformMonitor {
    processes: System,
}

impl Default for PlatformMonitor {
    fn default() -> Self {
        Self {
            processes: System::new(),
        }
    }
}

impl PlatformMonitor {
    pub fn probe(&mut self, settings: &PauzaSettings) -> PlatformSignals {
        PlatformSignals {
            idle_ms: resolve_idle_ms(detect_idle_ms),
            dnd_active: if settings.monitor_dnd {
                detect_dnd()
            } else {
                false
            },
            app_exclusion_match: if settings.app_exclusions_enabled {
                self.detect_app_exclusion(settings)
            } else {
                None
            },
        }
    }

    fn detect_app_exclusion(&mut self, settings: &PauzaSettings) -> Option<String> {
        let needles = settings.exclusion_commands();
        if needles.is_empty() {
            return None;
        }

        self.processes.refresh_processes(ProcessesToUpdate::All, true);

        for needle in needles {
            let query = needle.to_ascii_lowercase();
            let found = self.processes.processes().values().any(|process| {
                contains_case_insensitive(process.name().to_string_lossy().as_ref(), &query)
                    || process
                        .exe()
                        .map(|path| contains_case_insensitive(path.to_string_lossy().as_ref(), &query))
                        .unwrap_or(false)
                    || contains_case_insensitive(&join_command(process.cmd()), &query)
            });

            if found {
                return Some(needle);
            }
        }

        None
    }
}

fn resolve_idle_ms<F>(detect_idle: F) -> u64
where
    F: FnOnce() -> u64,
{
    detect_idle()
}

fn contains_case_insensitive(source: &str, query: &str) -> bool {
    source.to_ascii_lowercase().contains(query)
}

fn join_command(parts: &[OsString]) -> String {
    parts
        .iter()
        .map(|part| part.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ")
}

fn detect_idle_ms() -> u64 {
    match std::env::consts::OS {
        "macos" => detect_idle_macos(),
        "linux" => detect_idle_linux(),
        "windows" => detect_idle_windows(),
        _ => 0,
    }
}

fn detect_dnd() -> bool {
    match std::env::consts::OS {
        "macos" => detect_dnd_macos(),
        "linux" => detect_dnd_linux(),
        "windows" => false,
        _ => false,
    }
}

fn detect_idle_macos() -> u64 {
    let output = Command::new("ioreg")
        .args(["-r", "-c", "IOHIDSystem"])
        .output();

    let Ok(output) = output else {
        return 0;
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .find(|line| line.contains("HIDIdleTime"))
        .and_then(extract_digits)
        .map(|raw| raw / 1_000_000)
        .unwrap_or(0)
}

fn detect_idle_linux() -> u64 {
    let output = Command::new("xprintidle").output();
    let Ok(output) = output else {
        return 0;
    };

    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<u64>()
        .unwrap_or(0)
}

fn detect_idle_windows() -> u64 {
    let script = r#"
$signature = @"
[StructLayout(LayoutKind.Sequential)]
public struct LASTINPUTINFO {
  public uint cbSize;
  public uint dwTime;
}
[DllImport("user32.dll")]
public static extern bool GetLastInputInfo(ref LASTINPUTINFO plii);
"@
Add-Type -MemberDefinition $signature -Name User32 -Namespace Pauza | Out-Null
$info = New-Object Pauza.User32+LASTINPUTINFO
$info.cbSize = [System.Runtime.InteropServices.Marshal]::SizeOf($info)
[Pauza.User32]::GetLastInputInfo([ref]$info) | Out-Null
[Environment]::TickCount64 - $info.dwTime
"#;

    powershell_value(script)
}

fn detect_dnd_macos() -> bool {
    let variants = [
        ["read", "com.apple.controlcenter", "NSStatusItem VisibleCC FocusModes"],
        ["read", "com.apple.controlcenter", "NSStatusItem Visible FocusModes"],
    ];

    variants.iter().any(|args| {
        let output = Command::new("defaults").args(*args).output();
        let Ok(output) = output else {
            return false;
        };

        normalize_flag(&String::from_utf8_lossy(&output.stdout)) == Some(true)
    })
}

fn detect_dnd_linux() -> bool {
    let output = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.notifications", "show-banners"])
        .output();

    let Ok(output) = output else {
        return false;
    };

    normalize_flag(&String::from_utf8_lossy(&output.stdout)) == Some(false)
}

fn normalize_flag(value: &str) -> Option<bool> {
    match value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase()
        .as_str()
    {
        "1" | "true" => Some(true),
        "0" | "false" => Some(false),
        _ => None,
    }
}

fn extract_digits(line: &str) -> Option<u64> {
    let digits = line.chars().filter(|ch| ch.is_ascii_digit()).collect::<String>();
    digits.parse::<u64>().ok()
}

fn powershell_value(script: &str) -> u64 {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", script])
        .output();

    let Ok(output) = output else {
        return 0;
    };

    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<u64>()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idle_signal_is_available_even_without_natural_breaks() {
        assert_eq!(resolve_idle_ms(|| 12_345), 12_345);
    }
}
