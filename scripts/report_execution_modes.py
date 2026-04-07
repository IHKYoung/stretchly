#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""Summarize execution mode and fallback reason distribution for a given day."""

from __future__ import annotations

import argparse
import datetime as dt
import re
import sys
from collections import Counter
from pathlib import Path
from typing import Dict, List

TASK_ID_RE = re.compile(r"(?m)^\s*-\s*Task-ID\s*[:：]\s*(TID-\d{8}-[a-z0-9-]+)\s*$")
FIELD_RE = re.compile(r"(?m)^\s*-\s*(Execution Mode|Fallback Reason Code|Goal)\s*[:：]\s*(.+?)\s*$")


def _repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def _split_log_blocks(text: str) -> Dict[str, str]:
    matches = list(TASK_ID_RE.finditer(text))
    blocks: Dict[str, str] = {}
    for i, match in enumerate(matches):
        start = match.start()
        end = matches[i + 1].start() if i + 1 < len(matches) else len(text)
        blocks[match.group(1)] = text[start:end].strip()
    return blocks


def _field_value(block: str, field: str) -> str:
    for match in FIELD_RE.finditer(block):
        if match.group(1) == field:
            return match.group(2).strip()
    return ""


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Report execution modes and fallback reason codes for one date.")
    parser.add_argument("--date", default=dt.date.today().isoformat(), help="Target date in YYYY-MM-DD format.")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    repo_root = _repo_root()
    log_path = repo_root / "docs" / "logs" / f"{args.date}.md"
    if not log_path.exists():
        print(f"❌ log file not found: {log_path}", file=sys.stderr)
        return 1

    blocks = _split_log_blocks(log_path.read_text(encoding="utf-8"))
    mode_counter: Counter[str] = Counter()
    fallback_reason_counter: Counter[str] = Counter()
    fallback_tasks: List[str] = []

    for tid, block in blocks.items():
        mode = _field_value(block, "Execution Mode") or "missing"
        mode_counter[mode] += 1
        if mode == "single-agent-fallback":
            reason = _field_value(block, "Fallback Reason Code") or "missing"
            fallback_reason_counter[reason] += 1
            fallback_tasks.append(f"{tid}: {reason}")

    print(f"## Execution Mode Report · {args.date}")
    print(f"- Total Tasks: {len(blocks)}")
    if mode_counter:
        print("- Execution Modes:")
        for mode, count in sorted(mode_counter.items()):
            print(f"  - {mode}: {count}")
    if fallback_reason_counter:
        print("- Fallback Reasons:")
        for reason, count in sorted(fallback_reason_counter.items()):
            print(f"  - {reason}: {count}")
        print("- Fallback Tasks:")
        for line in fallback_tasks:
            print(f"  - {line}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
