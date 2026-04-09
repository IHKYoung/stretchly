#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Validate multi-agent config invariants for AhaKnow AGENTS.

- Orchestrator must remain the default agent and sole approval owner
- All subagents must use approval_policy = "never"
- Sandbox modes must stay within the approved least-privilege matrix
- Agent registry/path drift must be machine-checkable
- Readiness/pre-commit hooks can use this as a hard gate
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, Mapping, Optional, Sequence

import tomllib


REQUIRED_SUBAGENTS: Dict[str, str] = {
    "ui_designer": "read-only",
    "architect": "read-only",
    "coder": "workspace-write",
    "tester": "workspace-write",
    "evidence_collector": "workspace-write",
    "reality_checker": "read-only",
    "scribe": "workspace-write",
}
ORCHESTRATOR_ROLE = "orchestrator"
MAX_AGENT_THREADS = 10
SUBAGENT_APPROVAL_POLICY = "never"
ORCHESTRATOR_APPROVAL_POLICY = "never"
ORCHESTRATOR_CONFIG_FILE = "agents/orchestrator.toml"
ORCHESTRATOR_SANDBOX_MODE = "danger-full-access"
ALLOWED_SANDBOX_MODES = {"read-only", "workspace-write", "danger-full-access"}
REQUIRED_AGENT_CONFIG_FILES: Dict[str, str] = {
    ORCHESTRATOR_ROLE: ORCHESTRATOR_CONFIG_FILE,
    **{role: f"agents/{role}.toml" for role in REQUIRED_SUBAGENTS},
}
ORCHESTRATOR_REQUIRED_SNIPPETS = (
    "## Permission Governance (Owner: Orchestrator)",
    "子 agent 的权限审批归属 Orchestrator",
    "sandbox_mode = \"danger-full-access\"",
    "approval_policy = \"never\"",
    "高信任 orchestrator",
    "repo-local 本地命令默认由你直接执行",
    "git status`、`git diff`、`git add -- <explicit paths...>`",
    "用户确认保留给破坏性操作、范围外扩、付费/外部副作用、新增依赖/密钥等决策",
    "Agent Config Gate",
    "所有非 orchestrator 子 agent 必须保持 `approval_policy = \"never\"`",
)
GENERIC_SUBAGENT_REQUIRED_SNIPPETS = (
    "不得直接向用户发起权限审批请求",
    "上报 Orchestrator",
    "approval_policy = \"never\"",
)
MACHINE_SUMMARY_SNIPPET = "Machine Summary"
ROLE_MACHINE_SUMMARY_STAGES: Dict[str, str] = {
    "coder": "coding",
    "tester": "test",
    "evidence_collector": "evidence",
    "reality_checker": "reality_check",
    "scribe": "doc_gate",
}
MACHINE_SUMMARY_REQUIRED_FIELDS: Sequence[str] = ("stage", "status", "executed_by", "summary", "artifacts", "next")
JSON_FENCE_RE = re.compile(r"```json\s*(\{.*?\})\s*```", re.DOTALL)
ROLE_BOUNDARY_SNIPPETS: Dict[str, Sequence[str]] = {
    "ui_designer": (
        "负责 UI/交互规范与一致性复核（不写代码）。",
        "不写前端组件代码、不写后端逻辑",
    ),
    "architect": (
        "负责系统/数据/契约/边界设计与一致性 Review（不写生产代码）。",
        "Severity: P0 | P1 | P2",
    ),
    "coder": (
        "不执行 git commit/push",
        "RED -> GREEN -> REFACTOR",
        "优先行为/契约测试",
        "Red case",
        "Residual risks / Verification gaps",
    ),
    "tester": (
        "允许：tests/, __tests__/, e2e/, testutils/, fixtures/",
        "禁止：应用源码路径（如 apps/desktop/src/, src/, lib/）",
        "run_role_guard.py --role tester",
        "validate_role_file_scope.py --role tester",
        "code-shaped tests",
        "PASS 时也要报告验证缺口",
        "Verification Gaps",
        "Test Quality Risks",
    ),
    "evidence_collector": (
        "允许：docs/specs/<Task-ID>/evidence/**",
        "禁止：除上述路径外的任何写入",
        "run_role_guard.py --role evidence_collector",
        "validate_role_file_scope.py --role evidence_collector",
    ),
    "reality_checker": (
        "不修改任何文件、不写代码、不跑命令（只读审计）",
    ),
    "scribe": (
        "允许：docs/**, 根目录文档（如 README.md / AGENTS.md）, .githooks/**, scripts/(workflow-related)",
        "禁止：生产代码路径",
        "Git staging 不由你执行",
        "`git add -- <explicit paths...>` 仅由 Orchestrator 直接执行",
        "Git index / staging 命令不得通过 `run_role_guard.py` 或 Python helper 包装",
        "run_role_guard.py --role scribe",
        "validate_role_file_scope.py --role scribe",
    ),
}


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


def _load_toml(path: Path) -> Mapping[str, object]:
    with path.open("rb") as fh:
        return tomllib.load(fh)


def _string_value(data: Mapping[str, object], key: str) -> Optional[str]:
    value = data.get(key)
    return value if isinstance(value, str) else None


def _mapping_value(data: Mapping[str, object], key: str) -> Mapping[str, object]:
    value = data.get(key)
    return value if isinstance(value, dict) else {}


def _relpath(repo_root: Path, path: Path) -> str:
    return str(path.resolve().relative_to(repo_root.resolve()))


def _validate_machine_summary(role: str, instructions: str, relpath: str) -> list[ValidationError]:
    errors: list[ValidationError] = []
    expected_stage = ROLE_MACHINE_SUMMARY_STAGES.get(role)
    if not expected_stage:
        return errors

    if MACHINE_SUMMARY_SNIPPET not in instructions:
        return [
            ValidationError(
                relpath,
                f"`developer_instructions` 缺少关键授权约束：{MACHINE_SUMMARY_SNIPPET}",
            )
        ]

    parsed_blocks: list[Mapping[str, object]] = []
    parse_failures = 0
    for block in JSON_FENCE_RE.findall(instructions):
        try:
            candidate = json.loads(block)
        except json.JSONDecodeError:
            parse_failures += 1
            continue
        if isinstance(candidate, dict):
            parsed_blocks.append(candidate)

    summary_block = next(
        (
            block
            for block in parsed_blocks
            if block.get("stage") == expected_stage
        ),
        None,
    )
    if summary_block is None:
        detail = "未找到可解析的 Machine Summary JSON block"
        if parse_failures:
            detail += f"（JSON 解析失败 {parse_failures} 个）"
        errors.append(
            ValidationError(
                relpath,
                f"`developer_instructions` 缺少 stage=`{expected_stage}` 的 Machine Summary schema：{detail}",
            )
        )
        return errors

    for field in MACHINE_SUMMARY_REQUIRED_FIELDS:
        if field not in summary_block:
            errors.append(
                ValidationError(
                    relpath,
                    f"`Machine Summary` 缺少字段：{field}",
                )
            )

    if summary_block.get("executed_by") != role:
        errors.append(
            ValidationError(
                relpath,
                f"`Machine Summary.executed_by` 必须为 `{role}`，当前为 `{summary_block.get('executed_by')}`。",
            )
        )

    if not isinstance(summary_block.get("artifacts"), dict):
        errors.append(
            ValidationError(
                relpath,
                "`Machine Summary.artifacts` 必须是 JSON object。",
            )
        )

    return errors


def _validate_main_config(repo_root: Path) -> tuple[list[ValidationError], dict[str, Path]]:
    errors: list[ValidationError] = []
    registered_agent_paths: dict[str, Path] = {}

    config_path = repo_root / "config" / "config.toml"
    if not config_path.exists():
        return [ValidationError("config", f"缺少主配置文件：{config_path}")], registered_agent_paths

    config = _load_toml(config_path)
    features = _mapping_value(config, "features")
    if features.get("multi_agent") is not True:
        errors.append(ValidationError("config/config.toml", "`[features].multi_agent` 必须为 true。"))

    agents = _mapping_value(config, "agents")
    max_threads = agents.get("max_threads")
    if not isinstance(max_threads, int):
        errors.append(ValidationError("config/config.toml", "`[agents].max_threads` 必须为整数。"))
    elif max_threads < 1 or max_threads > MAX_AGENT_THREADS:
        errors.append(
            ValidationError(
                "config/config.toml",
                f"`[agents].max_threads` 必须位于 1..{MAX_AGENT_THREADS}，当前为 {max_threads}。",
            )
        )

    default_cfg = _mapping_value(agents, "default")
    default_path = _string_value(default_cfg, "config_file")
    if default_path != ORCHESTRATOR_CONFIG_FILE:
        errors.append(
            ValidationError(
                "config/config.toml",
                f"`[agents.default].config_file` 必须指向 `{ORCHESTRATOR_CONFIG_FILE}`。",
            )
        )

    role_tables = {key for key, value in agents.items() if isinstance(value, dict)}
    expected_tables = {"default", *REQUIRED_AGENT_CONFIG_FILES.keys()}
    unexpected_tables = sorted(role_tables - expected_tables)
    for role in unexpected_tables:
        errors.append(
            ValidationError(
                "config/config.toml",
                f"检测到未注册的 `[agents.{role}]` 表；请先扩展 validator 再新增角色。",
            )
        )

    seen_paths: dict[Path, str] = {}
    for role, expected_config in sorted(REQUIRED_AGENT_CONFIG_FILES.items()):
        role_cfg = _mapping_value(agents, role)
        config_file = _string_value(role_cfg, "config_file")
        if not config_file:
            errors.append(ValidationError("config/config.toml", f"缺少 `[agents.{role}]` 或 `config_file`。"))
            continue
        if config_file != expected_config:
            errors.append(
                ValidationError(
                    "config/config.toml",
                    f"`[agents.{role}]`.config_file 必须为 `{expected_config}`，当前为 `{config_file}`。",
                )
            )

        rel_path = Path("config") / config_file
        abs_path = repo_root / rel_path
        registered_agent_paths[role] = abs_path
        if not abs_path.exists():
            errors.append(ValidationError("config/config.toml", f"`[agents.{role}]` 指向的文件不存在：{rel_path}"))
            continue

        previous_role = seen_paths.get(abs_path.resolve())
        if previous_role:
            errors.append(
                ValidationError(
                    "config/config.toml",
                    f"`[agents.{previous_role}]` 与 `[agents.{role}]` 指向了同一配置文件：{rel_path}",
                )
            )
        else:
            seen_paths[abs_path.resolve()] = role

    return errors, registered_agent_paths


def _validate_agent_config_directory(repo_root: Path, registered_agent_paths: Mapping[str, Path]) -> list[ValidationError]:
    errors: list[ValidationError] = []
    agent_dir = repo_root / "config" / "agents"
    if not agent_dir.exists():
        return [ValidationError("config/agents", f"缺少 agent 配置目录：{agent_dir}")]

    actual_files = {path.resolve(): path for path in sorted(agent_dir.glob("*.toml"))}
    expected_files = {
        (repo_root / "config" / rel_path).resolve(): role
        for role, rel_path in REQUIRED_AGENT_CONFIG_FILES.items()
    }
    registered_files = {
        path.resolve(): role
        for role, path in registered_agent_paths.items()
        if path.exists()
    }

    for expected_path, role in sorted(expected_files.items(), key=lambda item: item[1]):
        if expected_path not in actual_files:
            errors.append(
                ValidationError(
                    "config/agents",
                    f"缺少 `{role}` 的配置文件：{_relpath(repo_root, expected_path)}",
                )
            )

    for actual_path, actual_file in sorted(actual_files.items(), key=lambda item: item[1].name):
        if actual_path not in expected_files:
            errors.append(
                ValidationError(
                    "config/agents",
                    f"检测到未注册或孤立的 agent 配置文件：{_relpath(repo_root, actual_file)}",
                )
            )
            continue
        if actual_path not in registered_files:
            errors.append(
                ValidationError(
                    "config/config.toml",
                    f"`config/config.toml` 未注册 agent 配置文件：{_relpath(repo_root, actual_file)}",
                )
            )

    return errors


def _validate_orchestrator(repo_root: Path, agent_path: Path) -> list[ValidationError]:
    errors: list[ValidationError] = []
    config = _load_toml(agent_path)
    relpath = _relpath(repo_root, agent_path)

    sandbox_mode = _string_value(config, "sandbox_mode")
    if sandbox_mode != ORCHESTRATOR_SANDBOX_MODE:
        errors.append(
            ValidationError(
                relpath,
                f"`orchestrator` 的 `sandbox_mode` 必须为 `{ORCHESTRATOR_SANDBOX_MODE}`。",
            )
        )

    approval_policy = _string_value(config, "approval_policy")
    if approval_policy != ORCHESTRATOR_APPROVAL_POLICY:
        errors.append(
            ValidationError(
                relpath,
                f"`orchestrator` 必须显式设置 `approval_policy = \"{ORCHESTRATOR_APPROVAL_POLICY}\"`。",
            )
        )

    instructions = _string_value(config, "developer_instructions") or ""
    for snippet in ORCHESTRATOR_REQUIRED_SNIPPETS:
        if snippet not in instructions:
            errors.append(
                ValidationError(
                    relpath,
                    f"`developer_instructions` 缺少关键授权约束：{snippet}",
                )
            )

    return errors


def _validate_subagent(repo_root: Path, role: str, agent_path: Path, expected_sandbox: str) -> list[ValidationError]:
    errors: list[ValidationError] = []
    config = _load_toml(agent_path)
    relpath = _relpath(repo_root, agent_path)

    sandbox_mode = _string_value(config, "sandbox_mode")
    if sandbox_mode not in ALLOWED_SANDBOX_MODES:
        errors.append(
            ValidationError(
                relpath,
                f"`{role}` 的 `sandbox_mode` 必须属于 {sorted(ALLOWED_SANDBOX_MODES)}，当前为 `{sandbox_mode}`。",
            )
        )
    if sandbox_mode != expected_sandbox:
        errors.append(
            ValidationError(
                relpath,
                f"`{role}` 的 `sandbox_mode` 必须为 `{expected_sandbox}`，当前为 `{sandbox_mode}`。",
            )
        )

    approval_policy = _string_value(config, "approval_policy")
    if approval_policy != SUBAGENT_APPROVAL_POLICY:
        errors.append(
            ValidationError(
                relpath,
                f"`{role}` 必须显式设置 `approval_policy = \"{SUBAGENT_APPROVAL_POLICY}\"`。",
            )
        )

    instructions = _string_value(config, "developer_instructions") or ""
    required_snippets = [*GENERIC_SUBAGENT_REQUIRED_SNIPPETS, *ROLE_BOUNDARY_SNIPPETS.get(role, ())]
    for snippet in required_snippets:
        if snippet not in instructions:
            errors.append(
                ValidationError(
                    relpath,
                    f"`developer_instructions` 缺少关键授权约束：{snippet}",
                )
            )

    errors.extend(_validate_machine_summary(role, instructions, relpath))
    return errors


def _print_errors(errors: Sequence[ValidationError]) -> None:
    print("❌ Agent config validation FAILED\n")
    for err in errors:
        print(f"- [{err.where}] {err.message}")
    print("\n建议：")
    print("- 保持 `[agents.default] -> agents/orchestrator.toml`")
    print(f"- `orchestrator.toml` 显式使用 `sandbox_mode = \"{ORCHESTRATOR_SANDBOX_MODE}\"`")
    print(f"- `orchestrator.toml` 显式使用 `approval_policy = \"{ORCHESTRATOR_APPROVAL_POLICY}\"`")
    print("- 所有非 orchestrator 角色显式设置 `approval_policy = \"never\"`")
    print("- 子 agent 使用最小化 sandbox，不要保留 `danger-full-access`")


def main(argv: Optional[Sequence[str]] = None) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo-root", default="", help="仓库根目录（默认自动探测）")
    args = ap.parse_args(argv)

    repo_root = Path(args.repo_root).resolve() if args.repo_root else _detect_repo_root()
    main_config_path = repo_root / "config" / "config.toml"
    agent_dir_path = repo_root / "config" / "agents"

    if not main_config_path.exists() and not agent_dir_path.exists():
        print("[SKIP] Agent config validation skipped: no multi-agent config detected ✅")
        return 0

    errors, registered_paths = _validate_main_config(repo_root)
    errors.extend(_validate_agent_config_directory(repo_root, registered_paths))

    orchestrator_path = registered_paths.get(ORCHESTRATOR_ROLE)
    if orchestrator_path and orchestrator_path.exists():
        errors.extend(_validate_orchestrator(repo_root, orchestrator_path))

    for role, expected_sandbox in REQUIRED_SUBAGENTS.items():
        agent_path = registered_paths.get(role)
        if agent_path and agent_path.exists():
            errors.extend(_validate_subagent(repo_root, role, agent_path, expected_sandbox))

    if errors:
        _print_errors(errors)
        return 1

    print("[OK] Agent config validation passed ✅")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
