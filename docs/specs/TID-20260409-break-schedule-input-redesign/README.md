# Task-ID: TID-20260409-break-schedule-input-redesign

## Meta
- Title: 重构微休息与休息的预设和手动输入
- Date: 2026-04-09
- Level: moderate
- Lane: fast
- Execution Profile: single-task
- Status: DONE

## Links
- Plan (daily): ../../plans/2026-04-09.md
- Log (daily): ../../logs/2026-04-09.md
- Architecture Spec: ./arch.md
- UI Spec: ./ui.md
- Plan Summary: ./plan.md
- Test Plan: ./testplan.md

## Decision Log
- 四组节奏参数的 preset 统一为 5 个候选项。
- preset 与自定义输入不再冲突，输入框允许临时清空并在 blur/Enter 时提交。
- 将 preset/草稿提交 helper 收到 `apps/desktop/src/lib/settings-controls.ts`，便于测试。

## Governance Notes
- Requirement Brief: 用户指出微休息/休息的候选项不合理，且手动输入无法先清空再录入；本任务重构 preset 分布和输入交互，但不改设置自动保存模式。
- Interaction Impact: none
- Interaction Freeze: N/A
- Execution Safety Block: service_impact=仅限设置页节奏输入控件；touches_running_service=no；backup_required=no；backup_plan=以 Vitest、TypeScript、build 为边界；rollback_plan=回退 `App.tsx`、settings helper 与测试；destructive_operations=none；operator_approval_required=no；rationale=前端表单交互修复，不涉及数据迁移和外部副作用。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；问题边界集中在设置页节奏输入控件。
- Retention Decision: keep

## Notes
- `test/desktopSettingsControls.js` 现覆盖 preset 分布与数字草稿提交 helper。
