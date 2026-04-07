#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Ensure workflow kit readiness for any repo (idempotent).

- Check required workflow assets/hooks
- Auto-heal missing parts by invoking bootstrap_workflow_repo.py
- Re-check and return non-zero if still not ready

Usage:
  python3 ~/.codex/scripts/ensure_workflow_ready.py --target . --hooks required
"""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
from pathlib import Path
from typing import Optional, Sequence, Tuple

from workflow_asset_manifest import (
    HOOK_FILES,
    READINESS_REQUIRED_FILES,
    READY_PY_COMPILE_FILES,
    VALIDATE_AGENT_CONFIG,
    VALIDATE_WORKFLOW_KIT_SYNC,
)


def _run(cmd: Sequence[str], cwd: Optional[Path] = None) -> Tuple[int, str]:
    try:
        out = subprocess.check_output(
            cmd,
            cwd=str(cwd) if cwd else None,
            stderr=subprocess.STDOUT,
            text=True,
        )
        return 0, out.strip()
    except subprocess.CalledProcessError as e:
        return e.returncode, (e.output or "").strip()


def _detect_repo_root(target: Path) -> Path:
    code, out = _run(["git", "-C", str(target), "rev-parse", "--show-toplevel"])
    if code == 0 and out:
        return Path(out).resolve()
    return target.resolve()


def _path_exists(repo_root: Path, rel: str) -> bool:
    return (repo_root / rel).exists()


def _syntax_check_without_repo_write(script_path: Path) -> Optional[str]:
    try:
        source = script_path.read_text(encoding="utf-8")
    except Exception as exc:
        return str(exc).strip()
    try:
        compile(source, str(script_path), "exec")
        return None
    except SyntaxError as exc:
        lineno = exc.lineno or "?"
        msg = exc.msg or "syntax error"
        return f"line {lineno}: {msg}"
    except Exception as exc:  # pragma: no cover - defensive fallback
        return str(exc).strip()


def _run_agent_config_validation(repo_root: Path) -> Optional[str]:
    script = repo_root / VALIDATE_AGENT_CONFIG
    if not script.exists():
        return None

    code, out = _run([sys.executable, str(script), "--repo-root", str(repo_root)])
    if code == 0:
        return None

    summary = next((line.strip() for line in out.splitlines() if line.strip()), "validation failed")
    return f"validate_agent_configs failed: {summary}"


def _collect_gaps(repo_root: Path, hooks_mode: str) -> list[str]:
    required = list(READINESS_REQUIRED_FILES)
    gaps = [p for p in required if not _path_exists(repo_root, p)]

    validation_gap = _run_agent_config_validation(repo_root)
    if validation_gap:
        gaps.append(validation_gap)

    if hooks_mode == "required":
        for hook in HOOK_FILES:
            hook_path = repo_root / hook
            if not hook_path.exists():
                gaps.append(hook)
                continue
            if not os.access(hook_path, os.X_OK):
                gaps.append(f"{hook} not executable")

        code, hooks_path = _run(["git", "-C", str(repo_root), "config", "core.hooksPath"])
        if code != 0 or hooks_path.strip() != ".githooks":
            gaps.append("git config core.hooksPath != .githooks")

    return gaps


def _collect_semantic_gaps(repo_root: Path) -> list[str]:
    gaps: list[str] = []
    for rel in READY_PY_COMPILE_FILES:
        script_path = repo_root / rel
        if not script_path.exists():
            continue
        detail = _syntax_check_without_repo_write(script_path)
        if detail:
            gaps.append(f"py_compile failed: {rel} ({detail})")

    agent_validator = repo_root / VALIDATE_AGENT_CONFIG
    if agent_validator.exists():
        code, out = _run([sys.executable, str(agent_validator), "--repo-root", str(repo_root)], cwd=repo_root)
        if code != 0:
            detail = out.splitlines()[-1] if out else "agent config validation failed"
            gaps.append(f"agent config validation failed: {detail}")

    kit_sync_validator = repo_root / VALIDATE_WORKFLOW_KIT_SYNC
    if kit_sync_validator.exists():
        code, out = _run([sys.executable, str(kit_sync_validator), "--repo-root", str(repo_root)], cwd=repo_root)
        if code != 0:
            detail = out.splitlines()[-1] if out else "workflow kit sync validation failed"
            gaps.append(f"workflow kit sync validation failed: {detail}")

    return gaps


def _call_bootstrap(script_dir: Path, repo_root: Path, hooks_mode: str, bootstrap_task: str, dry_run: bool) -> int:
    bootstrap = script_dir / "bootstrap_workflow_repo.py"
    if not bootstrap.exists():
        print(f"❌ 找不到 bootstrap 脚本：{bootstrap}")
        return 2

    cmd = [
        sys.executable,
        str(bootstrap),
        "--target",
        str(repo_root),
        "--hooks",
        hooks_mode,
        "--bootstrap-task",
        bootstrap_task,
    ]
    if dry_run:
        cmd.append("--dry-run")

    code, out = _run(cmd)
    if out:
        print(out)
    return code


def main(argv: Optional[Sequence[str]] = None) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--target", default=".", help="目标仓库路径（可在子目录执行）")
    ap.add_argument("--hooks", choices=["required", "optional", "off"], default="required")
    ap.add_argument("--bootstrap-task", choices=["auto", "on", "off"], default="off")
    ap.add_argument("--check-only", action="store_true", help="仅检查，不自动修复")
    ap.add_argument("--dry-run", action="store_true", help="修复流程仅预览")
    args = ap.parse_args(argv)

    target = Path(args.target).expanduser()
    repo_root = _detect_repo_root(target)

    print(f"📍 ensure workflow for repo: {repo_root}")
    gaps = _collect_gaps(repo_root, args.hooks)
    semantic_gaps = _collect_semantic_gaps(repo_root) if not gaps else []
    all_gaps = [*gaps, *semantic_gaps]
    if not all_gaps:
        print("✅ workflow assets are ready.")
        return 0

    print("⚠️ detected gaps:")
    for g in all_gaps:
        print(f"- {g}")

    if args.check_only:
        return 1

    print("\n== Auto-heal via bootstrap_workflow_repo.py ==")
    rc = _call_bootstrap(
        script_dir=Path(__file__).resolve().parent,
        repo_root=repo_root,
        hooks_mode=args.hooks,
        bootstrap_task=args.bootstrap_task,
        dry_run=args.dry_run,
    )
    if rc != 0:
        print(f"❌ bootstrap failed with exit code {rc}")
        return rc

    if args.dry_run:
        print("🧪 DRY-RUN completed (no files were modified).")
        return 0

    final_gaps = _collect_gaps(repo_root, args.hooks)
    final_semantic_gaps = _collect_semantic_gaps(repo_root) if not final_gaps else []
    remaining_gaps = [*final_gaps, *final_semantic_gaps]
    if remaining_gaps:
        print("❌ auto-heal finished but gaps still exist:")
        for g in remaining_gaps:
            print(f"- {g}")
        return 3

    print("✅ workflow ready after auto-heal.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
