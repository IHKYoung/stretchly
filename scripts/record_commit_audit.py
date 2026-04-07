#!/usr/bin/env python3
"""Append commit/merge audit records into docs/commits/YYYY-MM-DD.md."""

from __future__ import annotations

import argparse
import datetime as dt
import subprocess
import sys
from pathlib import Path
from typing import Iterable, List, Sequence, Set, Tuple

from commit_audit_lib import (
    TASK_ID_MISSING,
    TASK_ID_MULTIPLE,
    TASK_ID_TOKEN_RE,
    is_special_task_id,
    related_task_ids_to_text,
    resolve_task_id_identity,
)


def run_git(args: Iterable[str]) -> str:
    result = subprocess.run(
        ["git", *args],
        text=True,
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        message = (result.stderr or result.stdout or "git command failed").strip()
        raise RuntimeError(message)
    return (result.stdout or "").strip()


def repo_root() -> Path:
    return Path(run_git(["rev-parse", "--show-toplevel"]))


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Record commit audit entries.")
    parser.add_argument(
        "--type",
        choices=("auto", "normal", "merge"),
        default="auto",
        help="Audit record type.",
    )
    parser.add_argument(
        "--commit",
        default="HEAD",
        help="Commit reference to record (default: HEAD).",
    )
    parser.add_argument(
        "--task-id",
        default="",
        help="Explicit Task-ID. If omitted, script resolves from logs/plans.",
    )
    parser.add_argument("--source-branch", default="", help="Source branch for merge records.")
    parser.add_argument("--target-branch", default="", help="Target branch for merge records.")
    parser.add_argument(
        "--conflict-resolution",
        default="",
        help="Conflict resolution summary for merge records.",
    )
    parser.add_argument(
        "--only-merge",
        action="store_true",
        help="Skip when target commit is not a merge commit.",
    )
    return parser.parse_args()


def commit_hash(commit_ref: str) -> str:
    return run_git(["rev-parse", commit_ref]).strip()


def commit_date(commit_ref: str) -> str:
    return run_git(["show", "-s", "--format=%cs", commit_ref]).strip()


def commit_subject(commit_ref: str) -> str:
    return run_git(["show", "-s", "--format=%s", commit_ref]).strip()


def commit_body(commit_ref: str) -> str:
    return run_git(["show", "-s", "--format=%b", commit_ref]).strip()


def current_branch() -> str:
    return run_git(["rev-parse", "--abbrev-ref", "HEAD"]).strip()


def commit_parent_hashes(commit_ref: str) -> List[str]:
    line = run_git(["rev-list", "--parents", "-n", "1", commit_ref]).strip()
    if not line:
        return []
    parts = line.split()
    return parts[1:]


def is_merge_commit(commit_ref: str) -> bool:
    return len(commit_parent_hashes(commit_ref)) >= 2


def changed_paths_in_commit(commit_ref: str) -> List[str]:
    output = run_git(["show", "--pretty=format:", "--name-only", commit_ref])
    return [line.strip().replace("\\", "/").lstrip("./") for line in output.splitlines() if line.strip()]


def is_commit_audit_only_commit(paths: Sequence[str]) -> bool:
    if not paths:
        return False
    return all(path.startswith("docs/commits/") for path in paths)


def ensure_commit_file(root: Path, date_str: str) -> Path:
    commits_dir = root / "docs" / "commits"
    commits_dir.mkdir(parents=True, exist_ok=True)
    file_path = commits_dir / f"{date_str}.md"
    if not file_path.exists():
        file_path.write_text(f"## {date_str}\n\n", encoding="utf-8")
    return file_path


def detect_merge_branches(subject: str) -> Tuple[str, str]:
    match = re.search(r"Merge branch '([^']+)'(?: into ([^ ]+))?", subject)
    if match:
        source = match.group(1).strip()
        target = (match.group(2) or "").strip()
        return source, target
    match = re.search(r"Merge remote-tracking branch '([^']+)'", subject)
    if match:
        return match.group(1).strip(), ""
    return "", ""


def compose_normal_entry(
    *,
    date_str: str,
    task_id: str,
    related_task_ids: Sequence[str],
    task_id_resolution: str,
    full_hash: str,
    subject: str,
    branch: str,
) -> str:
    spec_readme = f"docs/specs/{task_id}/README.md" if TASK_ID_TOKEN_RE.fullmatch(task_id) else "N/A（Task-ID 未解析）"
    spec_plan = f"docs/specs/{task_id}/plan.md" if TASK_ID_TOKEN_RE.fullmatch(task_id) else "N/A（Task-ID 未解析）"
    return "\n".join(
        [
            f"### 自动记录：{subject}",
            f"- Timestamp: {dt.datetime.now().isoformat(timespec='seconds')}",
            f"- Task-ID: {task_id}",
            f"- Related Task-IDs: {related_task_ids_to_text(related_task_ids)}",
            f"- Task-ID Resolution: {task_id_resolution}",
            "- Type: normal",
            f"- Commit Hash: {full_hash}",
            f"- Branch: {branch}",
            f"- Subject: {subject}",
            f"- Change Summary: 自动记录普通 commit，详见提交主题与代码差异。",
            f"- Basis Trace: {spec_plan} | docs/plans/{date_str}.md",
            f"- Execution Trace: docs/logs/{date_str}.md",
            f"- Verification: 自动记录；验证详情见 docs/logs/{date_str}.md。",
            f"- Verification Trace: docs/logs/{date_str}.md",
            f"- Links: {spec_readme} | docs/logs/{date_str}.md | docs/plans/{date_str}.md",
            "",
        ]
    )


def compose_merge_entry(
    *,
    date_str: str,
    task_id: str,
    related_task_ids: Sequence[str],
    task_id_resolution: str,
    full_hash: str,
    subject: str,
    source_branch: str,
    target_branch: str,
    conflict_resolution: str,
) -> str:
    spec_readme = f"docs/specs/{task_id}/README.md" if TASK_ID_TOKEN_RE.fullmatch(task_id) else "N/A（Task-ID 未解析）"
    spec_plan = f"docs/specs/{task_id}/plan.md" if TASK_ID_TOKEN_RE.fullmatch(task_id) else "N/A（Task-ID 未解析）"
    resolution = conflict_resolution.strip() or "自动记录：无冲突或未提供冲突说明。"
    source = source_branch.strip() or "UNKNOWN"
    target = target_branch.strip() or "UNKNOWN"
    return "\n".join(
        [
            f"### 自动记录：{subject}",
            f"- Timestamp: {dt.datetime.now().isoformat(timespec='seconds')}",
            f"- Task-ID: {task_id}",
            f"- Related Task-IDs: {related_task_ids_to_text(related_task_ids)}",
            f"- Task-ID Resolution: {task_id_resolution}",
            "- Type: merge",
            f"- Merge Commit Hash: {full_hash}",
            f"- Source Branch: {source}",
            f"- Target Branch: {target}",
            f"- Subject: {subject}",
            f"- Conflict Resolution: {resolution}",
            f"- Basis Trace: {spec_plan} | docs/plans/{date_str}.md",
            f"- Execution Trace: docs/logs/{date_str}.md",
            f"- Verification: 自动记录；合并验证详情见 docs/logs/{date_str}.md。",
            f"- Verification Trace: docs/logs/{date_str}.md",
            f"- Links: {spec_readme} | docs/logs/{date_str}.md | docs/plans/{date_str}.md",
            "",
        ]
    )


def append_if_missing(file_path: Path, entry: str, full_hash: str) -> bool:
    text = file_path.read_text(encoding="utf-8")
    short_hash = full_hash[:8]
    if full_hash in text or short_hash in text:
        return False

    placeholder = "> 当前尚未发生普通 commit 或 merge commit；后续发生时按 `docs/commits/README.md` 模板追加记录。"
    cleaned = text.replace(placeholder + "\n", "").replace(placeholder, "")
    if cleaned != text:
        file_path.write_text(cleaned.rstrip() + "\n\n", encoding="utf-8")
        text = file_path.read_text(encoding="utf-8")

    with file_path.open("a", encoding="utf-8") as handle:
        if not text.endswith("\n"):
            handle.write("\n")
        handle.write(entry)
    return True


def main() -> int:
    args = parse_args()
    try:
        root = repo_root()
        full_hash = commit_hash(args.commit)
        date_str = commit_date(args.commit)
        subject = commit_subject(args.commit)
        body = commit_body(args.commit)
        merge_commit = is_merge_commit(args.commit)

        if args.only_merge and not merge_commit:
            print("skip: target commit is not a merge commit.")
            return 0

        commit_paths = changed_paths_in_commit(args.commit)
        if is_commit_audit_only_commit(commit_paths):
            print("skip: commit only changes docs/commits.")
            return 0

        record_type = args.type
        if record_type == "auto":
            record_type = "merge" if merge_commit else "normal"
        if record_type == "merge" and not merge_commit:
            print("skip: requested merge record but commit is not merge.")
            return 0

        commit_message = f"{subject}\n{body}".strip()
        resolution = resolve_task_id_identity(root, date_str, args.task_id, commit_message, commit_paths)
        task_id = resolution.task_id
        if task_id == TASK_ID_MISSING:
            print(
                "warning: Task-ID not found in docs/logs or docs/plans for this date.",
                file=sys.stderr,
            )
        elif task_id == TASK_ID_MULTIPLE:
            print(
                f"warning: multiple Task-ID candidates found ({related_task_ids_to_text(resolution.related_task_ids)}); "
                "please pass --task-id explicitly when you want a single owner task.",
                file=sys.stderr,
            )

        commit_file = ensure_commit_file(root, date_str)

        if record_type == "normal":
            entry = compose_normal_entry(
                date_str=date_str,
                task_id=task_id,
                related_task_ids=resolution.related_task_ids,
                task_id_resolution=resolution.resolution,
                full_hash=full_hash,
                subject=subject,
                branch=current_branch(),
            )
        else:
            detected_source, detected_target = detect_merge_branches(subject)
            entry = compose_merge_entry(
                date_str=date_str,
                task_id=task_id,
                related_task_ids=resolution.related_task_ids,
                task_id_resolution=resolution.resolution,
                full_hash=full_hash,
                subject=subject,
                source_branch=args.source_branch or detected_source,
                target_branch=args.target_branch or detected_target or current_branch(),
                conflict_resolution=args.conflict_resolution,
            )

        appended = append_if_missing(commit_file, entry, full_hash)
        if appended:
            print(f"recorded: {commit_file}")
        else:
            print(f"skip: hash already recorded in {commit_file}")
        return 0
    except Exception as error:  # pragma: no cover - defensive CLI guard
        print(f"record failed: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
