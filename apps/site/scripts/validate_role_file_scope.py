#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""Validate whether a file list stays inside a role's allowed write scope."""

from __future__ import annotations

import argparse
import fnmatch
import subprocess
from pathlib import Path
from typing import Iterable, List, Optional, Sequence, Tuple

from workflow_asset_manifest import KIT_MIRRORED_ASSETS, WORKFLOW_DOCS_ALLOWLIST


READ_ONLY_ROLES = {"ui_designer", "architect", "reality_checker"}
TEST_PREFIXES = ("tests/", "__tests__/", "e2e/", "testutils/", "fixtures/")
TEST_CONFIG_GLOBS = (
    "conftest.py",
    "*/conftest.py",
    "pytest.ini",
    "tox.ini",
    "nose2.cfg",
    ".coveragerc",
    "playwright.config.*",
    "vitest.config.*",
    "jest.config.*",
    "cypress.config.*",
    "wdio.conf.*",
    "karma.conf.*",
    "ava.config.*",
    "phpunit.xml",
    "phpunit.xml.dist",
)
EVIDENCE_PREFIX = "docs/specs/"
SCRIBE_ROOT_DOCS = {"AGENTS.md", "AGENTS.local.md", "README.md", "IfYouNeed.md"}
SCRIBE_KIT_ASSETS = {f"kit/{path}" for path in KIT_MIRRORED_ASSETS}


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


def _run_git(repo_root: Path, args: Sequence[str]) -> str:
    out = subprocess.check_output(
        ["git", *args],
        cwd=str(repo_root),
        stderr=subprocess.STDOUT,
        text=True,
    )
    return out.strip()


def _normalize_paths(paths: Iterable[str]) -> Tuple[str, ...]:
    out: List[str] = []
    for path in paths:
        normalized = path.strip().replace("\\", "/").lstrip("./")
        if normalized and normalized not in out:
            out.append(normalized)
    return tuple(out)


def _paths_from_git_diff(repo_root: Path, cached: bool, revspec: str) -> Tuple[str, ...]:
    args = ["diff", "--name-only"]
    if cached:
        args.insert(1, "--cached")
    if revspec:
        args.append(revspec)
    try:
        out = _run_git(repo_root, args)
    except Exception:
        return ()
    return _normalize_paths(out.splitlines())


def _allow_extra(path: str, extra_globs: Sequence[str]) -> bool:
    return any(fnmatch.fnmatch(path, pattern) for pattern in extra_globs)


def _is_tester_path(path: str) -> bool:
    if path.startswith(TEST_PREFIXES):
        return True
    return any(fnmatch.fnmatch(path, pattern) for pattern in TEST_CONFIG_GLOBS)


def _is_scribe_path(path: str) -> bool:
    if path.startswith("docs/") or path.startswith("kb/") or path.startswith(".githooks/"):
        return True
    if path in SCRIBE_ROOT_DOCS:
        return True
    if path in SCRIBE_KIT_ASSETS:
        return True
    return path in WORKFLOW_DOCS_ALLOWLIST


def _is_evidence_path(path: str) -> bool:
    if not path.startswith(EVIDENCE_PREFIX):
        return False
    parts = path.split("/")
    if len(parts) < 5:
        return False
    return parts[1] == "specs" and parts[3] == "evidence"


def _is_allowed(role: str, path: str, extra_globs: Sequence[str]) -> bool:
    if _allow_extra(path, extra_globs):
        return True
    if role in READ_ONLY_ROLES:
        return False
    if role == "tester":
        return _is_tester_path(path)
    if role == "scribe":
        return _is_scribe_path(path)
    if role == "evidence_collector":
        return _is_evidence_path(path)
    return True


def _describe_scope(role: str) -> str:
    if role == "tester":
        return "tests/, __tests__/, e2e/, testutils/, fixtures/ 与必要测试配置文件"
    if role == "scribe":
        return "docs/**, kb/**, 根目录文档（如 README.md / AGENTS.md）, kit/ 中的 workflow 镜像资产, .githooks/**, scripts/(workflow-related allowlist)"
    if role == "evidence_collector":
        return "docs/specs/<Task-ID>/evidence/**"
    if role in READ_ONLY_ROLES:
        return "read-only（不允许任何写入）"
    return "无额外文件范围限制"


def main(argv: Optional[Sequence[str]] = None) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument(
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
    ap.add_argument("--repo-root", default="", help="仓库根目录（默认自动探测）")
    ap.add_argument("--paths", nargs="*", default=[], help="显式传入要校验的路径列表")
    ap.add_argument("--read-stdin", action="store_true", help="从 stdin 按行读取路径")
    ap.add_argument("--git-diff", action="store_true", help="从 git diff --name-only 读取路径")
    ap.add_argument("--cached", action="store_true", help="与 --git-diff 配合，读取暂存区 diff")
    ap.add_argument("--revspec", default="", help="与 --git-diff 配合，指定 revspec")
    ap.add_argument(
        "--extra-allow",
        action="append",
        default=[],
        help="额外允许的路径 glob（可重复传入）",
    )
    args = ap.parse_args(argv)

    repo_root = Path(args.repo_root).resolve() if args.repo_root else _detect_repo_root()
    paths: List[str] = list(args.paths)
    if args.read_stdin:
        import sys

        paths.extend(line.rstrip("\n") for line in sys.stdin)
    if args.git_diff:
        paths.extend(_paths_from_git_diff(repo_root, cached=args.cached, revspec=args.revspec))

    normalized_paths = _normalize_paths(paths)
    if not normalized_paths:
        print(f"[OK] role file scope validation passed for `{args.role}`: no paths provided ✅")
        return 0

    invalid = [path for path in normalized_paths if not _is_allowed(args.role, path, args.extra_allow)]
    if invalid:
        print("❌ Role file scope validation FAILED\n")
        print(f"- Role: {args.role}")
        print(f"- Allowed scope: {_describe_scope(args.role)}")
        print("- Invalid paths:")
        for path in invalid:
            print(f"  - {path}")
        if args.extra_allow:
            print(f"- Extra allow globs: {', '.join(args.extra_allow)}")
        return 1

    print(f"[OK] role file scope validation passed for `{args.role}` ✅")
    print(f"- Allowed scope: {_describe_scope(args.role)}")
    print(f"- Paths checked: {len(normalized_paths)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
