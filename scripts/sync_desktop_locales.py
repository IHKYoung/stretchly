#!/usr/bin/env python3

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

DEFAULT_LANGUAGE = "zh-CN"
BREAK_IDEA_KEYS = {"miniBreakIdeas", "longBreakIdeas"}


def repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def ensure_messages_exclude_break_ideas(path: Path, data: Any) -> None:
    if not isinstance(data, dict):
        return

    duplicated_keys = sorted(BREAK_IDEA_KEYS.intersection(data.keys()))
    if duplicated_keys:
        raise SystemExit(
            f"Desktop locale message file must not contain break idea keys `{', '.join(duplicated_keys)}` anymore: {path}"
        )


def write_json(path: Path, data: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    content = json.dumps(data, ensure_ascii=False, indent=2) + "\n"
    if path.exists() and path.read_text(encoding="utf-8") == content:
        return
    path.write_text(content, encoding="utf-8")


def read_config_files(config_dir: Path) -> list[dict[str, Any]]:
    configs: list[dict[str, Any]] = []
    for config_path in sorted(config_dir.glob("*.json")):
        config = read_json(config_path)
        if not isinstance(config, dict):
            raise SystemExit(f"Config file must contain an object: {config_path}")
        code = config.get("code")
        if code != config_path.stem:
            raise SystemExit(
                f"Config code mismatch: {config_path.name} declares `{code}`, expected `{config_path.stem}`"
            )
        configs.append(config)
    if not configs:
        raise SystemExit(f"No locale config files found in {config_dir}")
    return configs


def read_codes(directory: Path) -> set[str]:
    return {path.stem for path in directory.glob("*.json")}


def existing_language_order(registry_path: Path) -> dict[str, int]:
    if not registry_path.exists():
        return {}
    registry = read_json(registry_path)
    languages = registry.get("languages") if isinstance(registry, dict) else None
    if not isinstance(languages, list):
        return {}
    return {
        config.get("code"): index
        for index, config in enumerate(languages)
        if isinstance(config, dict) and isinstance(config.get("code"), str)
    }


def validate_locale_sources(
    *,
    configs: list[dict[str, Any]],
    message_codes: set[str],
) -> None:
    config_codes = {config["code"] for config in configs}
    missing_messages = sorted(config_codes - message_codes)
    orphan_messages = sorted(message_codes - config_codes)

    if missing_messages:
        raise SystemExit(
            "Missing desktop locale message files for config codes: "
            + ", ".join(missing_messages)
        )
    if orphan_messages:
        raise SystemExit(
            "Desktop locale message files without matching config: "
            + ", ".join(orphan_messages)
        )


def main() -> None:
    root = repo_root()
    locales_root = root / "apps" / "desktop" / "src" / "locales"
    messages_dir = locales_root / "messages"
    config_dir = locales_root / "config"
    registry_path = locales_root / "registry.generated.json"

    configs = read_config_files(config_dir)
    message_codes = read_codes(messages_dir)
    validate_locale_sources(
        configs=configs,
        message_codes=message_codes,
    )
    order = existing_language_order(registry_path)

    languages: list[dict[str, Any]] = []
    bundles: dict[str, Any] = {}

    for config in configs:
        code = config["code"]
        languages.append(config)

        message_path = messages_dir / f"{code}.json"
        if message_path.exists():
            message_data = read_json(message_path)
            ensure_messages_exclude_break_ideas(message_path, message_data)
            bundles[code] = message_data
        else:
            bundles[code] = {}

    languages.sort(key=lambda item: (order.get(item["code"], 10_000), item["code"]))

    registry = {
        "defaultLanguage": DEFAULT_LANGUAGE,
        "languages": languages,
        "bundles": bundles,
    }
    write_json(registry_path, registry)

    print(
        "[OK] Built desktop locale registry:",
        f"{len(languages)} languages, registry -> {registry_path.relative_to(root)}",
    )


if __name__ == "__main__":
    main()
