# Task-ID: TID-20260409-smart-reminder-ui-simplification

## Meta
- Title: 收回智能提醒阈值设置项并保留内部策略
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
- 前台不再暴露 `等待空档` 秒数控件。
- smart reminder 继续保留内部 `6s + bounded wait cap` 策略。
- locale 真源删除 `ui.waitForPause*` 过渡 key。

## Governance Notes
- Requirement Brief: 用户明确反馈不需要太多可设置值；本任务撤回 smart reminder 阈值控件，只保留内部策略。
- Interaction Impact: none  <!-- none | indirect | direct -->
- Interaction Freeze: N/A（仅 `interaction_impact != none` 时展开；展开后不得继续保留 `N/A/TBD`）
- Execution Safety Block: service_impact=仅限设置页 smart reminder 控件、locale 与相关文档；touches_running_service=no；backup_required=no；backup_plan=依赖 locale sync、npm test、typecheck、build 与 docs validator；rollback_plan=回退 `App.tsx`、locale、测试与 docs；destructive_operations=none；operator_approval_required=no；rationale=不改 host 状态机，不涉及线上服务、权限或数据迁移。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无
- Retention Decision: keep

## Notes
- 当前产品面向用户只保留 reminder mode 的二选一；阈值与等待上限不再进入设置心智。
