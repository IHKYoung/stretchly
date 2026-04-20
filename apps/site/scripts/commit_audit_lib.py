#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""Shared helpers for commit audit task-id attribution."""

from __future__ import annotations

import re
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, List, Sequence, Tuple


TASK_ID_RE = re.compile(r"(?im)^\s*(?:[-*]\s*)?Task-ID\s*[:：]\s*([A-Za-z0-9._-]+)\s*$")
TASK_ID_TOKEN_RE = re.compile(r"\bTID-\d{8}-[a-z0-9-]+\b")
RELATED_TASK_IDS_RE = re.compile(r"(?im)^\s*(?:[-*]\s*)?Related Task-IDs\s*[:：]\s*(.+?)\s*$")
SPEC_TASK_ID_RE = re.compile(r"docs/specs/(TID-\d{8}-[a-z0-9-]+)/")

TASK_ID_MISSING = "TASK-ID-MISSING"
TASK_ID_MULTIPLE = "TASK-ID-MULTIPLE"
SPECIAL_TASK_IDS = {TASK_ID_MISSING, TASK_ID_MULTIPLE}


@dataclass(frozen=True)
class TaskIdResolution:
    task_id: str
    related_task_ids: Tuple[str, ...]
    resolution: str


def _ordered_unique(items: Iterable[str]) -> Tuple[str, ...]:
    out: List[str] = []
    for item in items:
        token = item.strip()
        if token and token not in out:
            out.append(token)
    return tuple(out)


def extract_task_ids(text: str) -> Tuple[str, ...]:
    return _ordered_unique(match.group(1).strip() for match in TASK_ID_RE.finditer(text))


def extract_task_id_tokens(text: str) -> Tuple[str, ...]:
    return _ordered_unique(match.group(0) for match in TASK_ID_TOKEN_RE.finditer(text))


def task_ids_from_changed_paths(paths: Sequence[str]) -> Tuple[str, ...]:
    ids: List[str] = []
    for path in paths:
        match = SPEC_TASK_ID_RE.search(path)
        if not match:
            continue
        task_id = match.group(1)
        if task_id not in ids:
            ids.append(task_id)
    return tuple(ids)


def related_task_ids_to_text(task_ids: Sequence[str]) -> str:
    if not task_ids:
        return "N/A（未解析）"
    return " | ".join(task_ids)


def related_task_ids_from_text(raw: str) -> Tuple[str, ...]:
    return _ordered_unique(match.group(0) for match in TASK_ID_TOKEN_RE.finditer(raw or ""))


def is_special_task_id(task_id: str) -> bool:
    return task_id in SPECIAL_TASK_IDS


def extract_declared_task_identity(text: str, commit_paths: Sequence[str] = ()) -> TaskIdResolution | None:
    declared = extract_task_ids(text)
    if not declared:
        return None

    path_ids = task_ids_from_changed_paths(commit_paths)
    explicit_task_id = declared[0]
    explicit_related: Tuple[str, ...] = ()

    related_match = RELATED_TASK_IDS_RE.search(text or "")
    if related_match:
        explicit_related = related_task_ids_from_text(related_match.group(1))

    if explicit_task_id == TASK_ID_MULTIPLE:
        related = explicit_related or path_ids or related_task_ids_from_text(text)
        return TaskIdResolution(
            task_id=TASK_ID_MULTIPLE,
            related_task_ids=related,
            resolution="commit-message-explicit-multiple",
        )

    if explicit_task_id == TASK_ID_MISSING:
        return TaskIdResolution(
            task_id=TASK_ID_MISSING,
            related_task_ids=explicit_related or path_ids,
            resolution="commit-message-explicit-missing",
        )

    return TaskIdResolution(
        task_id=explicit_task_id,
        related_task_ids=_ordered_unique([explicit_task_id, *explicit_related, *path_ids]),
        resolution="commit-message-explicit",
    )


def resolve_task_id_identity(
    root: Path,
    date_str: str,
    explicit_task_id: str,
    commit_message: str,
    commit_paths: Sequence[str],
) -> TaskIdResolution:
    path_ids = task_ids_from_changed_paths(commit_paths)
    explicit = explicit_task_id.strip()
    if explicit:
        return TaskIdResolution(
            task_id=explicit,
            related_task_ids=_ordered_unique([explicit, *path_ids]),
            resolution="explicit",
        )

    declared = extract_declared_task_identity(commit_message, commit_paths)
    if declared:
        if declared.task_id not in SPECIAL_TASK_IDS and len(path_ids) > 1:
            return TaskIdResolution(
                task_id=TASK_ID_MULTIPLE,
                related_task_ids=path_ids,
                resolution="commit-message-conflicts-with-paths",
            )
        return declared

    message_ids = extract_task_id_tokens(commit_message)
    if len(message_ids) == 1:
        return TaskIdResolution(
            task_id=message_ids[0],
            related_task_ids=_ordered_unique([message_ids[0], *path_ids]),
            resolution="commit-message",
        )
    if len(message_ids) > 1:
        return TaskIdResolution(
            task_id=TASK_ID_MULTIPLE,
            related_task_ids=message_ids,
            resolution="commit-message-ambiguous",
        )

    if len(path_ids) == 1:
        return TaskIdResolution(
            task_id=path_ids[0],
            related_task_ids=path_ids,
            resolution="changed-paths",
        )
    if len(path_ids) > 1:
        return TaskIdResolution(
            task_id=TASK_ID_MULTIPLE,
            related_task_ids=path_ids,
            resolution="changed-paths-ambiguous",
        )

    log_path = root / "docs" / "logs" / f"{date_str}.md"
    plan_path = root / "docs" / "plans" / f"{date_str}.md"
    if log_path.exists() and plan_path.exists():
        log_ids = extract_task_ids(log_path.read_text(encoding="utf-8"))
        plan_ids = set(extract_task_ids(plan_path.read_text(encoding="utf-8")))
        shared = tuple(task_id for task_id in log_ids if task_id in plan_ids)
        if len(shared) == 1:
            return TaskIdResolution(
                task_id=shared[0],
                related_task_ids=shared,
                resolution="daily-docs",
            )
        if len(shared) > 1:
            return TaskIdResolution(
                task_id=TASK_ID_MULTIPLE,
                related_task_ids=shared,
                resolution="daily-docs-ambiguous",
            )

    return TaskIdResolution(
        task_id=TASK_ID_MISSING,
        related_task_ids=(),
        resolution="unresolved",
    )
