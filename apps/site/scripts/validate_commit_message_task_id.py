#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""Validate commit message Task-ID fields against staged task-scoped paths."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path
from typing import Iterable, List, Sequence

from commit_audit_lib import (
    TASK_ID_MULTIPLE,
    TASK_ID_TOKEN_RE,
    extract_declared_task_identity,
    related_task_ids_to_text,
    task_ids_from_changed_paths,
)


def _run_git(args: Iterable[str]) -> str:
    result = subprocess.run(["git", *args], text=True, capture_output=True, check=False)
    if result.returncode != 0:
        raise RuntimeError((result.stderr or result.stdout or "git command failed").strip())
    return (result.stdout or "").strip()


def _repo_root() -> Path:
    return Path(_run_git(["rev-parse", "--show-toplevel"]))


def _cached_paths() -> List[str]:
    output = _run_git(["diff", "--cached", "--name-only"])
    return [line.strip().replace("\\", "/").lstrip("./") for line in output.splitlines() if line.strip()]


def _validate_message_against_paths(commit_message: str, paths: Sequence[str]) -> List[str]:
    errors: List[str] = []
    staged_task_ids = task_ids_from_changed_paths(paths)
    declared = extract_declared_task_identity(commit_message, ())

    if len(staged_task_ids) == 1:
        expected = staged_task_ids[0]
        if not declared:
            errors.append(f"staged changes 命中了唯一 Task-ID `{expected}`，commit message 必须声明同一 `Task-ID`。")
            return errors
        if declared.task_id != expected:
            errors.append(f"commit message 的 `Task-ID` 为 `{declared.task_id}`，但 staged changes 命中的唯一 Task-ID 为 `{expected}`。")
        if declared.related_task_ids and declared.related_task_ids != (expected,):
            errors.append("单任务提交不应声明额外的 `Related Task-IDs`。")
        return errors

    if len(staged_task_ids) > 1:
        if not declared:
            errors.append(
                "staged changes 同时命中多个 Task-ID，commit message 必须显式声明 `Task-ID: TASK-ID-MULTIPLE` 与完整的 `Related Task-IDs`。"
            )
            return errors
        if declared.task_id != TASK_ID_MULTIPLE:
            errors.append(
                "staged changes 同时命中多个 Task-ID，commit message 必须使用 `Task-ID: TASK-ID-MULTIPLE`，不能伪装成唯一任务。"
            )
        if declared.related_task_ids != staged_task_ids:
            errors.append(
                "`Related Task-IDs` 与 staged changes 命中的任务集合不一致："
                f" 记录为 `{related_task_ids_to_text(declared.related_task_ids)}`，"
                f"应为 `{related_task_ids_to_text(staged_task_ids)}`。"
            )
        return errors

    if declared and declared.task_id == TASK_ID_MULTIPLE:
        errors.append("当前 staged changes 未检测到多个 Task-ID，不应声明 `TASK-ID-MULTIPLE`。")
    if declared and declared.task_id and not (
        declared.task_id == TASK_ID_MULTIPLE or TASK_ID_TOKEN_RE.fullmatch(declared.task_id)
    ):
        errors.append(f"`Task-ID` 格式无效：{declared.task_id}")
    return errors


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate commit message Task-ID fields against staged task ids.")
    parser.add_argument("--commit-msg-file", required=True, help="Path to the git commit message file.")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        root = _repo_root()
        commit_msg_path = Path(args.commit_msg_file)
        if not commit_msg_path.is_absolute():
            commit_msg_path = (root / commit_msg_path).resolve()
        message = commit_msg_path.read_text(encoding="utf-8")
        errors = _validate_message_against_paths(message, _cached_paths())
        if errors:
            print("❌ Commit message Task-ID validation FAILED", file=sys.stderr)
            print("", file=sys.stderr)
            for item in errors:
                print(f"- {item}", file=sys.stderr)
            return 1
        print("[OK] commit message Task-ID validation passed ✅")
        return 0
    except Exception as exc:  # pragma: no cover
        print(f"❌ Commit message Task-ID validation failed: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
