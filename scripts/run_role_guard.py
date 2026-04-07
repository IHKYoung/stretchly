#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""Run a command, then validate any repo file writes against a role's write scope."""

from __future__ import annotations

import argparse
import hashlib
import os
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Sequence, Tuple


def _detect_repo_root() -> Path:
    try:
        out = subprocess.check_output(
            ["git", "rev-parse", "--show-toplevel"],
            stderr=subprocess.STDOUT,
            text=True,
        ).strip()
        if out:
            return Path(out).resolve()
    except Exception:
        pass
    return Path.cwd().resolve()


def _normalize_path(path: Path, repo_root: Path) -> str:
    return str(path.relative_to(repo_root)).replace("\\", "/")


def _validator_path() -> Path:
    return Path(__file__).with_name("validate_role_file_scope.py")


def _file_fingerprint(path: Path) -> str:
    if path.is_symlink():
        return f"symlink:{os.readlink(path)}"
    hasher = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            hasher.update(chunk)
    return hasher.hexdigest()


def _collect_snapshot(repo_root: Path) -> Dict[str, str]:
    snapshot: Dict[str, str] = {}
    for root, dirs, files in os.walk(repo_root):
        dirs[:] = sorted(d for d in dirs if d != ".git")
        current_root = Path(root)
        for filename in sorted(files):
            path = current_root / filename
            rel = _normalize_path(path, repo_root)
            snapshot[rel] = _file_fingerprint(path)
    return snapshot


def _touched_paths(before: Dict[str, str], after: Dict[str, str]) -> Tuple[str, ...]:
    touched: List[str] = []
    for path in sorted(set(before) | set(after)):
        if before.get(path) != after.get(path):
            touched.append(path)
    return tuple(touched)


def _run_command(command: Sequence[str], repo_root: Path) -> int:
    env = os.environ.copy()
    env.setdefault("PYTHONDONTWRITEBYTECODE", "1")
    result = subprocess.run(list(command), cwd=str(repo_root), env=env, check=False)
    return int(result.returncode)


def _run_scope_validator(
    *,
    repo_root: Path,
    role: str,
    paths: Sequence[str],
    extra_allow: Sequence[str],
) -> subprocess.CompletedProcess[str]:
    validator = _validator_path()
    cmd = [sys.executable, str(validator), "--role", role, "--paths", *paths]
    for pattern in extra_allow:
        cmd.extend(["--extra-allow", pattern])
    return subprocess.run(
        cmd,
        cwd=str(repo_root),
        text=True,
        capture_output=True,
        check=False,
    )


def parse_args(argv: Sequence[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Run a command, then enforce role write scope.")
    parser.add_argument(
        "--role",
        required=True,
        choices=[
            "orchestrator",
            "ui_designer",
            "architect",
            "coder",
            "tester",
            "scribe",
            "evidence_collector",
            "reality_checker",
        ],
    )
    parser.add_argument("--repo-root", default="", help="仓库根目录（默认自动探测）")
    parser.add_argument(
        "--extra-allow",
        action="append",
        default=[],
        help="额外允许的路径 glob（可重复传入）",
    )
    parser.add_argument("command", nargs=argparse.REMAINDER, help="要执行的命令；需用 `--` 分隔")
    args = parser.parse_args(argv)
    if args.command and args.command[0] == "--":
        args.command = args.command[1:]
    if not args.command:
        parser.error("缺少要执行的命令，请使用 `-- <command ...>` 传入。")
    return args


def main(argv: Sequence[str] | None = None) -> int:
    args = parse_args(argv)
    repo_root = Path(args.repo_root).resolve() if args.repo_root else _detect_repo_root()
    if not repo_root.exists() or not repo_root.is_dir():
        print(f"❌ repo root not found or not a directory: {repo_root}", file=sys.stderr)
        return 2
    validator = _validator_path()
    if not validator.exists() or not validator.is_file():
        print(f"❌ role scope validator missing: {validator}", file=sys.stderr)
        return 2
    before = _collect_snapshot(repo_root)
    exit_code = _run_command(args.command, repo_root)
    after = _collect_snapshot(repo_root)
    touched = _touched_paths(before, after)

    print(f"[role-guard] role={args.role}")
    print(f"[role-guard] command={' '.join(args.command)}")
    print(f"[role-guard] command_exit={exit_code}")
    print(f"[role-guard] touched_paths={len(touched)}")
    for path in touched:
        print(f"  - {path}")

    validation = _run_scope_validator(
        repo_root=repo_root,
        role=args.role,
        paths=touched,
        extra_allow=args.extra_allow,
    )
    if validation.stdout:
        print(validation.stdout.rstrip())
    if validation.stderr:
        print(validation.stderr.rstrip(), file=sys.stderr)

    if validation.returncode != 0:
        print("❌ role guard detected out-of-scope file writes", file=sys.stderr)
        return 1
    if exit_code != 0:
        return exit_code

    print("[OK] role guard passed ✅")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
