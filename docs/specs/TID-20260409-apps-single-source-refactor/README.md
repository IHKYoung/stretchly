# Task-ID: TID-20260409-apps-single-source-refactor

## Meta
- Title: 收口到 apps 单真源并停用 app 目录
- Date: 2026-04-09
- Level: complex  <!-- trivial | moderate | complex -->
- Lane: deep    <!-- fast | deep -->
- Execution Profile: sequential-phases  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-09.md
- Log (daily): ../../logs/2026-04-09.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 默认运行、构建与测试链路统一收口到 `apps/desktop`。
- desktop locale registry 改为只由 `apps/desktop/src/locales/{messages,config}` 生成，`overrides/` 退出默认架构。
- break 消息页提示语继续独立，但可编辑真源改为 `apps/desktop/src/locales/break-message-copy.json`。
- `app/` 目录本轮不做物理删除，只保留为归档参考，避免在当前脏工作区下误删用户已有修改。

## Governance Notes
- Requirement Brief: 用户要求做真正的重构，而不是继续 patch 旧链路；目标是让 `apps/desktop` 成为唯一有效的桌面端真源，`app/` 停止参与默认运行链路，locale 不再使用 `overrides` 叠加。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅限本地桌面端源码归属、根脚本、测试引用、locale 生成链路与文档；touches_running_service=no；backup_required=no；backup_plan=依赖 Git diff、`python3 scripts/sync_desktop_locales.py`、`npm --prefix apps/desktop run typecheck`、`npm --prefix apps/desktop run build`、`npm test` 与 docs validator；rollback_plan=回退 `package.json`、`apps/desktop/**`、`test/**`、`scripts/sync_desktop_locales.py` 与相关 docs；destructive_operations=none；operator_approval_required=no；rationale=不涉及线上服务、外部副作用、提权或历史重写。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked（当前 session 未获用户显式授权调用 `spawn_agent`）
- Escalation Summary: 无；未触发需要用户额外批准的破坏性动作。
- Retention Decision: keep

## Notes
- 当前默认运行、构建、测试与 locale 生成链路都已不再依赖 `app/`。
- `app/` 仍在仓库里，但语义已退为 archived reference，而不是当前产品的一部分。
