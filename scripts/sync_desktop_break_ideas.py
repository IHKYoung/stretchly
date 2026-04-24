#!/usr/bin/env python3

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

DEFAULT_LANGUAGE = "zh-CN"
BREAK_IDEA_KEYS = ("miniBreakIdeas", "longBreakIdeas")


def repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


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


def validate_registry_source(
    source: Any,
    *,
    config_codes: set[str],
) -> tuple[str, set[str]]:
    if not isinstance(source, dict):
        raise SystemExit("Break idea registry source must contain an object.")

    default_language = source.get("defaultLanguage", DEFAULT_LANGUAGE)
    if not isinstance(default_language, str):
        raise SystemExit("Break idea registry source `defaultLanguage` must be a string.")
    if default_language not in config_codes:
        raise SystemExit(
            f"Break idea registry source default language `{default_language}` has no matching locale config."
        )

    official_languages = source.get("officialLanguages")
    if not isinstance(official_languages, list) or not official_languages:
        raise SystemExit("Break idea registry source `officialLanguages` must be a non-empty array.")

    official_codes: set[str] = set()
    for code in official_languages:
        if not isinstance(code, str):
            raise SystemExit("Break idea registry source official language codes must be strings.")
        if code not in config_codes:
            raise SystemExit(
                f"Break idea registry source official language `{code}` has no matching locale config."
            )
        official_codes.add(code)

    return default_language, official_codes


def validate_break_idea_bundle(path: Path) -> Any:
    data = read_json(path)
    if not isinstance(data, dict):
        raise SystemExit(f"Break idea file must contain an object: {path}")

    unknown_keys = sorted(set(data.keys()) - set(BREAK_IDEA_KEYS))
    if unknown_keys:
        raise SystemExit(
            f"Break idea file contains unexpected keys `{', '.join(unknown_keys)}`: {path}"
        )

    for key in BREAK_IDEA_KEYS:
        value = data.get(key)
        if not isinstance(value, dict):
            raise SystemExit(f"Break idea file `{path}` must contain object key `{key}`.")

    return data


def main() -> None:
    root = repo_root()
    locales_root = root / "apps" / "desktop" / "src" / "locales"
    config_dir = locales_root / "config"
    break_ideas_root = locales_root / "break-ideas"
    break_ideas_dir = break_ideas_root / "messages"
    registry_source_path = break_ideas_root / "registry.json"
    registry_output_path = break_ideas_root / "registry.generated.json"

    configs = read_config_files(config_dir)
    config_codes = {config["code"] for config in configs}
    order = existing_language_order(registry_output_path)

    default_language, official_codes = validate_registry_source(
        read_json(registry_source_path),
        config_codes=config_codes,
    )

    bundle_codes = read_codes(break_ideas_dir)
    orphan_bundles = sorted(bundle_codes - config_codes)
    if orphan_bundles:
        raise SystemExit(
            "Desktop break idea files without matching locale config: "
            + ", ".join(orphan_bundles)
        )

    languages: list[dict[str, Any]] = []
    bundles: dict[str, Any] = {}

    for config in configs:
        code = config["code"]
        languages.append(
            {
                "code": code,
                "fallback": config.get("fallback", ""),
                "tier": "official" if code in official_codes else "legacy",
                "available": code in bundle_codes,
            }
        )

        bundle_path = break_ideas_dir / f"{code}.json"
        if bundle_path.exists():
            bundles[code] = validate_break_idea_bundle(bundle_path)

    languages.sort(key=lambda item: (order.get(item["code"], 10_000), item["code"]))

    registry = {
        "defaultLanguage": default_language,
        "officialLanguages": sorted(official_codes, key=lambda code: (order.get(code, 10_000), code)),
        "languages": languages,
        "bundles": bundles,
    }
    write_json(registry_output_path, registry)

    print(
        "[OK] Built desktop break idea registry:",
        f"{len(languages)} languages, {len(bundles)} bundles, registry -> {registry_output_path.relative_to(root)}",
    )


if __name__ == "__main__":
    main()
