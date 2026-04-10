# Task-ID: TID-20260409-smart-reminder-wait-guard

## Meta
- Title: 缩短智能提醒阈值并阻止无限等待
- Date: 2026-04-09
- Level: trivial  <!-- trivial | moderate | complex -->
- Lane: fast    <!-- fast | deep -->
- Execution Profile: single-task  <!-- single-task | sequential-phases | merge-gate -->
- Status: DONE  <!-- INIT | IN_PROGRESS | REVIEW | DONE | BLOCKED -->

## Links
- Plan (daily): ../../plans/2026-04-09.md
- Log (daily): ../../logs/2026-04-09.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 默认空档阈值从 `12s` 收紧到 `6s`。
- smart 模式新增 bounded wait cap：仍未出现空档时，微休息和休息都不会无限等待。
- `idleOpportunitySeconds` 不再维持 hidden setting，现直接在设置页“智能暂停”分组暴露。

## Governance Notes
- Requirement Brief: 用户明确指出 smart reminder 当前阈值偏长，而且在持续操作电脑时可能一直等待；本任务需要同时调整默认值、状态机和设置页入口。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅限 smart reminder host 状态机、设置页智能暂停控件、locale 与相关文档；touches_running_service=no；backup_required=no；backup_plan=依赖 locale sync、test、typecheck、build 与 docs validator；rollback_plan=回退 `state.rs`、`App.tsx`、locale、测试与 docs；destructive_operations=none；operator_approval_required=no；rationale=无线上服务、提权、数据迁移或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；Rust 单测暴露 waiting snapshot 边角后已在同轮修正。
- Retention Decision: keep

## Notes
- 相关代码集中在 `apps/desktop/src-tauri/src/state.rs` 与 `apps/desktop/src/App.tsx`。
- 当前仅暴露 idle threshold；bounded wait cap 仍是 host 内置策略，不额外作为设置项。
