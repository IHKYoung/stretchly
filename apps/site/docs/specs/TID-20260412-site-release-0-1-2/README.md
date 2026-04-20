# Task-ID: TID-20260412-site-release-0-1-2

## Meta
- Title: 更新 site 下载链接到 0.1.2 release
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
- 将首页下载按钮的固定 href 与 `download/targets.js` 中的 pinned fallback 同步切到 `Pauza_0.1.2_aarch64.dmg`。
- 保留 GitHub latest release API，但新增“pinned 资产名优先”保护，避免 API 仍返回 `0.1.1` 时把按钮回退到旧版本。
- 本次任务同时补齐 workflow kit、任务 spec、daily plan/log、changelog 与提交门禁所需文档。

## Governance Notes
- Requirement Brief: 将官网下载入口更新到 `Pauza_0.1.2_aarch64.dmg`，并保证在 GitHub latest API 仍停留 `0.1.1` 时，页面不会把按钮自动回退到旧版本。
- Interaction Impact: direct  <!-- none | indirect | direct -->
- Interaction Freeze: 仅允许修改首页右上角下载按钮的目标解析与 release 钉住逻辑；页面布局、文案、动效与其余交互保持不变。
- Execution Safety Block: `service_impact=no; touches_running_service=no; backup_required=no; backup_plan=git revert HEAD 或手动恢复下载链接与脚本; rollback_plan=恢复到 v0.1.1 链接并去掉 pinned-version 保护; destructive_operations=none; operator_approval_required=no; rationale=纯静态站点改动，无数据迁移、权限变更或外部副作用。`
- Approval Owner: orchestrator
- Delegation Policy: `orchestrator` 在用户已授权其负责日常编排后，可自主决定是否 `spawn_agent`
- Execution Mode: single-agent-fallback
- Fallback Reason Code: platform-unavailable
- Escalation Summary: 未触发升级；当前会话受系统约束不能主动拉起多子 agent，因此由单 agent 兜底完成 scribe/coder/tester 职责。
- Retention Decision: keep（保留 workflow kit、任务文档与 pinned-version 保护逻辑，作为后续 release 更新基线）

## Notes
- Source Basis: `index.html`、`download/targets.js`、`script.js`、`README.md`
- Verification Evidence: `./evidence/README.md`
