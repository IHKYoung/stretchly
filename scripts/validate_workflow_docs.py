#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Workflow docs validator for AhaKnow Multi-Agent OS.

- Enforces docs/logs + docs/plans dual-write gates
- Enforces docs/specs/<Task-ID>/ package exists (README/arch/ui/plan/testplan)
- In pre-commit mode: validates staged task dates; if prod changes staged, requires CHANGELOG staged
"""

from __future__ import annotations

import argparse
import datetime as dt
import re
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, Iterable, List, Optional, Sequence, Set, Tuple

from commit_audit_lib import (
    TASK_ID_MISSING,
    TASK_ID_MULTIPLE,
    related_task_ids_from_text,
    resolve_task_id_identity,
)
from workflow_asset_manifest import WORKFLOW_DOCS_ALLOWLIST

DEFAULT_TZ = "Asia/Taipei"
STRICT_SCHEMA_START_DATE = dt.date(2026, 3, 5)
SPEC_COMPLETION_START_DATE = dt.date(2026, 3, 6)
SOURCE_BASIS_START_DATE = dt.date(2026, 3, 6)
EXECUTION_MODE_START_DATE = dt.date(2026, 3, 6)
COMMIT_AUDIT_VALIDATION_START_DATE = dt.date(2026, 3, 6)
INTERACTION_CONTRACT_GATE_START_DATE = dt.date(2026, 3, 31)

TASK_ID_RE = re.compile(
    r"(?m)^\s*(?:[-*]\s*)?(?:#+\s*)?Task-ID\s*[:：]\s*(TID-\d{8}-[a-z0-9-]+)\s*$"
)
DATE_DOC_RE = re.compile(r"^docs/(?:logs|plans)/(\d{4}-\d{2}-\d{2})\.md$")
COMMIT_AUDIT_DOC_RE = re.compile(r"^docs/commits/\d{4}-\d{2}-\d{2}\.md$")
COMMIT_ENTRY_RE = re.compile(r"(?m)^###\s+.+$")
LABEL_LINE_RE = re.compile(r"^\s*(?:[-*]\s*)?(?P<label>\*\*[^*]+\*\*|[^:：]+?)\s*[:：]\s*(?P<tail>.*)$")

FIELD_ALIASES: Dict[str, Sequence[str]] = {
    "Type": ("记录类型",),
    "Goal": ("目标",),
    "Requirement Brief": ("需求回执", "需求摘要", "需求简报"),
    "Scope": ("范围",),
    "Status": ("状态",),
    "Links": ("链接",),
    "Source Basis": ("阅读依据", "源码依据", "变更依据", "Read Set", "Read-Set"),
    "Level": ("任务等级", "任务级别"),
    "Lane": ("执行 Lane",),
    "Execution Profile": ("执行画像", "执行档位"),
    "Execution Safety Block": ("执行安全块", "安全块"),
    "Execution Mode": ("执行模式",),
    "Fallback Scope": ("降级范围", "Fallback 范围"),
    "Fallback Reason Code": ("降级原因代码", "Fallback 原因代码"),
    "Approval Owner": ("审批 Owner", "授权 Owner", "审批归属", "授权归属"),
    "Required Roles": ("必需角色", "所需角色"),
    "Subagent Approval Policy": ("子 Agent 审批策略", "子智能体审批策略"),
    "Escalation Route": ("升级路径", "提权路径", "审批升级路径"),
    "Approval Packet Fields": ("审批包字段",),
    "Approval needed": ("是否需要审批", "需要审批", "审批需求"),
    "Approved": ("审批结果", "已批准", "批准状态"),
    "Evidence required": ("是否需要证据", "证据需求"),
    "Required": ("是否必需",),
    "Interaction Impact": ("交互影响",),
    "Interaction Freeze": ("交互冻结",),
    "Primary visible flow": ("主可见流程", "主可见路径"),
    "Fallback / secondary flow": ("降级/次级流程", "回退/次级流程", "回退路径"),
    "User-visible boundary": ("用户可见边界",),
    "Key visible states / transitions": ("关键可见状态/转移", "关键可见状态"),
    "Freeze status": ("冻结状态",),
    "Interaction authority / ownership boundary": ("交互归属边界", "交互责任边界"),
    "Visible entrypoints / handoff cues": ("可见入口/交接提示", "可见入口"),
    "In-scope interactions": ("范围内交互",),
    "Out-of-scope interactions": ("范围外交互",),
    "Interaction acceptance criteria": ("交互验收标准",),
    "Change Summary": ("变更总结", "变更摘要"),
    "Verification": ("验证", "验证结果"),
    "Knowledge Capture": ("知识沉淀",),
    "Knowledge Route": ("知识路由", "沉淀路由"),
    "Retention Review": ("留存审查",),
    "Retention Decision": ("留存决策", "保留决策"),
    "phase_stop_conditions": ("阶段停止条件", "停止条件"),
    "Warmup Required Roles": ("预热必需角色",),
    "Warmup Ready Roles": ("预热就绪角色",),
    "Warmup Agent IDs": ("预热 Agent IDs", "预热智能体 IDs"),
    "Warmup Verification": ("预热验证", "预热回执", "预热校验"),
    "Commit Hash": ("提交哈希",),
    "Merge Commit Hash": ("合并提交哈希",),
    "Branch": ("分支",),
    "Subject": ("提交主题",),
    "Related Task-IDs": ("关联 Task-ID", "关联 Task-IDs"),
    "Task-ID Resolution": ("Task-ID 归因", "Task-ID Resolution"),
}

KNOWN_ROLES: Set[str] = {
    "orchestrator",
    "scribe",
    "ui_designer",
    "architect",
    "coder",
    "tester",
    "evidence_collector",
    "reality_checker",
}
SPAWN_EXEMPT_ROLES: Set[str] = {"orchestrator"}
PLACEHOLDER_TOKENS: Set[str] = {
    "tbd",
    "(tbd)",
    "n/a",
    "na",
    "none",
    "null",
    "nil",
    "unknown",
    "待定",
    "暂无",
    "未填写",
}
AGENT_ID_RE = re.compile(r"\b[0-9a-f]{8,}(?:-[0-9a-f]{3,}){2,}\b", re.IGNORECASE)
FALLBACK_TOKEN = "single-agent-fallback"
FALLBACK_REASON_CODES = {
    "trivial-scribe-only",
    "warmup-interrupted-or-timeout",
    "control-plane-self-repair",
    "platform-unavailable",
    "external-blocked",
    "other-blocked",
}
SPEC_NA_RE = re.compile(r"(?i)\bN/?A\b|不涉及")


def _parse_iso_date(date_iso: str) -> dt.date:
    return dt.date.fromisoformat(date_iso)


def _is_strict_schema_date(date_iso: str) -> bool:
    try:
        return _parse_iso_date(date_iso) >= STRICT_SCHEMA_START_DATE
    except ValueError:
        return True


def _is_spec_completion_date(date_iso: str) -> bool:
    try:
        return _parse_iso_date(date_iso) >= SPEC_COMPLETION_START_DATE
    except ValueError:
        return True


def _is_source_basis_date(date_iso: str) -> bool:
    try:
        return _parse_iso_date(date_iso) >= SOURCE_BASIS_START_DATE
    except ValueError:
        return True


def _is_execution_mode_date(date_iso: str) -> bool:
    try:
        return _parse_iso_date(date_iso) >= EXECUTION_MODE_START_DATE
    except ValueError:
        return True


def _is_commit_audit_validation_date(date_iso: str) -> bool:
    try:
        return _parse_iso_date(date_iso) >= COMMIT_AUDIT_VALIDATION_START_DATE
    except ValueError:
        return True


def _is_interaction_contract_date(date_iso: str) -> bool:
    try:
        return _parse_iso_date(date_iso) >= INTERACTION_CONTRACT_GATE_START_DATE
    except ValueError:
        return True

def _now_date_iso(tz_name: str = DEFAULT_TZ) -> str:
    try:
        from zoneinfo import ZoneInfo
        return dt.datetime.now(ZoneInfo(tz_name)).date().isoformat()
    except Exception:
        return dt.date.today().isoformat()

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
    try:
        out = subprocess.check_output(
            ["git", *args],
            cwd=str(repo_root),
            stderr=subprocess.STDOUT,
            text=True,
        )
        return out.strip()
    except subprocess.CalledProcessError as e:
        raise RuntimeError(e.output.strip() or f"git {' '.join(args)} failed")


def _normalize_relpath(path: str) -> str:
    return path.strip().replace("\\", "/").lstrip("./")


def _staged_paths(repo_root: Path) -> List[str]:
    out = _run_git(repo_root, ["diff", "--cached", "--name-only"])
    return [_normalize_relpath(line) for line in out.splitlines() if line.strip()]


def _read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except Exception:
        return path.read_text(errors="replace")

def _split_task_blocks(text: str) -> Dict[str, str]:
    matches = list(TASK_ID_RE.finditer(text))
    blocks: Dict[str, str] = {}
    for i, m in enumerate(matches):
        tid = m.group(1)
        start = m.start()
        end = matches[i + 1].start() if i + 1 < len(matches) else len(text)
        blocks[tid] = text[start:end].strip()
    return blocks


def _find_duplicate_task_ids(text: str) -> Dict[str, int]:
    counts: Dict[str, int] = {}
    for m in TASK_ID_RE.finditer(text):
        tid = m.group(1)
        counts[tid] = counts.get(tid, 0) + 1
    return {tid: n for tid, n in counts.items() if n > 1}


def _normalize_label(label: str) -> str:
    v = label.strip().replace("**", "")
    v = re.sub(r"[（(][^）)]*[）)]", "", v)
    return " ".join(v.split()).strip().lower()


def _accepted_labels(field: str) -> Set[str]:
    labels = {field, *(FIELD_ALIASES.get(field, ()))}
    return {_normalize_label(x) for x in labels if _normalize_label(x)}


def _known_field_labels() -> Set[str]:
    labels: Set[str] = set()
    for field, aliases in FIELD_ALIASES.items():
        labels.add(_normalize_label(field))
        labels.update(_normalize_label(alias) for alias in aliases)
    return {x for x in labels if x}


KNOWN_FIELD_LABELS = _known_field_labels()


def _field_value(block: str, field: str) -> Optional[str]:
    accepted = _accepted_labels(field)
    lines = block.splitlines()

    for i, line in enumerate(lines):
        stripped = line.strip()
        if not stripped:
            continue

        if stripped.startswith("#"):
            heading = re.sub(r"^#+\s*", "", stripped)
            if _normalize_label(heading) in accepted:
                values: List[str] = []
                for nxt in lines[i + 1 : i + 51]:
                    nxt_strip = nxt.strip()
                    if not nxt_strip:
                        continue
                    if nxt_strip.startswith("#"):
                        break
                    m_nxt = LABEL_LINE_RE.match(nxt)
                    if m_nxt and _normalize_label(m_nxt.group("label")) in KNOWN_FIELD_LABELS:
                        break
                    values.append(nxt_strip)
                out = "\n".join(values).strip()
                return out or None
            continue

        m = LABEL_LINE_RE.match(line)
        if not m:
            continue

        label_norm = _normalize_label(m.group("label"))
        if label_norm not in accepted:
            continue

        values: List[str] = []
        tail = (m.group("tail") or "").strip()
        if tail:
            values.append(tail)

        for nxt in lines[i + 1 : i + 26]:
            if TASK_ID_RE.match(nxt):
                break
            nxt_strip = nxt.strip()
            if not nxt_strip:
                continue
            if nxt_strip.startswith("#"):
                break
            m_nxt = LABEL_LINE_RE.match(nxt)
            if m_nxt:
                nxt_label = _normalize_label(m_nxt.group("label"))
                # Only recognized workflow labels end the current field block.
                # Normal bullet lines with ":" should still count as field content.
                if nxt_label in KNOWN_FIELD_LABELS:
                    break
            values.append(nxt_strip)
        out = "\n".join(values).strip()
        return out or None

    return None


def _field_present(block: str, field: str) -> bool:
    return _field_value(block, field) is not None


def _normalize_value_token(token: str) -> str:
    v = re.sub(r"<!--.*?-->", "", token, flags=re.S).strip().lower()
    v = re.sub(r"^[\s\-\*\[\](){}]+", "", v)
    v = re.sub(r"[\s\-\*\[\](){}]+$", "", v)
    return v


def _split_value_tokens(raw: str) -> List[str]:
    if not raw:
        return []
    raw = re.sub(r"<!--.*?-->", "", raw, flags=re.S)
    normalized = (
        raw.replace("，", ",")
        .replace("、", ",")
        .replace("；", ";")
        .replace("｜", "|")
        .replace("\n", ",")
    )
    tokens: List[str] = []
    for part in re.split(r"[,;/|]+", normalized):
        piece = _normalize_value_token(part)
        if piece:
            tokens.append(piece)
    return tokens


def _extract_roles(raw: str) -> Set[str]:
    if not raw:
        return set()
    lower = raw.lower()
    found = {role for role in KNOWN_ROLES if re.search(rf"(?<![a-z_]){re.escape(role)}(?![a-z_])", lower)}
    if found:
        return found
    return {tok for tok in _split_value_tokens(raw) if tok in KNOWN_ROLES}


def _extract_agent_ids(raw: str) -> List[str]:
    if not raw:
        return []
    return sorted({m.group(0) for m in AGENT_ID_RE.finditer(raw)})


def _contains_placeholder_only(raw: str) -> bool:
    tokens = _split_value_tokens(raw)
    if not tokens:
        return True
    return all(tok in PLACEHOLDER_TOKENS for tok in tokens)


def _field_unresolved(raw: str) -> bool:
    if not raw or not raw.strip():
        return True
    if _contains_placeholder_only(raw):
        return True
    return re.search(r"(?i)\b(?:tbd|待定)\b", raw) is not None


def _has_blocked_reason(raw: str) -> bool:
    if not raw:
        return False
    if re.search(r"(?i)\bblocked\b\s*[:：]\s*\S+", raw):
        return True
    if re.search(r"阻塞\s*[:：]\s*\S+", raw):
        return True
    return False


def _normalize_status_token(raw: str) -> str:
    tokens = _split_value_tokens(raw)
    return tokens[0] if tokens else _normalize_value_token(raw)


def _normalize_execution_mode(raw: str) -> str:
    token = _normalize_status_token(raw)
    if token in {"multi-agent", "multiagent"}:
        return "multi-agent"
    if token in {"single-agent-fallback", "singleagentfallback", "fallback"}:
        return "single-agent-fallback"
    return token


def _normalize_interaction_impact(raw: str) -> str:
    token = _normalize_status_token(raw)
    if token in {"", "n/a", "na"}:
        return ""
    if token in {"none", "no"}:
        return "none"
    if token in {"indirect", "direct"}:
        return token
    return token


def _normalize_evidence_requirement(raw: str) -> str:
    token = _normalize_status_token(raw)
    if token in {"yes", "required"}:
        return "yes"
    if token == "partial":
        return "partial"
    if token in {"", "no", "none", "n/a", "na"}:
        return "no"
    return token


def _status_requires_completed_specs(raw: str) -> bool:
    status = _normalize_status_token(raw)
    return bool(status) and status not in {"init", "todo", "planned"}


def _interaction_detail_unresolved(raw: str) -> bool:
    if _field_unresolved(raw):
        return True
    return SPEC_NA_RE.search(raw or "") is not None


def _spec_file_marked_na(content: str) -> bool:
    return SPEC_NA_RE.search(content) is not None


def _spec_file_has_placeholder(content: str) -> bool:
    for raw_line in content.splitlines():
        line = raw_line.strip()
        if not line:
            continue
        if re.search(r"(?i)^(?:\(?tbd\)?|待定)$", line):
            return True
        if re.search(r"(?i)^[*-]\s*(?:\(?tbd\)?|待定)$", line):
            return True
        if re.search(r"(?i)^\d+\.\s*(?:\(?tbd\)?|待定)$", line):
            return True
        if re.search(r"(?i)^.*[:：]\s*(?:\(?tbd\)?|待定)\s*$", line):
            return True
    return False

@dataclass(frozen=True)
class ValidationError:
    where: str
    message: str

def _validate_blocks(
    blocks: Dict[str, str],
    required_fields: Sequence[str],
    file_label: str,
) -> List[ValidationError]:
    errors: List[ValidationError] = []
    if not blocks:
        errors.append(ValidationError(file_label, "未找到任何 Task-ID 条目（需要独立行 `Task-ID: TID-...` 作为任务块锚点）。"))
        return errors

    for tid, block in sorted(blocks.items()):
        for field in required_fields:
            if not _field_present(block, field):
                errors.append(ValidationError(f"{file_label}:{tid}", f"缺少必填字段：{field}"))
    return errors


def _validate_warmup_semantics(blocks: Dict[str, str], file_label: str) -> List[ValidationError]:
    errors: List[ValidationError] = []

    for tid, block in sorted(blocks.items()):
        required_raw = _field_value(block, "Warmup Required Roles") or ""
        ready_raw = _field_value(block, "Warmup Ready Roles") or ""
        ids_raw = _field_value(block, "Warmup Agent IDs") or ""
        verify_raw = _field_value(block, "Warmup Verification") or ""

        required_roles = _extract_roles(required_raw)
        ready_roles = _extract_roles(ready_raw)
        required_spawn_roles = {r for r in required_roles if r not in SPAWN_EXEMPT_ROLES}
        fallback_enabled = FALLBACK_TOKEN in ready_raw.lower()
        blocked_fallback = _has_blocked_reason(verify_raw)
        scribe_only_fallback = required_spawn_roles == {"scribe"}

        if fallback_enabled:
            if not (scribe_only_fallback or blocked_fallback):
                errors.append(
                    ValidationError(
                        f"{file_label}:{tid}",
                        "Warmup Ready Roles 使用了 single-agent-fallback，但不满足允许条件（仅 trivial+scribe-only 或明确 BLOCKED 原因）。",
                    )
                )
            continue

        missing_roles = sorted(required_spawn_roles - ready_roles)
        if missing_roles:
            errors.append(
                ValidationError(
                    f"{file_label}:{tid}",
                    f"Warmup Ready Roles 未覆盖 Required Roles（缺少：{', '.join(missing_roles)}）。",
                )
            )

        if _contains_placeholder_only(ids_raw):
            errors.append(
                ValidationError(
                    f"{file_label}:{tid}",
                    "Warmup Agent IDs 仍是占位值（如 TBD/N/A/none）；请填写真实 agent_id。",
                )
            )
            continue

        agent_ids = _extract_agent_ids(ids_raw)
        if not agent_ids:
            errors.append(
                ValidationError(
                    f"{file_label}:{tid}",
                    "Warmup Agent IDs 未检测到有效 agent_id（应为 spawn_agent 返回的 id）。",
                )
            )
            continue

        if len(agent_ids) < len(required_spawn_roles):
            errors.append(
                ValidationError(
                    f"{file_label}:{tid}",
                    f"Warmup Agent IDs 数量不足：需要至少 {len(required_spawn_roles)} 个，当前 {len(agent_ids)} 个。",
                )
            )

    return errors

def _validate_plan_links(plan_block: str, tid: str) -> Optional[ValidationError]:
    if re.search(rf"(?i)specs/{re.escape(tid)}", plan_block) is None:
        return ValidationError(
            f"plans:{tid}",
            "Plans 的 Links 中未发现 `specs/<Task-ID>` 引用（建议链接到 docs/specs/<Task-ID>/README.md）。",
        )
    return None

def _validate_spec_package(repo_root: Path, tid: str) -> List[ValidationError]:
    errors: List[ValidationError] = []
    spec_dir = repo_root / "docs" / "specs" / tid
    required_files = ["README.md", "plan.md", "testplan.md", "arch.md", "ui.md"]

    if not spec_dir.exists() or not spec_dir.is_dir():
        return [ValidationError(f"specs:{tid}", f"缺少 Spec 包目录：{spec_dir}")]

    for fn in required_files:
        p = spec_dir / fn
        if not p.exists():
            errors.append(ValidationError(f"specs:{tid}", f"缺少 Spec 文件：{p}"))
            continue

        content = _read_text(p)
        if re.search(rf"(?i)Task-ID\s*[:：]\s*{re.escape(tid)}\b", content) is None:
            errors.append(ValidationError(f"specs:{tid}:{fn}", "Spec 文件中缺少 `Task-ID: <Task-ID>` 标记。"))

    return errors


def _spec_readme_level(repo_root: Path, tid: str) -> str:
    readme_path = repo_root / "docs" / "specs" / tid / "README.md"
    if not readme_path.exists():
        return ""
    content = _read_text(readme_path)
    return _normalize_value_token(_field_value(content, "Level") or "")


def _validate_spec_completion(repo_root: Path, tid: str, plan_status_raw: str) -> List[ValidationError]:
    errors: List[ValidationError] = []
    if not _status_requires_completed_specs(plan_status_raw):
        return errors

    spec_dir = repo_root / "docs" / "specs" / tid
    readme_path = spec_dir / "README.md"
    plan_path = spec_dir / "plan.md"
    testplan_path = spec_dir / "testplan.md"
    arch_path = spec_dir / "arch.md"
    ui_path = spec_dir / "ui.md"

    if readme_path.exists():
        readme_content = _read_text(readme_path)
        readme_status = _normalize_status_token(_field_value(readme_content, "Status") or "")
        if readme_status == "init":
            errors.append(
                ValidationError(
                    f"specs:{tid}:README.md",
                    "任务已进入非 INIT 状态，但 `docs/specs/<Task-ID>/README.md` 仍保持 `Status: INIT`。",
                )
            )

    core_files = [plan_path, testplan_path]
    for path in core_files:
        if not path.exists():
            continue
        content = _read_text(path)
        if _spec_file_has_placeholder(content):
            errors.append(
                ValidationError(
                    f"specs:{tid}:{path.name}",
                    "任务已进入非 INIT 状态，但核心 Spec 仍包含 `TBD/待定` 占位内容。",
                )
            )

    optional_files = [arch_path, ui_path]
    for path in optional_files:
        if not path.exists():
            continue
        content = _read_text(path)
        if _spec_file_marked_na(content):
            continue
        if _spec_file_has_placeholder(content):
            errors.append(
                ValidationError(
                    f"specs:{tid}:{path.name}",
                    "该 Spec 未标记为 `N/A`，但仍包含 `TBD/待定` 占位内容。",
                )
            )

    if plan_path.exists():
        plan_content = _read_text(plan_path)
        approval_owner = _normalize_status_token(_field_value(plan_content, "Approval Owner") or "")
        if approval_owner != "orchestrator":
            errors.append(
                ValidationError(
                    f"specs:{tid}:plan.md",
                    "Plan 缺少 `Approval Owner: orchestrator`，无法明确授权 owner。",
                )
            )
        subagent_policy = _field_value(plan_content, "Subagent Approval Policy") or ""
        if 'approval_policy = "never"' not in subagent_policy:
            errors.append(
                ValidationError(
                    f"specs:{tid}:plan.md",
                    "Plan 缺少 `Subagent Approval Policy` 硬约束（应明确非 orchestrator 使用 `approval_policy = \"never\"`）。",
                )
            )
        escalation_route = (_field_value(plan_content, "Escalation Route") or "").lower()
        if "orchestrator" not in escalation_route:
            errors.append(
                ValidationError(
                    f"specs:{tid}:plan.md",
                    "Plan 缺少经过 `orchestrator` 的 `Escalation Route`，授权链无法审计。",
                )
            )
        approval_packet_fields = (_field_value(plan_content, "Approval Packet Fields") or "").lower()
        required_packet_tokens = ("command", "purpose", "risk", "rollback")
        if any(token not in approval_packet_fields for token in required_packet_tokens):
            errors.append(
                ValidationError(
                    f"specs:{tid}:plan.md",
                    "Plan 缺少完整的 `Approval Packet Fields`（至少包含 command / purpose / risk / rollback）。",
                )
            )

    return errors


def _validate_task_level_semantics(repo_root: Path, tid: str, plan_block: str, log_block: str) -> List[ValidationError]:
    errors: List[ValidationError] = []
    level = _spec_readme_level(repo_root, tid)
    if level not in {"trivial", "moderate", "complex"}:
        return errors

    plan_path = repo_root / "docs" / "specs" / tid / "plan.md"
    required_raw = _field_value(plan_block, "Warmup Required Roles") or _field_value(log_block, "Warmup Required Roles") or ""
    ready_raw = _field_value(plan_block, "Warmup Ready Roles") or _field_value(log_block, "Warmup Ready Roles") or ""
    verify_raw = _field_value(plan_block, "Warmup Verification") or _field_value(log_block, "Warmup Verification") or ""
    plan_status_raw = _field_value(plan_block, "Status") or ""

    required_roles = _extract_roles(required_raw)
    ready_roles = _extract_roles(ready_raw)
    fallback_enabled = FALLBACK_TOKEN in ready_raw.lower()
    plan_required_roles: Set[str] = set()
    approval_needed = ""
    approved_state = ""

    if plan_path.exists():
        plan_content = _read_text(plan_path)
        plan_required_roles = _extract_roles(_field_value(plan_content, "Required Roles") or "")
        approval_needed = _normalize_status_token(_field_value(plan_content, "Approval needed") or "")
        approved_state = _normalize_status_token(_field_value(plan_content, "Approved") or "")

    if plan_required_roles and plan_required_roles != required_roles:
        errors.append(
            ValidationError(
                f"specs:{tid}:plan.md",
                "Plan 中的 `Required Roles` 与 logs/plans 的 `Warmup Required Roles` 不一致。",
            )
        )

    if level in {"moderate", "complex"}:
        minimum_roles = {"architect", "coder", "tester", "scribe"}
        missing_roles = sorted(minimum_roles - required_roles)
        if missing_roles:
            errors.append(
                ValidationError(
                    f"specs:{tid}:README.md",
                    f"`{level}` 任务的 `Warmup Required Roles` 至少需要覆盖 architect/coder/tester/scribe（缺少：{', '.join(missing_roles)}）。",
                )
            )
        if approval_needed != "yes":
            errors.append(
                ValidationError(
                    f"specs:{tid}:plan.md",
                    f"`{level}` 任务必须在 plan.md 中声明 `Approval needed: yes`。",
                )
            )
        if _status_requires_completed_specs(plan_status_raw):
            if not approved_state or approved_state in PLACEHOLDER_TOKENS:
                errors.append(
                    ValidationError(
                        f"specs:{tid}:plan.md",
                        f"`{level}` 任务进入非 INIT 状态后，`Approved` 不能继续为占位值。",
                    )
                )

    if fallback_enabled and level != "trivial" and not _has_blocked_reason(verify_raw):
        errors.append(
            ValidationError(
                f"specs:{tid}:README.md",
                "`moderate/complex` 任务若使用 `single-agent-fallback`，必须在 Warmup Verification 中记录明确 BLOCKED 原因。",
            )
        )

    if level == "trivial":
        if fallback_enabled and required_roles != {"scribe"} and not _has_blocked_reason(verify_raw):
            errors.append(
                ValidationError(
                    f"specs:{tid}:README.md",
                    "`trivial` 任务仅在 `Required Roles = scribe` 时可直接使用 `single-agent-fallback`。",
                )
            )
        if approval_needed and approval_needed not in {"no", "n/a", "na"}:
            errors.append(
                ValidationError(
                    f"specs:{tid}:plan.md",
                    "`trivial` 任务默认应记录 `Approval needed: no`，除非另有明确阻塞原因。",
                )
            )

    return errors


def _validate_traceability_semantics(repo_root: Path, tid: str, plan_block: str, log_block: str) -> List[ValidationError]:
    errors: List[ValidationError] = []
    plan_status_raw = _field_value(plan_block, "Status") or ""
    if not _status_requires_completed_specs(plan_status_raw):
        return errors

    plan_source_basis = _field_value(plan_block, "Source Basis") or ""
    log_source_basis = _field_value(log_block, "Source Basis") or ""
    log_change_summary = _field_value(log_block, "Change Summary") or ""
    log_verification = _field_value(log_block, "Verification") or ""

    if _field_unresolved(plan_source_basis):
        errors.append(
            ValidationError(
                f"plans:{tid}",
                "任务进入非 INIT 状态后，plans 中的 `Source Basis` 不能缺失或保留 `TBD/待定` 占位。",
            )
        )
    if _field_unresolved(log_source_basis):
        errors.append(
            ValidationError(
                f"logs:{tid}",
                "任务进入非 INIT 状态后，logs 中的 `Source Basis` 不能缺失或保留 `TBD/待定` 占位。",
            )
        )
    if _field_unresolved(log_change_summary):
        errors.append(
            ValidationError(
                f"logs:{tid}",
                "任务进入非 INIT 状态后，logs 中的 `Change Summary` 不能缺失或保留 `TBD/待定` 占位。",
            )
        )
    if _field_unresolved(log_verification):
        errors.append(
            ValidationError(
                f"logs:{tid}",
                "任务进入非 INIT 状态后，logs 中的 `Verification` 不能缺失或保留 `TBD/待定` 占位。",
            )
        )

    plan_path = repo_root / "docs" / "specs" / tid / "plan.md"
    if plan_path.exists():
        plan_content = _read_text(plan_path)
        spec_source_basis = _field_value(plan_content, "Source Basis") or ""
        if _field_unresolved(spec_source_basis):
            errors.append(
                ValidationError(
                    f"specs:{tid}:plan.md",
                    "任务进入非 INIT 状态后，spec plan 中的 `Source Basis` 不能缺失或保留 `TBD/待定` 占位。",
                )
            )

    return errors


def _validate_interaction_contract_semantics(repo_root: Path, tid: str, plan_block: str) -> List[ValidationError]:
    errors: List[ValidationError] = []
    plan_status_raw = _field_value(plan_block, "Status") or ""
    if not _status_requires_completed_specs(plan_status_raw):
        return errors

    spec_dir = repo_root / "docs" / "specs" / tid
    readme_path = spec_dir / "README.md"
    plan_path = spec_dir / "plan.md"
    testplan_path = spec_dir / "testplan.md"
    if not (readme_path.exists() and plan_path.exists() and testplan_path.exists()):
        return errors

    readme_content = _read_text(readme_path)
    plan_content = _read_text(plan_path)
    testplan_content = _read_text(testplan_path)

    raw_impacts = {
        "README.md": _field_value(readme_content, "Interaction Impact") or "",
        "plan.md": _field_value(plan_content, "Interaction Impact") or "",
        "testplan.md": _field_value(testplan_content, "Interaction Impact") or "",
    }
    normalized_impacts = {
        name: _normalize_interaction_impact(raw)
        for name, raw in raw_impacts.items()
        if raw.strip()
    }
    invalid_impacts = {
        name: impact
        for name, impact in normalized_impacts.items()
        if impact not in {"none", "indirect", "direct"}
    }
    for name, impact in sorted(invalid_impacts.items()):
        errors.append(
            ValidationError(
                f"specs:{tid}:{name}",
                f"`Interaction Impact` 值无效：{impact}（应为 none / indirect / direct）。",
            )
        )

    valid_impacts = {
        name: impact
        for name, impact in normalized_impacts.items()
        if impact in {"none", "indirect", "direct"}
    }
    if len(set(valid_impacts.values())) > 1:
        joined = ", ".join(f"{name}={impact}" for name, impact in sorted(valid_impacts.items()))
        errors.append(
            ValidationError(
                f"specs:{tid}:interaction-contract",
                f"`README.md` / `plan.md` / `testplan.md` 的 `Interaction Impact` 不一致：{joined}",
            )
        )

    effective_impact = (
        valid_impacts.get("plan.md")
        or valid_impacts.get("testplan.md")
        or valid_impacts.get("README.md")
        or ""
    )
    if effective_impact in {"", "none"}:
        return errors

    if _interaction_detail_unresolved(_field_value(readme_content, "Interaction Impact") or ""):
        errors.append(
            ValidationError(
                f"specs:{tid}:README.md",
                "`interaction_impact != none` 时，README.md 必须显式汇总 `Interaction Impact`。",
            )
        )
    if _interaction_detail_unresolved(_field_value(readme_content, "Interaction Freeze") or ""):
        errors.append(
            ValidationError(
                f"specs:{tid}:README.md",
                "`interaction_impact != none` 时，README.md 的 `Interaction Freeze` 不能继续为 `N/A/TBD`。",
            )
        )

    plan_required_fields = [
        "Primary visible flow",
        "Fallback / secondary flow",
        "User-visible boundary",
        "Key visible states / transitions",
        "Freeze status",
        "Primary flow",
        "Interaction authority / ownership boundary",
        "Visible entrypoints / handoff cues",
        "In-scope interactions",
        "Out-of-scope interactions",
        "Interaction acceptance criteria",
    ]
    missing_plan_fields = [
        field
        for field in plan_required_fields
        if _interaction_detail_unresolved(_field_value(plan_content, field) or "")
    ]
    if missing_plan_fields:
        errors.append(
            ValidationError(
                f"specs:{tid}:plan.md",
                "`interaction_impact != none` 时，plan.md 必须补齐 Requirement Brief 与 Interaction Freeze 的交互字段；当前缺少或仍为占位："
                + ", ".join(missing_plan_fields),
            )
        )

    testplan_required_fields = [
        "Primary flow -> tests/evidence",
        "Fallback / secondary flow -> tests/evidence",
        "Visible states / transitions -> tests/evidence",
    ]
    missing_testplan_fields = [
        field
        for field in testplan_required_fields
        if _interaction_detail_unresolved(_field_value(testplan_content, field) or "")
    ]
    if missing_testplan_fields:
        errors.append(
            ValidationError(
                f"specs:{tid}:testplan.md",
                "`interaction_impact != none` 时，testplan.md 必须补齐 Interaction Contract Coverage；当前缺少或仍为占位："
                + ", ".join(missing_testplan_fields),
            )
        )

    plan_evidence_required = _normalize_evidence_requirement(_field_value(plan_content, "Evidence required") or "")
    testplan_evidence_required = _normalize_evidence_requirement(_field_value(testplan_content, "Required") or "")
    if plan_evidence_required not in {"yes", "partial"}:
        errors.append(
            ValidationError(
                f"specs:{tid}:plan.md",
                "`interaction_impact != none` 时，plan.md 的 `Evidence required` 必须为 `yes` 或 `partial`。",
            )
        )
    if testplan_evidence_required not in {"yes", "partial"}:
        errors.append(
            ValidationError(
                f"specs:{tid}:testplan.md",
                "`interaction_impact != none` 时，testplan.md 的 `Evidence Capture -> Required` 必须为 `yes` 或 `partial`。",
            )
        )
    if (
        plan_evidence_required in {"yes", "partial"}
        and testplan_evidence_required in {"yes", "partial"}
        and plan_evidence_required != testplan_evidence_required
    ):
        errors.append(
            ValidationError(
                f"specs:{tid}:interaction-contract",
                "plan.md 与 testplan.md 的 evidence requirement 不一致；两者必须使用同一个 `yes/partial` 结论。",
            )
        )

    return errors


def _split_commit_entries(text: str) -> List[str]:
    matches = list(COMMIT_ENTRY_RE.finditer(text))
    blocks: List[str] = []
    for i, match in enumerate(matches):
        start = match.start()
        end = matches[i + 1].start() if i + 1 < len(matches) else len(text)
        blocks.append(text[start:end].strip())
    return blocks


def _validate_execution_mode_semantics(tid: str, plan_block: str, log_block: str) -> List[ValidationError]:
    errors: List[ValidationError] = []
    plan_mode = _normalize_execution_mode(_field_value(plan_block, "Execution Mode") or "")
    log_mode = _normalize_execution_mode(_field_value(log_block, "Execution Mode") or "")
    verify_raw = _field_value(plan_block, "Warmup Verification") or _field_value(log_block, "Warmup Verification") or ""
    ready_raw = _field_value(plan_block, "Warmup Ready Roles") or _field_value(log_block, "Warmup Ready Roles") or ""
    required_raw = _field_value(plan_block, "Warmup Required Roles") or _field_value(log_block, "Warmup Required Roles") or ""
    fallback_scope_plan = (_field_value(plan_block, "Fallback Scope") or "").strip()
    fallback_scope_log = (_field_value(log_block, "Fallback Scope") or "").strip()
    fallback_reason_plan = (_field_value(plan_block, "Fallback Reason Code") or "").strip()
    fallback_reason_log = (_field_value(log_block, "Fallback Reason Code") or "").strip()
    required_roles = _extract_roles(required_raw)
    scribe_only_fallback = required_roles == {"scribe"}

    effective_modes = {mode for mode in (plan_mode, log_mode) if mode}
    if len(effective_modes) > 1:
        errors.append(
            ValidationError(
                f"execution-mode:{tid}",
                "plans/logs 中的 `Execution Mode` 不一致。",
            )
        )
    effective_mode = plan_mode or log_mode
    fallback_enabled = FALLBACK_TOKEN in ready_raw.lower()

    if effective_mode and effective_mode not in {"multi-agent", "single-agent-fallback"}:
        errors.append(
            ValidationError(
                f"execution-mode:{tid}",
                f"`Execution Mode` 值无效：{effective_mode}",
            )
        )

    if fallback_enabled and effective_mode != "single-agent-fallback":
        errors.append(
            ValidationError(
                f"execution-mode:{tid}",
                "Warmup 已使用 `single-agent-fallback`，但 `Execution Mode` 未同步标记为 `single-agent-fallback`。",
            )
        )
    if not fallback_enabled and effective_mode == "single-agent-fallback":
        errors.append(
            ValidationError(
                f"execution-mode:{tid}",
                "`Execution Mode` 标记为 `single-agent-fallback`，但 Warmup Ready Roles 未记录 fallback。",
            )
        )
    if effective_mode == "single-agent-fallback":
        fallback_scope = fallback_scope_plan or fallback_scope_log
        fallback_reason = _normalize_value_token(fallback_reason_plan or fallback_reason_log)
        if _field_unresolved(fallback_scope):
            errors.append(
                ValidationError(
                    f"execution-mode:{tid}",
                    "`single-agent-fallback` 必须补充 `Fallback Scope`，说明降级执行只覆盖哪些范围。",
                )
            )
        if _field_unresolved(fallback_reason):
            errors.append(
                ValidationError(
                    f"execution-mode:{tid}",
                    "`single-agent-fallback` 必须补充 `Fallback Reason Code`，说明降级原因类型。",
                )
            )
        elif fallback_reason not in FALLBACK_REASON_CODES:
            errors.append(
                ValidationError(
                    f"execution-mode:{tid}",
                    f"`Fallback Reason Code` 值无效：{fallback_reason}",
                )
            )
        if scribe_only_fallback and fallback_reason and fallback_reason != "trivial-scribe-only":
            errors.append(
                ValidationError(
                    f"execution-mode:{tid}",
                    "`trivial + scribe-only` fallback 必须使用 `Fallback Reason Code: trivial-scribe-only`。",
                )
            )
        if not scribe_only_fallback and fallback_reason == "trivial-scribe-only":
            errors.append(
                ValidationError(
                    f"execution-mode:{tid}",
                    "非 trivial+scribe-only 场景不得使用 `Fallback Reason Code: trivial-scribe-only`。",
                )
            )
        if not scribe_only_fallback and not _has_blocked_reason(verify_raw):
            errors.append(
                ValidationError(
                    f"execution-mode:{tid}",
                    "`single-agent-fallback` 必须在 `Warmup Verification` 中留下明确 BLOCKED 原因。",
                )
            )
    if fallback_scope_plan and fallback_scope_log and fallback_scope_plan != fallback_scope_log:
        errors.append(
            ValidationError(
                f"execution-mode:{tid}",
                "plans/logs 中的 `Fallback Scope` 不一致。",
            )
        )
    if fallback_reason_plan and fallback_reason_log and _normalize_value_token(fallback_reason_plan) != _normalize_value_token(fallback_reason_log):
        errors.append(
            ValidationError(
                f"execution-mode:{tid}",
                "plans/logs 中的 `Fallback Reason Code` 不一致。",
            )
        )

    return errors


def _validate_commit_audit_file(repo_root: Path, date_iso: str) -> List[ValidationError]:
    errors: List[ValidationError] = []
    commit_path = repo_root / "docs" / "commits" / f"{date_iso}.md"
    if not commit_path.exists():
        return errors

    entries = _split_commit_entries(_read_text(commit_path))
    for index, block in enumerate(entries, start=1):
        entry_label = f"commits:{date_iso}:entry-{index}"
        entry_type = _normalize_status_token(_field_value(block, "Type") or "")
        hash_field = "Merge Commit Hash" if entry_type == "merge" else "Commit Hash"
        commit_hash = (_field_value(block, hash_field) or "").strip()
        if not commit_hash:
            errors.append(ValidationError(entry_label, f"缺少 `{hash_field}`。"))
            continue

        try:
            actual_date = _run_git(repo_root, ["show", "-s", "--format=%cs", commit_hash])
            actual_subject = _run_git(repo_root, ["show", "-s", "--format=%s", commit_hash])
            actual_body = _run_git(repo_root, ["show", "-s", "--format=%b", commit_hash])
        except Exception as exc:
            errors.append(ValidationError(entry_label, f"无法读取 commit `{commit_hash}`：{exc}"))
            continue

        if actual_date != date_iso:
            errors.append(
                ValidationError(
                    entry_label,
                    f"commit `{commit_hash}` 的日期为 {actual_date}，与审计文件日期 {date_iso} 不一致。",
                )
            )

        subject_raw = (_field_value(block, "Subject") or "").strip()
        if subject_raw and subject_raw != actual_subject:
            errors.append(
                ValidationError(
                    entry_label,
                    f"`Subject` 与真实 commit subject 不一致：记录为 `{subject_raw}`，实际为 `{actual_subject}`。",
                )
            )

        resolved = resolve_task_id_identity(
            repo_root,
            date_iso,
            explicit_task_id="",
            commit_message=f"{actual_subject}\n{actual_body}".strip(),
            commit_paths=_changed_paths_for_revspec(repo_root, commit_hash),
        )
        task_id_raw = (_field_value(block, "Task-ID") or "").strip()
        if not task_id_raw:
            errors.append(ValidationError(entry_label, "缺少 `Task-ID`。"))
        elif task_id_raw != resolved.task_id:
            errors.append(
                ValidationError(
                    entry_label,
                    f"`Task-ID` 与可重算结果不一致：记录为 `{task_id_raw}`，应为 `{resolved.task_id}`。",
                )
            )

        related_task_ids_raw = (_field_value(block, "Related Task-IDs") or "").strip()
        if related_task_ids_raw:
            reported_related = related_task_ids_from_text(related_task_ids_raw)
            if reported_related != resolved.related_task_ids:
                errors.append(
                    ValidationError(
                        entry_label,
                        "`Related Task-IDs` 与真实 commit 影响范围不一致。",
                    )
                )
        elif resolved.task_id == TASK_ID_MULTIPLE:
            errors.append(
                ValidationError(
                    entry_label,
                    "`Task-ID` 为 `TASK-ID-MULTIPLE` 时，必须补充 `Related Task-IDs`。",
                )
            )

        resolution_raw = (_field_value(block, "Task-ID Resolution") or "").strip()
        if resolution_raw and _normalize_value_token(resolution_raw) != _normalize_value_token(resolved.resolution):
            errors.append(
                ValidationError(
                    entry_label,
                    f"`Task-ID Resolution` 不一致：记录为 `{resolution_raw}`，应为 `{resolved.resolution}`。",
                )
            )

    return errors

def _classify_paths(paths: Iterable[str]) -> Tuple[Set[str], Set[str], Set[str]]:
    doc_like: Set[str] = set()
    test_like: Set[str] = set()
    prod_like: Set[str] = set()

    TEST_PREFIXES = ("tests/", "__tests__/", "e2e/", "testutils/", "fixtures/")
    DOC_PREFIXES = ("docs/", ".githooks/")
    WORKFLOW_SCRIPT_ALLOWLIST = set(WORKFLOW_DOCS_ALLOWLIST)

    for p in paths:
        p = p.strip()
        if not p:
            continue

        if p.startswith(DOC_PREFIXES):
            doc_like.add(p)
            continue
        if p.startswith(TEST_PREFIXES):
            test_like.add(p)
            continue
        if p.startswith("scripts/"):
            if p in WORKFLOW_SCRIPT_ALLOWLIST:
                doc_like.add(p)
            else:
                prod_like.add(p)
            continue

        prod_like.add(p)

    return doc_like, test_like, prod_like

def _git_is_tracked(repo_root: Path, relpath: str) -> bool:
    try:
        _run_git(repo_root, ["ls-files", "--error-unmatch", "--", relpath])
        return True
    except Exception:
        return False


def _changed_paths_for_revspec(repo_root: Path, revspec: str) -> List[str]:
    revspec = revspec.strip()
    if not revspec:
        return []
    try:
        if ".." in revspec:
            out = _run_git(repo_root, ["diff", "--name-only", revspec])
        else:
            out = _run_git(repo_root, ["show", "--pretty=format:", "--name-only", revspec])
    except Exception:
        return []
    return [_normalize_relpath(line) for line in out.splitlines() if line.strip()]


def _collect_prepush_dates(repo_root: Path, rev_range: str) -> List[str]:
    specs: List[str] = []
    if rev_range.strip():
        specs = [x.strip() for x in rev_range.split(",") if x.strip()]
    else:
        code, upstream = 0, ""
        try:
            upstream = _run_git(repo_root, ["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{upstream}"])
        except Exception:
            code = 1
        if code == 0 and upstream:
            specs = [f"{upstream}..HEAD"]
        else:
            specs = ["HEAD"]

    dates: Set[str] = set()
    for spec in specs:
        for path in _changed_paths_for_revspec(repo_root, spec):
            m = DATE_DOC_RE.match(path)
            if m:
                dates.add(m.group(1))
    return sorted(dates)


def _collect_dates_from_paths(paths: Iterable[str]) -> List[str]:
    dates: Set[str] = set()
    for p in paths:
        m = DATE_DOC_RE.match(_normalize_relpath(p))
        if m:
            dates.add(m.group(1))
    return sorted(dates)


def _is_commit_audit_only(staged_paths: Sequence[str]) -> bool:
    normalized = [_normalize_relpath(p) for p in staged_paths if p.strip()]
    if not normalized:
        return False
    return all(COMMIT_AUDIT_DOC_RE.match(p) for p in normalized)


def _validate_precommit_staging(
    repo_root: Path,
    date_iso: str,
    task_ids: Set[str],
    *,
    enforce_spec_package: bool,
    staged_paths: Optional[Sequence[str]] = None,
) -> List[ValidationError]:
    errors: List[ValidationError] = []
    if staged_paths is None:
        staged_paths = _staged_paths(repo_root)
    else:
        staged_paths = [_normalize_relpath(p) for p in staged_paths if p.strip()]
    if not staged_paths:
        return errors

    staged_set = set(staged_paths)
    logs_rel = f"docs/logs/{date_iso}.md"
    plans_rel = f"docs/plans/{date_iso}.md"
    changelog_rel = "docs/CHANGELOG.md"

    if logs_rel not in staged_set:
        errors.append(ValidationError("pre-commit", f"任务 logs 未加入暂存区：{logs_rel}"))
    if plans_rel not in staged_set:
        errors.append(ValidationError("pre-commit", f"任务 plans 未加入暂存区：{plans_rel}"))

    _, _, prod_like = _classify_paths(staged_paths)
    if prod_like and changelog_rel not in staged_set:
        errors.append(ValidationError("pre-commit", f"检测到生产代码变更（{len(prod_like)} 个文件），但未暂存 CHANGELOG：{changelog_rel}"))

    if enforce_spec_package:
        for tid in sorted(task_ids):
            for rel in [
                f"docs/specs/{tid}/README.md",
                f"docs/specs/{tid}/plan.md",
                f"docs/specs/{tid}/testplan.md",
                f"docs/specs/{tid}/arch.md",
                f"docs/specs/{tid}/ui.md",
            ]:
                if not _git_is_tracked(repo_root, rel):
                    errors.append(ValidationError("pre-commit", f"Spec 文件未被 Git 跟踪（可能新建但忘记 git add）：{rel}"))

    return errors

def _print_errors(errors: Sequence[ValidationError]) -> None:
    print("❌ Workflow docs validation FAILED\n")
    for e in errors:
        print(f"- [{e.where}] {e.message}")
    print("\n建议：")
    print("- 用 scripts/scaffold_task.py 生成任务 logs/plans + specs 包并加入暂存区")
    print("- 确保 logs/plans 每条任务块包含全部必填字段（含 Warmup Receipt）")
    print("- 若有生产代码变更，同步更新 docs/CHANGELOG.md 并加入暂存区")


def _validate_one_date(
    repo_root: Path,
    date_iso: str,
    mode: str,
    *,
    staged_paths: Optional[Sequence[str]] = None,
) -> List[ValidationError]:
    strict_schema = _is_strict_schema_date(date_iso)
    require_completed_specs = _is_spec_completion_date(date_iso)
    require_source_basis = _is_source_basis_date(date_iso)
    require_execution_mode = _is_execution_mode_date(date_iso)
    require_commit_audit_validation = _is_commit_audit_validation_date(date_iso)
    require_interaction_contract = _is_interaction_contract_date(date_iso)

    logs_path = repo_root / "docs" / "logs" / f"{date_iso}.md"
    plans_path = repo_root / "docs" / "plans" / f"{date_iso}.md"

    errors: List[ValidationError] = []
    if not logs_path.exists():
        errors.append(ValidationError(f"logs:{date_iso}", f"缺少日志文件：{logs_path}"))
    if not plans_path.exists():
        errors.append(ValidationError(f"plans:{date_iso}", f"缺少计划文件：{plans_path}"))
    if errors:
        return errors

    logs_text = _read_text(logs_path)
    plans_text = _read_text(plans_path)
    logs_blocks = _split_task_blocks(logs_text)
    plans_blocks = _split_task_blocks(plans_text)

    for tid, n in sorted(_find_duplicate_task_ids(logs_text).items()):
        errors.append(ValidationError(f"logs:{date_iso}", f"检测到重复 Task-ID（{n} 次）：{tid}"))
    for tid, n in sorted(_find_duplicate_task_ids(plans_text).items()):
        errors.append(ValidationError(f"plans:{date_iso}", f"检测到重复 Task-ID（{n} 次）：{tid}"))

    required_logs_fields = ["Goal", "Requirement Brief", "Change Summary", "Verification"]
    required_plans_fields = ["Goal", "Requirement Brief", "Scope", "Status", "Links"]
    if require_source_basis:
        required_logs_fields.append("Source Basis")
        required_plans_fields.append("Source Basis")
    if require_execution_mode:
        required_logs_fields.append("Execution Mode")
        required_plans_fields.append("Execution Mode")
    if strict_schema:
        required_logs_fields.extend(
            [
                "Level",
                "Lane",
                "Execution Profile",
                "Execution Safety Block",
                "Knowledge Capture",
                "Knowledge Route",
                "Retention Review",
                "Retention Decision",
                "Warmup Required Roles",
                "Warmup Ready Roles",
                "Warmup Agent IDs",
                "Warmup Verification",
            ]
        )
        required_plans_fields.extend(
            [
                "Level",
                "Lane",
                "Execution Profile",
                "Execution Safety Block",
                "Knowledge Capture",
                "Knowledge Route",
                "phase_stop_conditions",
                "Warmup Required Roles",
                "Warmup Ready Roles",
                "Warmup Agent IDs",
                "Warmup Verification",
            ]
        )

    errors.extend(_validate_blocks(logs_blocks, required_logs_fields, f"logs:{date_iso}"))
    errors.extend(_validate_blocks(plans_blocks, required_plans_fields, f"plans:{date_iso}"))
    if strict_schema:
        errors.extend(_validate_warmup_semantics(logs_blocks, f"logs:{date_iso}"))
        errors.extend(_validate_warmup_semantics(plans_blocks, f"plans:{date_iso}"))

    logs_ids = set(logs_blocks.keys())
    plans_ids = set(plans_blocks.keys())
    if logs_ids != plans_ids:
        only_logs = sorted(logs_ids - plans_ids)
        only_plans = sorted(plans_ids - logs_ids)
        if only_logs:
            errors.append(ValidationError(f"cross-check:{date_iso}", f"以下 Task-ID 仅出现在 logs：{', '.join(only_logs)}"))
        if only_plans:
            errors.append(ValidationError(f"cross-check:{date_iso}", f"以下 Task-ID 仅出现在 plans：{', '.join(only_plans)}"))

    if strict_schema:
        for tid in sorted(logs_ids | plans_ids):
            errors.extend(_validate_spec_package(repo_root, tid))
            if tid in plans_blocks:
                e = _validate_plan_links(plans_blocks[tid], tid)
                if e:
                    errors.append(e)
                errors.extend(_validate_task_level_semantics(repo_root, tid, plans_blocks[tid], logs_blocks.get(tid, "")))
                if require_execution_mode:
                    errors.extend(_validate_execution_mode_semantics(tid, plans_blocks[tid], logs_blocks.get(tid, "")))
                if require_source_basis:
                    errors.extend(_validate_traceability_semantics(repo_root, tid, plans_blocks[tid], logs_blocks.get(tid, "")))
                if require_interaction_contract:
                    errors.extend(_validate_interaction_contract_semantics(repo_root, tid, plans_blocks[tid]))
                if require_completed_specs:
                    status_raw = _field_value(plans_blocks[tid], "Status") or ""
                    errors.extend(_validate_spec_completion(repo_root, tid, status_raw))
    if require_commit_audit_validation:
        errors.extend(_validate_commit_audit_file(repo_root, date_iso))

    if mode == "pre-commit":
        errors.extend(
            _validate_precommit_staging(
                repo_root,
                date_iso,
                logs_ids | plans_ids,
                enforce_spec_package=strict_schema,
                staged_paths=staged_paths,
            )
        )

    return errors

def main(argv: Optional[Sequence[str]] = None) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--date", default=None, help="验证的日期（YYYY-MM-DD）。默认 Asia/Taipei 今日。")
    ap.add_argument("--today", default=None, help=argparse.SUPPRESS)  # backward-compatible alias
    ap.add_argument("--tz", default=DEFAULT_TZ, help="默认日期时区")
    ap.add_argument("--mode", choices=["manual", "pre-commit", "pre-push"], default="manual")
    ap.add_argument("--rev-range", default="", help=argparse.SUPPRESS)  # for pre-push hook
    args = ap.parse_args(argv)

    repo_root = _detect_repo_root()
    target_dates: List[str]
    date_iso = args.date or args.today
    staged_paths: Optional[List[str]] = None
    if args.mode == "pre-push":
        if date_iso:
            target_dates = [date_iso]
        else:
            target_dates = _collect_prepush_dates(repo_root, args.rev_range)
        if not target_dates:
            print("[OK] pre-push: no docs/logs or docs/plans changes in outgoing commits ✅")
            return 0
    elif args.mode == "pre-commit":
        staged_paths = _staged_paths(repo_root)
        if not staged_paths:
            print("[OK] pre-commit: no staged changes ✅")
            return 0
        if date_iso:
            target_dates = [date_iso]
        else:
            if _is_commit_audit_only(staged_paths):
                print("[OK] pre-commit: commit-audit-only changes detected, skip logs/plans gate ✅")
                return 0
            target_dates = _collect_dates_from_paths(staged_paths)
            if not target_dates:
                _print_errors(
                    [
                        ValidationError(
                            "pre-commit",
                            "未检测到暂存的 docs/logs 或 docs/plans 日期文件。请先 scaffold task 并暂存双写文件。",
                        )
                    ]
                )
                return 1
    else:
        target_dates = [date_iso or _now_date_iso(args.tz)]

    errors: List[ValidationError] = []
    for d in target_dates:
        errors.extend(_validate_one_date(repo_root, d, mode=args.mode, staged_paths=staged_paths))

    if errors:
        _print_errors(errors)
        return 1

    if len(target_dates) == 1:
        print(f"[OK] Workflow docs validation passed for {target_dates[0]} ✅")
    else:
        print(f"[OK] Workflow docs validation passed for {', '.join(target_dates)} ✅")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
