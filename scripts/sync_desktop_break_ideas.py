#!/usr/bin/env python3

from __future__ import annotations

import json
import re
import unicodedata
from collections import Counter
from pathlib import Path
from typing import Any

DEFAULT_LANGUAGE = "zh-CN"
BATCH_SCHEMA_VERSION = 1
BREAK_IDEA_KEYS = ("miniBreakIdeas", "longBreakIdeas")
BREAK_IDEA_FIELDS = {
    "miniBreakIdeas": ("text",),
    "longBreakIdeas": ("title", "text"),
}
IDEA_ID_PATTERN = re.compile(r"^[a-z]{3}$")
BATCH_KEYS = {
    "schemaVersion",
    "batchId",
    "sourceLanguage",
    "officialLanguages",
    "categories",
    "targets",
    "limits",
    "copyRules",
    "entries",
}


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


def require_object(value: Any, context: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{context} must be an object.")
    return value


def require_exact_keys(value: dict[str, Any], expected: set[str], context: str) -> None:
    actual = set(value)
    if actual == expected:
        return
    missing = sorted(expected - actual)
    unknown = sorted(actual - expected)
    details: list[str] = []
    if missing:
        details.append("missing " + ", ".join(missing))
    if unknown:
        details.append("unexpected " + ", ".join(unknown))
    raise SystemExit(f"{context} has invalid fields ({'; '.join(details)}).")


def require_non_empty_string(value: Any, context: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{context} must be a non-empty string.")
    if value != value.strip():
        raise SystemExit(f"{context} must not have leading or trailing whitespace.")
    return value


def idea_id_number(idea_id: str, context: str) -> int:
    if not IDEA_ID_PATTERN.fullmatch(idea_id):
        raise SystemExit(f"{context} must be a three-letter lowercase ID, got `{idea_id}`.")
    result = 0
    for character in idea_id:
        result = result * 26 + ord(character) - ord("a")
    return result


def idea_id_from_number(value: int) -> str:
    if value < 0 or value >= 26**3:
        raise SystemExit(f"Break idea ID number is outside the three-letter range: {value}")
    characters = ["a", "a", "a"]
    for index in range(2, -1, -1):
        characters[index] = chr(ord("a") + value % 26)
        value //= 26
    return "".join(characters)


def idea_id_range(first_id: str, last_id: str, context: str) -> list[str]:
    first = idea_id_number(first_id, f"{context} firstId")
    last = idea_id_number(last_id, f"{context} lastId")
    if last < first:
        raise SystemExit(f"{context} lastId `{last_id}` precedes firstId `{first_id}`.")
    return [idea_id_from_number(value) for value in range(first, last + 1)]


def normalize_copy(value: str) -> str:
    normalized = unicodedata.normalize("NFKC", value).casefold()
    return "".join(
        character
        for character in normalized
        if not character.isspace()
        and not unicodedata.category(character).startswith(("P", "S"))
    )


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
) -> tuple[str, list[str]]:
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

    official_codes: list[str] = []
    for code in official_languages:
        if not isinstance(code, str):
            raise SystemExit("Break idea registry source official language codes must be strings.")
        if code not in config_codes:
            raise SystemExit(
                f"Break idea registry source official language `{code}` has no matching locale config."
            )
        if code in official_codes:
            raise SystemExit(f"Break idea registry source repeats official language `{code}`.")
        official_codes.append(code)

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


def validate_official_bundles(
    bundles: dict[str, Any],
    official_languages: list[str],
) -> None:
    for code in official_languages:
        if code not in bundles:
            raise SystemExit(f"Official break idea language `{code}` has no message bundle.")

    reference_code = official_languages[0]
    reference_bundle = bundles[reference_code]

    for key in BREAK_IDEA_KEYS:
        reference_ids = list(reference_bundle[key])
        if not reference_ids:
            raise SystemExit(f"Official break idea source `{reference_code}.{key}` is empty.")

        expected_all_ids = [idea_id_from_number(value) for value in range(len(reference_ids))]
        if reference_ids != expected_all_ids:
            raise SystemExit(
                f"Official break idea IDs in `{reference_code}.{key}` must be continuous and ordered "
                f"from `aaa` through `{expected_all_ids[-1]}`."
            )

        for code in official_languages:
            source = bundles[code][key]
            source_ids = list(source)
            if source_ids != reference_ids:
                missing = sorted(set(reference_ids) - set(source_ids))
                extra = sorted(set(source_ids) - set(reference_ids))
                detail = []
                if missing:
                    detail.append("missing " + ", ".join(missing[:8]))
                if extra:
                    detail.append("extra " + ", ".join(extra[:8]))
                if not detail:
                    detail.append("same IDs but different source order")
                raise SystemExit(
                    f"Official break idea key parity failed for `{code}.{key}` "
                    f"against `{reference_code}` ({'; '.join(detail)})."
                )

            expected_fields = set(BREAK_IDEA_FIELDS[key])
            seen_copy: dict[str, dict[str, str]] = {
                field: {} for field in BREAK_IDEA_FIELDS[key]
            }
            for idea_id, raw_entry in source.items():
                context = f"{code}.{key}.{idea_id}"
                entry = require_object(raw_entry, context)
                require_exact_keys(entry, expected_fields, context)
                for field in BREAK_IDEA_FIELDS[key]:
                    require_non_empty_string(entry.get(field), f"{context}.{field}")

                for field in BREAK_IDEA_FIELDS[key]:
                    normalized_value = normalize_copy(entry[field])
                    if not normalized_value:
                        raise SystemExit(f"{context}.{field} is empty after normalization.")
                    previous_id = seen_copy[field].get(normalized_value)
                    if previous_id is not None:
                        raise SystemExit(
                            f"Normalized duplicate `{field}` copy in `{code}.{key}`: "
                            f"`{previous_id}` and `{idea_id}`."
                        )
                    seen_copy[field][normalized_value] = idea_id


def read_length_range(value: Any, context: str) -> tuple[int, int]:
    if (
        not isinstance(value, list)
        or len(value) != 2
        or any(not isinstance(item, int) or isinstance(item, bool) for item in value)
    ):
        raise SystemExit(f"{context} must be a two-integer `[min, max]` array.")
    minimum, maximum = value
    if minimum < 1 or maximum < minimum:
        raise SystemExit(f"{context} has an invalid range: {value}.")
    return minimum, maximum


def validate_batch_limits(
    *,
    batch: dict[str, Any],
    batch_path: Path,
    batch_entries: dict[str, list[dict[str, str]]],
    bundles: dict[str, Any],
    official_languages: list[str],
) -> None:
    context = f"Batch `{batch_path.name}`"
    limits = require_object(batch["limits"], f"{context} limits")
    require_exact_keys(limits, set(official_languages), f"{context} limits")

    copy_rules = require_object(batch["copyRules"], f"{context} copyRules")
    require_exact_keys(copy_rules, set(official_languages), f"{context} copyRules")

    for code in official_languages:
        locale_limits = require_object(limits[code], f"{context} limits.{code}")
        require_exact_keys(locale_limits, set(BREAK_IDEA_KEYS), f"{context} limits.{code}")

        locale_rules = require_object(copy_rules[code], f"{context} copyRules.{code}")
        require_exact_keys(locale_rules, {"forbiddenPhrases"}, f"{context} copyRules.{code}")
        forbidden_phrases = locale_rules["forbiddenPhrases"]
        if not isinstance(forbidden_phrases, list) or any(
            not isinstance(phrase, str) or not phrase for phrase in forbidden_phrases
        ):
            raise SystemExit(
                f"{context} copyRules.{code}.forbiddenPhrases must be an array of non-empty strings."
            )
        if len(set(forbidden_phrases)) != len(forbidden_phrases):
            raise SystemExit(
                f"{context} copyRules.{code}.forbiddenPhrases must not repeat phrases."
            )

        for key in BREAK_IDEA_KEYS:
            field_limits = require_object(
                locale_limits[key], f"{context} limits.{code}.{key}"
            )
            expected_fields = set(BREAK_IDEA_FIELDS[key])
            require_exact_keys(
                field_limits,
                expected_fields,
                f"{context} limits.{code}.{key}",
            )
            parsed_limits = {
                field: read_length_range(
                    field_limits[field], f"{context} limits.{code}.{key}.{field}"
                )
                for field in BREAK_IDEA_FIELDS[key]
            }

            for metadata in batch_entries[key]:
                idea_id = metadata["id"]
                entry = bundles[code][key][idea_id]
                combined_copy = "\n".join(entry[field] for field in BREAK_IDEA_FIELDS[key])

                for phrase in forbidden_phrases:
                    if phrase.casefold() in combined_copy.casefold():
                        raise SystemExit(
                            f"{context} forbids phrase `{phrase}` in `{code}.{key}.{idea_id}`."
                        )

                for field, (minimum, maximum) in parsed_limits.items():
                    length = len(entry[field])
                    if not minimum <= length <= maximum:
                        raise SystemExit(
                            f"{code}.{key}.{idea_id}.{field} has {length} characters; "
                            f"batch `{batch['batchId']}` allows {minimum}..{maximum}."
                        )


def validate_batches(
    batches_dir: Path,
    *,
    bundles: dict[str, Any],
    official_languages: list[str],
) -> int:
    batch_paths = sorted(batches_dir.glob("*.json")) if batches_dir.exists() else []
    if not batch_paths:
        return 0

    previous_after: dict[str, int] = {}
    managed_start: dict[str, int] = {}
    seen_batch_ids: set[str] = set()
    seen_idea_ids: dict[str, set[str]] = {key: set() for key in BREAK_IDEA_KEYS}

    for batch_path in batch_paths:
        context = f"Batch `{batch_path.name}`"
        batch = require_object(read_json(batch_path), context)
        require_exact_keys(batch, BATCH_KEYS, context)

        if batch["schemaVersion"] != BATCH_SCHEMA_VERSION:
            raise SystemExit(
                f"{context} schemaVersion must be {BATCH_SCHEMA_VERSION}, "
                f"got `{batch['schemaVersion']}`."
            )

        batch_id = require_non_empty_string(batch["batchId"], f"{context} batchId")
        if batch_id != batch_path.stem:
            raise SystemExit(
                f"{context} declares batchId `{batch_id}`, expected `{batch_path.stem}`."
            )
        if batch_id in seen_batch_ids:
            raise SystemExit(f"Duplicate break idea batchId `{batch_id}`.")
        seen_batch_ids.add(batch_id)

        source_language = require_non_empty_string(
            batch["sourceLanguage"], f"{context} sourceLanguage"
        )
        if source_language not in official_languages:
            raise SystemExit(f"{context} sourceLanguage `{source_language}` is not official.")
        if batch["officialLanguages"] != official_languages:
            raise SystemExit(
                f"{context} officialLanguages must exactly match registry source order: "
                + ", ".join(official_languages)
            )

        categories = batch["categories"]
        if (
            not isinstance(categories, list)
            or not categories
            or any(not isinstance(category, str) or not category for category in categories)
            or len(set(categories)) != len(categories)
        ):
            raise SystemExit(f"{context} categories must be a non-empty unique string array.")
        category_set = set(categories)

        targets = require_object(batch["targets"], f"{context} targets")
        entries = require_object(batch["entries"], f"{context} entries")
        require_exact_keys(targets, set(BREAK_IDEA_KEYS), f"{context} targets")
        require_exact_keys(entries, set(BREAK_IDEA_KEYS), f"{context} entries")
        parsed_entries: dict[str, list[dict[str, str]]] = {}

        for key in BREAK_IDEA_KEYS:
            target = require_object(targets[key], f"{context} targets.{key}")
            require_exact_keys(
                target,
                {
                    "before",
                    "added",
                    "after",
                    "firstId",
                    "lastId",
                    "maxConsecutiveCategory",
                    "maxCategoryImbalance",
                },
                f"{context} targets.{key}",
            )

            numeric_fields = (
                "before",
                "added",
                "after",
                "maxConsecutiveCategory",
                "maxCategoryImbalance",
            )
            for field in numeric_fields:
                value = target[field]
                if not isinstance(value, int) or isinstance(value, bool) or value < 0:
                    raise SystemExit(f"{context} targets.{key}.{field} must be a non-negative integer.")
            if target["added"] < 1 or target["maxConsecutiveCategory"] < 1:
                raise SystemExit(f"{context} targets.{key} must add entries and allow a category run.")
            if target["before"] + target["added"] != target["after"]:
                raise SystemExit(f"{context} targets.{key} before + added must equal after.")

            expected_ids = idea_id_range(
                target["firstId"], target["lastId"], f"{context} targets.{key}"
            )
            if len(expected_ids) != target["added"]:
                raise SystemExit(
                    f"{context} targets.{key} ID range has {len(expected_ids)} IDs, "
                    f"expected added={target['added']}."
                )
            if idea_id_number(target["firstId"], f"{context} targets.{key}.firstId") != target["before"]:
                raise SystemExit(
                    f"{context} targets.{key}.firstId must immediately follow its before count."
                )
            if idea_id_number(target["lastId"], f"{context} targets.{key}.lastId") + 1 != target["after"]:
                raise SystemExit(
                    f"{context} targets.{key}.lastId must align with its after count."
                )
            if key in previous_after and target["before"] != previous_after[key]:
                raise SystemExit(
                    f"{context} targets.{key}.before must equal prior batch after={previous_after[key]}."
                )
            managed_start.setdefault(key, target["before"])
            previous_after[key] = target["after"]

            raw_entries = entries[key]
            if not isinstance(raw_entries, list):
                raise SystemExit(f"{context} entries.{key} must be an array.")
            parsed: list[dict[str, str]] = []
            for index, raw_entry in enumerate(raw_entries):
                entry_context = f"{context} entries.{key}[{index}]"
                metadata = require_object(raw_entry, entry_context)
                require_exact_keys(metadata, {"id", "category"}, entry_context)
                idea_id = require_non_empty_string(metadata["id"], f"{entry_context}.id")
                idea_id_number(idea_id, f"{entry_context}.id")
                category = require_non_empty_string(
                    metadata["category"], f"{entry_context}.category"
                )
                if category not in category_set:
                    raise SystemExit(f"{entry_context} uses unknown category `{category}`.")
                if idea_id in seen_idea_ids[key]:
                    raise SystemExit(f"Break idea ID `{idea_id}` is repeated across batch entries for `{key}`.")
                seen_idea_ids[key].add(idea_id)
                parsed.append({"id": idea_id, "category": category})

            parsed_ids = [metadata["id"] for metadata in parsed]
            if parsed_ids != expected_ids:
                raise SystemExit(
                    f"{context} entries.{key} must list every ID in order from "
                    f"`{target['firstId']}` through `{target['lastId']}`."
                )

            category_counts = Counter(metadata["category"] for metadata in parsed)
            if set(category_counts) != category_set:
                missing_categories = sorted(category_set - set(category_counts))
                raise SystemExit(
                    f"{context} entries.{key} does not use categories: "
                    + ", ".join(missing_categories)
                )
            imbalance = max(category_counts.values()) - min(category_counts.values())
            if imbalance > target["maxCategoryImbalance"]:
                raise SystemExit(
                    f"{context} entries.{key} category imbalance is {imbalance}, "
                    f"maximum is {target['maxCategoryImbalance']}."
                )

            run_category = ""
            run_length = 0
            for metadata in parsed:
                if metadata["category"] == run_category:
                    run_length += 1
                else:
                    run_category = metadata["category"]
                    run_length = 1
                if run_length > target["maxConsecutiveCategory"]:
                    raise SystemExit(
                        f"{context} entries.{key} has more than "
                        f"{target['maxConsecutiveCategory']} consecutive `{run_category}` entries."
                    )

            for code in official_languages:
                missing_ids = [idea_id for idea_id in expected_ids if idea_id not in bundles[code][key]]
                if missing_ids:
                    raise SystemExit(
                        f"{context} IDs missing from `{code}.{key}`: " + ", ".join(missing_ids[:8])
                    )

            parsed_entries[key] = parsed

        validate_batch_limits(
            batch=batch,
            batch_path=batch_path,
            batch_entries=parsed_entries,
            bundles=bundles,
            official_languages=official_languages,
        )

    for key in BREAK_IDEA_KEYS:
        source_count = len(bundles[official_languages[0]][key])
        if previous_after[key] != source_count:
            raise SystemExit(
                f"Latest batch target for `{key}` is {previous_after[key]}, "
                f"but official source contains {source_count} entries."
            )
        expected_managed_ids = {
            idea_id_from_number(value) for value in range(managed_start[key], source_count)
        }
        if seen_idea_ids[key] != expected_managed_ids:
            missing_ids = sorted(expected_managed_ids - seen_idea_ids[key])
            unexpected_ids = sorted(seen_idea_ids[key] - expected_managed_ids)
            detail: list[str] = []
            if missing_ids:
                detail.append("missing " + ", ".join(missing_ids[:8]))
            if unexpected_ids:
                detail.append("unexpected " + ", ".join(unexpected_ids[:8]))
            raise SystemExit(
                f"Managed batch coverage failed for `{key}` ({'; '.join(detail)})."
            )

    return len(batch_paths)


def main() -> None:
    root = repo_root()
    locales_root = root / "apps" / "desktop" / "src" / "locales"
    config_dir = locales_root / "config"
    break_ideas_root = locales_root / "break-ideas"
    break_ideas_dir = break_ideas_root / "messages"
    batches_dir = break_ideas_root / "batches"
    registry_source_path = break_ideas_root / "registry.json"
    registry_output_path = break_ideas_root / "registry.generated.json"

    configs = read_config_files(config_dir)
    config_codes = {config["code"] for config in configs}
    order = existing_language_order(registry_output_path)

    default_language, official_languages = validate_registry_source(
        read_json(registry_source_path),
        config_codes=config_codes,
    )
    official_codes = set(official_languages)

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

    validate_official_bundles(bundles, official_languages)
    batch_count = validate_batches(
        batches_dir,
        bundles=bundles,
        official_languages=official_languages,
    )

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
        f"{len(languages)} languages, {len(bundles)} bundles, {batch_count} managed batches, "
        f"registry -> {registry_output_path.relative_to(root)}",
    )


if __name__ == "__main__":
    main()
