#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Single source of truth for workflow asset inventories.
"""

from __future__ import annotations

from typing import Dict, List, Tuple

WORKFLOW_ASSET_MANIFEST = "scripts/workflow_asset_manifest.py"
VALIDATE_AGENT_CONFIG = "scripts/validate_agent_configs.py"
VALIDATE_WORKFLOW_KIT_SYNC = "scripts/validate_workflow_kit_sync.py"
VALIDATE_WORKFLOW_DOCS = "scripts/validate_workflow_docs.py"
COMMIT_AUDIT_LIB = "scripts/commit_audit_lib.py"
VALIDATE_COMMIT_MESSAGE_TASK_ID = "scripts/validate_commit_message_task_id.py"
SCAFFOLD_TASK = "scripts/scaffold_task.py"
RECORD_COMMIT_AUDIT = "scripts/record_commit_audit.py"
ENSURE_WORKFLOW_READY = "scripts/ensure_workflow_ready.py"
REPORT_EXECUTION_MODES = "scripts/report_execution_modes.py"
VALIDATE_ROLE_FILE_SCOPE = "scripts/validate_role_file_scope.py"
RUN_ROLE_GUARD = "scripts/run_role_guard.py"
BOOTSTRAP_WORKFLOW_REPO = "scripts/bootstrap_workflow_repo.py"
INSTALL_WORKFLOW_KIT = "scripts/install_workflow_kit.py"
INSTALL_GIT_HOOKS = "scripts/install_git_hooks.sh"
SYNC_TASK_SUMMARY = "scripts/sync_task_summary.py"

REPOSITORY_GUIDELINES = "docs/RepositoryGuidelines.md"
CODE_MAP = "docs/CodeMap.md"
CHANGELOG = "docs/CHANGELOG.md"

TEMPLATE_FILES: Tuple[str, ...] = (
    "docs/specs/_template/README.md",
    "docs/specs/_template/arch.md",
    "docs/specs/_template/ui.md",
    "docs/specs/_template/plan.md",
    "docs/specs/_template/testplan.md",
)

HOOK_FILES: Tuple[str, ...] = (
    ".githooks/commit-msg",
    ".githooks/pre-commit",
    ".githooks/pre-push",
    ".githooks/post-commit",
    ".githooks/post-merge",
)

INSTALLABLE_SCRIPT_FILES: Tuple[str, ...] = (
    WORKFLOW_ASSET_MANIFEST,
    VALIDATE_AGENT_CONFIG,
    VALIDATE_WORKFLOW_KIT_SYNC,
    VALIDATE_WORKFLOW_DOCS,
    COMMIT_AUDIT_LIB,
    VALIDATE_COMMIT_MESSAGE_TASK_ID,
    SCAFFOLD_TASK,
    RECORD_COMMIT_AUDIT,
    ENSURE_WORKFLOW_READY,
    REPORT_EXECUTION_MODES,
    VALIDATE_ROLE_FILE_SCOPE,
    RUN_ROLE_GUARD,
)

KIT_MIRRORED_ASSETS: Tuple[str, ...] = (
    *HOOK_FILES,
    *INSTALLABLE_SCRIPT_FILES,
    *TEMPLATE_FILES,
)

READINESS_REQUIRED_FILES: Tuple[str, ...] = (
    *TEMPLATE_FILES,
    *INSTALLABLE_SCRIPT_FILES,
    REPOSITORY_GUIDELINES,
    CODE_MAP,
)

READY_PY_COMPILE_FILES: Tuple[str, ...] = (
    WORKFLOW_ASSET_MANIFEST,
    VALIDATE_WORKFLOW_DOCS,
    VALIDATE_AGENT_CONFIG,
    VALIDATE_WORKFLOW_KIT_SYNC,
    COMMIT_AUDIT_LIB,
    VALIDATE_COMMIT_MESSAGE_TASK_ID,
    ENSURE_WORKFLOW_READY,
    REPORT_EXECUTION_MODES,
    VALIDATE_ROLE_FILE_SCOPE,
    RUN_ROLE_GUARD,
)

BOOTSTRAP_REQUIRED_FILES: Tuple[str, ...] = (
    *TEMPLATE_FILES,
    *INSTALLABLE_SCRIPT_FILES,
)

WORKFLOW_DOCS_ALLOWLIST: Tuple[str, ...] = (
    *INSTALLABLE_SCRIPT_FILES,
    BOOTSTRAP_WORKFLOW_REPO,
    INSTALL_WORKFLOW_KIT,
    INSTALL_GIT_HOOKS,
    SYNC_TASK_SUMMARY,
)

COMPONENT_MANIFEST: Dict[str, List[str]] = {
    "templates": list(TEMPLATE_FILES),
    "scripts": list(INSTALLABLE_SCRIPT_FILES),
    "hooks": list(HOOK_FILES),
}
