#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Scaffold a Task skeleton:
- Creates docs/specs/<Task-ID>/ from docs/specs/_template (placeholder substitution)
- Appends skeleton entries to docs/plans/YYYY-MM-DD.md and docs/logs/YYYY-MM-DD.md

Usage:
  python3 scripts/scaffold_task.py \
    --task-id TID-20260305-workflow-bootstrap \
    --title "初始化工作流" \
    --level trivial --lane fast --execution-profile single-task --roles "scribe"
"""

from __future__ import annotations

import argparse
import datetime as dt
import re
import sys
from pathlib import Path
from typing import Dict, Optional, Sequence

DEFAULT_TZ = "Asia/Taipei"
TASK_ID_RE = re.compile(r"^TID-\d{8}-[a-z0-9-]+$")

def _now_date_iso(tz_name: str = DEFAULT_TZ) -> str:
    try:
        from zoneinfo import ZoneInfo
        return dt.datetime.now(ZoneInfo(tz_name)).date().isoformat()
    except Exception:
        return dt.date.today().isoformat()

def _ensure_dir(p: Path) -> None:
    p.mkdir(parents=True, exist_ok=True)

def _read_text(p: Path) -> str:
    return p.read_text(encoding="utf-8")

def _write_text(p: Path, s: str) -> None:
    p.write_text(s, encoding="utf-8")

def _append_text(p: Path, s: str) -> None:
    if p.exists():
        with p.open("a", encoding="utf-8") as f:
            if not s.startswith("\n"):
                f.write("\n")
            f.write(s)
            if not s.endswith("\n"):
                f.write("\n")
    else:
        _ensure_dir(p.parent)
        _write_text(p, s if s.endswith("\n") else s + "\n")


def _has_task_entry(p: Path, task_id: str) -> bool:
    if not p.exists():
        return False
    text = _read_text(p)
    pattern = re.compile(
        rf"(?mi)^\s*(?:[-*]\s*)?(?:#+\s*)?Task-ID\s*[:：]\s*{re.escape(task_id)}\s*$"
    )
    return pattern.search(text) is not None

def _substitute(template: str, mapping: Dict[str, str]) -> str:
    out = template
    for k, v in mapping.items():
        out = out.replace("{{" + k + "}}", v)
    return out

def _copy_templates(template_dir: Path, out_dir: Path, mapping: Dict[str, str]) -> None:
    if not template_dir.exists():
        raise RuntimeError(f"Template dir not found: {template_dir}")
    _ensure_dir(out_dir)
    for src in sorted(template_dir.glob("*.md")):
        content = _substitute(_read_text(src), mapping)
        _write_text(out_dir / src.name, content)

def _make_plan_entry(mapping: Dict[str, str]) -> str:
    tid = mapping["TASK_ID"]
    title = mapping["TITLE"]
    date = mapping["DATE"]
    roles = mapping["REQUIRED_ROLES"]
    execution_profile = mapping["EXECUTION_PROFILE"]

    return f"""## {tid} · {title}

- Task-ID: {tid}
- Goal: (TBD)
- Requirement Brief: (TBD)
- Scope: (TBD)
- Source Basis: (TBD)
- Level: {mapping["LEVEL"]}
- Lane: {mapping["LANE"]}
- Execution Profile: {execution_profile}
- Execution Safety Block: (TBD)
- Status: INIT
- Links:
  - Specs: ../specs/{tid}/README.md
  - Logs: ../logs/{date}.md
- Knowledge Capture: (TBD)
- Knowledge Route: none（如无沉淀，请写原因）
- Execution Mode: (TBD)
- Fallback Scope: N/A（仅 fallback 时填写）
- Fallback Reason Code: N/A（仅 fallback 时填写）
- Warmup Required Roles: {roles}
- Warmup Ready Roles: (TBD)
- Warmup Agent IDs: (TBD)
- Warmup Verification: (TBD)
- phase_stop_conditions: N/A（仅 sequential-phases 时填写）

### Plan Checklist
- [ ] 形成 Requirement Brief
- [ ] 明确 Execution Safety Block
- [ ] 产出/落盘 specs（arch/ui/plan/testplan）
- [ ] 明确验收标准（AC）
- [ ] 明确风险与回滚
- [ ] （如需）等待用户 Approved
"""

def _make_log_entry(mapping: Dict[str, str]) -> str:
    tid = mapping["TASK_ID"]
    title = mapping["TITLE"]
    roles = mapping["REQUIRED_ROLES"]
    execution_profile = mapping["EXECUTION_PROFILE"]

    return f"""## {tid} · {title}

- Task-ID: {tid}
- Goal: (TBD)
- Requirement Brief: (TBD)
- Assumptions: (TBD)
- Source Basis: (TBD)
- Level: {mapping["LEVEL"]}
- Lane: {mapping["LANE"]}
- Execution Profile: {execution_profile}
- Execution Safety Block: (TBD)
- Change Summary: (TBD)
- Verification: (TBD)
- Knowledge Capture: (TBD)
- Knowledge Route: none（如无沉淀，请写原因）
- Execution Mode: (TBD)
- Fallback Scope: N/A（仅 fallback 时填写）
- Fallback Reason Code: N/A（仅 fallback 时填写）
- Warmup Required Roles: {roles}
- Warmup Ready Roles: (TBD)
- Warmup Agent IDs: (TBD)
- Warmup Verification: (TBD)
- Risk & Rollback: (TBD)
- Retention Review: (TBD)
- Retention Decision: (TBD)

### Execution Checklist
- [ ] 初始化/预热完成并记录回执
- [ ] 实现/改动完成
- [ ] 测试与验证通过（含证据）
- [ ] 更新 CHANGELOG（如含代码/行为变更）
- [ ] 结案汇报
"""

def main(argv: Optional[Sequence[str]] = None) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--task-id", required=True, help="TID-YYYYMMDD-<slug>")
    ap.add_argument("--title", required=True, help="任务标题")
    ap.add_argument("--date", default=None, help="YYYY-MM-DD（默认 Asia/Taipei 今日）")
    ap.add_argument("--tz", default=DEFAULT_TZ)
    ap.add_argument("--level", choices=["trivial", "moderate", "complex"], default="trivial")
    ap.add_argument("--lane", choices=["fast", "deep"], default="fast")
    ap.add_argument(
        "--execution-profile",
        choices=["single-task", "sequential-phases", "merge-gate"],
        default="single-task",
    )
    ap.add_argument("--roles", default="scribe", help="逗号分隔 Required Roles，如 scribe,coder,tester")
    ap.add_argument("--repo-root", default=".", help="仓库根目录（默认当前目录）")
    ap.add_argument(
        "--force-overwrite-specs",
        action="store_true",
        help="若 docs/specs/<Task-ID> 已存在，强制覆盖模板文件",
    )
    args = ap.parse_args(argv)

    tid = args.task_id.strip()
    if not TASK_ID_RE.match(tid):
        print(f"Invalid Task-ID: {tid}", file=sys.stderr)
        return 2

    repo_root = Path(args.repo_root).resolve()
    date_iso = args.date or _now_date_iso(args.tz)

    template_dir = repo_root / "docs" / "specs" / "_template"
    out_dir = repo_root / "docs" / "specs" / tid

    mapping = {
        "TASK_ID": tid,
        "DATE": date_iso,
        "TITLE": args.title.strip(),
        "LEVEL": args.level,
        "LANE": args.lane,
        "EXECUTION_PROFILE": args.execution_profile,
        "REQUIRED_ROLES": args.roles.strip(),
        "APPROVAL_NEEDED": "no" if args.level == "trivial" else "yes",
        "APPROVED_STATE": "N/A（trivial 默认直行；如需审批请手动填写）" if args.level == "trivial" else "(TBD)",
    }

    spec_written = False
    if out_dir.exists() and any(out_dir.glob("*.md")) and not args.force_overwrite_specs:
        print(f"[SKIP] Spec dir already exists (use --force-overwrite-specs to overwrite): {out_dir}")
    else:
        if out_dir.exists():
            print(f"[WARN] Spec dir exists; template files will be overwritten: {out_dir}")
        _copy_templates(template_dir, out_dir, mapping)
        spec_written = True

    plans_path = repo_root / "docs" / "plans" / f"{date_iso}.md"
    logs_path = repo_root / "docs" / "logs" / f"{date_iso}.md"

    plan_written = False
    if _has_task_entry(plans_path, tid):
        print(f"[SKIP] plans already contains Task-ID {tid}: {plans_path}")
    else:
        _append_text(plans_path, _make_plan_entry(mapping))
        plan_written = True

    log_written = False
    if _has_task_entry(logs_path, tid):
        print(f"[SKIP] logs already contains Task-ID {tid}: {logs_path}")
    else:
        _append_text(logs_path, _make_log_entry(mapping))
        log_written = True

    if not any((spec_written, plan_written, log_written)):
        print(f"[OK] {tid} already scaffolded; no changes made.")
    else:
        print(f"[OK] Scaffolded {tid} ✅")
    print(f"- specs: {out_dir}")
    print(f"- plans: {plans_path}")
    print(f"- logs : {logs_path}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
