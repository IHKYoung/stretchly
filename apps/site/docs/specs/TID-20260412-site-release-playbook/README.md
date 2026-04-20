# Task-ID: TID-20260412-site-release-playbook

## Meta
- Title: 沉淀站点 release 一键发布流程
- Date: 2026-04-12
- Level: trivial  <!-- trivial | moderate | complex -->
- Lane: fast    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-12.md
- Log (daily): ../../logs/2026-04-12.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 新增 `scripts/publish_site_release.py`，把 dmg 版本解析、站点 pinned 链接更新与 GitHub release create/upload 收敛到一条命令。
- 新增 `docs/ReleasePlaybook.md`，把前置条件、标准命令、变体用法、回滚方式写成常驻手册。
- 新增 `.gitignore`，避免 `__pycache__` 与 `.playwright-mcp` 污染工作树。

## Governance Notes
- Requirement Brief: 把这次 release 发布经验沉淀成下次可直接复用的标准流程，至少包含一个可执行脚本和一份清晰的发布手册。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: `service_impact=no; touches_running_service=no; backup_required=no; backup_plan=git revert HEAD 或删除新增脚本与文档; rollback_plan=移除 publish_site_release.py、ReleasePlaybook 与 .gitignore 规则; destructive_operations=none; operator_approval_required=no; rationale=本次只新增本地脚本与文档，不执行新的线上副作用。`
- Approval Owner: orchestrator
- Delegation Policy: `orchestrator` 在用户已授权其负责日常编排后，可自主决定是否 `spawn_agent`
- Execution Mode: single-agent-fallback
- Fallback Reason Code: platform-unavailable
- Escalation Summary: 未触发升级；当前会话仍受系统约束不能主动拉起多子 agent，因此由单 agent 合并完成 scribe/coder/tester 职责。
- Retention Decision: keep（发布脚本、playbook 与忽略规则都应长期保留）

## Notes
- Source Basis: `README.md`、`docs/RepositoryGuidelines.md`、`docs/CodeMap.md`、`download/targets.js`、`index.html`、`scripts/validate_workflow_docs.py`
- Verification: `python3 scripts/publish_site_release.py --asset <current dmg> --skip-release`、`python3 scripts/publish_site_release.py --asset <current dmg> --dry-run`、workflow validators
