#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Validate that workflow assets stay in sync with kit/ mirrors.

- Only becomes active in repos that actually contain kit/
- Downstream business repos without kit/ should skip explicitly
"""

from __future__ import annotations

import argparse
import hashlib
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import List, Optional, Sequence

from workflow_asset_manifest import KIT_MIRRORED_ASSETS


@dataclass(frozen=True)
class ValidationError:
    where: str
    message: str


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


def _sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def _validate_sync(repo_root: Path) -> List[ValidationError]:
    errors: List[ValidationError] = []
    for rel in KIT_MIRRORED_ASSETS:
        root_path = repo_root / rel
        kit_path = repo_root / "kit" / rel

        if not root_path.exists():
            errors.append(ValidationError(rel, f"根仓库缺少镜像源文件：{root_path}"))
            continue
        if not kit_path.exists():
            errors.append(ValidationError(rel, f"kit 缺少镜像文件：{kit_path}"))
            continue

        if _sha256(root_path) != _sha256(kit_path):
            errors.append(
                ValidationError(
                    rel,
                    f"根仓库与 kit 镜像内容不一致：{root_path} != {kit_path}",
                )
            )

    return errors


def _print_errors(errors: Sequence[ValidationError]) -> None:
    print("❌ Workflow kit sync validation FAILED\n")
    for err in errors:
        print(f"- [{err.where}] {err.message}")
    print("\n建议：")
    print("- 修改 root 版本后同步更新 kit 镜像")
    print("- 提交前重新运行 `python3 scripts/validate_workflow_kit_sync.py`")


def main(argv: Optional[Sequence[str]] = None) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo-root", default="", help="仓库根目录（默认自动探测）")
    args = ap.parse_args(argv)

    repo_root = Path(args.repo_root).resolve() if args.repo_root else _detect_repo_root()
    kit_root = repo_root / "kit"
    if not kit_root.exists():
        print("[SKIP] Workflow kit sync validation skipped: no kit directory detected ✅")
        return 0

    errors = _validate_sync(repo_root)
    if errors:
        _print_errors(errors)
        return 1

    print("[OK] Workflow kit sync validation passed ✅")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
