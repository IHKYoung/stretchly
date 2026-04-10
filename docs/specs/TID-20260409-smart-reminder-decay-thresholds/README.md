# Task-ID: TID-20260409-smart-reminder-decay-thresholds

## Meta
- Title: 将智能提醒改为递减阈值曲线
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
- 智能提醒不再使用单一 idle opportunity 阈值。
- 微休息采用 `6s -> 3s -> 1s -> 45s deadline`。
- 休息采用 `8s -> 4s -> 1s -> 90s deadline`。
- 这套策略只保留为 host 内置默认，不重新变成用户设置。

## Governance Notes
- Requirement Brief: 用户明确认可“先等较长空档，再逐步放宽”的设计方向；本任务把当前 smart reminder 收口成递减阈值曲线，并要求把设计记录进文档。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅限 smart reminder host 状态机、locale 文案与 reminder 设计文档；touches_running_service=no；backup_required=no；backup_plan=依赖 locale sync、cargo test、npm test、typecheck、build 与 docs validator；rollback_plan=回退 `state.rs`、locale、文档与测试；destructive_operations=none；operator_approval_required=no；rationale=不涉及线上服务、提权、数据迁移或外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无
- Retention Decision: keep

## Notes
- 这轮设计已经同步到 `docs/ReminderScheduling.md`，作为当前长期策略说明。
