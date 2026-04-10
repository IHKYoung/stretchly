# Task-ID: TID-20260409-settings-next-cycle-apply-fix

## Meta
- Title: 修复设置更新只在下一轮休息生效
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
- `update_settings()` 不再无条件 `reset_schedule(now)`。
- 若当前存在进行中的 break 或已排队的下一次 break，则只更新 settings，不重置当前节奏。
- 新增 Rust 单测覆盖“当前 break 保持”和“已排队 break 保持”。

## Governance Notes
- Requirement Brief: 用户指出调整微休息/休息设置后，不应该打断当前 break 状态机或重置当前节奏；本任务修复 Rust 侧设置更新策略，但不改变 pause/focus/reset 等显式动作的语义。
- Interaction Impact: none
- Interaction Freeze: N/A
- Execution Safety Block: service_impact=本地 break 调度状态机；touches_running_service=no；backup_required=no；backup_plan=以 Rust tests、前端 tests/build 为边界；rollback_plan=回退 `state.rs` 与对应测试；destructive_operations=none；operator_approval_required=no；rationale=本地调度逻辑修复，不涉及外部系统和数据迁移。
- Approval Owner: orchestrator
- Execution Mode: single-agent-fallback
- Fallback Reason Code: other-blocked
- Escalation Summary: 无；问题直接定位到 `update_settings()` 的 schedule reset。
- Retention Decision: keep

## Notes
- 新增两条 Rust 单测，分别验证“当前 break 不被中断”和“已排队下一次 break 不被重排”。
